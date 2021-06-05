use algonaut_client::indexer::v2::message::QueryAccount;
use algonaut_client::Indexer;
use dotenv::dotenv;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer = Indexer::new()
        .bind(env::var("INDEXER_URL")?.as_ref())
        .client_v2()?;

    // query Account using default query parameters (all None).
    let accounts = indexer.accounts(&QueryAccount::default())?.accounts;
    println!("found {} accounts", accounts.len());

    // query Applications with custom query parameters.
    let mut accounts_query = QueryAccount::default();
    accounts_query.limit = Some(2); // why 2? see: https://github.com/algorand/indexer/issues/516

    let accounts = indexer.accounts(&accounts_query)?.accounts;
    println!("found {} accounts", accounts.len());

    Ok(())
}

