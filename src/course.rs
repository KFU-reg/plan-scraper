use select::document::Document;
use select::node::Node;
// use select::predicate::{Attr, Class, Name, Predicate};
use select::predicate::Name;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Course {
    pub code: String,
    pub name: String,
    pub credits: usize,
    pub pre_requisites: Vec<String>,
    pub co_requisites: Vec<String>,
    /// starts at zero
    pub semster_index: usize,
}

impl<'a> Course {
    /// Takes a `<tr>` (table row) Node
    pub fn new(courses: Node<'a>, semster_index: usize) -> Self {
        let course = courses.find(Name("td")).collect::<Vec<Node>>();
        let code = course[0].text();
        let name = course[1].text();
        let credits = course[2].text().parse::<usize>().unwrap();

        // gets the PreRequists Node
        let mut pre_requisites: Vec<String> = vec![];
        let mut co_requisites: Vec<String> = vec![];
        if let Some(requisites_node) = course[3].first_child() {
            pre_requisites = get_pre_requisites(requisites_node);
            co_requisites = get_co_requisites(requisites_node);
        }

        Course {
            code,
            name,
            credits,
            pre_requisites,
            co_requisites,
            semster_index,
        }
    }
}

pub fn parse_courses(html: String) -> Vec<Course> {
    let document = Document::from(&html[..]);
    // let document = Document::from(include_str!("../plan.html"));
    let mut parsed_courses: Vec<Course> = vec![];

    // for each semster
    for (semester_index, semster) in document.find(Name("tbody")).enumerate() {
        // and each row
        for courses in semster.find(Name("tr")) {
            // get course info
            let course = Course::new(courses, semester_index + 1);
            parsed_courses.push(course.clone());
        }
    }
    parsed_courses
}

// helper functions

// " [0817-140] تفاضل وتكامل 1 - غير متزامن"
fn get_pre_requisites(node: Node) -> Vec<String> {
    get_requisites(node, &|course: &String| course.contains("غير متزامن"))
}
// " [0817-140] تفاضل وتكامل 1 متزامن"
fn get_co_requisites(node: Node) -> Vec<String> {
    get_requisites(node, &|course: &String| !course.contains("غير متزامن"))
}

fn get_requisites(node: Node, filter: &dyn Fn(&String) -> bool) -> Vec<String> {
    // the inner attribute contains html (bruh)
    // like this: `<div data-content="<span/>"/>`
    let data_content = node.attr("data-content").unwrap_or("");
    let doc = Document::from(data_content);

    doc.find(Name("div"))
        .map(|node| node.text())
        .filter(|course| filter(course))
        .map(get_code)
        .collect()
}

// input
// " [0817-140] تفاضل وتكامل 1 - غير متزامن"
// output
// 0817140
fn get_code(raw: String) -> String {
    let first = raw.find('[').unwrap() + 1;
    let last = raw.find(']').unwrap() - 1;
    let raw = raw.replace("-", "");

    raw[first..last].parse::<usize>().unwrap(); //making sure it is a number
    raw[first..last].to_string()
}
