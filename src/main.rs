use epub::doc::EpubDoc;
use std::io;

fn strip_end_line(s: String) -> Result<String, &'static str> {
    match s.strip_suffix("\r\n") {
        Some(val) => Ok(val.to_string()),
        None => match s.strip_suffix("\n") {
            Some(val) => Ok(val.to_string()),
            None => Err("No end line found!"),
        },
    }
}

fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    strip_end_line(buffer).expect("Error: ")
}

fn process_commands<R: io::Read + io::Seek>(
    doc: &mut EpubDoc<R>,
    input: String,
) -> Option<&'static str> {
    match input.as_str() {
        "next()" => doc.go_next().unwrap(),
        "show()" => println!("{}", doc.get_current_str().unwrap()),
        "exit()" => return Some("break"),
        _ => (),
    };
    None
}

fn main() {
    let mut doc = EpubDoc::new("test.epub").unwrap();
    loop {
        if let Some("break") = process_commands(&mut doc, get_input()) {
            break;
        }
    }
}
