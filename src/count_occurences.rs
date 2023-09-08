use std::{collections::HashMap, fs};

use warp::Filter;

pub fn route(
    project_root: &str,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    root(project_root).or(count())
}

fn root(
    project_root: &str,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let html = fs::read_to_string(format!("{}/src/html/count_occurences.html", project_root)).unwrap();
    warp::path!("count-occurences")
        .and(warp::path::end())
        .map(move || warp::reply::html(crate::build_page("Count Occurences", &html)))
}

fn count() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let count_occurences = warp::path!("count")
        .and(warp::post())
        .and(warp::body::form())
        .map(
            |simple_map: HashMap<String, String>| match (simple_map.get("inputString"), simple_map.get("searchString")) {
                (Some(input), Some(search)) => warp::reply::html(format!(
                    r#"
<tr class="result">
    <td>{}</td>
    <td>{}</td>
    <td class="count">{}</td>
</tr>
                    "#,
                    html_escape::encode_text(input).replace("\n", "<br>"),
                    html_escape::encode_text(search).replace("\n", "<br>"),
                    input.match_indices(search).collect::<Vec<_>>().len()
                )),
                _ => warp::reply::html("<p>Missing request</p>".to_owned()),
            },
        );
    warp::path("count-occurences").and(count_occurences)
}
