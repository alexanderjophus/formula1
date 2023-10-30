use dioxus::prelude::*;
use dioxus_charts::LineChart;
use graphql_client::{GraphQLQuery, Response};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use crate::{footer, get_resp_body_from_gql};

pub fn DriversComponent(cx: Scope) -> Element {
    let year = use_state(cx, || "current".to_string());
    let compare_drivers = use_state(cx, || HashSet::<String>::new());

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
                    compare_drivers.set(HashSet::<String>::new());
                    year.set(event.value.to_string());
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                ShowDriverGraph { year: year, compare_drivers: compare_drivers },
                ShowDrivers { year: year, compare_drivers: compare_drivers },
            }
        }
        footer::Footer {}
    })
}

#[derive(PartialEq, Props)]
struct ShowDriverGraphProps<'a> {
    year: &'a UseState<String>,
    compare_drivers: &'a UseState<HashSet<String>>,
}

fn ShowDriverGraph<'a>(cx: Scope<'a, ShowDriverGraphProps<'a>>) -> Element {
    let graph_future = use_future(cx, cx.props.year, |year| async move {
        let variables = drivers_graph::Variables {
            year: year.get().to_string(),
        };
        driver_graph(variables).await
    });

    cx.render(match graph_future.value() {
        Some(Ok((codes_to_series, labels))) => {
            let (series, series_labels) = if cx.props.compare_drivers.get().len() > 0 {
                (
                    codes_to_series
                        .iter()
                        .filter(|(code, _)| cx.props.compare_drivers.contains(*code))
                        .map(|(_, series)| series.to_vec())
                        .collect::<Vec<Vec<f32>>>(),
                    codes_to_series
                        .iter()
                        .filter(|(code, _)| cx.props.compare_drivers.contains(*code))
                        .map(|(code, _)| code.to_string())
                        .collect::<Vec<String>>(),
                )
            } else {
                (
                    codes_to_series
                        .iter()
                        .map(|(_, series)| series.to_vec())
                        .collect::<Vec<Vec<f32>>>(),
                    codes_to_series
                        .iter()
                        .map(|(code, _)| code.to_string())
                        .collect::<Vec<String>>(),
                )
            };

            rsx! {
                LineChart{
                    series: series,
                    labels: labels.to_vec(),
                    series_labels: series_labels.to_vec(),
                    padding_top: 30,
                    padding_left: 65,
                    padding_right: 80,
                    padding_bottom: 30,
                }
            }
        }
        Some(Err(_)) => rsx! {
            div {
                "error"
            }
        },
        _ => rsx! {
            div {
                "loading"
            }
        },
    })
}

#[derive(PartialEq, Props)]
struct ShowDriversProps<'a> {
    year: &'a UseState<String>,
    compare_drivers: &'a UseState<HashSet<String>>,
}

fn ShowDrivers<'a>(cx: Scope<'a, ShowDriversProps<'a>>) -> Element {
    let driver_standings_future = use_future(cx, cx.props.year, |year| async move {
        let variables = drivers::Variables {
            year: year.get().to_string(),
        };
        driver_standings(variables).await
    });

    cx.render(rsx! {
        match driver_standings_future.value() {
            Some(Ok(drivers)) => rsx!(
                table {
                    border_collapse: "collapse",
                    thead {
                        tr {
                            th { "Compare" }
                            th { "Position" }
                            th { "Code" }
                            th { "Driver" }
                            th { "Points" }
                        }
                    }
                    tbody {
                        for driver in drivers {
                            if let Some(driver) = driver {
                                rsx! {ShowDriver { driver: driver, compare_drivers: cx.props.compare_drivers }}
                            }
                        }
                    }
                }
            ),
            Some(Err(_)) => rsx! {
                tr {
                    td {
                        "error"
                    }
                }
            },
            _ => rsx! {
                tr {
                    td {
                        "loading"
                    }
                }
            },
        }
    })
}

#[derive(PartialEq, Props)]
struct ShowDriverProps<'a> {
    driver: &'a drivers::DriversDriverStandingsDrivers,
    compare_drivers: &'a UseState<HashSet<String>>,
}

fn ShowDriver<'a>(cx: Scope<'a, ShowDriverProps<'a>>) -> Element {
    let driver = cx.props.driver;
    let driver_details = driver.driver.as_ref().expect("no details for driver");

    cx.render(rsx! {
        tr {
            class: "border-2 hover:bg-gray-100 hover:ring-2 hover:ring-inset",
            text_align: "center",
            td {
                input {
                    r#type: "checkbox",
                    onchange: move |event: Event<FormData>| {
                        let mut compare_drivers = cx.props.compare_drivers.get().clone();
                        if event.value == "true" {
                            compare_drivers.insert(driver_details.code.as_ref().expect("no code").to_string());
                        } else {
                            compare_drivers.remove(driver_details.code.as_ref().expect("no code"));
                        }
                        cx.props.compare_drivers.set(compare_drivers);
                    }
                }
            }
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
    response_derives = "PartialEq,Debug",
    variables_derives = "Debug"
)]
pub struct DriversGraph;

async fn driver_graph(
    variables: drivers_graph::Variables,
) -> Result<(HashMap<String, Vec<f32>>, Vec<String>), Box<dyn Error>> {
    let request_body = DriversGraph::build_query(variables);
    let response_body: Response<drivers_graph::ResponseData> =
        get_resp_body_from_gql(&request_body).await.json().await?;

    let drivers = response_body
        .data
        .ok_or("missing response data")?
        .drivers_seasonal_records
        .ok_or("missing driver standings")?
        .drivers
        .ok_or("missing drivers")?;

    let series = drivers
        .iter()
        .map(|driver| {
            let driver = driver.as_ref().expect("no driver");
            let driver_details = driver.driver.as_ref().expect("no details for driver");
            let code = driver_details.code.as_ref().expect("no code");
            let mut sum = 0.0;
            let records = driver
                .records
                .as_ref()
                .expect("no records")
                .iter()
                .map(|record| {
                    let record = record.as_ref().expect("no record");
                    let points = record.points.as_ref().expect("no points");
                    sum += points.parse::<f32>().expect("failed to parse points");
                    sum
                })
                .collect::<Vec<f32>>();
            (code.to_string(), records)
        })
        .collect::<HashMap<String, Vec<f32>>>();

    let labels = drivers[0]
        .as_ref()
        .expect("no driver")
        .records
        .as_ref()
        .expect("no records")
        .iter()
        .map(|record| {
            let record = record.as_ref().expect("no record");
            let round = record.round.as_ref().expect("no round");
            round.to_string()
        })
        .collect::<Vec<String>>();

    Ok((series, labels))
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graph/schema.graphql",
    query_path = "graph/query.graphql",
    response_derives = "PartialEq"
)]
pub struct Drivers;

async fn driver_standings(
    variables: drivers::Variables,
) -> Result<Vec<Option<drivers::DriversDriverStandingsDrivers>>, Box<dyn Error>> {
    let request_body = Drivers::build_query(variables);

    let response_body: Response<drivers::ResponseData> =
        get_resp_body_from_gql(&request_body).await.json().await?;
    Ok(response_body
        .data
        .ok_or("missing response data")?
        .driver_standings
        .ok_or("missing driver standings")?
        .drivers
        .ok_or("missing drivers")?)
}
