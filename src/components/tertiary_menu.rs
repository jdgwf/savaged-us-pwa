use standard_components::libs::local_storage_shortcuts::{get_local_storage_string, set_local_storage_string};
use web_sys::MouseEvent;
use yew::prelude::*;
use standard_components::ui::nbsp::Nbsp;

#[derive(PartialEq, Clone)]
pub struct TertiaryMenuItem {
    pub tag: String,
    pub label: String,
    pub callback: Option<Callback<String>>,
    pub class: Option<String>,
    pub title: Option<String>,
    pub icon_class: Option<String>,
    pub separate: bool,
}

#[derive(Properties, PartialEq)]
pub struct TertiaryMenuProps {
    // pub global_vars: GlobalVars,
    pub server_side_renderer: bool,
    pub local_storage_variable: AttrValue,
    pub menu_items: Vec<TertiaryMenuItem>,
    pub menu_changed_callback: Callback<String>,
}
pub enum TertiaryMenuMessage {
    SetDropdownOpen(bool),
}

pub struct TertiaryMenu {

    open_dropdown: bool,
}

impl Component for TertiaryMenu {
    type Message = TertiaryMenuMessage;
    type Properties = TertiaryMenuProps;

    fn create(_ctx: &Context<Self>) -> Self {

        TertiaryMenu {
            open_dropdown: false,
        }
    }

    fn update(
        &mut self,
        _ctx: &Context<Self>,
        msg: TertiaryMenuMessage
    ) -> bool {

        match msg {
            TertiaryMenuMessage::SetDropdownOpen( open ) => {
                self.open_dropdown = open;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let mut filter_type = "character".to_owned();

        if !ctx.props().server_side_renderer {
            filter_type = get_local_storage_string( ctx.props().local_storage_variable.as_str(), "character".to_string());
        }

        let menu_changed_callback = ctx.props().menu_changed_callback.clone();
        let local_storage_variable = ctx.props().local_storage_variable.clone();

        let menu_items = ctx.props().menu_items.clone();
        let mut class = "tertiary-menu-mobile".to_owned();
        if self.open_dropdown {
            class += &" show-dd";
        }
        let set_dd_menu = ctx.link().callback(TertiaryMenuMessage::SetDropdownOpen);
        let open_dropdown = self.open_dropdown;
        html! {
        <div class="width-limit">
            <div class={class}>
                <div class="flex">
                {menu_items.clone().into_iter().map(  | item | {
                    if item.tag.as_str() == &filter_type {
                        let set_dd_menu = set_dd_menu.clone();
                        let mut icon = html!{<></>};

                        match item.icon_class {
                            Some( icon_class ) => {
                                icon = html!{<><i class={icon_class} /><Nbsp /></>}
                            }
                            None => {}
                        }

                        html!{<div
                            class={"current flex-grow-1"}
                            onclick={ move | _ | { set_dd_menu.emit(!open_dropdown) }}
                        >
                            {icon}{item.label.to_owned()}
                            <i class="fa fa-arrow-up" />
                            <i class="fa fa-arrow-down" />
                        </div>}
                    } else if item.separate {
                        let local_storage_variable = local_storage_variable.to_owned();

                        let menu_changed_callback = menu_changed_callback.clone();
                        let set_dd_menu = set_dd_menu.clone();

                        let on_click= move | e: MouseEvent | {
                            let item_callback = item.callback.clone();
                            e.prevent_default();

                            match item_callback {
                                Some( the_callback ) => {

                                    the_callback.emit( item.tag.to_owned());
                                    set_dd_menu.emit( false);
                                }
                                None => {
                                    if item.tag != "__all__".to_owned() {
                                        set_local_storage_string( local_storage_variable.as_str(), item.tag.clone());
                                        menu_changed_callback.emit(item.tag.to_owned());
                                        set_dd_menu.emit( false);
                                    }
                                }
                            }

                        };

                        let item_title = item.title.unwrap_or("".to_owned());

                        let mut icon = html!{<></>};
                        match item.icon_class {
                            Some( icon_class ) => {
                                icon = html!{<><i class={icon_class} /><Nbsp /></>}
                            }
                            None => {}
                        }

                        let mut base_class = "btn btn-primary".to_owned();

                        match item.class {
                            Some( the_class ) => {
                                base_class += &" ";
                                base_class += &the_class;
                            }
                            None => {}
                        }

                        html!{
                            <div class="flex-grow-0">
                                <a
                                    href="#"
                                    onclick={on_click}
                                    title={item_title}
                                    class={base_class}
                                >
                                    {icon}{item.label.to_owned()}
                                </a>
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
                        let local_storage_variable = local_storage_variable.to_owned();
                        let filter_type = filter_type.to_owned();
                        let menu_changed_callback = menu_changed_callback.clone();
                        let set_dd_menu = set_dd_menu.clone();
                        let tag = item.tag.clone();

                        let on_click= move | e: MouseEvent | {
                            let item_callback = item.callback.clone();
                            e.prevent_default();

                            match item_callback {
                                Some( the_callback ) => {

                                    the_callback.emit( item.tag.to_owned());
                                    set_dd_menu.emit( false );
                                }
                                None => {
                                    if item.tag != "__all__".to_owned() {
                                        set_local_storage_string( local_storage_variable.as_str(), item.tag.clone());
                                        menu_changed_callback.emit(item.tag.to_owned());
                                        set_dd_menu.emit( false );
                                    }
                                }
                            }

                        };

                        let base_class = "".to_owned();

                        let item_title = item.title.unwrap_or("".to_owned());

                        let mut icon = html!{<></>};
                        match item.icon_class {
                            Some( icon_class ) => {
                                icon = html!{<><i class={icon_class} /><Nbsp /></>}
                            }
                            None => {}
                        }

                        html!{<li
                            class={class_is_active( filter_type, &tag.as_ref(), base_class)}
                        >
                            <a
                                href="#"
                                onclick={on_click}
                                title={item_title}
                            >
                                {icon}{item.label.to_owned()}
                            </a>
                        </li>}
                    }
                }).collect::<Html>()}
                </ul>
            </div>
            <ul class="tertiary-menu">
                {menu_items.clone().into_iter().map( | item | {
                    let local_storage_variable = local_storage_variable.to_owned();
                    let filter_type = filter_type.to_owned();
                    let menu_changed_callback = menu_changed_callback.clone();
                    let tag = item.tag.clone();

                    let on_click= move | e: MouseEvent | {
                        let item_callback = item.callback.clone();
                        e.prevent_default();

                        if item.tag != "__all__".to_owned() {
                            set_local_storage_string( local_storage_variable.as_str(), item.tag.clone());
                        }
                        match item_callback {
                            Some( the_callback ) => {
                                the_callback.emit( item.tag.to_owned());
                            }
                            None => {
                                menu_changed_callback.emit(item.tag.to_owned());
                            }
                        }

                    };

                    let base_class = item.class.unwrap_or("".to_owned());

                    let item_title = item.title.unwrap_or("".to_owned());

                    let mut icon = html!{<></>};
                    match item.icon_class {
                        Some( icon_class ) => {
                            icon = html!{<><i class={icon_class} /><Nbsp /></>}
                        }
                        None => {}
                    }

                    html!{<li
                        class={class_is_active( filter_type, &tag.as_ref(), base_class)}
                    >
                        <a
                            href="#"
                            onclick={on_click}
                            title={item_title}
                        >
                            {icon}{item.label.to_owned()}
                        </a>
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