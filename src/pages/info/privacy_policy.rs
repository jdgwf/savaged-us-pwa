use yew::{function_component, Properties, Html, html};
use yew_router::prelude::Link;
use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use crate::pages::info::InfoRoute;
use standard_components::ui::nbsp::Nbsp;
#[derive(Properties, PartialEq)]
pub struct InfoPrivacyPolicyProps {
    pub global_vars: GlobalVars,
}
#[function_component(InfoPrivacyPolicy)]
pub fn info_privacy_policy(
    props: &InfoPrivacyPolicyProps,
) -> Html {

    let mut global_vars = props.global_vars.clone();
    global_vars.current_menu = "main-info".to_owned();
    global_vars.current_sub_menu = "info-privacy-policy".to_owned();

    html! {
    <UIPage
        global_vars={global_vars}
        page_title="Our Privacy Policy"
        submenu_tag={"info".to_owned()}
    >
            <h2><i class="fa fa-user-secret" /><Nbsp />{"Privacy Policy"}</h2>
            <div class="undefined content">

                <p>{"This privacy notice discloses the privacy practices for"}<Nbsp /><a href="https://savaged.us">{"https://savaged.us"}</a>{"."}<Nbsp />
                {"This privacy notice applies solely to information collected by this website. It will notify you of the following:"}</p>
                    <ol>
                        <li>{"We collect only your email address, your name (or alias), and optionally your twitter username and a profile image. Your first name, last name may be shared when an item is shared by you, but you have the ability to \"mask\" your shared name with a nickname. Your twitter and profile image are optional."}</li>
                        <li>{"Your email address is"}<Nbsp/><strong>{"never"}</strong><Nbsp />{"shared on this website."}</li>
                        <li>{"You have full access to share or mask as much information as possible when you share one of your creations."}</li>
                        <li>{"Your email address is stored in plain-text on the website, but your password is double-encrypted and effectively undecryptable."}</li>
                        <li>{"You can change any and all information we have on the server by visiting your Profile/Settings page."}</li>
                    </ol>

                    <h3 class="text-header">{"Information Collection, Use, and Sharing"}</h3>
                    <p>{"We are the sole owners of the information collected on this site. We only have access to/collect information that you voluntarily give us via email or other direct contact from you."}<Nbsp /><strong>{"We will not sell or rent this information to anyone"}</strong>{"."}</p>
                    <p>{"We will use your information to respond to you, regarding the reason you contacted us. We will not share your information with any third party outside of our organization, other than as necessary to fulfill your request."}</p>
                    <p>{"Unless you ask us not to, we may contact you via email in the future to tell you about specials, new products or services, or changes to this privacy policy."}</p>

                    <h3 class="text-header">{"Public Statistical Data"}</h3>
                    <p><strong>{"We may display anonymous site data on this web site"}</strong><Nbsp />{"as a bit of pride and boasting its usefulness. This could include total number of users, number of saves per user, shares per user, and number of users per day at some point as examples. That information will be lumped into a whole and then averaged. Your individual data is not singled out and shared."}</p>

                    <h3 class="text-header">{"Your Access to and Control Over Information"}</h3>
                    <p>{"You may opt out of any future contacts from us at any time. You can do the following at any time by contacting us via"}<Nbsp /><Link<InfoRoute> to={InfoRoute::InfoContactUs}>{"our contact page"}</Link<InfoRoute>>{":"}</p>
                        <ol>
                            <li>{"See what data we have about you, if any."}</li>
                            <li>{"Change/correct any data we have about you."}</li>
                            <li>{"Have us delete any data we have about you."}</li>
                            <li>{"View your and unshare your publicly shared items."}</li>
                            <li>{"Express any concern you have about our use of your data."}</li>
                        </ol>
                    <h3 class="text-header">{"Security"}</h3>
                    <p>{"We take precautions to protect your information. When you submit sensitive information via the website, your information is protected both online and offline."}</p>
                    <p>{"Wherever we collect sensitive information (such as credit card data), that information is encrypted and transmitted to us in a secure way. You can verify this by looking for a lock icon in the address bar and looking for \"https\" at the beginning of the address of the Web page."}</p>
                    <p>{"While we use encryption to protect sensitive information transmitted online, we also protect your information offline. Only employees who need the information to perform a specific job (for example, billing or customer service) are granted access to personally identifiable information. The computers/servers in which we store personally identifiable information are kept in a secure environment."}</p>
                    <p>{"If you feel that we are not abiding by this privacy policy, you should contact us immediately via our"}<Nbsp /><Link<InfoRoute> to={InfoRoute::InfoContactUs} >{"contact page"}</Link<InfoRoute>>{"."}</p>

            </div>
        </UIPage>
    }
}

