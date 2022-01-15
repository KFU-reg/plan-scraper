use kfu_reg_scraper::class;
use kfu_reg_scraper::consts::{avialable_courses_urls, CLASS_LIST_PROPS};
use kfu_reg_scraper::helpers;

use std::fs;
// This is used to extract the courses, requists, credits...
// not classes or sections

fn main() {
    let class_urls = avialable_courses_urls();

    class_urls.iter().enumerate().for_each(|((i, url))| {
        let c = CLASS_LIST_PROPS[i];
        let html = helpers::download(url).expect("failed to download");
        // let html = "".to_string();

        let parsed_classes = class::parse(html);
        let without_duplicates = class::merge_duplicates(&parsed_classes);
        let json = serde_json::to_string_pretty(&without_duplicates).unwrap();
        // println!("json: {}", json);
        fs::write("output/Classes_".to_owned() + c.name_en + ".json", json).unwrap();
        eprintln!("Done! {}", c.name_en);
    });
}
