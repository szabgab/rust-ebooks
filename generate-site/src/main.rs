use std::{
    os::unix::process::ExitStatusExt, path::PathBuf, process::{Command, ExitStatus}
};


use serde::Deserialize;
use serde_yaml;
use tempdir::TempDir;

#[derive(Debug, Deserialize)]
struct Book {
    repo: String,
    title: String,
    file: String,
}

fn main() {
    let book_yaml_file = "../books.yaml";
    let content = std::fs::read_to_string(book_yaml_file).unwrap();
    let books: Vec<Book> = serde_yaml::from_str(&content).unwrap();
    println!("{:#?}", books);

    let mut page = std::fs::read_to_string("index.md").unwrap();

    for book in books {
        let tmp_dir = TempDir::new("example").unwrap();
        let path = tmp_dir.path().join("repo").display().to_string();
        println!("{path}");
        let _result = Command::new("git")
        .args(["clone", "--depth", "1", &book.repo, &path])
        .output()
        .expect("command failed to start");

        let _result = Command::new("mdbook-epub")
        .args(["-s", "true", &path])
        .output()
        .expect("command failed to start");

        let book_path = tmp_dir.path().join("repo").join("book");
        let filenames = book_path
        .read_dir()
        .unwrap()
        .map(|de| de.unwrap().file_name().to_str().unwrap().to_owned())
        .collect::<Vec<String>>();

        println!("{filenames:?}");
        let epub_path = book_path.join(&filenames[0]);
        println!("{epub_path:?}");

        let out = PathBuf::from("../_site/books");
        std::fs::create_dir_all(&out).unwrap();

        std::fs::copy(epub_path, out.join(&book.file)).unwrap();

        page.push_str(&format!("* [{}](/books/{}) [source]({})", book.title, book.file, book.repo));
    }

    std::fs::write("../site/pages/index.md", page).unwrap();

}
