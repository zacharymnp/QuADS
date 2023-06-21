use reqwest::Error;
use scraper::{Selector, Html};
use lazy_static::lazy_static;

//I want to go through each of the last 6 years of Catalogs, navigate to programs, then iterate thru each program

#[doc = "Generates CSS Selector."]
fn make_selector(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

lazy_static! {
    static ref TABLE: Selector = make_selector("table"); //selector for table
    static ref ROW: Selector = make_selector("tr"); //selector for table rows
    static ref NAV: Selector = make_selector("div.n2_links > a.navbar"); //selector for the immediate child navbar of a div element with class n2_links
}

//hard-coding in the URLs because it's hard to access them. I'll figure it out at some point
//TODO: Figure out how to do this automatically. I'm so close, but can;t figure out when it skips a number
#[doc = "Getting the URL of the catalog homepage from a given year"]
pub fn get_url(year: u8) -> Option<&'static str> {
    match year {
        18 => Some("https://catalog.rpi.edu/index.php?catoid=18"),
        19 => Some("https://catalog.rpi.edu/index.php?catoid=20"),
        20 => Some("https://catalog.rpi.edu/index.php?catoid=21"),
        21 => Some("https://catalog.rpi.edu/index.php?catoid=22"),
        22 => Some("https://catalog.rpi.edu/index.php?catoid=24"),
        _ => None,
    }
}

#[doc = "Grabs raw HTML from a given URL."]
//TODO: Implement request throttling, whatever that really means
pub fn request_html(url: &str) -> Result<String, Error> {
    let response = reqwest::blocking::get(url)?; //'?' should return Result<Error> on failure
    let raw_html_string = response.text().unwrap();

    Ok(raw_html_string)
}

#[doc = "Given the raw HTML of a catalog homepage, returns the raw HTML of the Programs page."]
pub fn nav_to_programs(raw: &String) -> Result<String, Error> {
    //turns string into HTML document
    let document = Html::parse_document(&raw);

    //TODO: Make this less stupid
    //Selects the table with the most rows in the document, until nested navigation table is reached
    let nav_table = document.select(&TABLE).max_by_key(|table| {
            table.select(&ROW).count()
        }).expect("No tables found in document").select(&TABLE).max_by_key(|table| {
            table.select(&ROW).count()
        }).expect("No tables found in top table").select(&TABLE).max_by_key(|table| {
            table.select(&ROW).count()
        }).expect("No tables found in block table");

    let nav_html = Html::parse_fragment(&nav_table.html());

    //TODO: Improve error checking here
    //finding the URL of the "Programs" page from the homepage's navbar
    let mut url: Option<String> = None;
    if let Some(element) = nav_html.select(&NAV).find(|el| el.text().next() == Some("Programs")) {
        url = element.value().attr("href").map(|href| href.to_owned());
    }
    let true_url = String::from("https://catalog.rpi.edu") + &url.unwrap();

    let raw_programs = request_html(&true_url).expect("fetching programs failed");
    Ok(raw_programs)
}

