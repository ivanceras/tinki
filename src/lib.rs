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

mod route_service;



pub struct Model{
    route_service: RouteService<String>,
    fetch_service: FetchService,
    fetch_callback: Callback<Result<String, Error>>,
    task: Option<FetchTask>,
    content: String,

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
        let url = route_service.get_route();
        info!("current url is: {}", url);
        let fetch_callback = link.send_back(|file: Result<String,Error>| Msg::FileReady(file));
        let fetch_service = FetchService::new();
        let mut model = Model {
            route_service,
            fetch_callback,
            fetch_service,
            task: None,
            content: "".to_string(),
        };
        let task = model.fetch_file(&url);
        model.task = Some(task);
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::UrlChanged(url) => {
                info!("url changed: {}", url);
                self.task = Some(self.fetch_file(&url));
            }
            Msg::FileReady(file) => {
                info!("got file: {:?}", file);
                if let Ok(raw) = file{
                    self.content = spongedown::parse(&raw).unwrap();
                }
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let js_svg = js! {
            var div = document.createElement("div");
            div.innerHTML = @{self.content.to_owned()};
            return div;
        };
        let node = Node::try_from(js_svg).expect("convert js_svg");
        let sponge = VNode::VRef(node);
        html! {
            <>
            { sponge }
            <a href="/#/md/SUMMARY.md",>{"/#/md/SUMMARY.md"}</a>
            <a href="#/md/SUMMARY.md",>{"#/md/SUMMARY.md"}</a>
            <a href="/md/SUMMARY.md",>{"/md/SUMMARY.md"}</a>
            <a href="md/SUMMARY.md",>{"md/SUMMARY.md"}</a>
            </>
        }
    }
}

impl Model{

    pub fn fetch_file(&mut self, file: &str) -> FetchTask {
        info!("fetching file");
        let mut file = file.trim_left_matches("/#/");
        let url = format!("http://localhost:8080/file/{}", file);
        info!("requesting url here...-->: {}", url);
        let fetch_callback = self.fetch_callback.clone();
        info!("then building the handler here..");
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
        let request = match Request::get(url.as_str()).body(Nothing){
            Ok(request) => request,
            Err(e) =>  {
                info!("unable to build request : {}", url);
                panic!();
            }
        };
        info!("doing the fetch.. request.. ");
        let task = self.fetch_service.fetch(request, handler.into());
        task
    }
}
