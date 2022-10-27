extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'[')
    .add(b']')
    .add(b'#')
    .add(b'(')
    .add(b')');

pub fn construct_crates_search_url(query: &str) -> String {
    if query == "crates" {
        let crates_dotcom = "https://crates.io";

        crates_dotcom.to_string()

    // Peel off the prefix for crates.io search
    } else if &query[..6] == "crates" {
        construct_crates_search(&query[7..])
    } else {
        // Just search the query on crates.io
        construct_crates_search(&query)
    }
}

pub fn construct_crates_search(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let crates_search_url = format!("https://crates.io/search?q={}", encoded_query);

    crates_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_crates_search_url() {
        let fake_query = "crates";
        assert_eq!(construct_crates_search_url(fake_query), "https://crates.io");
    }

    #[test]
    fn test_construct_crates_search_url_with_g() {
        let fake_query = "crates hello";
        assert_eq!(
            construct_crates_search_url(fake_query),
            "https://crates.io/search?q=hello"
        );
    }

    #[test]
    fn test_construct_crates_search_url_with_encoding() {
        let fake_query = "hello world";
        assert_eq!(
            construct_crates_search_url(fake_query),
            "https://crates.io/search?q=hello%20world"
        );
    }
}
