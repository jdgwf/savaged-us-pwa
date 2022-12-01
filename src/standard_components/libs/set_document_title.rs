use web_sys;

pub fn set_document_title(
    site_title: String,
    new_value: String,
    no_calls: bool,
) {
    if no_calls {
        return;
    }
    let document = web_sys::window().unwrap().document().unwrap();

    let concat_title = new_value + &" | ".to_string() + &site_title;
    document.set_title( concat_title.as_ref() );

}