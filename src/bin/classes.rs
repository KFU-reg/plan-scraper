use kfu_reg_scraper::class;
use kfu_reg_scraper::helpers;

use std::fs;
// This is used to extract the courses, requists, credits...
// not classes or sections

fn main() {
    // MAJORS_URLS.iter().for_each(|(name, url)| {
    //     eprintln!("Getting {}", name);
    let url = "https://banner.kfu.edu.sa:7710/KFU/ws?p_trm_code=144210&p_col_code=22&p_sex_code=11";
    let html = helpers::download(url).expect("failed to download");
    // let html = "".to_string();
    let parsed_major = class::parse(html);
    let json = serde_json::to_string_pretty(&parsed_major).unwrap();
    println!("json: {}", json);
    // fs::write("output/".to_owned() + name + ".json", json).unwrap();
    //     eprintln!("Done! {}", name);
    // });
}
