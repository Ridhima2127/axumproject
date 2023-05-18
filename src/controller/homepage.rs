
use std::fs;
use axum_core::response::IntoResponse;
use hyper::body::HttpBody;
use r#typeof::type_of;
use serde_json::json;
use crate::model::database::{posts, select_posts};
//use crate::model::database::{posts, select_posts, connection};

pub async fn get_all_posts() -> impl IntoResponse
{

    let mut handlebars= handlebars::Handlebars::new();

    let index_template = fs::read_to_string("templates/index.hbs").unwrap();
    handlebars
        .register_template_string("index", &index_template).expect("message");


    //let all_posts_to_front_end=connection().await.expect("message");

   let all_posts_in_struct:Vec<posts>=select_posts().await.expect("message");

    println!("ASDasds");
    let html = handlebars.render("index", &json!({ "q":&all_posts_in_struct})).unwrap() ;




    println!("{}", html );

return html;








}

