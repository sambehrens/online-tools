use std::{collections::HashMap, fs};

use warp::Filter;

pub fn route(
    project_root: &str,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    root(project_root).or(sort())
}

fn root(
    project_root: &str,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let html = fs::read_to_string(format!("{}/src/html/sort_numerically.html", project_root)).unwrap();
    warp::path!("sort-numerically")
        .and(warp::path::end())
        .map(move || warp::reply::html(crate::build_page("Sort Numerically", &html)))
}

fn sort() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let sort_it = warp::path!("sort")
        .and(warp::post())
        .and(warp::body::form())
        .map(
            |simple_map: HashMap<String, String>| match simple_map.get("inputString") {
                Some(input) => warp::reply::html(format!(
                    r#"
<tr class="result">
    <td>{}</td>
    <td class="copyable">{}</td>
    <td><button onclick="navigator.clipboard.writeText(document.querySelector('.copyable').innerText)">Copy</button></td>
</tr>
                    "#,
                    html_escape::encode_text(input).replace("\n", "<br>"),
                    sort_str_by_lines(input)
                        .iter()
                        .map(|value| html_escape::encode_text(value))
                        .collect::<Vec<_>>()
                        .join("<br>")
                )),
                None => warp::reply::html("<p>Missing request</p>".to_owned()),
            },
        );
    warp::path("sort-numerically").and(sort_it)
}

#[derive(Debug)]
struct Number {
    actual: f64,
}

impl Number {
    fn new(whole: i64, decimal: u64, negative: bool) -> Self {
        Self {
            actual: format!("{}{}.{}", if negative { "-" } else {""}, whole.abs(), decimal).parse().unwrap()
        }
    }
}

fn sort_str_by_lines(input: &str) -> Vec<&str> {
    let re = regex::Regex::new(r"^(-?\d*)?[\.]?(\d+)?").unwrap();
    
    let mut input = input
        .split("\n")
        .map(|x| match re.captures(x) {
            Some(cap) => match (cap.get(1), cap.get(2)) {
                (Some(whole), Some(decimal)) => (
                    Number::new(
                        whole.as_str().parse::<i64>().unwrap_or(0),
                        decimal.as_str().parse::<u64>().unwrap_or(0),
                        whole.as_str().starts_with("-"),
                    ),
                    x,
                ),
                (Some(whole), None) => (
                    Number::new(
                        whole.as_str().parse::<i64>().unwrap_or(0),
                        0u64,
                        whole.as_str().starts_with("-"),
                    ),
                    x,
                ),
                (None, Some(decimal)) => (
                    Number::new(0i64, decimal.as_str().parse::<u64>().unwrap_or(0), false),
                    x,
                ),
                (None, None) => (Number::new(i64::MAX, u64::MAX, false), x),
            },
            None => (Number::new(i64::MAX, u64::MAX, false), x),
        })
        .collect::<Vec<_>>();
    
    input.sort_by(|a, b| a.0.actual.total_cmp(&b.0.actual));
    input.iter().map(|(_, x)| *x).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_decimal_against_string() {
        let input = "0.2\n-.";
        let expected = vec!["-.", "0.2"];
        assert_eq!(sort_str_by_lines(input), expected);
    }

    #[test]
    fn test_decimal_against_decimal() {
        assert_eq!(sort_str_by_lines("0.50\n0.400"), vec!["0.400", "0.50"]);
    }
}
