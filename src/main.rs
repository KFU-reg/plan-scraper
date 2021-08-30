mod consts;
mod course;

use std::fs;

use course::Course;
use select::document::Document;
use select::predicate::Name;

use crate::consts::MAJORS_URLS;

fn main() {
    MAJORS_URLS.iter().for_each(|(name, url)| {
        eprintln!("Getting {}", name);
        let html = download(url).expect("failed to download");
        // let html = "".to_string();
        let parsed_major = parse_major(html);
        let json = serde_json::to_string_pretty(&parsed_major).unwrap();
        fs::write("output/".to_owned() + name, json).unwrap();
        eprintln!("Done! {}", name);
    });
}

fn download(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?.text()?;
    Ok(resp)
}

fn parse_major(html: String) -> Vec<Course> {
    let document = Document::from(&html[..]);
    // let document = Document::from(include_str!("../plan.html"));
    let mut parsed_courses: Vec<Course> = vec![];

    // for each semster
    for (semester_index, semster) in document.find(Name("tbody")).enumerate() {
        // and each row
        for courses in semster.find(Name("tr")) {
            // get course info
            let course = Course::new(courses, semester_index);
            parsed_courses.push(course.clone());
        }
    }
    parsed_courses
}
