#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::export::candid::{CandidType, Decode};
    use ic_cdk::export::Principal;
    use ic_cdk::test;

    #[test]
    fn test_create_proposal() {
        let description = "Test Proposal".to_string();
        let is_active = true;
        let proposal = CreateProposal {
            description: description.clone(),
            is_active,
        };

        if let Err(error) = create_proposal(proposal.clone()) {
            panic!("Error creating proposal: {:?}", error);
        }

        let proposal_key = 1u64;
        match get_proposal(proposal_key) {
            Ok(stored_proposal) => {
                assert_eq!(stored_proposal.description, description);
                assert_eq!(stored_proposal.is_active, is_active);
            }
            Err(error) => panic!("Error retrieving proposal: {:?}", error),
        }
    }

    #[test]
    fn test_place_bid() {
        let item_key = 1u64;
        let auction_item = CreateAuctionItem {
            title: "Test Item".to_string(),
            description: "Test Description".to_string(),
            start_price: 100,
            is_active: true,
        };
        let proposal_key = 1u64;
        let proposal = CreateProposal {
            description: "Test Proposal".to_string(),
            is_active: true,
        };

        if let Err(error) = create_auction_item(auction_item.clone()) {
            panic!("Error creating auction item: {:?}", error);
        }

        if let Err(error) = create_proposal(proposal.clone()) {
            panic!("Error creating proposal: {:?}", error);
        }

        let bid_amount = 150;
        let place_bid_choice = AuctionChoice::PlaceBid(item_key, bid_amount);

        if let Err(error) = place_bid(place_bid_choice.clone()) {
            panic!("Error placing bid: {:?}", error);
        }

        match get_auction_item(item_key) {
            Ok(updated_item) => {
                assert_eq!(updated_item.current_highest_bid, bid_amount);
                assert_eq!(updated_item.current_highest_bidder, Some(caller()));
            }
            Err(error) => panic!("Error retrieving auction item: {:?}", error),
        }

        let withdraw_bid_choice = AuctionChoice::WithdrawBid(item_key);

        if let Err(error) = withdraw_bid(withdraw_bid_choice.clone()) {
            panic!("Error withdrawing bid: {:?}", error);
        }

        match get_auction_item(item_key) {
            Ok(withdrawn_item) => {
                assert_eq!(withdrawn_item.current_highest_bid, 0);
                assert_eq!(withdrawn_item.current_highest_bidder, None);
            }
            Err(error) => panic!("Error retrieving auction item: {:?}", error),
        }
    }

    // Rest of your code...
}
