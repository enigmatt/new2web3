use std::include_bytes;

// use fluence::marine;
use fluence::module_manifest;
use json;
use json::JsonValue;
use marine_rs_sdk::marine;
use marine_rs_sdk::WasmLoggerBuilder;
use marine_rs_sdk::MountedBinaryResult;

fn main() {
    WasmLoggerBuilder::new().build().unwrap();

    if false {
        let opensea_json = get_opensea_items_json();
        if false { println!("{}", opensea_json); };
        
        let rarible_json = get_rarible_items_json();
        if false { println!("{}", rarible_json); };
        
        let item_page = get_opensea_item_page();
        if false { dbg!(&item_page); };
    }
}

module_manifest!();

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    fn curl(cmd: Vec<&str>) -> MountedBinaryResult;
}

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

#[marine]
#[derive(Debug)]
pub struct ItemPage {
    pub marketplace: String,
    pub opensea_next_offset: i32,
    pub rarible_continuation: String,
    pub items: Vec<Item>,
}

#[marine]
#[derive(Debug)]
pub struct ItemPages {
    pub opensea_page: ItemPage,
    pub rarible_page: ItemPage,
}

#[marine]
pub fn download(url: &str) -> String {
    log::info!("download called with url {}", url);
    
    let result = unsafe { curl(vec![url]) };
    String::from_utf8(result.stdout).unwrap()
}

#[marine]
pub fn get_first_opensea_page() -> ItemPage {
    get_opensea_item_page()
}

#[marine]
pub fn get_opensea_continuation(opensea_next_offset: i32) -> ItemPage {
    get_opensea_item_page()
}

#[marine]
pub fn get_first_rarible_page() -> ItemPage {
    let json = get_rarible_items_json();
    rarible_page(&json)
}

#[marine]
pub fn get_rarible_continuation(rarible_continuation: &str) -> ItemPage {
    let json = get_rarible_continuation_json(rarible_continuation);
    rarible_page(&json)
}

#[marine]
pub fn collect_these(opensea_page: ItemPage, rarible_page: ItemPage) -> ItemPages {
    ItemPages {
        opensea_page,
        rarible_page,
    }
}

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

fn get_opensea_item_page() -> ItemPage {
    let json_string: String = get_opensea_items_json();
    let json_string: String = json_string.replacen("\n", " ", 100000);
    let parsed = json::parse(&json_string).unwrap();
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

fn rarible_page(json_string: &str) -> ItemPage {
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

fn get_opensea_items_json() -> String {
    let json_bytes = include_bytes!("opensea_list.json");
    let json_string = String::from_utf8_lossy(json_bytes);
    json_string.to_string()
}

fn get_rarible_continuation_json(continuation: &str) -> String {
    // https://api.rarible.org/v0.1/items/all?size=20&continuation=ETHEREUM:1643760949000_0x966731dfd9b9925dd105ff465687f5aa8f54ee9f:2040;TEZOS:1643760966000_KT1Cdfc4Ynz4WTZn4aX12wG2tpT88WXJCQAF:7;FLOW:1643760952544_A.0b2a3299cc857e29.TopShot:21281652
    let mut continuation_url =
        String::from("https://api.rarible.org/v0.1/items/all?size=20&continuation=");
    continuation_url.push_str(continuation);
    download(&continuation_url)
}

fn get_rarible_items_json() -> String {
    // let json_bytes = include_bytes!("rarible_list.json");
    // let json_string = String::from_utf8_lossy(json_bytes);
    // json_string.to_string()
    let rarible_url = "https://api.rarible.org/v0.1/items/all?size=20";
    download(rarible_url)
}
