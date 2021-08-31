use kfu_reg_scraper::consts::MAJORS_URLS;
use kfu_reg_scraper::course;

use std::fs;

fn main() {
    MAJORS_URLS.iter().for_each(|(name, url)| {
        eprintln!("Getting {}", name);
        let html = download(url).expect("failed to download");
        // let html = "".to_string();
        let parsed_major = course::parse_courses(html);
        let json = serde_json::to_string_pretty(&parsed_major).unwrap();
        fs::write("output/".to_owned() + name + ".json", json).unwrap();
        eprintln!("Done! {}", name);
    });
}

fn download(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?.text()?;
    Ok(resp)
}
