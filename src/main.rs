mod course;

use course::Course;
use select::document::Document;
use select::predicate::Name;

fn main() {
    let document = Document::from(include_str!("../plan-CS.html"));
    let mut parsed_courses: Vec<Course> = vec![];

    // for each semster
    for (i, semster) in document.find(Name("tbody")).enumerate() {
        // and each row
        for courses in semster.find(Name("tr")) {
            // get course info
            let mut course = Course::from(courses);
            // and asign semster number
            course.semster = i;
            parsed_courses.push(course.clone());

            println!("{:?}", course);
            // println!("{:#?}", course)
        }
    }
}
