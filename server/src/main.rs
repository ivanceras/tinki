use actix_web::{
    http::ContentEncoding,
    server,
    App,
    Path,
    Responder,
    HttpResponse,
    HttpRequest,
    middleware::cors::Cors,
};

use std::path::PathBuf;

use include_dir::Dir;
use include_dir::*;

static DEPLOY_DIR: Dir = include_dir!("../target/deploy/");

fn file(path: &str) -> HttpResponse {
    println!("serving file {}", path);
    match file::get(path){
        Ok(data) => {
            HttpResponse::Ok()
                .body(data)
        }
        Err(e) => {
            HttpResponse::NotFound()
                .finish()
        }
    }
}

fn absolute_file_req(req: &HttpRequest) -> HttpResponse {
    let path : PathBuf = req.match_info().query("path").expect("static file path should be defined");
    let path = path.to_str().unwrap();
    println!("path: {}", path);
    let absolute_path = format!("/{}",path);
    file(&absolute_path)
}

fn file_req(req: &HttpRequest) -> HttpResponse {
    let path : PathBuf = req.match_info().query("path").expect("static file path should be defined");
    let path = path.to_str().unwrap();
    println!("path: {}", path);
    file(&path)
}

fn index(req: &HttpRequest) -> HttpResponse {
    static_file("index.html")
}

fn static_file(path: &str) -> HttpResponse {
    if let Some(file) = DEPLOY_DIR.get_file(&path){
        let content = file.contents();
        println!("file: {}", path);
        HttpResponse::Ok()
            .content_encoding(ContentEncoding::Br)
            .body(content)
    }else{
        HttpResponse::NotFound()
            .finish()
    }
}
fn static_file_req(req: &HttpRequest) -> HttpResponse {
    let path : String = req.match_info().query("static_file").expect("static file path should be defined");
    static_file(&path)
}

fn main() {
    server::new(
        || App::new()
            .configure(|app|{
                       Cors::for_app(app)
                       .send_wildcard()
            .resource("/file/{path:.*}", |r|r.f(file_req))
            .resource("/absolute_file/{path:.*}", |r|r.f(absolute_file_req))
            .resource("/", |r|r.f(index))
            .resource("/{static_file}", |r|r.f(static_file_req))
            .register()
            })
        )
        .bind("0.0.0.0:8080").unwrap()
        .run();
}
