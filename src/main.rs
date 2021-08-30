mod course;

use course::Course;
use select::document::Document;
use select::predicate::Name;

fn main() {
    let document = Document::from(include_str!("../plan.html"));
    let mut parsed_courses: Vec<Course> = vec![];

    // for each semster
    for (semester_index, semster) in document.find(Name("tbody")).enumerate() {
        // and each row
        for courses in semster.find(Name("tr")) {
            // get course info
            let course = Course::new(courses, semester_index);
            parsed_courses.push(course.clone());

            println!("{:?}", course);
        }
    }
}
