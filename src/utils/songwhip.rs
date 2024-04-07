extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b'|')
    .add(b'?')
    .add(b'*')
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

pub fn construct_songwhip_search_url(query: &str) -> String {
    if query == "sw" {
        let songwhip_dotcom = "https://songwhip.com/create";

        songwhip_dotcom.to_string()

    // Peel off the prefix for songwhip search
    } else if &query[..3] == "sw " {
        construct_songwhip_search(&query[3..])
    } else {
        // Just search the query on songwhip
        construct_songwhip_search(&query)
    }
}

pub fn construct_songwhip_search(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let songwhip_search_url = format!("https://songwhip.com/create?q={}", encoded_query);

    songwhip_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_songwhip_search_url() {
        let fake_query = "hello";
        assert_eq!(
            construct_songwhip_search_url(fake_query),
            "https://songwhip.com/create?q=hello"
        );
    }

    #[test]
    fn test_construct_songwhip_search_url_with_g() {
        let fake_query = "sw hello";
        assert_eq!(
            construct_songwhip_search_url(fake_query),
            "https://songwhip.com/create?q=hello"
        );
    }

    #[test]
    fn test_construct_songwhip_search_url_with_encoding() {
        let fake_query = "hello world";
        assert_eq!(
            construct_songwhip_search_url(fake_query),
            "https://songwhip.com/create?q=hello%20world"
        );
    }
}
