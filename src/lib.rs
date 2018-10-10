use failure::*;
use stdweb::web::Location;
use stdweb::web::window;
use stdweb::web::Node;
use stdweb::*;
use stdweb::unstable::TryFrom;

use yew::*;
use yew::prelude::*;
use log::*;
use self::route_service::RouteService;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::Task;
use yew::format::Nothing;

use yew::virtual_dom::VNode;
use stdweb::web::document;
use stdweb::web::IParentNode;
use stdweb::web::event::IEvent;
use stdweb::web::IEventTarget;
use stdweb::web::IElement;
use stdweb::web::Element;
use stdweb::web::INode;
use url::Url;
use std::path::PathBuf;
use url_path::UrlPath;

mod route_service;
mod inject;

static MINIMAL_CSS: &'static str = include_str!("../static/minimal.css");


pub struct Model{
    route_service: RouteService<String>,
    fetch_service: FetchService,
    fetch_callback: Callback<Result<String, Error>>,
    task: Option<FetchTask>,
    content: String,
    current_file: Option<UrlPath>,

}

pub enum Msg{
    UrlChanged(String),
    FileReady(Result<String,Error>),
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|url: String| Msg::UrlChanged(url));
        let mut route_service = RouteService::new();
        route_service.register_callback(callback);
        let fetch_callback = link.send_back(|file: Result<String,Error>| Msg::FileReady(file));
        let fetch_service = FetchService::new();
        let mut model = Model {
            route_service,
            fetch_callback,
            fetch_service,
            task: None,
            content: "".to_string(),
            current_file: None,
        };
        inject::inject_css(MINIMAL_CSS);
        inject::set_body_class("markdown-body");
        model.set_current_file();
        model.fetch_current_file();
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::UrlChanged(url) => {
                self.set_current_file_with(&url);
                self.fetch_current_file();
            }
            Msg::FileReady(file) => {
                if let Ok(raw) = file{
                    if let Ok(html) = self.md_to_html(&raw){
                        self.content = html;
                    }else{
                        self.content = "Error parsing html".to_string();
                    }
                }else{
                    self.content = format!("Error reading file {:?}", file);
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("in change");
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let article = format!("<article>{}</article>", &self.content);
        let node = Node::from_html(&article).expect("Error converting html to node");
        let sponge = VNode::VRef(node);
        let html = html! {
            <div>
                <nav>
                    <a href="/#Home.md",>
                        {"Home"}
                    </a>
                </nav>
                { sponge }
            </div>
        };
        html
    }
}

impl Model{

    fn reconstruct_file_url(&self, file: &str) -> String {
        let (is_absolute, is_external) = match self.current_file{
            Some(ref current_file) => (current_file.is_absolute(), current_file.is_external()),
            None => (false, false),
        };
        let url = if is_external{
            file.to_string()
        }else{
            format!("/{}", file)
        };
        url
    }

    fn fetch_file(&mut self, file: &str) -> FetchTask {
        let url = self.reconstruct_file_url(file);
        let fetch_callback = self.fetch_callback.clone();
        let handler = move |response: Response<Result<String, Error>>| {
            info!("got file response");
            let (meta, data) = response.into_parts();
            if meta.status.is_success() {
                fetch_callback.emit(data)
            } else {
                info!("fail!");
                // format_err! is a macro in crate `failure`
                fetch_callback.emit(Err(format_err!(
                    "{}: error getting file ",
                    meta.status
                )))
            }
        };
        info!("building the request... {}", url);
        if let Ok(request) = Request::get(&url).body(Nothing){
            self.fetch_service.fetch(request, handler.into())
        }else{
            error!("Unable to build request");
            panic!("Unable to build request");
        }
    }

    /// fetch the file that is set in the current_dir and current_file
    fn fetch_current_file(&mut self) {
        if let Some(ref current_file) = self.current_file{
            let normalized:String = current_file.normalize();
            info!("fetching file: {}", normalized);
            let task = self.fetch_file(&normalized);
            self.task = Some(task);
        }else{
            error!("No current file set");
        }
    }

    /// set the current file based on the current route
    fn set_current_file(&mut self) {
        let url = self.route_service.get_route();
        self.set_current_file_with(&url);
    }

    /// set the base directory and current file based on the supplied url
    fn set_current_file_with(&mut self, url: &str) {
        let trim1 = url.trim_left_matches("/index.html#");
        info!("trim1: {}", trim1);
        let trim2 = trim1.trim_left_matches("/#");
        info!("trim2: {}", trim2);
        let url_path = if trim2.is_empty() || trim2 == "/"{
            UrlPath::new("Home.md") // default file to fetch when url is empty
        }else{
            UrlPath::new(&trim2)
        };
        self.current_file = Some(url_path);
    }

    fn get_current_dir(&self) -> Option<String> {
        match self.current_file{
            Some(ref current_file) => match current_file.parent(){
                Some(ref parent) => Some(parent.to_string()),
                None => None,
            }
            None => None
        }
    }

    fn md_to_html(&self, raw: &str) -> Result<String, ()> {
        let prefix = if let Some(base_dir) = self.get_current_dir(){
            base_dir
        }else{
            "./".to_string()
        };
        let html = spongedown::parse_with_base_dir(&raw, &prefix);

        if let Ok(html) = html{
            Ok(html)
        }else{
            Err(())
        }
    }


}



