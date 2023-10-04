extern crate url;

use url::Url;
use serde_json::{Value, Map};
use async_recursion::async_recursion;

const AT_CONTEXT: &'static str = "@context";
const AT_ID:      &'static str = "@id";
const AT_REF:     &'static str = "@ref";
// const AT_TYPE:    &'static str = "@type";

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
        Err(format!("Failed to parse JSON-LD from given url.\nurl: {url}\nresponse: {}", body.as_str()).into())
      }
    }
    Err(e) => {
      Err(format!("Failed to parse JSON from given url (maybe NOT JSON).\nurl: {url}\nresponse: {}\nerror: {e}", body.as_str()).into())
    }
  }
}

#[async_recursion]
async fn scan_json_ld_obj(obj: &mut Map<String, Value>, depth: u8, load_context: bool) {
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
              println!("Error occurred: {}", e);
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
        if is_valid_url(val_str) {
          let json_ld = load_json_ld(val_str, depth - 1, load_context).await;
          match json_ld {
            Ok(new_obj) => {
              obj.remove(&key);
              obj.insert(key, Value::from(new_obj));
            }
            Err(e) => {
                println!("Maybe NOT JSON-LD: {} {}", val_str, e);
            }
          }
        }
      }
    }
  }

}

#[async_recursion]
async fn scan_json_ld_array(arr: &mut Vec<Value>, depth: u8, load_context: bool) {
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
              println!("Maybe NOT JSON-LD: {} {}", val_str, e);
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