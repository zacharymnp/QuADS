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
            {}
            <form method="post" action="/submit">
                <label for="message">Enter a message:</label><br>
                <input type="text" id="message" name="message"><br>
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