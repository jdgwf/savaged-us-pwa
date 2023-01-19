use chrono::prelude::*;
use savaged_libs::{public_user_info::PublicUserInfo, user::User};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminTableOwnershipBadgeProps {

    pub current_user: User,

    #[prop_or_default]
    pub updated_on: Option<DateTime<Utc>>,
    #[prop_or_default]
    pub created_on: Option<DateTime<Utc>>,
    #[prop_or_default]
    pub deleted_on: Option<DateTime<Utc>>,

    #[prop_or_default]
    pub updated_by: Option<PublicUserInfo>,
    #[prop_or_default]
    pub created_by: Option<PublicUserInfo>,
    #[prop_or_default]
    pub deleted_by: Option<PublicUserInfo>,

}

#[function_component(AdminTableOwnershipBadge)]
pub fn admin_table_ownership_badge (
    props: &AdminTableOwnershipBadgeProps,
) -> Html {

    // let mut created_by_html = html!(<></>);
    // let mut created_on_html = html!(<></>);
    let mut updated_on_html = html!(<></>);
    let mut updated_by_html = html!(<></>);

    // let created_on: Option<DateTime<Utc>> = props.created_on.clone();
    // let updated_on: Option<DateTime<Utc>> = props.updated_on.clone();

    // match props.created_by.clone() {
    //     Some( user ) => {
    //         created_by_html = html!{ <>{user.name}</>}
    //     }
    //     None => {

    //     }
    // }

    match props.updated_by.clone() {
        Some( user ) => {
            updated_by_html = html!{ <>{user.name}</>}
        }
        None => {

        }
    }

    // match props.created_on {
    //     Some( dt ) => {
    //         created_on_html = html!{ <>{props.global_vars.current_user.format_datetime(dt, false, true, false)}</>}
    //     }
    //     None => {
    //     }
    // }

    match props.updated_on.clone() {
        Some( dt ) => {
            updated_on_html = html!{ <>{props.current_user.format_datetime(dt, false, true, false)}</>}
        }
        None => {

        }
    }

    return html!{

        <div class="admin-table-created-updated small-text">
            // {created_by_html}<br />
            // if updated_on != created_on {
            //     {created_on_html}<br />
            //     {updated_on_html}<br />
            // } else {
            //     {created_on_html}<br />
            // }
            {updated_by_html}<br />
            {updated_on_html}<br />

        </div>
    };

}