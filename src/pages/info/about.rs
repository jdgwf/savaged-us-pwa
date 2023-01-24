use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use standard_components::ui::content_box::ContentBox;
use standard_components::ui::nbsp::Nbsp;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct InfoAboutProps {
    pub global_vars: GlobalVars,
}
#[function_component(InfoAbout)]
pub fn info_about(props: &InfoAboutProps) -> Html {
    let mut global_vars = props.global_vars.clone();

    global_vars.current_sub_menu = "info-about".to_owned();

    html! {
    <UIPage
        global_vars={global_vars}
        page_title="About Savaged.us"
    >
            <h2><i class="fa fa-circle-info" /><Nbsp />{"About Savaged.us"}</h2>
            <div class="row">
        <div class="col-md-6">
            <div class="alliante-bg">
                <div class="row">
                    <div class="col-xs-6 text-center">

                        <img src="/images/alliante-logo.svg" alt="Alliante Logo" />
                    </div>
                    <div class="col-xs-6 text-center">
                        <span style={"margin-left: -1rem"}>

                            <img src="/images/alliante-sigils.svg" alt="Alliante Logo" />
                        </span><br />
                        <br />
                        <p>
                            {"This software is produced by"}<br />
                          <Nbsp /><a target="alliante" href="https://alliante.com">{"Alliante Entertainment"}</a>
                        </p>
                    </div>
                </div>

            </div>

            <ContentBox
                label="Thank you"
            >
                <ul>
                    <li>{"A special thank you goes out to Shane Hensley, Clint Black, Jodi Black, and the rest of the Pinnacle Team for both creating such a great Role Playing system and nurturing such a wonderful and friendly fan community."}</li>
                    <li>{"Thank you to all the Savage Fans for making Savage Worlds so awesome."}</li>
                </ul>
                <h3 class="text-center color-h3">{"Very Special Thanks"}</h3>
                <ul class="styleless">
                    <li><strong>{"John Paikuli"}</strong>{", for nearly all the data entry of the new Rifts® source book data and amazing feedback and support help!"}</li>
                    <li><strong>{"Daniel Machuca"}</strong>{", for his relentless work helping squashing bugs on this relatively immense project. It's his bug reporting and testing that helped this project shine!"}</li>
                </ul>
            </ContentBox>

            <ContentBox
                label="Disclaimer"
            >
                <p>{"All copyrights to character, vehicle, and other rules and settings are owned by their respective copyright holders. This application makes no claim against"}<Nbsp /><strong>{"any"}</strong><Nbsp />{"properties."}</p>
            </ContentBox>

            <ContentBox
                label="Attributions"
            >
                <ul>
                    <li>
                        {"Image \"brown papyrus-paper\" is"}<Nbsp /><a target="ext-image" href="https://www.freepik.com/free-photo/brown-papyrus-paper_2765776.htm">{"Designed by Rawpixel.com"}</a>
                        <div class="text-right small-text">{"used as a background in the character sheet pdf"}</div>
                    </li>
                    <li>
                        {"Image \"Old Paper Texture\" is"}<Nbsp /><a target="ext-image" href="https://www.deviantart.com/caminopalmero/art/Old-Paper-Texture-63582296">{"Designed by caminopalmero"}</a>
                        <div class="text-right small-text">{"used as a background in the character sheet pdf"}</div>
                    </li>
                    <li>
                        {"Many Font icons are provided by"}<Nbsp /><a target="ext-image" href="https://fontawesome.com/">{"Font Awesome"}</a><br />
                    </li>
                    <li>
                        {"Some Game Specific Font icons are provided by"}<Nbsp /><a target="ext-image" href="https://nagoshiashumari.github.io/Rpg-Awesome/">{"RPG Awesome"}</a><br />
                    </li>
                    <li>
                        {"Some icons are also provided by the wonderful"}<Nbsp /><a target="ext-image" href="https://game-icons.net/">{"https://game-icons.net/"}</a>{". Your work is awesome!"}<br />
                    </li>
                    <li>
                        {"Playing Card images provided by Byron Knoll:"}<Nbsp /><a href="http://code.google.com/p/vector-playing-cards/" target="ext-image">{"http://code.google.com/p/vector-playing-cards/"}</a>
                    </li>
                </ul>
            </ContentBox>

        </div>
        <div class="col-md-6">
            <ContentBox
                label="License and Copyrights Information"
            >
            <>
            <p><a href="https://www.peginc.com/licensing/" title="Click here for more info on Pinnacle Entertainment Groups's licensing policies"><img style={"float: left; margin-top: .5rem ; margin-right: 1rem; width: 50%; margin-bottom: 1rem"} src="/images/sw_ace.png?v=2" alt="Savage Worlds Officially Licensed Product" /></a></p>
            <p>{"This application references the Savage Worlds game system, available from"}<Nbsp /><a href="https://peginc.com/?ref=savaged-us" target="peg">{"Pinnacle Entertainment Group"}</a> <Nbsp />{"at"}<Nbsp /><a href="https://peginc.com/?ref=savaged-us" target="peg">{"www.peginc.com"}</a>{"."}</p>
            <p>{"Savage Worlds and all associated logos and trademarks are copyrights of Pinnacle Entertainment Group. Used with permission. Pinnacle makes no representation or warranty as to the quality, viability, or suitability for purpose of this product."}</p>
            <p>{"Deadlands, Deadlands: Lost Colony, and other Pinnacle Trademarks and all associated logos and trademarks are copyrights of Pinnacle Entertainment Group. Used with permission. Pinnacle makes no representation or warranty as to the quality, viability, or suitability for purpose of this product."}</p>
            <p>{"Flash Gordon and other associated properties are ©2018 King Features Syndicate, Inc. TM Hearst Holdings, Inc."}</p>

            <h4 class="color-h4">{"Savage Rifts®"}</h4>
            <p>
              <Nbsp /><a href="https://palladiumbooks.com/?ref=savaged-us" target="palladium">
                    <img src="/images/palladium-books-logo-white.png" style={"float: left, fill: #eee, stroke: #eee; margin-top: .5rem ; margin-right: 1rem; width: 150px; margin-bottom: 1rem"} />
                </a>
              <Nbsp /><a href="https://palladiumbooks.com/futuristic/rifts-rpg/?ref=savaged-us">{"Rifts®"}</a> <Nbsp />{"and Megaverse® are Registered Trademarks of"}<Nbsp /><a href="https://palladiumbooks.com/?ref=savaged-us" target="palladium">{"Palladium Books, Inc"}</a>{". All character names and likenesses are copyright and trademarks owned by"}<Nbsp /><a href="https://palladiumbooks.com/?ref=savaged-us" target="palladium">{"Palladium Books, Inc."}</a> <Nbsp />{"and used with permission."}
            </p>

            <h4 class="color-h4">{"Venom Assault®"}</h4>
            <p>
              <Nbsp /><a href="http://spyglassgames.com/venom_assault.html" target="spyglass">
                    <img src="/images/spyglass-logo.jpg" style={"float: left; margin-top: .5rem; margin-right: 1rem; width: 150px; margin-bottom: 1rem"} />
                </a>
              <Nbsp /><a href="http://spyglassgames.com/venom_assault.html?ref=savaged-us">{"Venom Assault"}</a> <Nbsp />{"used in the Freedom Squadron rules are registered trademarks of"}<Nbsp /><a href="http://spyglassgames.com/?ref=savaged-us" target="spyglass">{"SpyGlass Games"}</a>{". All character names and likenesses are copyright and trademarks owned by SpyGlass and used with permission."}
            </p>

            <h4 class="color-h4">{"Pathfinder®"}</h4>
            <p>
              <Nbsp /><a href="https://paizo.com/pathfinder" target="pathfinder">
                    <img src="/images/pathfinder-logo.png" style={"float: left; margin-top:.5rem; margin-right: 1rem; width: 150px; margin-bottom: 1rem"} />
                </a>
              <Nbsp /><a href="https://paizo.com/pathfinder?ref=savaged-us">{"Pathfinder"}</a> <Nbsp />{"used in the Pathfinder for Savaged Worlds® is registered trademarks of"}<Nbsp /><a href="https://paizo.com/?ref=savaged-us" target="pathfinder">{"Paizo Inc."}</a>{". All relevant copyrights and trademarks owned by Paizo Inc. and used with permission."}
            </p>
            <hr />
            <p><strong>{"Any other content, included in our Global data, from other Aces or 3rd party books have been obtained with permission from their copyright owners, and we make absolutely no claim on their properties, and we (as a whole community!) thank them for allowing us to have our users play in their worlds."}</strong></p>
            <hr />
            <p>{"Any user shared or created content is the responsibility of the user, and not of savaged.us."}</p>
            <p>{"This stated, it's likely that any copyrighted characters, places, or things named are likely handled under"}<Nbsp /><strong>{"Fair Use"}</strong><Nbsp />{"for personal games and no ownership is asserted by the user."}</p>
        </>
            </ContentBox>

        </div>
    </div>
        </UIPage>
    }
}
