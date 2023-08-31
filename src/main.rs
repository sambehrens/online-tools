use std::{collections::HashMap, fs};

use warp::{filters::compression, Filter};

#[tokio::main]
async fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let html = fs::read_to_string(format!("{}/html/index.html", root)).unwrap();
    let index = warp::path::end().map(move || warp::reply::html(html.clone()));
    let clicked = warp::path("clicked").map(|| warp::reply::html("<p>You clicked me</p>"));

    let html = fs::read_to_string(format!("{}/html/md5.html", root)).unwrap();
    let md5_root = warp::path("md5")
        .and(warp::path::end())
        .map(move || warp::reply::html(html.clone()));
    let md5_encode = warp::path("encode")
        .and(warp::post())
        .and(warp::body::form())
        .map(
            |simple_map: HashMap<String, String>| match simple_map.get("inputString") {
                Some(request) => warp::reply::html(format!(
                    r#"<tr class="result"><td>{}</td><td>{:x}</td></tr>"#,
                    request,
                    md5::compute(request)
                )),
                None => warp::reply::html("<p>Missing request</p>".to_owned()),
            },
        );
    let md5_encode = warp::path("md5").and(md5_encode);

    let routes = index
        .or(clicked)
        .or(md5_root)
        .or(md5_encode)
        .with(compression::brotli());

    warp::serve(routes)
        .run(([0, 0, 0, 0, 0, 0, 0, 0], 3030))
        .await;
}
