#[macro_use]
extern crate log;
extern crate env_logger as logger;
use json_ld_utils::{load_json_ld, scan_json_ld_obj, AT_TYPE, SC_NAME, SC_DISTRIBUTION, DBP_CRON_CONFIG, DBP_MOVE_CONFIG, DBP_MOVE_FROM, DBP_MOVE_TO};
use serde_json::Value;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  env::set_var("RUST_LOG", "DEBUG");
  logger::init();

  // Load a JSON-LD from specified URL and follow URLs in JSON-LD until the depth of 3.
  let loaded_json_ld = load_json_ld("https://dev-rwdb.srv.exdata.co.jp/api/v0/dataset/23/?format=json", 3, false).await;
  match loaded_json_ld {
    Ok(json_ld) => {
      info!("Full JSON-LD: {:?}", json_ld);
      info!("{}: {}", AT_TYPE, json_ld.get(AT_TYPE).unwrap());
      info!("{}: {}", SC_NAME, json_ld.get(SC_NAME).unwrap());
      info!("{}: {:?}", SC_DISTRIBUTION, json_ld.get(SC_DISTRIBUTION).unwrap());
    }
    Err(e) => {
      error!("Error occurred: {}", e);
    }
  }

  // Load a JSON-LD Object `json_val` (parsed from `json_str`) and follow URLs in JSON-LD until the depth of 4.
  let json_str = "{\"@id\":\"https://dev-rwdb.srv.exdata.co.jp/api/v0/periodic_move_configs/27/?format=json\",\"@type\":\"dbp:RealWorldDataPeriodicBrewingConfig\"}";
  match serde_json::from_str::<Value>(json_str) {
    Ok(json_val) => {
      if let Some(json_ld_shared) = json_val.as_object() {
        let mut json_ld = json_ld_shared.to_owned();
        scan_json_ld_obj(&mut json_ld, 4, false).await;
        info!("Full JSON-LD: {:?}", json_ld);
        info!("{}: {}", AT_TYPE, json_ld[AT_TYPE].as_str().unwrap());
        info!("{}: {}", SC_NAME, json_ld[SC_NAME].as_str().unwrap());
        info!("{}: {}", DBP_CRON_CONFIG, json_ld[DBP_CRON_CONFIG].as_str().unwrap());
        info!("{} > {}: {:?}", DBP_MOVE_CONFIG, DBP_MOVE_FROM, json_ld.get(DBP_MOVE_CONFIG).unwrap().get(DBP_MOVE_FROM).unwrap());
        info!("{} > {}: {:?}", DBP_MOVE_CONFIG, DBP_MOVE_TO, json_ld.get(DBP_MOVE_CONFIG).unwrap().get(DBP_MOVE_TO).unwrap());
        } else {
        error!("Failed to process JSON-LD: {json_str}");
      }
    }
    Err(e) => {
      error!("Error occurred: {}", e);
    }
  }

  Ok(())
}
