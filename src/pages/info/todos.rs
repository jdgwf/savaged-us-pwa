use crate::components::ui_page::UIPage;
use crate::libs::site_vars::SiteVars;
use standard_components::ui::{nbsp::Nbsp, content_box::ContentBox};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct InfoTodosProps {
    pub site_vars: SiteVars,
}

#[function_component(InfoTodos)]
pub fn info_todos(props: &InfoTodosProps) -> Html {
    let mut site_vars = props.site_vars.clone();

    site_vars.current_sub_menu = "info-todos".to_owned();

    let title = html!{<><i class="fa fa-list" /><Nbsp />{ "Development To-Dos (the short list)" }</>};
    html! {
    <UIPage
        site_vars={site_vars}
        page_title="Development To-Dos"
    >

    <ContentBox label_html={title}>

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
            </ContentBox>
        </UIPage>
    }
}
