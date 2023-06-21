mod catalog_scraper;
mod html_generator;
mod student;

use catalog_scraper::nav_to_programs;
use warp::{Filter, Reply};
use html_generator::generate_html;
use student::Student;
use std::clone::Clone;

#[doc = "Main and stuff"]
#[tokio::main]
async fn main() {
    #![allow(unused_variables)]
    #![allow(dead_code)]

    //defining the root route
    let root_route = warp::path::end()
        .and(warp::get()) //GET
        .map(|| {
            let html = generate_html(None);
            warp::reply::html(html)
        });
    //defining the submit route
    let submit_route = warp::path("submit")
        .and(warp::post()) //POST
        .and(warp::body::content_length_limit(1024 * 32))//limit at 32 kb
        .and(warp::body::form())
        .and_then(submit_student);
    //combining routes into a single filter
    let routes = root_route.or(submit_route);
    //starting the web server at 'http://localhost:8080'
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    //Everything after starting the server needs to be done through server stuff, as in I can't put anything else in main

    //TODO: Using Student, request HTML from the specific year's catalog home page
//    let home = catalog_scraper::request_html("https://catalog.rpi.edu/index.php").expect("fetching homepage failed");
//    let programs = nav_to_programs(&home).expect("fetching programs failed");

    //TODO: Check thru each of Student's majors, paying attention to concentration

    //TODO: Check thru each of Student's minors

    //TODO: Generate some form of feedback

    //TODO: Display feedback post-processing on site
}

async fn submit_student(form: Student) -> Result<impl Reply, warp::Rejection> {
    let student = form.clone();

    println!("Year: {}", student.year);
    println!("Majors: {}", student.majors);
    println!("Concentrations: {}", student.concentrations);
    println!("Minors: {}", student.minors);
    println!("Pathway: {}", student.pathway);
    println!("Courses: {}", student.courses);

    Ok(warp::reply::html("Submission received"))
}

//Consider downloading Github copilot