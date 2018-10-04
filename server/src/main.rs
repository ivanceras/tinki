use actix_web::{http,
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
                .content_type(get_content_type(path))
                .body(data)
        }
        Err(e) => {
            HttpResponse::NotFound()
                .finish()
        }
    }
}

fn get_content_type(path: &str)->String{
    let file_path = std::path::Path::new(path);
    let default = "application/octet-stream";
    let content_type = if let Some(ext) = file_path.extension() {
        if let Some(ext) = ext.to_str(){
            match ext{
                "html" => "text/html; charset=utf-8",
                "css" => "text/css; charset=utf-8",
                "js" => "application/javascript; charset=utf-8",
                "md" => "text/plain; charset=utf-8",
                "txt" => "text/plain; charset=utf-8",
                "wasm" => "application/wasm",
                _ => "application/octet-stream",
            }
        }else{
            default
        }
    }else{
        default
    };
    content_type.to_string()
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
            .content_type(get_content_type(path))
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
