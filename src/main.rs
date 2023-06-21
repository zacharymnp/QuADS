mod catalog_scraper;
mod html_generator;
mod student;

use catalog_scraper::{nav_to_programs, get_url, request_html};
use html_generator::generate_html;
use student::Student;

use warp::{Filter, Reply};
use std::clone::Clone;
use tokio::task::spawn_blocking;

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

    //TODO: Check thru each of Student's majors, paying attention to concentration

    //TODO: Check thru each of Student's minors

    //TODO: Generate some form of feedback

    //TODO: Display feedback post-processing on site
}

async fn submit_student(submission: Student) -> Result<impl Reply, warp::Rejection> {
    let student = submission.clone();
    let url = get_url(student.year).unwrap();

    //'spawn_blocking' allows blocking operations to execute without blocking asynchronous execution flow
    //'move' brings 'url' and 'home' into scope of closure
    let home = spawn_blocking(move || {
        request_html(url).expect("fetching homepage failed")
    }).await.expect("blocking task failed for homepage");
    let programs = spawn_blocking(move || {
        nav_to_programs(&home).expect("fetching programs failed")
    }).await.expect("blocking task failed for programs");
    println!("Programs HTML: {}", programs);

    Ok(warp::reply::html("Submission received"))
}

//Consider downloading Github copilot