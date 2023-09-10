# ICP-Bootcamp-FinalCase



This project is developed using Rust programming language and runs on the Internet Computer (ICP) to create a Web3 Auction Decentralized Application (DApp). With this DApp, users can perform the following actions:

- List items for auction
- Place bids on items
- Update the listing of an item
- End an item listing

Key Features:

- Listing and Bidding: Users can list items and place bids on them. Bids are tracked in a StableBTreeMap, providing transparent records of who bid how much.

- Editing and Stopping Listings: Item owners can edit the listing or stop it at any time. When a listing is stopped, the highest bidder becomes the owner.

- Item Management: The project allows the management of a list of items. You can query specific items, the list of all items, the total number of items listed, the item sold for the highest price, and the item with the most bids.

- Security Checks: Basic security checks are implemented to ensure that only the listing owner can update or stop it.

  ## Changes and Additions

You can find the changes and additions made to this project below:

Token Integration: Token functionality has been added to the project. Users can now make bids using ICPT (Internet Computer Principal Token).

- The `CreateBid` structure has been created and used to store ICPT amounts.
- ICPT token functionalities have been integrated for bid creation and bid management.
- Necessary data structures for storing and managing bids have been added.

Unique Bid Identities: A custom function (`generate_unique_bid_id`) has been added to create unique identities for bids. These identities are used to ensure the uniqueness of bids.

README Updates: The README file has been updated to provide information about the project's new functionality.

## Getting Started

To get started with this project, follow these steps:

1. Clone this repository to your local development environment.

2. Configure your development environment to work with ICP and Rust according to the provided documentation.

3. Customize the project as needed and start adding Rust code to implement your specific functionalities.

## Testing Requirements

Make sure to include comprehensive test cases to validate the correctness of your Rust code. Proper testing is essential to ensure the reliability of your smart contract.

## Conclusion

Congratulations on completing this Web3 Auction DApp project developed in Rust and running on the Internet Computer (ICP). It should have been an excellent learning experience for Rust and ICP development. If you have any questions or feedback, please don't hesitate to reach out.




To learn more before you start working with final, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/tutorials/developer-journey/)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install/)
- [Rust Canister Devlopment Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk/latest/ic_cdk/)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros/latest/ic_cdk_macros/)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.icp0.io/)


 If you want to start working on your project right away, you might want to try the following commands:
 ```
 cd final/
dfx help
dfx canister --help

```

# Running the project locally

If you want to test your project locally, you can use the following commands:
 ```
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
 ```

Once the job completes, your application will be available at http://localhost:4943?canisterId={asset_canister_id}.

If you have made changes to your backend canister, you can generate a new candid interface with
```
npm run generate
```
at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run dfx deploy.

If you are making frontend changes, you can start a development server with

 ```
npm start
 ```

Which will start a server at http://localhost:8080, proxying API requests to the replica at port 4943.



# Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

setDFX_NETWORK to production if you are using Webpack

- use your own preferred method to replace process.env.DFX_NETWORK in the autogenerated declarations
*  Setting canisters -> {asset_canister_id} -> declarations -> env_override to a string in dfx.json will replace process.env.DFX_NETWORK with the string in the autogenerated declarations
- Write your own createActor constructor


