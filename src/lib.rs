
use stdweb::web::Location;
use stdweb::web::window;

use yew::*;
use yew::prelude::*;
use log::*;
use self::route_service::RouteService;

mod route_service;



pub struct Model{
    route_service: RouteService<String>,
}

pub enum Msg{
    UrlChanged(String),
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|url: String| Msg::UrlChanged(url));
        let mut route_service = RouteService::new();
        route_service.register_callback(callback);
        Model {
            route_service
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::UrlChanged(url) => {
                info!("url changed: {}", url);
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <>
            {"Welcome"}
            </>
        }
    }
}
