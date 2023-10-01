use ethers::prelude::*;
use ethers_contract::prelude::*;

#[eth_abi(ActionTracker)]
trait ActionTracker {
    fn create_action(&mut self, title: String, description: String, status: String);

    fn get_action(&self, index: U256) -> (String, String, String);

    fn get_action_count(&self) -> U256;

    fn update_action(&mut self, index: U256, title: String, description: String, status: String);
}

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to an Ethereum node
    let provider = Provider::<Http>::connect("http://localhost:8545").await?;
    let wallet = Wallet::new(Default::default(), provider);

    // Deploy the contract
    let factory = ContractFactory::new(
        include_bytes!("ActionTracker.json"),
        wallet.clone(),
    );
    let contract = factory.deploy(()).await?;
    println!("Contract deployed to: {}", contract.address());

    // Interact with the contract
    let mut action_tracker = contract.connect(wallet);

    action_tracker
        .create_action("Task 1".to_string(), "Description 1".to_string(), "Pending".to_string())
        .send()
        .await?;

    let (title, description, status) = action_tracker.get_action(0.into()).call().await?;
    println!("Action: Title={}, Description={}, Status={}", title, description, status);

    action_tracker
        .update_action(0.into(), "Task 1 Updated".to_string(), "Updated Description".to_string(), "Completed".to_string())
        .send()
        .await?;

    let (updated_title, updated_description, updated_status) = action_tracker.get_action(0.into()).call().await?;
    println!("Updated Action: Title={}, Description={}, Status={}", updated_title, updated_description, updated_status);

    Ok(())
}
