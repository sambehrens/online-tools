use std::fs;

use warp::Filter;

mod count_occurences;
mod md5;
mod sort_alphabetically;
mod sort_numerically;

#[tokio::main]
async fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let html = fs::read_to_string(format!("{}/html/index.html", root)).unwrap();
    let index = warp::path::end().map(move || warp::reply::html(build_page("Online Tools", &html)));
    let clicked = warp::path("clicked").map(|| warp::reply::html("<p>You clicked me</p>"));

    let routes = index
        .or(clicked)
        .or(md5::route(&root))
        .or(count_occurences::route(&root))
        .or(sort_alphabetically::route(&root))
        .or(sort_numerically::route(&root));

    warp::serve(routes)
        .run(([0, 0, 0, 0, 0, 0, 0, 0], 3030))
        .await;
}

fn build_page(title: &str, content: &str) -> String {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let html = fs::read_to_string(format!("{}/html/page_template.html", root)).unwrap();
    let html = html.replace("{title}", title);
    let html = html.replace("{body}", content);
    html
}

