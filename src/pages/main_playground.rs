// use stdweb::web::Window;
// use savaged_libs::websocket_message::{
//     WebSocketMessage,
//     WebsocketMessageType,
// };
use wasm_bindgen::JsCast;
// use yew::prelude::*;
// use standard_components::ui::nbsp::Nbsp;

use standard_components::libs::set_document_title::set_document_title;
use yew::{ Properties, Context, Html, html, Component };

// use crate::lib::fetch_api::fetch_api;
// use crate::lib::fetch_api::savaged_login;

// use web_sys::console;
// use wasm_bindgen_futures::spawn_local;
// use gloo_utils::format::JsValueSerdeExt;
use crate::libs::global_vars::GlobalVars;
use gloo_console::log;
// use savaged_libs::user::User;
// use savaged_libs::user::LoginTokenResult;
// extern crate stdweb;
use web_sys::{
    EventTarget, HtmlElement, MouseEvent, TouchEvent,
    // Touch, TouchList
};

#[derive(Properties, PartialEq)]
pub struct MainPlaygroundProps {
    pub global_vars: GlobalVars,
}

pub enum MainPlaygroundMessage {
    // DragStart(u32, u32),
    // Drag(DragEvent),
    // AllowDrop(DragEvent),
    // DoDrop(DragEvent),
    // DragEnd(DragEvent),

    MousePickup(MouseEvent),
    MouseDrop(MouseEvent),
    MouseMove(MouseEvent),

    TouchPickup(TouchEvent),
    TouchDrop(TouchEvent),
    TouchMove(TouchEvent),
}

pub struct MainPlayground {
    // dragging_item: Option<String>,
    dropped_items: Vec<String>,

    moving_item: Option<HtmlElement>,
    moving_item_original_style: String,
    under_element_id: String,
    moving_item_height: i32,
    moving_item_width: i32,
}

impl Component for MainPlayground {
    type Message = MainPlaygroundMessage;
    type Properties = MainPlaygroundProps;

    fn create(
        ctx: &Context<Self>
    ) -> Self {

        let global_vars = ctx.props().global_vars.clone();

        set_document_title(global_vars.site_title.to_owned(), "Playground".to_owned(), global_vars.server_side_renderer,);
        MainPlayground {
            // dragging_item: None,
            dropped_items: Vec::new(),
            moving_item: None,
            moving_item_original_style: "".to_owned(),
            under_element_id: "".to_owned(),
            moving_item_height: 0,
            moving_item_width: 0,
        }
    }

    fn update(
        &mut self,
        _ctx: &Context<Self>,
        msg: Self::Message
    ) -> bool {
        match msg {
            MainPlaygroundMessage::MouseDrop(e) => {
                // let target = e.target().unwrap();

                // let target: Option<EventTarget> = e.target();

                // target.
                // e.data_transfer().unwrap().set_data("text", id.as_ref());
                // let item = target.and_then(|t| t.dyn_into::<HtmlElement>().ok()).unwrap();



                // let id: String = item.
                // if let Some(input) = input {
                //     input_value_handle.set(input.value());
                // }
                // self.dragging_item = Some( item.id().to_owned() );
                log!("MouseDrop");


                if self.under_element_id == "drop-alpha".to_owned() {
                    self.dropped_items.push( self.moving_item.clone().unwrap().id() );
                }

                self.moving_item = None;
                self.under_element_id = "".to_owned();
                self.moving_item_original_style = "".to_string();
            }
            MainPlaygroundMessage::MouseMove(e) => {
                // let target = e.target().unwrap();

                let target: Option<EventTarget> = e.target();

                // target.
                // e.data_transfer().unwrap().set_data("text", id.as_ref());
                let item = target.and_then(|t| t.dyn_into::<HtmlElement>().ok()).unwrap();


                // let left = touch.client_x() - current_mover.client_width() / 2;
                // let right = touch.client_y() - current_mover.client_height() / 2;
                // let left = e.page_x();
                // let right = e.page_y();
                let document = web_sys::window().unwrap().document().unwrap();

                let element_option = document.element_from_point(e.page_x() as f32, e.page_y() as f32);
                self.under_element_id = "".to_owned();
                // for element_option in element_array.iter() {
                    // element_option.
                    match element_option {
                        Some( element ) => {
                            self.under_element_id = element.id();
                        }
                        None => {

                        }
                    }
                // }
                match self.moving_item.clone() {
                    Some( moving_item ) => {

                        let mut style = self.moving_item_original_style.to_owned();
                        style = style + &";z-index: 100".to_owned();
                        style = style + &";position: fixed".to_owned();
                        style = style + &";width: ".to_owned() + &self.moving_item_width.to_string() + &"px";
                        style = style + &";height: ".to_owned() + &self.moving_item_height.to_string() + &"px";

                        let _ = moving_item.set_attribute( "style", style.as_str());
                    }
                    None => {}
                }

                // let id: String = item.
                // if let Some(input) = input {
                //     input_value_handle.set(input.value());
                // }
                // self.dragging_item = Some( item.id().to_owned() );
                log!("MouseMove ", item.id());
            }
            MainPlaygroundMessage::MousePickup(e) => {
                // let target = e.target().unwrap();

                let target: Option<EventTarget> = e.target();

                // target.
                // e.data_transfer().unwrap().set_data("text", id.as_ref());
                // let item = target.and_then(|t| t.dyn_into::<HtmlElement>().ok()).unwrap();


                self.moving_item = Some(target.and_then(|t| t.dyn_into::<HtmlElement>().ok()).unwrap());



                match self.moving_item.clone() {
                    Some( moving_item ) => {

                        let style_option = moving_item.get_attribute("style");
                        match style_option {
                            Some( orig_style ) => {
                                self.moving_item_original_style = orig_style.to_owned();
                            }
                            None => {
                                self.moving_item_original_style = "".to_owned();
                            }
                        }

                        let mut style = self.moving_item_original_style.to_owned();

                        self.moving_item_height = moving_item.client_height();
                        self.moving_item_width = moving_item.client_width();


                        style = style + &";z-index: 100".to_owned();
                        style = style + &";position: fixed".to_owned();
                        style = style + &";width: ".to_owned() + &self.moving_item_width.to_string() + &"px";
                        style = style + &";height: ".to_owned() + &self.moving_item_height.to_string() + &"px";


                        let _ = moving_item.set_attribute( "style", style.as_str());
                        // moving_item.style.height = moving_item.client_height();
                        // moving_item.style.width = moving_item.client_width();
                        // moving_item.style.position = "fixed".to_owned();
                    }
                    None => {}
                }

                // let id: String = item.
                // if let Some(input) = input {
                //     input_value_handle.set(input.value());
                // }
                // log!("MousePickup ", item.id());
                // self.moving_item = Some( item );
                // self.moving_item_height = moving_item.client_height();
                // self.moving_item_width = moving_item.client_width();

            }

            MainPlaygroundMessage::TouchDrop(_e) => {
                // let target = e.target().unwrap();

                // let target = e.target();

                // target.
                // e.data_transfer().unwrap().set_data("text", id.as_ref());
                // self.moving_item = Some(target.and_then(|t| t.dyn_into::<HtmlElement>().ok()).unwrap());



                match self.moving_item.clone() {
                    Some( moving_item ) => {

                        let style = self.moving_item_original_style.to_owned();

                        let _ = moving_item.set_attribute( "style", style.as_str());

                    }
                    None => {}
                }


                log!("TouchDrop ", self.moving_item.clone().unwrap().id());

                if self.under_element_id == "drop-alpha".to_owned() {
                    self.dropped_items.push( self.moving_item.clone().unwrap().id() );
                }

                self.moving_item = None;
                self.under_element_id = "".to_owned();
                self.moving_item_original_style = "".to_string();

            }
            MainPlaygroundMessage::TouchMove(e) => {

                let document = web_sys::window().unwrap().document().unwrap();


                match self.moving_item.clone() {
                    Some( current_mover ) => {

                        let mut style = self.moving_item_original_style.to_owned();
                        style = style + &";z-index: 100".to_owned();
                        style = style + &";position: fixed".to_owned();
                        style = style + &";width: ".to_owned() + &self.moving_item_width.to_string() + &"px";
                        style = style + &";height: ".to_owned() + &self.moving_item_height.to_string() + &"px";

                        // let event_client_x = e.page_x();
                        // let event_client_y = e.page_y();


                        // if (event_client_x) {
                        //     // mousemove

                        //     style = style + &";left: ".to_owned() + &(event_client_x - self.moving_item.client_width()/2).to_string();
                        //     style = style + &";top: ".to_owned() + &(event_client_y - self.moving_item.client_height()/2).to_string();
                        // } else {

                            let changed_touches = e.changed_touches();


                            // changed_touches
                            let touch_option = changed_touches.item( 0 );

                            // let touch = changed_touches[0];



                            // log!( format!("web_sys::TouchList touch_option {:?} {:?}", touch, changed_touches) );
                            match touch_option {



                                Some( touch ) => {
                                    let left = touch.client_x() - current_mover.client_width() / 2;
                                    let right = touch.client_y() - current_mover.client_height() / 2;


                                    let element_option = document.element_from_point(touch.client_x() as f32, touch.client_y() as f32);
                                    self.under_element_id = "".to_owned();
                                    // for element_option in element_array.iter() {
                                        // element_option.
                                        match element_option {
                                            Some( element ) => {
                                                self.under_element_id = element.id();
                                            }
                                            None => {

                                            }
                                        }
                                    // }

                                    style = style + &";left: ".to_owned() + &left.to_string() + &"px";
                                    style = style + &";top: ".to_owned() + &right.to_string() + &"px";
                                }
                                None => {

                                }
                            }
                            // // touchmove - assuming a single touchpoint
                            // // moving.style.left = event.changedTouches[0].clientX - moving.client_width()/2;
                            // // moving.style.top = event.changedTouches[0].clientY - moving.client_height()/2;


                            // }
                            // touchmove - assuming a single touchpoint
                            // moving.style.left = event.changedTouches[0].clientX - moving.client_width()/2;
                            // moving.style.top = event.changedTouches[0].clientY - moving.client_height()/2;

                            // style = style + &";left: ".to_owned() + &(changed_touches[0]. - self.moving_item.client_width()/2).to_string();
                            // style = style + &";top: ".to_owned() + &(event_client_y - self.moving_item.client_height()/2).to_string();
                        // }




                        let _ = current_mover.set_attribute( "style", style.as_str());
                        // self.moving_item = Some( current_mover );

                        log!("TouchMove ", self.moving_item.clone().unwrap().id(), style);
                    }
                    None => {
                        log!("No self.moving_item on move?");
                    }
                }

            }
            MainPlaygroundMessage::TouchPickup(e) => {
                // let target = e.target().unwrap();

                let target = e.target();

                // target.
                // e.data_transfer().unwrap().set_data("text", id.as_ref());
                self.moving_item = Some(target.and_then(|t| t.dyn_into::<HtmlElement>().ok()).unwrap());



                match self.moving_item.clone() {
                    Some( moving_item ) => {

                        let style_option = moving_item.get_attribute("style");
                        match style_option {
                            Some( orig_style ) => {
                                self.moving_item_original_style = orig_style.to_owned();
                            }
                            None => {
                                self.moving_item_original_style = "".to_owned();
                            }
                        }

                        let mut style = self.moving_item_original_style.to_owned();

                        self.moving_item_height = moving_item.client_height();
                        self.moving_item_width = moving_item.client_width();


                        style = style + &";z-index: 100".to_owned();
                        style = style + &";position: fixed".to_owned();
                        style = style + &";width: ".to_owned() + &self.moving_item_width.to_string() + &"px";
                        style = style + &";height: ".to_owned() + &self.moving_item_height.to_string() + &"px";


                        let _ = moving_item.set_attribute( "style", style.as_str());
                        // moving_item.style.height = moving_item.client_height();
                        // moving_item.style.width = moving_item.client_width();
                        // moving_item.style.position = "fixed".to_owned();
                    }
                    None => {}
                }


                // let id: String = item.
                // if let Some(input) = input {
                //     input_value_handle.set(input.value());
                // }
                // self.dragging_item = Some( item.id().to_owned() );
                log!("TouchPickup ", self.moving_item.clone().unwrap().id());
            }

            // MainPlaygroundMessage::DragEnd(e) => {
            //     // let target = e.target().unwrap();

            //     let target: Option<EventTarget> = e.target();

            //     // target.
            //     // e.data_transfer().unwrap().set_data("text", id.as_ref());
            //     let item = target.and_then(|t| t.dyn_into::<HtmlElement>().ok()).unwrap();



            //     // let id: String = item.
            //     // if let Some(input) = input {
            //     //     input_value_handle.set(input.value());
            //     // }

            //     log!("DragEnd ", item.id());

            //     self.dragging_item = None;
            // }
            // MainPlaygroundMessage::Drag(e) => {
            //     // let target = e.target().unwrap();

            //     let target: Option<EventTarget> = e.target();

            //     // target.
            //     // e.data_transfer().unwrap().set_data("text", id.as_ref());
            //     let item = target.and_then(|t| t.dyn_into::<HtmlElement>().ok()).unwrap();



            //     // let id: String = item.
            //     // if let Some(input) = input {
            //     //     input_value_handle.set(input.value());
            //     // }
            //     self.dragging_item = Some( item.id().to_owned() );
            //     log!("Drag ", item.id());
            // }
            // MainPlaygroundMessage::AllowDrop(e) => {
            //     e.prevent_default();
            //     let target: Option<EventTarget> = e.target();
            //     let item = target.and_then(|t| t.dyn_into::<HtmlElement>().ok()).unwrap();
            //     log!("AllowDrop", item.id());
            // }
            // MainPlaygroundMessage::DoDrop(e) => {
            //     e.prevent_default();
            //     let target: Option<EventTarget> = e.target();
            //     let item = target.and_then(|t| t.dyn_into::<HtmlElement>().ok()).unwrap();
            //     log!("DoDrop", item.id(), self.dragging_item.clone().unwrap());
            //     // let data = e.data_transfer().unwrap().get_data("text");
            //     // let destination = Element::try_from(e.target().expect("couldn't get target")).expect("couldn't convert to Element");
            //     // destination.append_child(&document().get_element_by_id(&*data).unwrap());
            //     if item.id() == "drop-alpha".to_owned() {
            //         self.dropped_items.push( self.dragging_item.clone().unwrap() );
            //     }
            // }
        }
        true
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        // let global_vars = ctx.props().global_vars.clone();


    let mut item_list: Vec<TestItem> = Vec::new();
    item_list.push(
        TestItem{
            id: 1,
            name: "Item 1".to_owned(),
        }
    );
    item_list.push(
        TestItem{
            id: 2,
            name: "Item 2".to_owned(),
        }
    );
    item_list.push(
        TestItem{
            id: 3,
            name: "Item 3".to_owned(),
        }
    );
    item_list.push(
        TestItem{
            id: 4,
            name: "Item 4".to_owned(),
        }
    );
    item_list.push(
        TestItem{
            id: 5,
            name: "Item 5".to_owned(),
        }
    );


        html! {
            <div

                onmousemove={ctx.link().callback(MainPlaygroundMessage::MouseMove)}
                onmouseup={ctx.link().callback(MainPlaygroundMessage::MouseDrop)}

                class="row"
            >
                <div class="col-xs-3 col-md-3">
                    {item_list.into_iter().map(
                        | item |
                        {
                            html! {
                                <div
                                    id={format!("drag-{}", item.id)}
                                    style="background: #333; box-sizing: content-box;text-align: center; margin: .5rem 0; border: solid 2px #fff;cursor: grab;"
                                    // draggable="true"
                                    // ondragend={ctx.link().callback(MainPlaygroundMessage::DragEnd)}
                                    // ondragstart={ctx.link().callback(MainPlaygroundMessage::Drag)}

                                    onmousedown={ctx.link().callback(MainPlaygroundMessage::MousePickup)}
                                    ontouchstart={ctx.link().callback(MainPlaygroundMessage::TouchPickup)}
                                    ontouchmove={ctx.link().callback(MainPlaygroundMessage::TouchMove)}
                                    ontouchend={ctx.link().callback(MainPlaygroundMessage::TouchDrop)}
                                >
                                    {item.name}
                                </div>
                            }
                        }
                    ).collect::<Html>()
                    }
                    {&self.under_element_id}
                </div>
                <div class="col-xs-9 col-md-9">
                    <div
                        style="padding: 1rem;position: relative; min-height: 500px; margin: .5rem 0; border: solid 2px #fff;"
                    >
                        if !self.dragging_item_is_empty() {

                                // ondragover={ctx.link().callback(MainPlaygroundMessage::AllowDrop)}
                                // ondrop={ctx.link().callback(MainPlaygroundMessage::DoDrop)}
                                if self.under_element_id == "drop-alpha" {
                                    <div
                                        id={"drop-alpha"}
                                        style="z-index: 1001; text-align: center; display: flex; align-items: center;position: absolute; height: 480px; top: 10px; left: 10px; right: 10px; bottom: 10px; background: rgba( 0,0,0,.4);"
                                    >
                                    {"Drop an item here"}
                                </div>
                                } else {
                                    <div
                                        id={"drop-alpha"}
                                        style="z-index: 1001; text-align: center; display: flex; align-items: center;position: absolute; height: 480px; top: 10px; left: 10px; right: 10px; bottom: 10px; background: rgba( 0,0,0,.5);"
                                    >
                                    {"Drop an item here"}
                                </div>
                                }


                        }
                            {self.dropped_items.clone().into_iter().map(
                                | item |
                                {
                                    html! {
                                        <div
                                            id={ item.clone() }
                                            style="text-align: center; margin: .5rem 0; border: solid 2px #fff;"
                                        >
                                            {item.clone()}
                                        </div>
                                    }
                                }
                            ).collect::<Html>()
                            }
                    </div>
                </div>
            </div>

        }

    }
}

impl MainPlayground {
    fn dragging_item_is_empty( &self ) -> bool {
        match &self.moving_item {
            Some( elem ) => {
                return elem.id().is_empty();
            }
            None => {
                return true;
            }
        }
    }
}

#[derive(Clone)]
struct TestItem {
    id: u32,
    name: String,
}

