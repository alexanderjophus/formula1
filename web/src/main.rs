#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;

mod circuits;
use circuits::CircuitsComponent;
mod constructors;
use constructors::ConstructorsComponent;
mod drivers;
use drivers::DriversComponent;
mod home;
use home::Home;
use serde::Serialize;
mod footer;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/constructors")]
        ConstructorsComponent {},
        #[route("/drivers")]
        DriversComponent {},
        #[route("/schedule")]
        CircuitsComponent {},
    #[end_layout]
    #[route("/:.._route")]
    PageNotFound {
        _route: Vec<String>,
    },
}

fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

#[inline_props]
fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            display: "flex",
            flex_direction: "row",
            justify_content: "space-between",
            align_items: "center",
            background_color: "grey",
            color: "white",
            padding: "10px",
            Link {
                to: "/",
                "Home"
            }
            Link {
                to: "/constructors",
                "Constructors"
            }
            Link {
                to: "/drivers",
                "Drivers"
            }
            Link {
                to: "/schedule",
                "Schedule"
            }
        }
        Outlet::<Route> {}
    }
}

#[inline_props]
fn PageNotFound(cx: Scope, _route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
    }
}

pub async fn get_resp_body_from_gql<T: Serialize + ?Sized>(request_body: &T) -> reqwest::Response {
    let gql_addr = env!("GQL_ADDR");

    let client = reqwest::Client::new();
    client
        .post(format!("{gql_addr}/query"))
        .json(&request_body)
        .send()
        .await
        .expect("failed to send request")
}
