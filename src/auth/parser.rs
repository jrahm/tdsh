/// Abstraction over a URL's path (Everything after the domain).
///
/// This will include the path and query parameters. The path is represented as
/// a `Vec<String>` while the query parameters are a `HashMap<String, String>`.
#[derive(Debug)]
pub struct Url {
    path: Vec<String>,
    query_params: std::collections::HashMap<String, String>,
}

// Splits a string once.
fn split_at(in_str: &str, ch: char) -> (&str, &str) {
    let mut splitter = in_str.splitn(2, ch);

    (splitter.next().unwrap_or(""), splitter.next().unwrap_or(""))
}

fn to_query_param_pair(in_str: &str) -> (&str, &str) {
    split_at(in_str, '=')
}

/// Parse a raw URL into an abstract `Url` structure.
pub fn parse_url(raw: &str) -> Url {
    let (path, params) = split_at(raw, '?');

    Url {
        path: path.split('/').map(|a| String::from(a)).collect(),
        query_params: params
            .split('&')
            .map(|p| to_query_param_pair(p))
            .map(|(k, v)| {
                (
                    String::from(
                    percent_encoding::percent_decode_str(k)
                        .decode_utf8()
                        .unwrap()),
                        String::from(
                    percent_encoding::percent_decode_str(v)
                        .decode_utf8()
                        .unwrap()),
                )
            })
            .collect(),
    }
}

pub fn get_param<S: std::string::ToString>(url: &Url, key: &S) -> Option<String> {
    return url.query_params.get(&key.to_string()).cloned();
}
