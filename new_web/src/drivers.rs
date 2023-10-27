use dioxus::prelude::*;
use graphql_client::{GraphQLQuery, Response};
use std::error::Error;

use crate::footer;

pub fn DriversComponent(cx: Scope) -> Element {
    let year = use_state(cx, || "current".to_string());

    let future = use_future(cx, year, |year| async move {
        let variables = drivers::Variables {
            year: year.get().to_string(),
        };
        perform_my_query(variables).await.unwrap_or_default()
    });

    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            h1 {
                b { "Drivers Standings" }
            }
            input {
                r#type: "text",
                placeholder: "current",
                oninput: move |event| {
                    year.set(event.value.to_string());
                }
            }
            match future.value() {
                Some(drivers) if drivers.len() > 0 => rsx! {ShowDrivers { drivers: &drivers }},
                _ => rsx! {render! { "loading" }}
            }
        }
        footer::Footer {}
    })
}

#[derive(PartialEq, Props)]
struct ShowDriversProps<'a> {
    drivers: &'a Vec<Option<drivers::DriversDriverStandingsDrivers>>,
}

fn ShowDrivers<'a>(cx: Scope<'a, ShowDriversProps<'a>>) -> Element {
    let drivers = cx.props.drivers;

    cx.render(rsx! {
        table {
            border_collapse: "collapse",
            thead {
                tr {
                    th { "Position" }
                    th { "Code" }
                    th { "Driver" }
                    th { "Points" }
                }
            }
            tbody {
                for driver in drivers {
                    if let Some(driver) = driver {
                        rsx! {ShowDriver { driver: driver }}
                    }
                }
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct ShowDriverProps<'a> {
    driver: &'a drivers::DriversDriverStandingsDrivers,
}

fn ShowDriver<'a>(cx: Scope<'a, ShowDriverProps<'a>>) -> Element {
    let driver = cx.props.driver;
    let driver_details = driver.driver.as_ref().expect("no details for driver");

    cx.render(rsx! {
        tr {
            class: "border-2 hover:bg-gray-100 hover:ring-2 hover:ring-inset",
            td {
                if let Some(position) = &driver.position {
                    match position.as_str() {
                        "1" => rsx! {render! { "🥇" }},
                        "2" => rsx! {render! { "🥈" }},
                        "3" => rsx! {render! { "🥉" }},
                        _ => rsx! {render! { position.to_string() }}
                    }
                }
            }
            td {
                if let Some(code) = &driver_details.code {
                    rsx! {render! { code.to_string() }}
                }
            }
            td {
                if let (Some(given_name), Some(family_name), Some(url)) = (&driver_details.given_name, &driver_details.family_name, &driver_details.url) {
                    rsx! {render! {
                        a {
                            href: "{url}",
                            target: "_blank",
                            "{given_name} {family_name}"
                        }
                    }}
                }
            }
            td {
                if let Some(points) = &driver.points {
                    rsx! {render! { points.to_string() }}
                }
            }
        }
    })
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graph/schema.graphql",
    query_path = "graph/query.graphql",
    response_derives = "PartialEq"
)]
pub struct Drivers;

async fn perform_my_query(
    variables: drivers::Variables,
) -> Result<Vec<Option<drivers::DriversDriverStandingsDrivers>>, Box<dyn Error>> {
    let request_body = Drivers::build_query(variables);

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/query")
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<drivers::ResponseData> = res.json().await?;
    Ok(response_body
        .data
        .ok_or("missing response data")?
        .driver_standings
        .ok_or("missing driver standings")?
        .drivers
        .ok_or("missing drivers")?)
}
