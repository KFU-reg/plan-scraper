use crate::course::Course;

#[derive(Clone, Debug)]
pub struct University<'a> {
    pub name: &'a str,
    // pub url: &'a str,
    // pub logo_url: &'a str,
    pub colleges: Vec<College<'a>>,
}
#[derive(Clone, Debug)]
pub struct College<'a> {
    pub name: &'a str,
    // pub url: &'a str,
    // pub logo_url: &'a str,
    pub majors: Vec<Major<'a>>,
}
impl Default for College<'_> {
    fn default() -> Self {
        todo!()
    }
}
impl College<'_> {
    /// This function downloads and saves the courses to TODO!
    pub fn fetch_courses(&self) {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct Major<'a> {
    pub name: &'a str,
    pub courses: Vec<Course>,
}
