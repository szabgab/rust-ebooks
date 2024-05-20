set -ex

mkdir books
git clone --depth 1 https://github.com/rust-lang/book books/book
mdbook-epub -s true books/book
ls -l books/book/book
