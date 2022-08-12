extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_lyrics_url(query: &str) -> String {
    let lyrics_base = "https://genius.com";
    if query == "lyrics" {
        lyrics_base.to_string()
    } else {
        // Assume the other match is "lyrics query"
        let encoded_query = utf8_percent_encode(&query[7..], FRAGMENT).to_string();
        let lyrics_url = format!("{}/search?q={}", lyrics_base, encoded_query);

        lyrics_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_github_profile_url_with_gh() {
        let fake_query = "lyrics";
        assert_eq!(construct_lyrics_url(fake_query), "https://genius.com");
    }

    #[test]
    fn test_construct_github_profile_url_with_query_url() {
        let fake_query = "lyrics what makes you beautiful";
        assert_eq!(
            construct_lyrics_url(fake_query),
            "https://genius.com/search?q=what%20makes%20you%20beautiful"
        );
    }
}
