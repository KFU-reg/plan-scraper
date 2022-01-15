use select::{document::Document, node::Node, predicate::Name};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ClassStatus {
    Available,
    UnAvailable,
    Full,
    Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Class {
    pub code: String,
    pub crn: String,
    pub section: String,
    /// 0:error, Sunday:1,Saturday:7
    pub days: Vec<usize>,
    pub starting_time: String,
    pub ending_time: String,
    pub instructor: String,
    pub allowed_majors: Vec<String>,
    pub allowed_colleges: Vec<String>,
    pub available: ClassStatus,
}

impl<'a> Class {
    /// Takes a `<tr>` (table row) Node
    pub fn new(courses: Node<'a>) -> Option<Self> {
        let class_node = courses.find(Name("td")).collect::<Vec<Node>>();

        // ignore any other table
        if class_node.len() != 13 {
            return None;
        }
        let code = class_node[0].text().replace("-", "").replace("\n", "");
        // ignore headings (they don't have a code)
        if code.parse::<usize>().is_err() {
            return None;
        }

        let code = code;
        let crn = class_node[1].text().replace("\n", "");
        let section = class_node[2].text().replace("\n", "");
        let instructor = class_node[9].text().replace("\n", "");
        let days = get_days(class_node[6].text().replace("\n", "").replace(" ", ""));
        let (starting_time, ending_time) =
            get_times(class_node[8].text().replace("\n", "").replace(" ", ""));
        let allowed_colleges = allowed(class_node[11].text().replace("\n", ""));
        let allowed_majors = allowed(class_node[12].text().replace("\n", ""));
        let available = available(&class_node[3].text().trim().replace("\n", ""));
        eprintln!("code: {}", code);
        eprintln!("crn: {}", crn);
        eprintln!("section: {}", section);
        eprintln!("instructor: {}", instructor);
        eprintln!("days: {:?}", days);
        eprintln!("times: {:?}", (starting_time.clone(), ending_time.clone()));
        eprintln!("allowed majors: {:?}", allowed_majors);
        eprintln!("Available: {:?}", available);

        // let credits = class_node[2].text().parse::<usize>().unwrap();

        // gets the PreRequists Node
        // let mut pre_requisites: Vec<String> = vec![];
        // let mut co_requisites: Vec<String> = vec![];
        // if let Some(requisites_node) = course_node[3].first_child() {
        // pre_requisites = get_pre_requisites(requisites_node);
        // co_requisites = get_co_requisites(requisites_node);
        // }

        Some(Class {
            code,
            crn,
            section,
            days,
            starting_time,
            ending_time,
            instructor,
            allowed_majors,
            allowed_colleges,
            available,
        })
    }
}

pub fn parse(html: String) -> Vec<Class> {
    let document = Document::from(&html[..]);
    // let document = Document::from(include_str!("../plan.html"));
    let mut parsed_classes: Vec<Class> = vec![];

    // for each class
    for class in document.find(Name("tr")) {
        // get class info
        let class = Class::new(class);
        if class.is_some() {
            parsed_classes.push(class.clone().unwrap());
        }
    }
    parsed_classes
}

/// Merges duplicates. some classes have same CRN but different days.
/// merge the days only!
pub fn merge_duplicates(classes_with_dups: &[Class]) -> Vec<Class> {
    let mut merged: Vec<Class> = vec![];
    for class_may_be_duplicate in classes_with_dups {
        if let Some(found_dup) = merged
            .iter()
            .position(|a| class_may_be_duplicate.crn == a.crn)
        {
            merged[found_dup].days.extend(&class_may_be_duplicate.days)
        } else {
            // it aint a dup!
            merged.push(class_may_be_duplicate.clone());
        }
    }
    merged
}

/// gets string in this form "حنث"
fn get_days(string: String) -> Vec<usize> {
    string
        .chars()
        .map(|day| match day {
            'ح' => 1, // sunday
            'ن' => 2, // monday
            'ث' => 3, // thursday
            'ر' => 4, // tuesday
            'خ' => 5, // wednesday
            'ج' => 6, // friday
            'س' => 7, // saturday
            _ => panic!("Weird day format {}", day),
        })
        .collect()
}

/// gets string in this form "0000-1111"
/// where 0000 is the starting and 1111 is the ending
fn get_times(string: String) -> (String, String) {
    let (s, e) = string.split_once('-').unwrap();
    (s.to_string(), e.to_string())
}
fn available(string: &str) -> ClassStatus {
    println!("{}", string);
    match string {
        "متاحه" => ClassStatus::Available,
        "غير متاحه" => ClassStatus::UnAvailable,
        "ممتلئة" => ClassStatus::Full,
        _ => todo!(),
    }
}
// for majors and colleges
fn allowed(string: String) -> Vec<String> {
    string
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect()
}
