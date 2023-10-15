pub use app::init_db;
use maud::{html, Markup, DOCTYPE};
pub mod device;

use device::components::*;

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
            body class="p-2 w-screen h-screen flex flex-col gap-y-2 dark:bg-zinc-900" {
                nav class="h-12 p-4 dark:bg-slate-700/80 rounded-lg" {}
                main class="flex flex-grow flex-row gap-x-2" {
                    div class="flex flex-grow p-4 bg-gradient-to-t dark:from-neutral-700/80 dark:to-slate-700/80 rounded-lg" {}
                    div class="w-1/5 p-4 bg-gradient-to-t dark:from-neutral-700/80 dark:to-slate-700/80 rounded-lg" {}
                }
            }
        }
    }
}
