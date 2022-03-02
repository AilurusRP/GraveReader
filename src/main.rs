use epub::doc::EpubDoc;

fn main() {
    let mut doc = EpubDoc::new("jsFp.epub").unwrap();
    for each in doc.spine.iter() {
        println!("{:?}", doc.resources.get(each).unwrap());
    }
    println!("{}", doc.get_resource_str_by_path("titlepage.xhtml").unwrap());
}
