use std::{net::{TcpListener, TcpStream}, error::Error, fs::File};
use std::io::prelude::*;

use comrak::{markdown_to_html, ComrakOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let listnenr = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listnenr.incoming() {
        let stream = stream?;

        handle_connection(stream);
    };

    Ok(())
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let mut file = File::open("hello.md").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();


    let markdown_html = markdown_to_html(&content, &ComrakOptions::default());
    
    // CSS를 HTML에 삽입
    let styled_html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {{
                    font-family: Arial, sans-serif;
                    background-color: black;
                    color: white;
                    margin: 40px;
                }}
                h1 {{
                    color: cyan;
                }}
                p {{
                    color: lightgrey;
                }}
                img {{
                    max-width: 100%;
                    height: auto;
                }}
            </style>
        </head>
        <body>
        {}
        </body>
        </html>
    "#, markdown_html);

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}",
        styled_html.len(),
        styled_html
    );


    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("\n  {}", response);
    println!("\n\n\n\n\n");

}