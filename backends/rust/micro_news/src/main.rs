use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::header};
use reqwest::Error;
use serde_json::Value;

const API_KEY: &str = "8dade0feadcf43e285e215cc7271de9c";

async fn fetch_data(url: String) -> Result<Value, Error> {
    let response = reqwest::get(&url).await?;
    let data: Value = response.json().await?;
    println!("{:#?}", data);
    Ok(data)
}

async fn get_article(title: String) -> impl Responder {
    let url = format!("https://newsapi.org/v2/everything?q={}&apiKey={}", title, API_KEY);
    let article = fetch_data(url).await;
    match article {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_author_articles(author: String) -> impl Responder {
    let url = format!("https://newsapi.org/v2/everything?sources={}&apiKey={}", author, API_KEY);
    let articles = fetch_data(url).await;
    match articles {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_top_headlines() -> impl Responder {
    let url = format!("https://newsapi.org/v2/top-headlines?country=us&apiKey={}", API_KEY);
    let headlines = fetch_data(url).await;
    match headlines {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::ACCESS_CONTROL_ALLOW_HEADERS])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(web::resource("/api/getArticle/{title}").to(get_article))
            .service(web::resource("/api/getAuthorArticles/{author}").to(get_author_articles))
            .service(web::resource("/api/getTopHeadlines").to(get_top_headlines))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
