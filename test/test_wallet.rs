use turnkey::{errors::TurnkeyResult, KeySelector, Turnkey};
use turnkey::client::Turnkey;
use dotenv::dotenv;
use mockito::Server;

// Testing
#[cfg(test)]
mod test {

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_example_wallet() -> TurnkeyResult<()> {
        dotenv().ok();
        let mut server = mockito::Server::new();

        let turnkey_client = Turnkey::new();

        Ok(())
    }

}