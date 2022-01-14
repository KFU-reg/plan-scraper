#![allow(dead_code, unused)]
extern crate custom_error;
extern crate glob;

use serde::{Deserialize, Serialize};
use std::fs;
use std::{fs::File, io};

use custom_error::custom_error;
use glob::glob;
use kfu_reg_scraper::class::{Class, ClassStatus};
use kfu_reg_scraper::consts::MAJORS_URLS;
use kfu_reg_scraper::course::Course;

fn main() {
    let all_classes: Vec<Class> = get_all_classes("output/Classes_*.json").unwrap();
    let all_major_plans: Vec<(String, Vec<Course>)> = get_all_plans("output/Plan_*.json").unwrap();

    for (major, courses) in all_major_plans {
        let plan: Plan = plan_for_major(courses, &all_classes);

        let json = serde_json::to_string_pretty(&plan).unwrap();
        fs::write("output/CP_".to_owned() + &major + ".json", json).unwrap();
    }
}

fn plan_for_major(courses: Vec<Course>, all_classes: &[Class]) -> Plan {
    let mut plan: Plan = vec![];
    for course in courses {
        let classes: Vec<&Class> = all_classes
            .iter()
            .filter(|class| class.code == course.code)
            .collect();

        let cp_course = classes.iter().map(|class| CP {
            code: class.code.clone(),
            name: course.name.clone(),
            crn: class.crn.clone(),
            section: class.section.clone(),
            days: class.days.clone(),
            time: class.starting_time.clone() + "-" + &class.ending_time,
            instructor: class.instructor.clone(),
            semster_index: course.semster_index,
            college_allowed: true, //TODO
            major_allowed: true,   // TODO
            co_req: course.co_requisites.clone(),
            credits: course.credits,
            available: class.available.clone(),
        });
        plan.extend(cp_course);
    }
    plan
}

fn get_all_classes(path: &str) -> Result<Vec<Class>, CliError> {
    let mut all_classes: Vec<Class> = vec![];
    for entry in glob(path).expect("Error While creating Glob pattern!") {
        let pathbuf = entry?;
        let file = File::open(pathbuf)?;
        let mut classes: Vec<Class> = serde_json::from_reader(file)?;

        all_classes.append(&mut classes);
    }

    Ok(all_classes)
}

fn get_all_plans(path: &str) -> Result<Vec<(String, Vec<Course>)>, CliError> {
    // the course name, and the Course itself
    let mut all_courses: Vec<(String, Vec<Course>)> = vec![];
    for entry in glob(path).expect("Error While creating Glob pattern!") {
        let pathbuf = entry?;
        let file = File::open(pathbuf.clone())?;

        let mut courses: Vec<Course> = serde_json::from_reader(file)?;

        let course_name = pathbuf
            .to_str()
            .unwrap()
            // ("output/Plan_", "Elect.json")
            .split_at(12)
            .1
            .rsplit_once(".json")
            // ("Elect", ".json")
            .unwrap()
            // "Elect"
            .0;
        all_courses.push((course_name.to_string(), courses.clone()));
    }

    Ok(all_courses)
}

type Plan = Vec<CP>;

// Classes For the Plan

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CP {
    code: String,
    name: String,
    crn: String,
    credits: usize,
    section: String,
    days: Vec<usize>,
    time: String,
    instructor: String,
    semster_index: usize,
    /// TODO: No restrictions on College?
    college_allowed: bool,
    /// TODO: No restrictions on Major?
    major_allowed: bool,
    ///Co requisites
    co_req: Vec<String>,
    available: ClassStatus,
}

custom_error! {CliError
    GlobError{source: glob::GlobError}		= "Error While Creating a glob path",
    Io{source: io::Error}			= "unable to read from the file",
    SerdeJson{source: serde_json::Error}        = "Unable to Parse JSON",

}
