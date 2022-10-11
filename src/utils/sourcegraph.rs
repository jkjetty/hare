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

pub fn construct_sourcegraph_search_url(query: &str) -> String {
    if query == "sg" {
        let sourcegraph_dotcom = "https://sourcegraph.com/search";

        sourcegraph_dotcom.to_string()

    // Peel off the prefix for sourcegraph search
    } else if &query[..3] == "sg " {
        construct_sourcegraph_search(&query[3..])
    } else {
        // Just search the query on sourcegraph
        construct_sourcegraph_search(&query)
    }
}

pub fn construct_sourcegraph_search(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let sourcegraph_search_url = format!(
        "https://sourcegraph.com/search?q=context:global+{}+",
        encoded_query
    );

    sourcegraph_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_sourcegraph_search_url() {
        let fake_query = "hello";
        assert_eq!(
            construct_sourcegraph_search_url(fake_query),
            "https://sourcegraph.com/search?q=context:global+hello+"
        );
    }

    #[test]
    fn test_construct_sourcegraph_search_url_with_g() {
        let fake_query = "sg hello";
        assert_eq!(
            construct_sourcegraph_search_url(fake_query),
            "https://sourcegraph.com/search?q=context:global+hello+"
        );
    }

    #[test]
    fn test_construct_sourcegraph_search_url_with_encoding() {
        let fake_query = "hello world";
        assert_eq!(
            construct_sourcegraph_search_url(fake_query),
            "https://sourcegraph.com/search?q=context:global+hello%20world+"
        );
    }

    #[test]
    fn test_construct_sourcegraph_search_url_with_encoding_complex() {
        let fake_query = "#[serde(transparent)]";
        assert_eq!(
            construct_sourcegraph_search_url(fake_query),
            "https://sourcegraph.com/search?q=context:global+%23%5Bserde%28transparent%29%5D+"
        );
    }
}
