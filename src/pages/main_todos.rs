use yew::{function_component, Properties, Html, html};
use crate::libs::global_vars::GlobalVars;
use standard_components::libs::set_document_title::set_document_title;

use standard_components::ui::nbsp::Nbsp;

#[derive(Properties, PartialEq)]
pub struct MainTodosProps {
    pub global_vars: GlobalVars,
}

#[function_component(MainTodos)]
pub fn main_todos(
    props: &MainTodosProps,
) -> Html {
    set_document_title(
        props.global_vars.site_title.to_owned(),
        "To-Dos".to_owned(),
        props.global_vars.no_calls,
    );

    html! {
        <div class={"main-content"}>
            <h2><i class="fa fa-list" /><Nbsp />{ "Development To-Dos (the short list)" }</h2>
            <div class={"row"}>
                <div class={"col-md"}>

                    <h3>{ "Front-End To-Dos (the short list)" }</h3>

                    <p>{"The following needs to be squared away before the character sheet can be seriously worked on:"}</p>

                    <ul>

                        <li>
                        <strong>{"User Registration"}</strong>
                            // <Nbsp />{"-"}<Nbsp />
                            // {""}
                        </li>
                        <li>
                        <strong>{"User Subscriptions and Purchases"}</strong>
                            // <Nbsp />{"-"}<Nbsp />
                            // {""}
                        </li>
                        <li>
                            <strong>{"Forgot Password"}</strong>
                            // <Nbsp />{"-"}<Nbsp />
                            // {""}
                        </li>

                        <li>
                        <strong>{"Admin"}</strong>
                            <Nbsp />{"-"}<Nbsp />
                            {"I'd like to finish the new admin. Many custom item in the character creation will depend on the forms shared there."}
                        </li>

                        <li>
                        <strong>{"Help"}</strong>
                            <Nbsp />{"-"}<Nbsp />
                            {"I'd also like for the Help area to be a \"CMS\" editable in markdown in the admin instead of edited static pages"}
                        </li>

                        <li>
                        <strong>{"User Saves Management"}</strong>
                            // <Nbsp />{"-"}<Nbsp />
                            // {""}
                        </li>

                    </ul>

                    <h3>{ "What (little) Works" }</h3>
                    <p>{"The following should be working fairly consistently:"}</p>

                    <ul>

                        <li>
                            <strong>{"User Devices Management"}</strong>
                            <Nbsp />{"-"}<Nbsp />
                            {"The ability to rename devices and remove authentication tokens is essential"}
                        </li>

                        <li>

                            <strong>{"API Key Regeneration"}</strong>
                            <Nbsp />{"-"}<Nbsp />
                            <del>{"This will require creating a reusable modal confirmation component"}</del>
                            <Nbsp />
                            {"Confirmation dialog working!"}
                        </li>

                        <li>
                            <strong>{"Login/Logoff"}</strong>
                            // <Nbsp />{"-"}<Nbsp />
                            // {""}
                        </li>
                        <li>
                            <strong>{"Private Settings"}</strong>
                            <Nbsp />{"-"}<Nbsp />
                            {"This entire page should work as expected"}
                        </li>
                        <li>
                            <strong>{"Notifications"}</strong>
                            <Nbsp />{"-"}<Nbsp />
                            {"This entire page should work as expected"}
                        </li>

                        <li>
                        <li>
                            <strong>{"Public settings - User Image Upload Component"}</strong>
                            <Nbsp />{"-"}<Nbsp />
                            {"This should be reusable component which uploads and sets a 'session image' or a permanent user image... the server should handle this as needed depending on name and endpoint."}
                        </li>

                        <strong>{"Public settings - Change Username"}</strong>
                            // <Nbsp />{"-"}<Nbsp />
                            // {"The ability to rename devices and remove authentication tokens is essential"}
                        </li>

                        <li>
                        <strong>{"Public settings - Timezone Selection"}</strong>
                            // <Nbsp />{"-"}<Nbsp />
                            // {"The ability to rename devices and remove authentication tokens is essential"}
                        </li>

                    </ul>
                    </div>

                    <div class={"col-md"}>

                    <h3>{ "Server To-Dos (the short list)" }</h3>

                    <ul>
                        <li>
                        <strong>{"404 pages sent to app"}</strong>
                            <Nbsp />{"-"}<Nbsp />
                            {"Right now anything other than / and a few selected pages the server is pushing a blank page (page refreshes are hurt by this)"}
                        </li>

                        <li>
                        <strong>{"User Settings Update/Saving"}</strong>
                            <Nbsp />{"-"}<Nbsp />
                            {"This is working on the front-end since I was originally working off the old server, but now the endpoints need to be executed on the new Actix server."}
                        </li>

                        <li>
                        <strong>{"User Image Uploads"}</strong>
                            <Nbsp />{"-"}<Nbsp />
                            {"This should be a all-for-one function to upload user images."}
                        </li>

                    </ul>

                    <h3>{ "What (little) Works" }</h3>
                    <p>{"The following should be working fairly consistently:"}</p>

                    <ul>

                        <li>
                            <strong>{"It serves a website"}</strong>
                        </li>

                        <li>

                            <strong>{"User Login"}</strong>

                        </li>
                        <li>
                        <strong>{"Device Edit/Delete"}</strong>
                            // <Nbsp />{"-"}<Nbsp />
                            // {""}
                        </li>

                        <li>
                        <strong>{"Notifications Edit/Delete/Mark all"}</strong>
                            // <Nbsp />{"-"}<Nbsp />
                            // {""}
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}

