use wasm_bindgen::prelude::*;
use regex::Regex;

#[wasm_bindgen]
pub fn split_address(address: &str) -> Vec<String> {
    let re_pref = Regex::new(r"[都道府県]").unwrap();
    let re_cdtv = Regex::new(r"[市区町村]").unwrap();
    let mut split_address:Vec<String> = vec![];
    let mut start_pos = 0;

    match re_pref.find(address) {
        Some(m) => {
            split_address.push((&address[0..m.end()]).to_string());
            start_pos = m.end();
        },
        None => split_address.push(String::new()),
    }

    match re_cdtv.find(address) {
        Some(m) => {
            split_address.push((&address[start_pos..m.end()]).to_string());
            split_address.push((&address[m.end()..]).to_string());
        },
        None => split_address.push(String::new()),
    }

    split_address
}
