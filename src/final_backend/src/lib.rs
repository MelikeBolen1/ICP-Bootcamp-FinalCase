use candid::{CandidType, Deserialize, Encode};
use ic_cdk::caller;
use ic_cdk::token::Cycles; // Ekledim
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 5000;

#[derive(CandidType, Deserialize)]
enum AuctionChoice {
    CreateAuctionItem, // Creates a new auction item
    PlaceBid(u64, u32), // Bids in auction (Gets item key and bid amount)
    EndAuction(u64), // Ends the auction (Get the item key)
    WithdrawBid(u64), // Withdraws bid from auction (Retrieves item key)
    IncreaseBid(u64, u32), // Bid raises in auction (Get item key and amount to be increased)
   
}

#[derive(CandidType, Deserialize)]
enum Choice {
    Approve,
    Reject,
    Pass,
}

#[derive(CandidType, Deserialize)]
enum VoteError {
    AlreadyVoted,
    ProposalIsNotActive,
    NoSuchProposal,
    AccessRejected,
    UpdateError,
}

#[derive(CandidType, Deserialize)]
struct Proposal {
    description: String,
    approve: u32,
    reject: u32,
    pass: u32,
    is_active: bool,
    voted: Vec<candid::Principal>,
    owner: candid::Principal,
}

#[derive(CandidType, Deserialize)]
struct CreateProposal {
    description: String,
    is_active: bool,
}

impl Storable for Proposal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Proposal {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(CandidType, Deserialize)]
struct AuctionItem {
    title: String,
    description: String,
    owner: candid::Principal,
    current_highest_bidder: Option<candid::Principal>,
    current_highest_bid: u32,
    is_active: bool,
}

#[derive(CandidType, Deserialize)]
struct CreateAuctionItem {
    title: String,
    description: String,
    start_price: u32,
    is_active: bool,
}

impl Storable for AuctionItem {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for AuctionItem {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(CandidType, Deserialize)]
enum AuctionError {
    UpdateError,
    NoSuchAuction,
    AuctionIsNotActive,
    Expired,
    AccessRejected,
    InvalidChoice,
}

#[derive(CandidType, Deserialize)]
enum BidError {
    BidAmountLessThanCurrent,
    UpdateError,
    NoSuchAuction,
    AuctionIsNotActive,
    Expired,
    ReachMaxBid,
    InvalidChoice,
    OwnerIsNotValid,
}

#[derive(CandidType, Deserialize)]
struct Bid {
    description: String,
    auction: u64, 
    owner: candid::Principal,
    currency: String,
    amount: u32,
    is_active: bool,
}

#[derive(CandidType, Deserialize)]
struct Item {
    title: String,
    description: String,
    owner: candid::Principal,
    new_owner: candid::Principal,
    currency: String,
    amount: u32,
    is_active: bool,
    start_time: String,
    end_time: String,
    bid: Vec<Bid>,
}

#[derive(CandidType, Deserialize)]
struct CreateBid {
    description: String,
    amount: u32,
    currency: String,
    is_active: bool,    
    owner: String,
    bid_amount: ic_cdk::token::Cycles, // New field that stores ICPT amount

}

#[derive(CandidType, Deserialize)]
struct CreateItem {
    title: String,
    description: String,
    is_active: bool,
    start_time: String,
    end_time: String,
    currency: String,
    amount: u32,
}

impl Storable for Item {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Item {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PROPOSAL_MAP: RefCell<StableBTreeMap<u64, Proposal, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        ));

    static AUCTION_MAP: RefCell<StableBTreeMap<u64, AuctionItem, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        ));

    static ITEM_MAP: RefCell<StableBTreeMap<u64, Item, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        ));
}

#[ic_cdk::query]
fn get_proposal(key: u64) -> Option<Proposal> {
    PROPOSAL_MAP.with(|p| p.borrow().get(&key))
}

#[ic_cdk::query]
fn get_auction_item(key: u64) -> Option<AuctionItem> {
    AUCTION_MAP.with(|p| p.borrow().get(&key))
}

#[ic_cdk::query]
fn get_item(key: u64) -> Option<Item> {
    ITEM_MAP.with(|i| i.borrow().get(&key))
}

// Gets the number of auction items.
#[ic_cdk::query]
fn get_auction_item_count() -> u64 {
    AUCTION_MAP.with(|p| p.borrow().len())
}

// Finds the item with the highest bid in the auction.
#[ic_cdk::query]
fn find_most_bidded_item() -> Option<AuctionItem> {
    AUCTION_MAP.with(|p| {
        let mut most_bidded_item: Option<AuctionItem> = None;
        let mut highest_bid_count = 0;

        for (_key, item) in p.borrow().iter() {
            if item.is_active {
                let bid_count = item.current_highest_bidder.as_ref().map_or(0, |_| 1);
                if bid_count > highest_bid_count {
                    highest_bid_count = bid_count;
                    most_bidded_item = Some(item.clone());
                }
            }
        }

        most_bidded_item
    })
}

//
it's bids at the auction.
#[ic_cdk::update]
fn place_bid(choice: AuctionChoice) -> Result<(), BidError> {
    match choice {
        AuctionChoice::PlaceBid(key, amount) => {
            AUCTION_MAP.with(|p| {
                let item_option = p.borrow().get(&key);
                let mut item = match item_option {
                    Some(value) => value,
                    None => return Err(BidError::NoSuchAuction),
                };

                if !item.is_active {
                    return Err(BidError::AuctionIsNotActive);
                }

                if item.current_highest_bidder == Some(caller()) {
                    return Err(BidError::AlreadyHighestBidder);
                }

                if amount <= item.current_highest_bid {
                    return Err(BidError::BidAmountLessThanCurrent);
                }

                item.current_highest_bidder = Some(caller());
                item.current_highest_bid = amount;

                let res = p.borrow_mut().insert(key, item);

                match res {
                    Some(_) => Ok(()),
                    None => Err(BidError::UpdateError),
                }
            })
        }
        _ => Err(BidError::InvalidChoice),
    }
}

// Withdraws the bid from the auction.
#[ic_cdk::update]
fn withdraw_bid(choice: AuctionChoice) -> Result<(), BidError> {
    match choice {
        AuctionChoice::WithdrawBid(key) => {
            AUCTION_MAP.with(|p| {
                let item_option = p.borrow().get(&key);
                let mut item = match item_option {
                    Some(value) => value,
                    None => return Err(BidError::NoSuchAuction),
                };

                if !item.is_active {
                    return Err(BidError::AuctionIsNotActive);
                }

                if item.current_highest_bidder != Some(caller()) {
                    return Err(BidError::NotBidder);
                }

                // Withdraw your offer here.
                

                let res = p.borrow_mut().insert(key, item);

                match res {
                    Some(_) => Ok(()),
                    None => Err(BidError::UpdateError),
                }
            })
        }
        _ => Err(BidError::InvalidChoice),
    }
}

// Increases bidding in auction.
#[ic_cdk::update]
fn increase_bid(choice: AuctionChoice) -> Result<(), BidError> {
    match choice {
        AuctionChoice::IncreaseBid(key, amount) => {
            AUCTION_MAP.with(|p| {
                let item_option = p.borrow().get(&key);
                let mut item = match item_option {
                    Some(value) => value,
                    None => return Err(BidError::NoSuchAuction),
                };

                if !item.is_active {
                    return Err(BidError::AuctionIsNotActive);
                }

                if item.current_highest_bidder != Some(caller()) {
                    return Err(BidError::NotBidder);
                }

                // Increase bid here.
               

                let res = p.borrow_mut().insert(key, item);

                match res {
                    Some(_) => Ok(()),
                    None => Err(BidError::UpdateError),
                }
            })
        }
        _ => Err(BidError::InvalidChoice),
    }
}



// It ends the auction and determines the bidder with the highest bid.
#[ic_cdk::update]
fn end_auction(choice: AuctionChoice) -> Result<(), AuctionError> {
    match choice {
        AuctionChoice::EndAuction(key) => {
            AUCTION_MAP.with(|p| {
                let item_option = p.borrow().get(&key);
                let mut item = match item_option {
                    Some(value) => value,
                    None => return Err(AuctionError::NoSuchAuction),
                };

                if caller() != item.owner {
                    return Err(AuctionError::AccessRejected);
                }

                if !item.is_active {
                    return Err(AuctionError::AuctionIsNotActive);
                }

                let highest_bidder = match item.current_highest_bidder.clone() {
                    Some(value) => value,
                    None => return Err(AuctionError::NoBids),
                };

                item.is_active = false;
                item.owner = highest_bidder;

                let res = p.borrow_mut().insert(key, item);

                match res {
                    Some(_) => Ok(()),
                    None => Err(AuctionError::UpdateError),
                }
            })
        }
        _ => Err(AuctionError::InvalidChoice),
    }
}



#[ic_cdk::update]
fn create_bid(create_bid: CreateBid) -> Result<(), BidError> {
    let bid_amount = create_bid.bid_amount;

    // Check if user has enough ICPT and make payment
    if ic_cdk::token::balance(&caller()) < bid_amount.get_cycles() {
        return Err(BidError::NotEnoughICPT);
    }

    // Deduct the bid amount from the user who submitted the ICPT
    let transfer_result = ic_cdk::token::transfer_to_self(bid_amount);

    match transfer_result {
        Ok(_) => {
            // Create the offer
            // For example, storing the quote or saving it to a database
            
           let new_bid = Bid {
                description: create_bid.description,
                auction: create_bid.auction,
                owner: caller(), // Teklifin sahibi çağırıcıdır
                currency: create_bid.currency,
                amount: create_bid.amount,
                is_active: create_bid.is_active,
            };

            // Add the offer to the database
            AUCTION_MAP.with(|p| {
                let mut auction_map = p.borrow_mut();
                let new_bid_id = generate_unique_bid_id(); // Teklif için benzersiz bir kimlik oluşturun
                auction_map.insert(new_bid_id, new_bid);
            });


            Ok(())
        }
        Err(_) => Err(BidError::TransferError),
    }
}

// An example function to generate a unique quote ID
fn generate_unique_bid_id() -> u64 {
     // You can customize this function according to the application
    // For example, you can generate a random ID
   // For now we are just returning a sample value

     12345
}


