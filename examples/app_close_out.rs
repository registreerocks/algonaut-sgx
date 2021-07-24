use algonaut::algod::AlgodBuilder;
use algonaut::transaction::TxnBuilder;
use algonaut_transaction::account::Account;
use algonaut_transaction::builder::CloseApplication;
use dotenv::dotenv;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let algod = AlgodBuilder::new()
        .bind(env::var("ALGOD_URL")?.as_ref())
        .auth(env::var("ALGOD_TOKEN")?.as_ref())
        .build_v2()?;

    let sender = Account::from_mnemonic("auction inquiry lava second expand liberty glass involve ginger illness length room item discover ahead table doctor term tackle cement bonus profit right above catch")?;

    let params = algod.suggested_transaction_params().await?;
    // to test this, create an application that sets local state and opt-in, for/with the account sending this transaction.
    // the approval program has to return success for the local state to be cleared.
    let t = TxnBuilder::with(params, CloseApplication::new(sender.address(), 5).build()).build();

    let signed_t = sender.sign_transaction(&t)?;

    let send_response = algod.broadcast_signed_transaction(&signed_t).await?;
    println!("response: {:?}", send_response);

    Ok(())
}
