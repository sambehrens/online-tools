use std::collections::HashMap;

use warp::Filter;

const SIMPLE_FORM_TEMPLATE: &str = include_str!("html/simple_form.html");

pub struct SimpleForm<F>
where
    F: (Fn(&str) -> String) + Sync,
{
    pub title: &'static str,
    pub heading: &'static str,
    pub root_route: &'static str,
    pub action_route: &'static str,
    pub input_placholder: &'static str,
    pub action_button_text: &'static str,
    pub result_table_header: &'static str,
    pub process: F,
}

impl<F> SimpleForm<F>
where
    F: (Fn(&str) -> String) + Sync,
{
    // pub fn new(
    //     title: &'static str,
    //     heading: &'static str,
    //     root_route: &'static str,
    //     action_route: &'static str,
    //     input_placholder: &'static str,
    //     action_button_text: &'static str,
    //     result_table_header: &'static str,
    //     process: F,
    // ) -> Self {
    //     Self {
    //         title,
    //         heading,
    //         root_route,
    //         action_route,
    //         input_placholder,
    //         action_button_text,
    //         result_table_header,
    //         process,
    //     }
    // }

    pub fn route(
        &self,
    ) -> impl Filter<Extract = (impl warp::Reply + '_,), Error = warp::Rejection> + Clone + '_ {
        self.root().or(self.action())
    }

    fn root(
        &self,
    ) -> impl Filter<Extract = (impl warp::Reply + '_,), Error = warp::Rejection> + Clone + '_ {
        warp::path(&self.root_route)
            .and(warp::path::end())
            .map(move || warp::reply::html(crate::build_page(&self.title, &self.generate_html())))
    }

    fn generate_html(&self) -> String {
        SIMPLE_FORM_TEMPLATE
            .replace("{{heading}}", &self.heading)
            .replace(
                "{{action_route}}",
                &format!("{}/{}", self.root_route, self.action_route),
            )
            .replace("{{input_placeholder}}", &self.input_placholder)
            .replace("{{action_button_text}}", &self.action_button_text)
            .replace("{{result_table_header}}", &self.result_table_header)
    }

    fn action(
        &self,
    ) -> impl Filter<Extract = (impl warp::Reply + '_,), Error = warp::Rejection> + Clone + '_ {
        let action_path = warp::path(&self.action_route)
            .and(warp::post())
            .and(warp::body::form())
            .map(
                |simple_map: HashMap<String, String>| match simple_map.get("inputString") {
                    Some(request) => warp::reply::html(format!(
                        r#"
    <tr class="result">
        <td>{}</td>
        <td class="copyable">{}</td>
        <td><button onclick="navigator.clipboard.writeText(document.querySelector('.copyable').innerText)">Copy</button></td>
    </tr>
                        "#,
                        html_escape::encode_text(request).replace("\n", "<br>"),
                        (self.process)(request)
                    )),
                    None => warp::reply::html("<p>Missing request</p>".to_owned()),
                },
            );
        warp::path(&self.root_route).and(action_path)
    }
}
