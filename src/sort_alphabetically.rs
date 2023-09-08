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
    let html =
        fs::read_to_string(format!("{}/src/html/sort_alphabetically.html", project_root)).unwrap();
    warp::path!("sort-alphabetically")
        .and(warp::path::end())
        .map(move || warp::reply::html(crate::build_page("Sort Alphabetically", &html)))
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
                    sort_string(input)
                )),
                None => warp::reply::html("<p>Missing request</p>".to_owned()),
            },
        );
    warp::path("sort-alphabetically").and(sort_it)
}

fn sort_string(input: &str) -> String {
    let escaped = html_escape::encode_text(input);
    let mut input = escaped.split("\n").collect::<Vec<_>>();
    input.sort();
    input.join("<br>")
}
