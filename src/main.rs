use std::{
    path::PathBuf, process::Command
};


use serde::Deserialize;
use tempdir::TempDir;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Book {
    repo: String,

    #[serde(default = "get_empty_sting")]
    folder: String,
    title: String,
    file: String,
    web: String,
    buy: Option<String>,
}

fn get_empty_sting() -> String {
    String::new()
}

fn main() {
    let book_yaml_file = "books.yaml";
    let content = std::fs::read_to_string(book_yaml_file).unwrap();
    let books: Vec<Book> = serde_yaml::from_str(&content).unwrap();
    println!("{:#?}", books);

    let mut page = String::new();
    for book in books {
        let tmp_dir = TempDir::new("example").unwrap();
        let path_to_git_workspace = tmp_dir.path().join("repo").display().to_string();
        println!("Cloning {}", book.repo);
        println!("{path_to_git_workspace}");
        let _result = Command::new("git")
        .args(["clone", "--depth", "1", &book.repo, &path_to_git_workspace])
        .output()
        .expect("command failed to start");

        let path_to_ebook_source = PathBuf::from(path_to_git_workspace).join(&book.folder);
        let _result = Command::new("mdbook-epub")
        .args(["-s", "true", &path_to_ebook_source.display().to_string()])
        .output()
        .expect("command failed to start");

        let book_path = tmp_dir.path().join("repo").join("book");
        let filenames = book_path
        .read_dir()
        .unwrap()
        .map(|de| de.unwrap().file_name().to_str().unwrap().to_owned())
        .filter(|name| name.ends_with(".epub"))
        .collect::<Vec<String>>();

        println!("{filenames:?}");
        let epub_path = book_path.join(&filenames[0]);
        println!("{epub_path:?}");

        let out = PathBuf::from("_site");
        std::fs::create_dir_all(&out).unwrap();

        std::fs::copy(epub_path, out.join(&book.file)).unwrap();

        page.push_str(&format!(r#"<li><a href="{}">{}</a> - [<a href="{}">web</a>]"#, book.file, book.title, book.web));
        match book.buy {
            Some(buy) => page.push_str(&format!(r#" - [<a href="{}">buy</a>]"#, buy)),
            None => {},
        }
        page.push_str(&format!(r#" - [<a href="{}">source</a>]</li>"#, book.repo));
    }


    let html = std::fs::read_to_string("index.html").unwrap();
    let html = html.replace("PLACEHOLDER", &page);

    println!("Creating markdown page");
    std::fs::write("_site/index.html", html).unwrap();

}
