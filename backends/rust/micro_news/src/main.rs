use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::header, middleware};
use reqwest::Error;
use serde_json::Value;
use env_logger::Env;

const API_KEY: &str = "8dade0feadcf43e285e215cc7271de9c";

async fn fetch_data(url: String) -> Result<Value, Error> {
    let client = reqwest::Client::builder()
        .user_agent("Micro News")
        .build()?;

    let response = client.get(&url).send().await?;
    let data: Value = response.json().await?;
    println!("{:#?}", data);
    Ok(data)
}

async fn get_article(info: web::Path<(String,)>) -> impl Responder {
    let title = &info.0;
    let url = format!("https://newsapi.org/v2/everything?apiKey={}&qInTitle={}", API_KEY, title);
    let article = fetch_data(url).await;
    match article {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_author_articles(info: web::Path<(String,)>) -> impl Responder {
    let author = &info.0;
    let url = format!("https://newsapi.org/v2/everything?language=en&apiKey={}&q={}", API_KEY, author);
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
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::ACCESS_CONTROL_ALLOW_HEADERS])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(
                middleware::Logger::new("%a %{User-Agent}i %b %D %r %s")
            )
            .service(web::resource("/api/getArticle/{title}").to(get_article))
            .service(web::resource("/api/getAuthorArticles/{author}").to(get_author_articles))
            .service(web::resource("/api/getTopHeadlines").to(get_top_headlines))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
