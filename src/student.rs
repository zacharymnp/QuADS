#![allow(unused_variables)]
#![allow(dead_code)]


#[doc = ""]
#[doc = "Limited to undergraduate students, since graduate degrees are weird. Some majors will not work,
    of which I will attempt to list."]
pub struct Student {
    year: u8, //last 2 digits of freshman year
    majors: Vec<String>,
    concentrations: Vec<String>, //this is gonna get messed up
        //TODO: Figure out how I'm going to deal with CS
    minors: Vec<String>, //for now, as many courses can be applied to minors as you want
    pathway: Vec<String>,
    courses: Vec<String>
    //TODO: Figure out how to deal with credit counts
}

#[doc = ""]
pub fn build_student(year: u8, majors: Vec<String>, concentrations: Vec<String>, minors: Vec<String>,
                     pathway: Vec<String>, courses: Vec<String>) -> Student {
    Student {
        year,
        majors,
        concentrations,
        minors,
        pathway,
        courses
    }
}

//TODO: Figure out how to deal with cross-listed courses
    //Go thru all courses and add known cross-lists?

//TODO: Go thru courses and check for HASS requirement first
//Workflow: Check for HASS, check pathway, check majors with concentration, check minors