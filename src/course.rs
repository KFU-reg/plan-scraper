use select::document::Document;
use select::node::Node;
// use select::predicate::{Attr, Class, Name, Predicate};
use select::predicate::Name;

#[derive(Debug, Clone)]
pub struct Course {
    pub code: usize,
    pub name: String,
    pub credits: usize,
    pub pre_requisites: Vec<usize>,
    pub co_requisites: Vec<usize>,
    /// starts at zero
    pub semster_index: usize,
}

impl<'a> Course {
    /// Takes a `<tr>` (table row) Node
    pub fn new(courses: Node<'a>, semster_index: usize) -> Self {
        let course = courses.find(Name("td")).collect::<Vec<Node>>();
        let code = course[0].text().parse::<usize>().unwrap();
        let name = course[1].text();
        let credits = course[2].text().parse::<usize>().unwrap();

        // gets the PreRequists Node
        let mut pre_requisites: Vec<usize> = vec![];
        let mut co_requisites: Vec<usize> = vec![];
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

// helper functions

// " [0817-140] تفاضل وتكامل 1 - غير متزامن"
fn get_pre_requisites(node: Node) -> Vec<usize> {
    get_requisites(node, &|course: &String| course.contains("غير متزامن"))
}
// " [0817-140] تفاضل وتكامل 1 متزامن"
fn get_co_requisites(node: Node) -> Vec<usize> {
    get_requisites(node, &|course: &String| !course.contains("غير متزامن"))
}

fn get_requisites(node: Node, filter: &dyn Fn(&String) -> bool) -> Vec<usize> {
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
fn get_code(raw: String) -> usize {
    let first = raw.find('[').unwrap() + 1;
    let last = raw.find(']').unwrap() - 1;
    let raw = raw.replace("-", "");

    raw[first..last].parse::<usize>().unwrap()
}
