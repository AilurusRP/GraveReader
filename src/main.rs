use epub::doc::EpubDoc;
use std::io;

fn main() {
    let mut doc = EpubDoc::new("test.epub").unwrap();
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        match &buffer[..] {
            "next()\r\n" | "next()\n" => doc.go_next().unwrap(),
            "show()\r\n" | "show()\n" => println!("{}", doc.get_current_str().unwrap()),
            "exit()\r\n" | "exit()\n" => break,
            _ => (),

        }
    }
}
