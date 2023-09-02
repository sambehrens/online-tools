use std::{collections::HashMap, fs};

use warp::Filter;

pub fn route(
    project_root: &str,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    root(project_root).or(encode())
}

fn root(
    project_root: &str,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let html = fs::read_to_string(format!("{}/html/md5.html", project_root)).unwrap();
    warp::path!("md5")
        .and(warp::path::end())
        .map(move || warp::reply::html(crate::build_page("MD5 Encode", &html)))
}

fn encode() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let md5_encode = warp::path!("encode")
        .and(warp::post())
        .and(warp::body::form())
        .map(
            |simple_map: HashMap<String, String>| match simple_map.get("inputString") {
                Some(request) => warp::reply::html(format!(
                    r#"
<tr class="result">
    <td>{}</td>
    <td class="copyable">{:x}</td>
    <td><button onclick="navigator.clipboard.writeText(document.querySelector('.copyable').innerText)">Copy</button></td>
</tr>
                    "#,
                    html_escape::encode_text(request).replace("\n", "<br>"),
                    md5::compute(request)
                )),
                None => warp::reply::html("<p>Missing request</p>".to_owned()),
            },
        );
    warp::path("md5").and(md5_encode)
}
