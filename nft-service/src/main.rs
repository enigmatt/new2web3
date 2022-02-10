// ========== Imports =================

use std::include_bytes;

use fluence::module_manifest;
use json;
use json::JsonValue;
use marine_rs_sdk::marine;
use marine_rs_sdk::WasmLoggerBuilder;
use marine_rs_sdk::MountedBinaryResult;

// ========== Structs =================

// Publish the structure of the Item objects that represent an NFT
// on either marketplace.
#[marine]
#[derive(Debug)]
pub struct Item {
    pub id: String,
    pub token_id: String,
    pub marketplace: String,
    pub blockchain: String,
    pub name: String,
    pub description: String,
    pub detail_url: String,
    pub image_url: String,
}

// Publish the structure of a page of NFT Items along with the name
// of the marketplace it came from and the information for getting
// the next page.
#[marine]
#[derive(Debug)]
pub struct ItemPage {
    pub marketplace: String,
    pub opensea_next_offset: i32,
    pub rarible_continuation: String,
    pub items: Vec<Item>,
}

// Publish the structure of a pair of ItemPages, one from OpenSea and
// one from Rarible. This is used in our example of using Aqua to run
// calls in parallel.
#[marine]
#[derive(Debug)]
pub struct ItemPages {
    pub opensea_page: ItemPage,
    pub rarible_page: ItemPage,
}

// ========== Initialization =================

// I included this line because one of the Rust tutorials included
// it. I do not know what it does. :D
module_manifest!();

// main initializes the logger
fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

// ========== External Library Links =================

// Set the curl function as a call to an external library.
#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    fn curl(cmd: Vec<&str>) -> MountedBinaryResult;
}

// This function enables URL downloading. This is used to hit the
// OpenSea and Rarible Web APIs.
#[marine]
pub fn download(url: &str) -> String {
    log::info!("download called with url {}", url);
    let result = unsafe { curl(vec![url]) };
    String::from_utf8(result.stdout).unwrap()
}

// ========== Public Functions =================

// This function gets an initial page of JSON from OpenSea, cleans up
// the line returns that cause the JSON parser to error out, and passes
// the result to the parser function.
#[marine]
pub fn get_first_opensea_page() -> ItemPage {
    let json_string: String = get_opensea_items_json();
    let json_string: String = json_string.replacen("\n", " ", 100000);
    parse_opensea_page(&json_string)
}

// This function does not actually paginate because we were unable
// to get OpenSea pagination to work. Any call we made to the OpenSea
// API with ?offset=20&limit=20 resulted in a 1020 error suggesting
// that we were being throttled. We submitted a registration request
// for a production token, but did not receive a response.
#[marine]
pub fn get_opensea_continuation(opensea_next_offset: i32) -> ItemPage {
    let json_string: String = get_opensea_items_json();
    let json_string: String = json_string.replacen("\n", " ", 100000);
    parse_opensea_page(&json_string)
}

// Pull the JSON from Rarible and pass it to the parser.
#[marine]
pub fn get_first_rarible_page() -> ItemPage {
    let json = get_rarible_items_json();
    parse_rarible_page(&json)
}

// Get a continuation page JSON from Rarible and pass it
// to the parser.
#[marine]
pub fn get_rarible_continuation(rarible_continuation: &str) -> ItemPage {
    let json = get_rarible_continuation_json(rarible_continuation);
    parse_rarible_page(&json)
}

// This tiny function wraps two ItemPages in a struct and
// passes it back. Javascript threw an error when we tried
// to connect it to an Aqua func that returned a pair of
// ItemPages as "ItemPage, ItemPage". This function is used
// for the demo of parallelizing services in Aqua.
#[marine]
pub fn collect_these(opensea_page: ItemPage, rarible_page: ItemPage) -> ItemPages {
    ItemPages {
        opensea_page,
        rarible_page,
    }
}

// ========== JSON Parsing =================

// This function parses OpenSea's JSON NFT Item structure and
// marshalls the content into a unified Item structure used for
// both Rarible and OpenSea, to make the front end code simpler.
fn item_from_opensea_asset(asset: &JsonValue) -> Item {
    let id = asset["id"].as_str().unwrap_or("").to_string();
    let token_id = asset["token_id"].as_str().unwrap_or("").to_string();
    let marketplace = String::from("OpenSea");
    let blockchain = String::from("ETHEREUM");
    let name = asset["name"].as_str().unwrap_or("").to_string();
    let description = asset["description"].as_str().unwrap_or("").to_string();
    let detail_url = asset["permalink"].as_str().unwrap_or("").to_string();
    let image_url = asset["image_url"].as_str().unwrap_or("").to_string();
    Item{
        id, token_id, marketplace, blockchain, name,
        description, detail_url, image_url,
    }
}

// This function parses Rarible's JSON NFT item structure and
// marshalls the content into a unified Item structure used for
// both Rarible and OpenSea, to make the front end code simpler.
fn item_from_rarible_element(element: &JsonValue) -> Option<Item> {
    let id = element["id"]
        .as_str().unwrap_or("").to_string();
    let token_id = element["tokenId"]
        .as_str().unwrap_or("").to_string();
    let marketplace = String::from("Rarible");
    let blockchain = element["blockchain"]
        .as_str().unwrap_or("").to_string();
    let name = element["meta"]["name"]
        .as_str().unwrap_or("").to_string();
    let description = element["meta"]["description"]
        .as_str().unwrap_or("").to_string();
    let mut detail_url =
        String::from("https://api.rarible.org/v0.1/items/");
    detail_url.push_str(&id);
    let content_elements = &element["meta"]["content"];
    let mut image_url = String::from("");
    for content_element in content_elements.members() {
        let content_type = content_element["@type"]
            .as_str().unwrap_or("");
        let representation = content_element["representation"]
            .as_str().unwrap_or("");
        if content_type.eq("IMAGE") && representation.eq("ORIGINAL") {
            image_url = content_element["url"]
                .as_str().unwrap_or("").to_string();
        }
    }
    if image_url.eq("") {
        None
    } else {
        Some(Item{
            id, token_id, marketplace, blockchain, name,
            description, detail_url, image_url,
        })
    }
}

// This method takes a full OpenSea JSON page and builds
// an ItemPage including a vector of NFT items and the
// continuation information.
fn parse_opensea_page(json_string: &str) -> ItemPage {
    let parsed = json::parse(json_string).unwrap();
    let assets = &parsed["assets"];
    let mut items: Vec<Item> = Vec::new();

    for asset in assets.members() {
        let item = item_from_opensea_asset(&asset);
        items.push(item);
    }
    ItemPage {
        marketplace: String::from("OpenSea"),
        opensea_next_offset: 20,
        rarible_continuation: String::from(""),
        items,
    }
}

// This method takes a full Rarible JSON page and builds
// an ItemPage including a vector of NFT items and the
// continuation information.
fn parse_rarible_page(json_string: &str) -> ItemPage {
    let parsed = json::parse(json_string).unwrap();
    let item_elements = &parsed["items"];
    let mut items: Vec<Item>  = Vec::new();

    for item_element in item_elements.members() {
        let result = item_from_rarible_element(&item_element);
        match result {
            Some(item) => items.push(item),
            None => {},// do nothing
        }
    }
    ItemPage {
        marketplace: String::from("Rarible"),
        opensea_next_offset: -1,
        rarible_continuation: parsed["continuation"]
            .as_str().unwrap_or("").to_string(),
        items,
    }
}

// ========== NFT Marketplace API Calls =================

// Since we were regularly getting 1020 (throttle) errors from OpenSea,
// we decided to stub out the call to OpenSea with static content
// to ensure a successful demo. The Rarible examples in the next
// two methods show what hitting OpenSea's API would look like if our
// API registration had been accepted and we could include an
// authentication token in the request header.
fn get_opensea_items_json() -> String {
    let json_bytes = include_bytes!("opensea_list.json");
    let json_string = String::from_utf8_lossy(json_bytes);
    json_string.to_string()
}

// This method hits the Rarible API for an initial page of NFT items
// and returns the response as a String. During initial testing we found
// that the content of the list was quite complete, obviating the need
// for a subsequent call to the item detail API for each NFT.
fn get_rarible_items_json() -> String {
    // let json_bytes = include_bytes!("rarible_list.json");
    // let json_string = String::from_utf8_lossy(json_bytes);
    // json_string.to_string()
    let rarible_url = "https://api.rarible.org/v0.1/items/all?size=20";
    download(rarible_url)
}

// This method hits the Rarible API for a continuation page of NFT
// items and returns the response as a string.
fn get_rarible_continuation_json(continuation: &str) -> String {
    // https://api.rarible.org/v0.1/items/all?size=20&continuation=ETHEREUM:1643760949000_0x966731dfd9b9925dd105ff465687f5aa8f54ee9f:2040;TEZOS:1643760966000_KT1Cdfc4Ynz4WTZn4aX12wG2tpT88WXJCQAF:7;FLOW:1643760952544_A.0b2a3299cc857e29.TopShot:21281652
    let mut continuation_url =
        String::from("https://api.rarible.org/v0.1/items/all?size=20&continuation=");
    continuation_url.push_str(continuation);
    download(&continuation_url)
}
