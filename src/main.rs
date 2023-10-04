#[macro_use]
extern crate log;
extern crate env_logger as logger;
use std::env;
use json_ld_utils::load_json_ld;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "DEBUG");
    logger::init();

    let loaded_json_ld = load_json_ld("https://dev-rwdb.srv.exdata.co.jp/api/v0/dataset/23/?format=json", 3, false).await;
    match loaded_json_ld {
        Ok(rwdataset) => {
            debug!("Full JSON-LD: {:?}", rwdataset);
            debug!("@type: {}", rwdataset.get("@type").unwrap());
            debug!("schema:distribution: {:?}", rwdataset.get("schema:distribution").unwrap());
        }
        Err(e) => {
            error!("Error occurred: {}", e);
        }
    }
    Ok(())
}
