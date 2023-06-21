//  https://www.youtube.com/watch?v=HNnbIW2Kzbc

#[doc = "Basic HTML code for the website"]
pub fn generate_html(dialog: Option<&str>) -> String {
    let dialog_html = match dialog {
        Some(message) => format!("<p>Submitted message: {}</p>", message),
        None => String::new(),
    };

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>QuADS</title>
        </head>
        <body>
            <h1>QuADS</h1>
            <p>Questionablly Accurate Degree Simulator</p>
            <p>If multiple inputs, please dilineate with comma then a space (, ). Department and course code for courses. </p>
            {}
            <form method="post" action="/submit">
                <label for="year">Year:</label><br>
                <input type="number" id="year" name="year"><br>

                <label for="majors">Majors:</label><br>
                <input type="text" id="majors" name="majors"><br>

                <label for="concentrations">Concentrations:</label><br>
                <input type="text" id="majors" name="concentrations"><br>

                <label for="minors">Minors:</label><br>
                <input type="text" id="majors" name="minors"><br>

                <label for="pathway">Pathway:</label><br>
                <input type="text" id="majors" name="pathway"><br>

                <label for="courses">Courses:</label><br>
                <input type="text" id="majors" name="courses"><br>

                <input type="submit" value="Submit">
            </form>
        </body>
        </html>
        "#,
        dialog_html
    );

    html.to_string()
}

//TODO: Get information to generate a Student item

//TODO: Display feedback post-processing