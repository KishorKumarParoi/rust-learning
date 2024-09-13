type Document = Vec<String>;

fn new_document(words: Vec<String>) -> Document {
    words
}

fn add_word(this: &mut Document, word: String) {
    this.push(word);
}

fn get_words(this: &Document) -> &[String] {
    this.as_slice()
}

fn main() {
    let mut doc = new_document(vec!["hello".to_string(), "world".to_string()]);
    println!("{:?}", doc);
    add_word(&mut doc, "Rust".to_string());
    let words = get_words(&doc);
    for word in words {
        println!("{}", word);
    }
}
