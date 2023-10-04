# dbp-json-ld-utils
JSON-LD Utilities for DBP

## Usage
Add a following line in Cargo.toml > `[dependencies]`:
```
json-ld-utils = { git = "https://github.com/exdata-inc/dbp-json-ld-utils.git", rev = "<latest commit hash>"}
```
You can also check behavior of this library by running a following command:
```
cargo run
```

## Functions
### `load_json_ld(url: &str, depth: u8, load_context: bool)`
Load a JSON-LD from specified `url` and follow URLs in JSON-LD until specified `depth`.
If `load_context = true`, load `@context` JSON-LD (otherwise not.) 

### `scan_json_ld_obj(obj: &mut Map<String, Value>, depth: u8, load_context: bool)`
Load a JSON-LD Object `obj` and follow URLs in JSON-LD until specified `depth`.
If `load_context = true`, load `@context` JSON-LD (otherwise not.) 

### `scan_json_ld_array(arr: &mut Vec<Value>, depth: u8, load_context: bool)`
Load a JSON-LD Array `arr` and follow URLs in JSON-LD until specified `depth`.
If `load_context = true`, load `@context` JSON-LD (otherwise not.) 
