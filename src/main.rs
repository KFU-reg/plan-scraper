mod course;

use course::Course;
use select::document::Document;
use select::predicate::Name;

fn main() {
    let document = Document::from(include_str!("../plan.html"));
    let mut parsed_courses: Vec<Course> = vec![];

    for (i, semster) in document.find(Name("tbody")).enumerate() {
        for courses in semster.find(Name("tr")) {
            let course = Course::from(courses);
            parsed_courses.push(course.clone());

            println!("{}", course.code);
            // println!("{:#?}", course)
        }
    }
}
