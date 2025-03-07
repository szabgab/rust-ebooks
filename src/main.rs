use std::{path::PathBuf, process::Command};

use chrono::{DateTime, Utc};
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
    let start = chrono::Utc::now();

    let mut page = String::new();
    for book in books {
        let tmp_dir = TempDir::new("example").unwrap();
        let path_to_git_workspace = tmp_dir.path().join("repo").display().to_string();
        println!("Cloning '{}' to {}", book.repo, path_to_git_workspace);

        let mut cmd = Command::new("git");
        cmd.args(["clone", "--depth", "1", &book.repo, &path_to_git_workspace]);
        println!("cmd: {:?}", cmd);

        let _result = cmd.output().expect("command failed to start");

        println!("Running mdbook-epub");
        let path_to_ebook_source = PathBuf::from(path_to_git_workspace).join(&book.folder);
        let mut cmd = Command::new("mdbook-epub");
        cmd.args(["--standalone", &path_to_ebook_source.display().to_string()]);
        println!("cmd: {:?}", cmd);

        let _result = cmd.output().expect("command failed to start");

        let book_path = tmp_dir.path().join("repo").join("book");
        println!("book_path: {:?}", book_path);
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

        page.push_str(&format!(
            r#"<li><a href="{}">{}</a> - [<a href="{}">web</a>]"#,
            book.file, book.title, book.web
        ));
        match book.buy {
            Some(buy) => page.push_str(&format!(r#" - [<a href="{}">buy</a>]"#, buy)),
            None => {}
        }
        page.push_str(&format!(r#" - [<a href="{}">source</a>]</li>"#, book.repo));
    }

    let end = chrono::Utc::now();

    generate_page("index.html", "_site/index.html", &page, start, end);
}

fn generate_page(
    template_filename: &str,
    html_filename: &str,
    content: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) {
    println!("Creating HTML page");
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse_file(template_filename)
        .unwrap();

    let globals = liquid::object!({
        "content": content,
        "generated_at": end.format("%Y-%m-%d %H:%M:%S").to_string(),
        "elapsed_time": (end - start).num_seconds().to_string(),
    });
    let output = template.render(&globals).unwrap();

    std::fs::write(html_filename, output).unwrap();
}
