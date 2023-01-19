use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct TertiaryLinksMenuItem {
    pub link: Html,
    pub tag: String,
    pub class: Option<String>,
    pub title: Option<String>,
    pub icon_class: Option<String>,
    pub separate: bool,
}

#[derive(Properties, PartialEq)]
pub struct TertiaryLinksMenuProps {
    pub server_side_renderer: bool,
    pub menu_items: Vec<TertiaryLinksMenuItem>,
    pub current_tag: String,
}
pub enum TertiaryLinksMenuMessage {
    SetDropdownOpen(bool),
}

pub struct TertiaryLinksMenu {
    open_dropdown: bool,
}

impl Component for TertiaryLinksMenu {
    type Message = TertiaryLinksMenuMessage;
    type Properties = TertiaryLinksMenuProps;

    fn create(_ctx: &Context<Self>) -> Self {

        TertiaryLinksMenu {
            open_dropdown: false,
        }
    }

    fn update(
        &mut self,
        _ctx: &Context<Self>,
        msg: TertiaryLinksMenuMessage
    ) -> bool {

        match msg {
            TertiaryLinksMenuMessage::SetDropdownOpen( open ) => {
                self.open_dropdown = open;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let filter_type = ctx.props().current_tag.to_owned();

        let menu_items = ctx.props().menu_items.clone();
        let mut class = "tertiary-menu-mobile".to_owned();
        if self.open_dropdown {
            class += &" show-dd";
        }

        let set_dd_menu = ctx.link().callback(TertiaryLinksMenuMessage::SetDropdownOpen);
        let open_dropdown = self.open_dropdown;
        html! {
        <div class="width-limit">
            <div class={class}>
                <div class="flex">
                {menu_items.clone().into_iter().map(  | item | {
                    if item.tag.as_str() == &filter_type {
                        let set_dd_menu = set_dd_menu.clone();

                        html!{<div
                            class={"current flex-grow-1"}
                            onclick={ move | _ | { set_dd_menu.emit(!open_dropdown) }}
                        >
                            {item.link}
                            <i class="fa fa-arrow-up" />
                            <i class="fa fa-arrow-down" />
                        </div>}
                    } else if item.separate {

                        let item_title = item.title.unwrap_or("".to_owned());

                        let mut base_class = "btn btn-primary".to_owned();

                        match item.class {
                            Some( the_class ) => {
                                base_class += &" ";
                                base_class += &the_class;
                            }
                            None => {}
                        }

                        html!{
                            <div
                                class="flex-grow-0"
                                title={item_title}
                                class={base_class}
                            >
                                {item.link}

                            </div>
                        }
                    } else {
                        html!{<></>}
                    }
                }).collect::<Html>()}
                </div>
                <ul>
                {menu_items.clone().into_iter().map(  | item | {
                    if item.separate {
                        html!{<></>}
                    } else {
                        let filter_type = filter_type.to_owned();
                        let tag = item.tag.clone();

                        let base_class = "".to_owned();

                        let item_title = item.title.unwrap_or("".to_owned());

                        html!{<li
                            class={class_is_active( filter_type, &tag.as_ref(), base_class)}
                            title={item_title}
                        >
                        {item.link}

                        </li>}
                    }
                }).collect::<Html>()}
                </ul>
            </div>
            <ul class="tertiary-menu">
                {menu_items.clone().into_iter().map( | item | {
                    let filter_type = ctx.props().current_tag.to_owned();
                    let tag = item.tag.clone();

                    let base_class = item.class.unwrap_or("".to_owned());

                    let item_title = item.title.unwrap_or("".to_owned());

                    html!{<li
                        class={class_is_active( filter_type, &tag.as_ref(), base_class)}
                        title={item_title}

                    >
                        {item.link}
                    </li>}
                }).collect::<Html>()}

            </ul>
        </div>
        }
    }
}

fn class_is_active(
    current_select: String,
    current_menu_item: &str,
    base_class: String,
) -> String {

    if current_select.as_str() == current_menu_item {
        return base_class + &" active";
    }
    return base_class;
}