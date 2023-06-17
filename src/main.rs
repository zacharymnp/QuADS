mod catalog_scraper;
mod html_generator;
mod student;

use crate::catalog_scraper::nav_to_programs;

#[doc = "Main and stuff"]
fn main() {
    #![allow(unused_variables)]
    #![allow(dead_code)]

    //TODO: Generate site to gather information to build a student

    //TODO: Using Student, request HTML from the specific year's catalog home page
    let home = catalog_scraper::request_html("https://catalog.rpi.edu/index.php").expect("fetching homepage failed");
    let programs = nav_to_programs(&home).expect("fetching programs failed");

    //TODO: Check thru each of Student's majors, paying attention to concentration

    //TODO: Check thru each of Student's minors

    //TODO: Generate some form of feedback

    //TODO: Display feedback post-processing on site
}
//Consider downloading Github copilot