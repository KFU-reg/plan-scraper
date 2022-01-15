use kfu_reg_scraper::consts::MAJORS_URLS;
use kfu_reg_scraper::course;
use kfu_reg_scraper::helpers;

use std::fs;
// This is used to extract the courses, requists, credits...
// not classes or sections

fn main() {
    MAJORS_URLS.iter().for_each(|(name, url)| {
        eprintln!("Getting {}", name);
        let html = helpers::download(url).expect("failed to download");
        // let html = "".to_string();
        let parsed_major = course::parse_courses(html);
        let json = serde_json::to_string_pretty(&parsed_major).unwrap();
        fs::write("output/Plan_".to_owned() + name + ".json", json).unwrap();
        eprintln!("Done! {}", name);
    });
}
