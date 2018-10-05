use stdweb::web::INode;
use stdweb::web::document;
use log::*;
use stdweb::web::IElement;

pub fn inject_css(css: &str) {
    if let Some(head) = document().head(){
        if let Ok(style) = document().create_element("style"){
           if let Ok(_) = style.set_attribute("type", "text/css"){
                let css_node = document().create_text_node(css);
                style.append_child(&css_node);
                head.append_child(&style);
           }else{
               error!("unable to set type=text/css to style");
           }
        }else{
            error!("unable to create style element");
        }
    }else{
        error!("Unable to obtain html head");
    }
}


pub fn set_body_class(class: &str){
    if let Some(body) = document().body(){
        let result = body.set_attribute("class", class);
        if let Err(e) = result{
            error!("Setting body class errored: {}", e)
        }
    }
}
