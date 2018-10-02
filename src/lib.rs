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
use url::Url;

mod route_service;



pub struct Model{
    route_service: RouteService<String>,
    fetch_service: FetchService,
    fetch_callback: Callback<Result<String, Error>>,
    dom_ready: Callback<()>,
    task: Option<FetchTask>,
    content: String,
    base_dir: Option<String>,

}

pub enum Msg{
    UrlChanged(String),
    FileReady(Result<String,Error>),
    DomMounted,
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|url: String| Msg::UrlChanged(url));
        let mut route_service = RouteService::new();
        route_service.register_callback(callback);
        let url = route_service.get_route();
        let trim1 = url.trim_left_matches("/#/");
        let segments:Vec<&str> = trim1.split("/").collect();
        debug!("segments: {:?}", segments);
        let n_segments = segments.len();
        let base_dir = segments.into_iter().take(n_segments - 1 )
                .filter(|s|!s.is_empty())
                .collect::<Vec<&str>>()
                .join("/");
        debug!("base_dir: {}", base_dir);
        let fetch_callback = link.send_back(|file: Result<String,Error>| Msg::FileReady(file));
        let fetch_service = FetchService::new();
        let mut model = Model {
            route_service,
            fetch_callback,
            fetch_service,
            task: None,
            content: "".to_string(),
            dom_ready: link.send_back(|_|Msg::DomMounted),
            base_dir: if base_dir.is_empty(){None}else{Some(base_dir)},
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
                    if let Some(ref base_dir) = self.base_dir{
                        self.content = spongedown::parse_with_base_dir(&raw, &base_dir).unwrap();
                    }else{
                        self.content = spongedown::parse(&raw).unwrap();
                    }
                    //debug!("content: {}", self.content);
                }
                self.dom_ready.emit(());
            }
            Msg::DomMounted => {
                debug!("dom mounted here");
                //self.intercept_links();
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
        let js_svg = js! {
            var div = document.createElement("div");
            div.innerHTML = @{self.content.to_owned()};
            return div;
        };
        let node = Node::try_from(js_svg).expect("convert js_svg");
        let sponge = VNode::VRef(node);
        let html = html! {
            <>
            { sponge }
            <a href="/#/md/SUMMARY.md",>{"/#/md/SUMMARY.md"}</a>
            </>
        };
        html
    }
}

impl Model{

    fn fetch_file(&mut self, file: &str) -> FetchTask {
        info!("fetching file");
        let file = file.trim_left_matches("/#/");
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

    fn intercept_links(&self) {
        info!("intercepting anchor tags");
        if let Ok(anchors) = document().query_selector_all("a"){
            info!("go anchors: {}", anchors.len());
            let len = anchors.len();
            for i in 0..len{
                if let Some(anchor) = anchors.item(i){
                    let elm_anchor:Result<Element,_> = TryFrom::try_from(anchor);
                    if let Ok(elm_anchor) = elm_anchor{
                        if let Some(href) = elm_anchor.get_attribute("href"){
                            info!("href: {:?}", href);
                            let new_href = format!("/md/{}",href);
                            info!("new_href: {:?}", new_href);
                            elm_anchor.set_attribute("href", &new_href);
                            info!("reading the set href: {:?}", elm_anchor.get_attribute("href"));
                            elm_anchor.add_event_listener(move |event:ClickEvent| {
                                info!("this element.is clicked");
                                event.prevent_default();
                                info!("find this file: {:?}", href);
                            });
                        }
                    }
                }
            }
        }
    }
}
