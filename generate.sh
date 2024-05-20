set -ex

mkdir books
mkdir -p _site/books
git clone --depth 1 https://github.com/rust-lang/book books/book
mdbook-epub -s true books/book
mv books/book/book _site/books/book
