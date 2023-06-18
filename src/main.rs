mod catalog_scraper;
mod html_generator;
mod student;

use catalog_scraper::nav_to_programs;
use warp::{Filter, Reply};
use warp::multipart::FormData;
use crate::html_generator::generate_html;

#[doc = "Main and stuff"]
#[tokio::main]
async fn main() {
    #![allow(unused_variables)]
    #![allow(dead_code)]

    //defining the root route
    let html_route = warp::path::end()
        .and(warp::get())
        .map(|| {
            let html = generate_html(None);
            warp::reply::html(html)
        });
    //defining the submit route
    let submit_route = warp::path("submit")
        .and(warp::post())
        .map(|| {
            warp::reply::html("Submitted")
        });
    //combining routes into a single filter
    let routes = html_route.or(submit_route);
    //starting the web server at 'http://localhost:8080'
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    //TODO: Using Student, request HTML from the specific year's catalog home page
//    let home = catalog_scraper::request_html("https://catalog.rpi.edu/index.php").expect("fetching homepage failed");
//    let programs = nav_to_programs(&home).expect("fetching programs failed");

    //TODO: Check thru each of Student's majors, paying attention to concentration

    //TODO: Check thru each of Student's minors

    //TODO: Generate some form of feedback

    //TODO: Display feedback post-processing on site
}
//Consider downloading Github copilot