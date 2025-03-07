# [Rust ebooks for Amazon Kindle](https://rust-ebooks.code-maven.com/)


Build the books and the site:


```
cargo install mdbook-epub
cargo run
```

To view the site locally install [Rustatic](https://rustatic.code-maven.com/) and run

```
rustatic --path _site --nice --indexfile index.html
```
