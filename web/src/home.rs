use dioxus::prelude::*;

use crate::footer;

pub fn Home(cx: Scope) -> Element {
    render! {
        div {
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            h1 {
                b { "Top 5 // todo" }
            }
        }
        footer::Footer {}
    }
}
