#![allow(unused_variables)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[doc = ""]
#[doc = "Limited to undergraduate students, since graduate degrees are weird. Some majors will not work,
    of which I will attempt to list."]
#[derive(Deserialize, Serialize, Clone)] //defines how to deserialize JSON data into Student's fields and to serialize fields into JSON data
pub struct Student {
    pub year: u8, //last 2 digits of freshman year
    pub majors: String,
    pub concentrations: String, //this is gonna get messed up
        //TODO: Figure out how I'm going to deal with CS
    pub minors: String, //for now, as many courses can be applied to minors as you want
    pub pathway: String,
    pub courses: String
    //TODO: Figure out how to deal with credit counts
}

#[doc = ""]
pub fn build_student(year: u8, majors: String, concentrations: String, minors: String,
                     pathway: String, courses: String) -> Student {
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