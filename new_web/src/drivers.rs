use dioxus::prelude::*;
use dioxus_charts::LineChart;
use graphql_client::{GraphQLQuery, Response};
use log::info;
use std::error::Error;

use crate::footer;

pub fn DriversComponent(cx: Scope) -> Element {
    let year = use_state(cx, || "current".to_string());

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
            div {
                display: "flex",
                flex_direction: "row",
                ShowDriverGraph { year: year },
                ShowDrivers { year: year }
            }
        }
        footer::Footer {}
    })
}

#[derive(PartialEq, Props)]
struct ShowDriverGraphProps<'a> {
    // graph_data: &'a Vec<Option<drivers_graph::DriversGraphDriversSeasonalRecordsDrivers>>,
    year: &'a UseState<String>,
}

fn ShowDriverGraph<'a>(cx: Scope<'a, ShowDriverGraphProps<'a>>) -> Element {
    let graph_future = use_future(cx, cx.props.year, |year| async move {
        let variables = drivers_graph::Variables {
            year: year.get().to_string(),
        };
        driver_graph(variables).await
    });

    cx.render(match graph_future.value() {
        Some(Ok((series, labels, series_labels))) => rsx! {
            LineChart{
                series: series,
                labels: labels.to_vec(),
                series_labels: series_labels.to_vec(),
                padding_top: 30,
                padding_left: 65,
                padding_right: 80,
                padding_bottom: 30,
            }
        },
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
                                rsx! {ShowDriver { driver: driver }}
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
) -> Result<(Vec<Vec<f32>>, Vec<String>, Vec<String>), Box<dyn Error>> {
    info!("variables: {:?}", variables);
    let request_body = DriversGraph::build_query(variables);

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/query")
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<drivers_graph::ResponseData> = res.json().await?;

    info!("response_body: {:?}", response_body);

    response_body.errors.iter().for_each(|e| {
        info!("error: {:?}", e);
    });
    if let Some(errors) = response_body.errors {
        if errors.len() > 0 {
            return Err("error".into());
        }
    }

    let drivers = response_body
        .data
        .ok_or("missing response data")?
        .drivers_seasonal_records
        .ok_or("missing driver standings")?
        .drivers
        .ok_or("missing drivers")?;

    info!("drivers: {:?}", drivers);

    let series = drivers
        .iter()
        .map(|driver| {
            let driver = driver.as_ref().expect("no driver");
            let records = driver.records.as_ref().expect("no records");
            let mut sum = 0.0;
            records
                .iter()
                .map(|record| {
                    let record = record.as_ref().expect("no record");
                    let points = record.points.as_ref().expect("no points");
                    sum += points.parse::<f32>().expect("points not a float");
                    sum
                })
                .collect::<Vec<f32>>()
        })
        .collect::<Vec<Vec<f32>>>();

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

    let series_labels = drivers
        .iter()
        .map(|driver| {
            let driver = driver.as_ref().expect("no driver");
            let driver_details = driver.driver.as_ref().expect("no details for driver");
            let code = driver_details.code.as_ref().expect("no code");
            format!("{}", code)
        })
        .collect::<Vec<String>>();

    Ok((series, labels, series_labels))
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
