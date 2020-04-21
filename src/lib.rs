use log::*;
use sauron::{prelude::*, Cmd, Component, Node, Program};
use std::collections::BTreeMap;
use url_path::UrlPath;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub enum Msg {
    UrlChanged(String),
    FileReady(Result<String, JsValue>),
}

pub struct App {
    raw: String,
    content: String,
    title: Option<String>,
    current_file: UrlPath,
}

impl App {
    fn new() -> Self {
        let hash = Browser::get_hash();
        let current_file = UrlPath::new(Self::clean_link(&hash));
        App {
            raw: String::new(),
            content: String::new(),
            title: None,
            current_file,
        }
    }

    fn clean_link(url: &str) -> &str {
        let url = url.trim_start_matches("#/");
        let url = url.trim_start_matches("#");
		if url.trim().is_empty(){
			"index.md"
		}else{
			url
		}
    }

    fn fetch_current_file(&self) -> Cmd<App, Msg> {
        let url = self.current_file.normalize();
        trace!("fetching {}", url);
        Http::fetch_with_text_response_decoder(&url, |v| v, Msg::FileReady)
    }

    fn markdown_to_html(&mut self) {
        let embed_files = BTreeMap::new();
        if let Ok(html) = spongedown::parse_with_base_dir(
            &self.raw,
            &self.current_file.parent().unwrap_or("/".to_string()),
            &Some(embed_files),
        ) {
            self.title = html.title;
            self.content = html.content;
            if let Some(title) = &self.title{
                Window::set_title(&title);
            }
        }
    }
}

impl Component<Msg> for App {
    fn init(&self) -> Cmd<App, Msg> {
        Cmd::batch(vec![
            Browser::onhashchange(Msg::UrlChanged),
            self.fetch_current_file(),
        ])
    }
    fn update(&mut self, msg: Msg) -> Cmd<App, Msg> {
        match msg {
            Msg::UrlChanged(url) => {
                let url = Self::clean_link(&url);
                trace!("url changed: {}", url);
                self.current_file = UrlPath::new(url);
                self.fetch_current_file()
            }
            Msg::FileReady(file) => {
                Browser::scroll_to_top();
                match file {
                    Ok(raw) => {
                        trace!("ok got file");
                        self.raw = raw;
                        self.markdown_to_html();
                    }
                    Err(e) => error!("display 404 here"),
                }
                Cmd::none()
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![],
            vec![
                nav(vec![], vec![a(vec![href("/")], vec![text("Home")])]),
                article(vec![inner_html(&self.content)], vec![]),
            ],
        )
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).unwrap();
    Program::mount_to_body(App::new());
}
