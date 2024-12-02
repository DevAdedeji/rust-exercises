fn main() {
    //
    struct Book {
        title: String,
        author: String,
        pages: u32,
    }

    impl Book {
        fn info(&self) {
            println!(
                "The book '{}' is written by {} and has {} pages",
                self.title, self.author, self.pages
            );
        }
    }

    let first_book = Book {
        title: String::from("Poor Dad, Rich Dad"),
        author: String::from("Robert Kiyosaki"),
        pages: 200,
    };
    first_book.info();
}
