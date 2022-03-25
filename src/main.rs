use kfu_reg_scraper::university::College;

fn main() {
    let ouput = "output";
    // list of colleges
    let colleges: Vec<College> = vec![Default::default()];
    // Download the raw html for courses
    colleges.iter().for_each(|c| c.fetch_courses());
    // then extract data to JSON
    colleges.iter().for_each(|c| c.);
}
