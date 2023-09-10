#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::export::candid::{CandidType, Decode};
    use ic_cdk::export::Principal;
    use ic_cdk::test;

    #[test]
    fn test_create_proposal() {
        // Proposal creation test
        let description = "Test Proposal".to_string();
        let is_active = true;
        let proposal = CreateProposal {
            description: description.clone(),
            is_active,
        };

        // Save proposal
        let proposal_key = 1u64;
        let result = create_proposal(proposal.clone());

        // Check if the Proposal has been registered correctly
        assert!(result.is_ok());

        // Receive and verify Proposal
        let stored_proposal = get_proposal(proposal_key).unwrap();
        assert_eq!(stored_proposal.description, description);
        assert_eq!(stored_proposal.is_active, is_active);
    }

    #[test]
    fn test_place_bid() {
        // To test the PlaceBid process, first create an AuctionItem and Proposal
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

        // Save AuctionItem and Proposal
        create_auction_item(auction_item.clone());
        create_proposal(proposal.clone());

        // Test an offer placement
        let bid_amount = 150;
        let place_bid_choice = AuctionChoice::PlaceBid(item_key, bid_amount);

        // Place offer
        let result = place_bid(place_bid_choice.clone());

        // Verify that the quote is placed correctly

        assert!(result.is_ok());

        // Check AuctionItem is updated and highest bid placed

        let updated_item = get_auction_item(item_key).unwrap();
        assert_eq!(updated_item.current_highest_bid, bid_amount);
        assert_eq!(updated_item.current_highest_bidder, Some(caller()));

        // Test bid retraction
        let withdraw_bid_choice = AuctionChoice::WithdrawBid(item_key);

        // Withdraw offer
        let withdraw_result = withdraw_bid(withdraw_bid_choice.clone());

        // Verify that the bid was withdrawn correctly
        assert!(withdraw_result.is_ok());

        // Check AuctionItem is updated and bid is withdrawn
        let withdrawn_item = get_auction_item(item_key).unwrap();
        assert_eq!(withdrawn_item.current_highest_bid, 0);
        assert_eq!(withdrawn_item.current_highest_bidder, None);
    }

    
}
