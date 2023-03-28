
use std::sync::Arc;

use handlebars::{Handlebars, template};
use serde::Serialize;
use serde_json::json;
use warp::Filter;

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars<'_>>) -> impl warp::Reply
where
    T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

//https://blog.csdn.net/lzsan_/article/details/123905640

#[tokio::main]
async fn main() {
    // let template = "<!DOCTYPE html>
    //                 <html>
    //                   <head>
    //                     <title>Warp Handlebars template example</title>
    //                   </head>
    //                   <body>
    //                     <h1>Hello {{user}}!</h1>
    //                   </body>
    //                 </html>";
    let template_1 = include_bytes!("./static/welcome.htm");
    let template_1 = String::from_utf8_lossy(template_1).to_owned();

    let mut hb = Handlebars::new();
    // register the template
    hb.register_template_string("template.html", template_1)
        .unwrap();

    // Turn Handlebars instance into a Filter so we can combine it
    // easily with others...
    let hb = Arc::new(hb);

    // Create a reusable closure to render template
    let handlebars = move |with_template| render(with_template, hb.clone());

    //GET /
    let route = warp::get()
        .and(warp::path::end())
        .map(|| WithTemplate {
            name: "template.html",
            value: json!({"user" : "Warp"}),
        })
        .map(handlebars);

    // dir already requires GET...
    let examples = warp::path("static").and(warp::fs::dir("./www/"));

    warp::serve(route.or(examples)).run(([0, 0, 0, 0], 8080)).await;
}   
