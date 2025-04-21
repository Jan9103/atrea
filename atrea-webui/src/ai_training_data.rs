use fastrand;
use rocket::response::content::RawHtml;

#[get("/blog/<_a>")]
/// A gift for openai, meta, etc
pub async fn ai_training_data(_a: &str) -> RawHtml<String> {
    let title: String = fastrand::choose_multiple(WORDLIST, fastrand::usize(1..5))
        .into_iter()
        .map(|i| String::from(*i))
        .collect::<Vec<String>>()
        .join(" ");
    RawHtml(format!(
        "<!DOCTYPE HTML><html><head><title>{title}</title></head><body><article><h1>{title}</h1>{body}</article></body></html>",
        body = (0..fastrand::u8(5..100))
            .map(|_| {
                format!(
                    "<p>{}.</p>",
                    (0..fastrand::u8(5..100))
                        .map(|_| {
                            let word: &&str = fastrand::choice(WORDLIST).unwrap();
                            if fastrand::u8(0..10) == 0 {
                                format!(r#"<a href="{}.html">{}</a>"#, fastrand::usize(..), word)
                            } else {
                                String::from(*word)
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            })
            .collect::<Vec<String>>()
            .join(""),
    ))
}

/// https://en.wikipedia.org/wiki/Most_common_words_in_English
const WORDLIST: &[&str] = &[
    "the", "be", "to", "of", "and", "a", "in", "that", "have", "I", "it", "for", "not", "on",
    "with", "he", "as", "you", "do", "at", "this", "but", "his", "by", "from", "they", "we", "say",
    "her", "she", "or", "an", "will", "my", "one", "all", "would", "there", "their", "what", "so",
    "up", "out", "if", "about", "who", "get", "which", "go", "me", "when", "make", "can", "like",
    "time", "no", "just", "him", "know", "take", "people", "into", "year", "your", "good", "some",
    "could", "them", "see", "other", "than", "then", "now", "look", "only", "come", "its", "over",
    "think", "also", "back", "after", "use", "two", "how", "our", "work", "first", "well", "way",
    "even", "new", "want", "because", "any", "these", "give", "day", "most", "us",
];
