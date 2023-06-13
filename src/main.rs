enum Language {
    English,
    French,
    Chinese,
}

fn say_hello(name: &str, language: Language) {
    if let Language::English = language {
        println!("Hello {}!", name);
    } else {
        println!("Sorry, I don't speak that language.")
    }
}

fn main() {
    say_hello("Jonathan", Language::English);
}
