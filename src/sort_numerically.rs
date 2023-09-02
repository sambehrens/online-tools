use std::{cmp::Ordering, collections::HashMap, fs};

use warp::Filter;

pub fn route(
    project_root: &str,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    root(project_root).or(sort())
}

fn root(
    project_root: &str,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let html = fs::read_to_string(format!("{}/html/sort_numerically.html", project_root)).unwrap();
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

struct Number {
    negative: bool,
    whole: i64,
    decimal: u64,
}

impl Number {
    fn new(whole: i64, decimal: u64, negative: bool) -> Self {
        Self {
            negative,
            whole,
            decimal,
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
    input.sort_by(|a, b| match a.0.whole.cmp(&b.0.whole) {
        Ordering::Equal => match a.0.decimal.cmp(&b.0.decimal) {
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => match a.0.negative {
                true => Ordering::Greater,
                false => Ordering::Less,
            },
            Ordering::Greater => match a.0.negative {
                true => Ordering::Less,
                false => Ordering::Greater,
            },
        },
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
    });
    input.iter().map(|(_, x)| *x).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_string() {
        assert_eq!(sort_str_by_lines("3a\n2b\n1c"), vec!["1c", "2b", "3a"])
    }
}
