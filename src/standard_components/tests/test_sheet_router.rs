use yew_router::prelude::*;
use yew::prelude::*;
use super::super::tests::test_sheet::TestSheet;
use yew::{function_component, html};
use super::super::tests::test_sheet_global_vars::TestSheetGlobalVars;
use super::super::super::standard_components::libs::local_storage_shortcuts::get_local_storage_bool;
use super::super::super::standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use super::super::libs::set_document_title::set_document_title;

pub type TestSheetGlobalVarsContext = UseReducerHandle<TestSheetGlobalVars>;

#[derive(Clone, Routable, PartialEq)]
pub enum TestSheetRoute {
    #[at("/test-sheet/home")]
    Home,
    #[at("/test-sheet/sub-route")]
    SubRoute,
    #[at("/404")]
    NotFound,
}

fn content_switch(
    routes: &TestSheetRoute,
    global_vars: TestSheetGlobalVars,
    update_global_vars: Callback<TestSheetGlobalVars>,
) -> Html {

    match routes {

        TestSheetRoute::Home => {
            html! {
                <TestSheet
                    global_vars={global_vars}
                    update_global_vars={update_global_vars}
                />
            }
        },

        TestSheetRoute::SubRoute => html! {
            <TestSheetSubRoute />
        },

        TestSheetRoute::NotFound => html! { <h1>{ "TestSheetRoute 404" }</h1> },
    }
}

pub fn top_menu_switch(
    routes: &TestSheetRoute,
) -> Html {
    let mut home_class_active = "".to_owned();
    let mut sub_route_class_active = "".to_owned();

    match routes {
        TestSheetRoute::Home => {
            home_class_active = "active".to_owned();

        },
        TestSheetRoute::SubRoute => {
            sub_route_class_active = "active".to_owned();
        },

        TestSheetRoute::NotFound => {

        },
    }

    html! {
        <ul class={"sub-menu"}>
            <li class={home_class_active}>
                <Link<TestSheetRoute> to={TestSheetRoute::Home}>{"Home"}</Link<TestSheetRoute>>
            </li>
            <li class={sub_route_class_active}>
                <Link<TestSheetRoute> to={TestSheetRoute::SubRoute}>{"SubRoute"}</Link<TestSheetRoute>>
            </li>
        </ul>
    }
}

#[derive(Properties, PartialEq)]
pub struct TestSheetRouterProps {
    #[prop_or_default]
    pub set_submenu: Callback<Html>,
}

#[function_component(TestSheetRouter)]
pub fn test_sheet_router(
    props: &TestSheetRouterProps
) -> Html {

    // let global_vars = TestSheetGlobalVars {
    //     api_key:  get_local_storage_string( "api_key", "".to_owned() ),
    //     test2:  get_local_storage_string( "test2", "".to_owned() ),
    //     check1:  get_local_storage_bool( "check1", false ),
    //     check2:  get_local_storage_bool( "check2", false ),
    //     check3:  get_local_storage_bool( "check3", false ),
    //     to_dos: Vec::new(),
    // };
    let test_sheet_global_vars_state = use_reducer(
        || TestSheetGlobalVars {
            api_key:  get_local_storage_string( "api_key", "".to_owned() ),
            test2:  get_local_storage_string( "test2", "".to_owned() ),
            check1:  get_local_storage_bool( "check1", false ),
            check2:  get_local_storage_bool( "check2", false ),
            check3:  get_local_storage_bool( "check3", false ),
            to_dos: Vec::new(),
        }
    );

    let update_global_vars = Callback::from(
        | _new_global_vars: TestSheetGlobalVars |  {
            // global_vars.check1 = new_global_vars.check1;
            // global_vars.check2 = new_global_vars.check2;
            // global_vars.check3 = new_global_vars.check3;

            // global_vars.to_dos = new_global_vars.to_dos.clone();
            // global_vars.test2 = new_global_vars.test2.to_owned();
            // global_vars.api_key = new_global_vars.api_key.to_owned();
        }
    );

    let submenu_html = html! {
        <BrowserRouter>
            <Switch<TestSheetRoute>
                render={Switch::render( top_menu_switch)}
            />
        </BrowserRouter>
    };

    let _ = &props.set_submenu.emit( submenu_html.clone() );

    let global_vars = (*test_sheet_global_vars_state).clone();
    html! {

        <ContextProvider<TestSheetGlobalVarsContext>
            context={test_sheet_global_vars_state}
        >
            <BrowserRouter>
                <div class={"main-content"}>
                    <Switch<TestSheetRoute>
                        render={Switch::render(
                            move |routes|
                            content_switch(
                                routes,
                                global_vars.clone(),
                                update_global_vars.clone(),
                            )
                        )}

                    />
                </div>
            </BrowserRouter>
        </ContextProvider<TestSheetGlobalVarsContext>>
     }

}

#[function_component(TestSheetSubRoute)]
pub fn test_sheet_sub_route() -> Html {

    set_document_title("Test Sheet".to_owned(), "Sub Route".to_owned(), true);

    html! {

        <div>
            {"SubRoute Says \"Hello\"!"}<br />
        </div>

     }
}

