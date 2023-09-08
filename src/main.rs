use std::fs;

use warp::Filter;

mod count_occurences;
mod hashers;
mod sort_alphabetically;
mod sort_numerically;
mod simple_form;

#[tokio::main]
async fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let html = fs::read_to_string(format!("{}/src/html/index.html", root)).unwrap();
    let index = warp::path::end().map(move || warp::reply::html(build_page("Online Tools", &html)));
    let clicked = warp::path("clicked").map(|| warp::reply::html("<p>You clicked me</p>"));
    
    let routes = index
        .or(clicked)
        .or(hashers::MD5.route())
        .or(hashers::SHA1.route())
        .or(count_occurences::route(&root))
        .or(sort_alphabetically::route(&root))
        .or(sort_numerically::route(&root));

    warp::serve(routes)
        .run(([0, 0, 0, 0, 0, 0, 0, 0], 3030))
        .await;
}

fn build_page(title: &str, content: &str) -> String {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let html = fs::read_to_string(format!("{}/src/html/page_template.html", root)).unwrap();
    let html = html.replace("{title}", title);
    let html = html.replace("{body}", content);
    html
}

// ideas
// - rcv - includes db probably (doesn't have to at first)
// - explain regex - hardest
// - build curl - somewhat easyish
// - aws cli command builder - useful
// - wifi speed test - interesting to learn
// - make page generation dynamic - hard
// - color picker
//- the rest of the hashers
