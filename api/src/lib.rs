pub use app::init_db;
use maud::{html, Markup, DOCTYPE};
pub mod device;

pub async fn index() -> Markup {
    html! {
        (DOCTYPE)
        html class="dark" lang="en" {
            head {
                meta charset="UTF-8" {}
                title { "💡️🏠️" }
                link href="./assets/app.css" rel="stylesheet" {}
                script type="text/javascript" src="./assets/htmx.min.js" {}
                script type="text/javascript" src="./assets/hyperscript.min.js" {}
            }
            body class="pl-2 pr-2 pt-2 pb-2 h-screen flex flex-col dark:bg-gray-900" {
                nav id="nav" class="h-14 p-2 dark:bg-slate-700/75 rounded-lg" {}
                div id="nav-toggle" class="h-2 grid place-items-center" {
                    @let toggle_nav = r#"
                        on click
                            toggle .pt-2 on body then
                            toggle .hidden on #nav
                    "#;
                    svg height="4" width="100" class="opacity-30 hover:opacity-100 bg-neutral-700 hover:cursor-pointer transition-opacity ease-out" _=(toggle_nav) {
                        line x1="2" y1="2" x2="98" y2="2" stroke="rgb(64,64,64)" stroke-width="4" stroke-linecap="round" {}
                    }
                }

                main class="flex flex-grow flex-row" {
                    div id="home" class="flex flex-grow p-2 bg-gradient-to-t dark:from-neutral-700/75 dark:to-slate-700/75 rounded-lg" {}
                    div id="side-bar-toggle" class="w-2 grid place-items-center" {
                        @let toggle_side_bar = r#"
                            on click
                                toggle .pr-2 on body then
                                toggle .mr-2 on #nav then
                                toggle .mr-2 on #nav-toggle then
                                toggle .hidden on #side-bar
                        "#;
                        svg height="100" width="4" class="opacity-30 hover:opacity-100 hover:cursor-pointer transition-opacity ease-out" _=(toggle_side_bar) {
                            line x1="2" y1="2" x2="2" y2="98" stroke="rgb(64,64,64)" stroke-width="4" stroke-linecap="round" {}
                        }
                    }
                    div id="side-bar" class="w-60 lg:w-96 p-2 bg-gradient-to-t dark:from-neutral-700/75 dark:to-slate-700/75 rounded-lg" {}
                }
            }
        }
    }
}
