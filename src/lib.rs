#[macro_use]
extern crate log;
extern crate url;

use chrono::{Local, DateTime, Duration, TimeZone, Datelike, Timelike};
use url::Url;
use serde_json::{Value, Map};
use async_recursion::async_recursion;

// JSON-LD @ Keys
pub const AT_CONTEXT: &'static str = "@context";
pub const AT_ID:      &'static str = "@id";
pub const AT_REF:     &'static str = "@ref";
pub const AT_TYPE:    &'static str = "@type";

// Frequently used schema.org Keys
pub const SC_NAME:                    &'static str = "schema:name";
pub const SC_URL:                     &'static str = "schema:url";
pub const SC_DISTRIBUTION:            &'static str = "schema:distribution";
pub const SC_AUTHOR:                  &'static str = "schema:author";
pub const SC_CONTENT_LOCATION:        &'static str = "schema:contentLocation";
pub const SC_DATE_CREATED:            &'static str = "schema:dateCreated";
pub const SC_DATE_MODIFIED:           &'static str = "schema:dateModified";
pub const SC_DATE_PUBLISHED:          &'static str = "schema:datePublished";
pub const SC_LICENSE:                 &'static str = "schema:license";
pub const SC_LOCATION_CREATED:        &'static str = "schema:locationCreated";
pub const SC_DESCRIPTION:             &'static str = "schema:description";
pub const SC_VALUE:                   &'static str = "schema:value";
pub const SC_START_TIME:              &'static str = "schema:startTime";
pub const SC_END_TIME:                &'static str = "schema:endTime";
pub const SC_STATUS:                  &'static str = "schema:status";
pub const SC_CONTENT_REFERENCE_TIME:  &'static str = "schema:contentReferenceTime";

// Frequently used schema.org Types
pub const SC_ENTRY_POINT:  &'static str = "schema:EntryPoint";
pub const SC_THING:        &'static str = "schema:Thing";

// DBP Keys
pub const DBP_STRUCTURE_INFO:             &'static str = "dbp:structureInfo";
pub const DBP_GENERATED_FROM:             &'static str = "dbp:generatedFrom";
pub const DBP_GENERATED_USING:            &'static str = "dbp:generatedUsing";
pub const DBP_GENERATED_ARGS:             &'static str = "dbp:generatedArgs";
pub const DBP_COLLECTION_INFO:            &'static str = "dbp:collectionInfo";
pub const DBP_KEY:                        &'static str = "dbp:key";
pub const DBP_INPUT_TYPE:                 &'static str = "dbp:inputType";
pub const DBP_INPUT_CHARACTERISTIC:       &'static str = "dbp:inputCharacteristic";
pub const DBP_DATASET:                    &'static str = "dbp:dataset";
pub const DBP_OUTPUT_TYPE:                &'static str = "dbp:outputType";
pub const DBP_OUTPUT_CHARACTERISTIC:      &'static str = "dbp:outputCharacteristic";
pub const DBP_ARGUMENT_TYPE:              &'static str = "dbp:argumentType";
pub const DBP_INPUT_SPECS:                &'static str = "dbp:inputSpecs";
pub const DBP_OUTPUT_SPECS:               &'static str = "dbp:outputSpecs";
pub const DBP_ARG_SPECS:                  &'static str = "dbp:argSpecs";
pub const DBP_CONVERSION_CHARACTERISTIC:  &'static str = "dbp:conversionCharacteristic";
pub const DBP_COLLECTION_STYLE:           &'static str = "dbp:collectionStyle";
pub const DBP_COLLECTION_PROTOCOL:        &'static str = "dbp:collectionProtocol";
pub const DBP_LISTEN_ADDRESS:             &'static str = "dbp:listenAddress";
pub const DBP_SERVER_ADDRESS:             &'static str = "dbp:serverAddress";
pub const DBP_ENTRY_POINT_KEY:            &'static str = "dbp:entryPoint";
pub const DBP_STRUCTURE:                  &'static str = "dbp:structure";
pub const DBP_GRAPHQL_SCHEMA:             &'static str = "dbp:graphqlSchema";
pub const DBP_BASE_URL:                   &'static str = "dbp:baseUrl";
pub const DBP_PATTERN:                    &'static str = "dbp:pattern";
pub const DBP_ACTIVE_CONNECTIONS:         &'static str = "dbp:activeConnections";
pub const DBP_TRAFFIC_STATISTICS:         &'static str = "dbp:trafficStatistics";
pub const DBP_BREWER_INFO:                &'static str = "dbp:brewerInfo";
pub const DBP_BREWER_INPUT:               &'static str = "dbp:brewerInput";
pub const DBP_BREWER_OUTPUT:              &'static str = "dbp:brewerOutput";
pub const DBP_BREWING_ARGUMENT:           &'static str = "dbp:brewingArgument";
pub const DBP_BREWER_OUTPUT_STORE:        &'static str = "dbp:brewerOutputStore";
pub const DBP_TIME_PERIOD_START:          &'static str = "dbp:timePeriodStart";
pub const DBP_TIME_PERIOD_END:            &'static str = "dbp:timePeriodEnd";
pub const DBP_BREWING_CONFIG:             &'static str = "dbp:brewingConfig";
pub const DBP_CRON_CONFIG:                &'static str = "dbp:cronConfig";
pub const DBP_SPARQL_QUERY:               &'static str = "dbp:sparqlQuery";
pub const DBP_GRAPHQL_QUERY:              &'static str = "dbp:graphqlQuery";
pub const DBP_MOVE_FROM:                  &'static str = "dbp:moveFrom";
pub const DBP_MOVE_TO:                    &'static str = "dbp:moveTo";
pub const DBP_MOVED_DATASET:              &'static str = "dbp:movedDataset";
pub const DBP_MOVE_CONFIG:                &'static str = "dbp:moveConfig";
pub const DBP_REMOVE_CONFIG:              &'static str = "dbp:removeConfig";

// DBP Types
pub const DBP_RW_DATASET:                   &'static str = "dbp:RealWorldDataset";
pub const DBP_RWD_BREWER_INPUT:             &'static str = "dbp:RealWorldDataBrewerInput";
pub const DBP_RWD_BREWER_OUTPUT:            &'static str = "dbp:RealWorldDataBrewerOutput";
pub const DBP_RWD_BREWING_ARGUMENT:         &'static str = "dbp:RealWorldDataBrewingArgument";
pub const DBP_RWD_BREWER_INFO:              &'static str = "dbp:RealWorldDataBrewerInfo";
pub const DBP_RWD_COLLECTION_INFO:          &'static str = "dbp:RealWorldDataCollectionInfo";
pub const DBP_RWD_STRUCTURE_INFO:           &'static str = "dbp:RealWorldDataStructureInfo";
pub const DBP_RWD_STORING_INFO:             &'static str = "dbp:RealWorldDataStoringInfo";
pub const DBP_RWD_REGISTER_DEMAND:          &'static str = "dbp:RealWorldDataRegisterDemand";
pub const DBP_RWD_REGISTER_SUPPLY:          &'static str = "dbp:RealWorldDataRegisterSupply";
pub const DBP_RWD_COLLECTION_DEMAND:        &'static str = "dbp:RealWorldDataCollectionDemand";
pub const DBP_RWD_COLLECTION_SUPPLY:        &'static str = "dbp:RealWorldDataCollectionSupply";
pub const DBP_RWD_COLLECTION_STATUS:        &'static str = "dbp:RealWorldDatCollectionStatus";
pub const DBP_RWD_BREWING_DEMAND:           &'static str = "dbp:RealWorldDataBrewingDemand";
pub const DBP_RWD_BREWING_SUPPLY:           &'static str = "dbp:RealWorldDataBrewingSupply";
pub const DBP_RWD_PERIODIC_BREWING_CONFIG:  &'static str = "dbp:RealWorldDataPeriodicBrewingConfig";
pub const DBP_RWD_READ_DEMAND:              &'static str = "dbp:RealWorldDataReadDemand";
pub const DBP_RWD_READ_SUPPLY:              &'static str = "dbp:RealWorldDataReadSupply";
pub const DBP_RWD_WRITE_DEMAND:             &'static str = "dbp:RealWorldDataWriteDemand";
pub const DBP_RWD_WRITE_SUPPLY:             &'static str = "dbp:RealWorldDataWriteSupply";
pub const DBP_RWD_MOVE_DEMAND:              &'static str = "dbp:RealWorldDataMoveDemand";
pub const DBP_RWD_MOVE_SUPPLY:              &'static str = "dbp:RealWorldDataMoveSupply";
pub const DBP_RWD_PERIODIC_MOVE_CONFIG:     &'static str = "dbp:RealWorldDataPeriodicMoveConfig";
pub const DBP_RWD_REMOVE_DEMAND:            &'static str = "dbp:RealWorldDataRemoveDemand";
pub const DBP_RWD_REMOVE_SUPPLY:            &'static str = "dbp:RealWorldDataRemoveSupply";
pub const DBP_RWD_PERIODIC_REMOVE_CONFIG:   &'static str = "dbp:RealWorldDataPeriodicRemoveConfig";



#[async_recursion]
pub async fn load_json_ld(url: &str, depth: u8, load_context: bool) -> Result<Map<String, Value>, Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let resp = client.get(url).send().await?;
  let body = resp.text().await?;

  match serde_json::from_str::<Value>(body.as_str()) {
    Ok(json_val) => {
      if let Some(json_ld_shared) = json_val.as_object() {
        let mut json_ld = json_ld_shared.to_owned();
        scan_json_ld_obj(&mut json_ld, depth, load_context).await;
        Ok(json_ld)
      } else {
        Err(format!("Failed to parse JSON-LD from given url.\nurl: {url}").into())
      }
    }
    Err(e) => {
      Err(format!("Failed to parse JSON from given url (maybe NON-JSON file).\nurl: {url}\nerror: {e}").into())
    }
  }
}

#[async_recursion]
pub async fn scan_json_ld_obj(obj: &mut Map<String, Value>, depth: u8, load_context: bool) {
  if depth == 0 {
    return;
  }
  if obj.len() < 3 {  // possibly ref to another json-ld
    let keys: Vec<String> = obj.keys().cloned().collect();
    for key in keys {
      if key == AT_ID || key == AT_REF {
        let url = obj.get(&key).unwrap().as_str().unwrap();
        let json_ld = load_json_ld(url, depth - 1, load_context).await;
        match json_ld {
          Ok(mut new_obj) => {
            obj.append(&mut new_obj);
          }
          Err(e) => {
            debug!("Error occurred: {}", e);
          }
        }
      }
    }
  }
  // check children recursive
  let keys: Vec<String> = obj.keys().cloned().collect();
  for key in keys {
    if !load_context && key == AT_CONTEXT {
      continue;
    }
    if key != AT_ID && key != AT_REF {
      let val = obj.get_mut(&key).unwrap();
      if val.is_array() {
        scan_json_ld_array(val.as_array_mut().unwrap(), depth - 1, load_context).await;
      }
      if val.is_object() {
        scan_json_ld_obj(val.as_object_mut().unwrap(), depth - 1, load_context).await;
      }
      if val.is_string() {
        let val_str = val.as_str().unwrap();
        if is_valid_url(val_str) && key != AT_TYPE {
          let json_ld = load_json_ld(val_str, depth - 1, load_context).await;
          match json_ld {
            Ok(new_obj) => {
              obj.remove(&key);
              obj.insert(key, Value::from(new_obj));
            }
            Err(e) => {
              debug!("Maybe NOT JSON-LD: {} {}", val_str, e);
            }
          }
        }
      }
    }
  }

}

#[async_recursion]
pub async fn scan_json_ld_array(arr: &mut Vec<Value>, depth: u8, load_context: bool) {
  if depth == 0 {
    return;
  }
  // Move all arr's elements to orig_arr
  let orig_arr: &mut Vec<Value> = &mut vec![];
  orig_arr.append(arr);
  // Push new vals to arr 
  for val in orig_arr {
    if val.is_array() {
      scan_json_ld_array(val.as_array_mut().unwrap(), depth - 1, load_context).await;
      arr.push(val.clone());
    } else if val.is_object() {
      scan_json_ld_obj(val.as_object_mut().unwrap(), depth - 1, load_context).await;
      arr.push(val.clone());
    } else if val.is_string() {
      let val_str = val.as_str().unwrap();
      if is_valid_url(val_str) {
        let json_ld = load_json_ld(val_str, depth - 1, load_context).await;
        match json_ld {
          Ok(new_obj) => {           
            arr.push(Value::from(new_obj));
          }
          Err(e) => {
            debug!("Maybe NOT JSON-LD: {} {}", val_str, e);
          }
        }
      } else {
        arr.push(val.clone());
      }
    } else {
      arr.push(val.clone());
    }
  }
}

fn is_valid_url(test_url: &str) -> bool {
  Url::parse(test_url).is_ok()
}

#[async_recursion]
pub async fn update_json_ld(url: &str, json_ld: &Map<String, Value>) -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let body = serde_json::to_string(json_ld).unwrap();
  let resp = client.put(url).header("Content-Type", "application/json").body(body).send().await?;
  debug!("Response: {}", resp.text().await?);
  Ok(())
}


pub const MINUS: &'static str = "-";
pub const SEC: &'static str = "S";
pub const MIN: &'static str = "M";
pub const HOUR: &'static str = "H";
pub const DAY: &'static str = "d";
// pub const MONTH: &'static str = "m";
// pub const YEAR: &'static str = "Y";
pub const F_SEC: &'static str = "%S";
pub const F_MIN: &'static str = "%M";
pub const F_HOUR: &'static str = "%H";
pub const F_DAY: &'static str = "%d";
// pub const F_MONTH: &'static str = "%m";
// pub const F_YEAR: &'static str = "%Y";

pub fn calc_dt_duration(tp_start: &str, tp_end: &str) -> (DateTime<Local>, DateTime<Local>) {
  let dt_now = Local::now();
  let mut dt_start: DateTime<Local> = Local::now();
  let mut dt_end: DateTime<Local> = Local::now();
  match tp_start {
    _ if tp_start.starts_with(MINUS) && tp_start.ends_with(SEC) => {    // 秒単位のデータ移動
      let dt_base = Local.with_ymd_and_hms(dt_now.year(), dt_now.month(), dt_now.day(), dt_now.hour(), dt_now.minute(), dt_now.second()).unwrap();
      dt_start = dt_base - Duration::seconds(tp_start.replace(MINUS, "").replace(SEC, "").parse::<i64>().unwrap());
      dt_end = dt_base - Duration::seconds(tp_end.replace(MINUS, "").replace(SEC, "").parse::<i64>().unwrap());
    },
    _ if tp_start.starts_with(MINUS) && tp_start.ends_with(MIN) => {    // 分単位のデータ移動
      let dt_base = Local.with_ymd_and_hms(dt_now.year(), dt_now.month(), dt_now.day(), dt_now.hour(), dt_now.minute(), 0).unwrap();
      dt_start = dt_base - Duration::minutes(tp_start.replace(MINUS, "").replace(MIN, "").parse::<i64>().unwrap());
      dt_end = dt_base - Duration::minutes(tp_end.replace(MINUS, "").replace(MIN, "").parse::<i64>().unwrap());
    },
    _ if tp_start.starts_with(MINUS) && tp_start.ends_with(HOUR) => {   // 時間単位のデータ移動
      let dt_base = Local.with_ymd_and_hms(dt_now.year(), dt_now.month(), dt_now.day(), dt_now.hour(), 0, 0).unwrap();
      dt_start = dt_base - Duration::hours(tp_start.replace(MINUS, "").replace(HOUR, "").parse::<i64>().unwrap());
      dt_end = dt_base - Duration::hours(tp_end.replace(MINUS, "").replace(HOUR, "").parse::<i64>().unwrap());
    },
    _ if tp_start.starts_with(MINUS) && tp_start.ends_with(DAY) => {    // 日単位のデータ移動
      let dt_base = Local.with_ymd_and_hms(dt_now.year(), dt_now.month(), dt_now.day(), 0, 0, 0).unwrap();
      dt_start = dt_base - Duration::days(tp_start.replace(MINUS, "").replace(DAY, "").parse::<i64>().unwrap());
      dt_end = dt_base - Duration::days(tp_end.replace(MINUS, "").replace(DAY, "").parse::<i64>().unwrap());
    },
    _ => {  // 具体的な日時指定
      dt_start = tp_start.parse().expect(format!("Failed to parse tp_start {tp_start}").as_str());
      dt_end = tp_end.parse().expect(format!("Failed to parse tp_end {tp_end}").as_str());
    },
  };
  debug!("calc_dt_duration: now: {}, range: {} ... {}", dt_now, dt_start, dt_end);
  (dt_start, dt_end)
}

pub fn calc_dt_step(path_template: &str) -> Duration {
  match path_template {
    _ if path_template.contains(F_SEC) => {     // 秒単位のデータ移動
      Duration::seconds(1)
    },
    _ if path_template.contains(F_MIN) => {     // 分単位のデータ移動
      Duration::minutes(1)
    },
    _ if path_template.contains(F_HOUR) => {    // 時間単位のデータ移動
      Duration::hours(1)
    },
    _ if path_template.contains(F_DAY) => {     // 日単位のデータ移動
      Duration::days(1)
    },
    _ => {                                      // 不明 → 秒単位にフォールバック…
      Duration::seconds(1)
    },
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn it_works() {
    // Loader Test
    let loaded_json_ld = load_json_ld("https://dev-rwdb.srv.exdata.co.jp/api/v0/dataset/23/?format=json", 3, false).await;
    assert_eq!(loaded_json_ld.is_ok(), true);
  }
}
