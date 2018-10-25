
use yew::prelude::*;
use log::*;
use tinki::Model;

fn main() {
    web_logger::init();
    trace!("Initializing yew...");
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
