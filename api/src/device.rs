use std::sync::{Arc, Mutex};

use app::{Connection, Command, device::{self, AddDevice}, device_rest};
use axum::{extract::State, Form, Router, routing::post, debug_handler};
use maud::{Markup, html};

type SqliteConn = Arc<Mutex<Connection>>;

pub fn device_router() -> Router<SqliteConn> {
    Router::new()
        .route("/add", post(add_device))
}

#[debug_handler]
pub async fn add_device(State(conn): State<SqliteConn>, Form(add_device): Form<AddDevice>) -> Markup {
    {
        let conn = conn.lock().expect("Encountered poisoned lock");
        add_device.execute(&conn).expect("DB Error inserting the device");
    }
    html!{
        div {"Device Added"}
    }
}

pub mod components {
    use app::{device::Device, device_rest::DeviceRest};
    use maud::{Markup, html};

    pub fn device_list() -> Markup {
        html! {
            div class="w-1/5 p-4 border dark:border-gray-600 rounded-lg" {
                "(device_add_form())"
            }
        }
    }

    pub fn device_add_form() -> Markup {
        html! {
            form #device-add-form 
                hx-post="/device/add"
            {
                label for="new-device-type" {"Type"}
                select #new-device-type name="product"{
                    option value="ss_1200" {"SecureSync 1200"}
                    option value="ss_2400" {"SecureSync 2400"}
                }
                label for="new-device-name" {"Name"}
                input #new-device-name name="name" type="text" {}
                label for="new-device-model" {"Model"}
                input #new-device-model name="model" type="text" {}
                label for="new-device-serial" {"Serial"}
                input #new-device-serial name="serial" type="text" {}
                button type="submit" {"Add"}
            }
        }
    }

    pub fn device_card(device: &Device) -> Markup {
        html! {

            "TODO: Make device card component"
        }
    }
}