use dioxus::prelude::*;
use graphql_client::{GraphQLQuery, Response};
use std::error::Error;

use crate::{footer, get_resp_body_from_gql};

pub fn CircuitsComponent(cx: Scope) -> Element {
    let year = use_state(cx, || "current".to_string());

    let future = use_future(cx, year, |year| async move {
        let variables = circuits::Variables {
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
                b { "Circuits" }
            }
            input {
                r#type: "text",
                placeholder: "current",
                oninput: move |event| {
                    year.set(event.value.to_string());
                }
            }
            match future.value() {
                Some(circuits) if circuits.len() > 0 => rsx! {ShowCircuits { circuits: circuits }},
                _ => rsx! {render! { "loading" }}
            }
        }
        footer::Footer {}
    })
}

#[derive(PartialEq, Props)]
struct ShowCircuitsProps<'a> {
    circuits: &'a Vec<Option<circuits::CircuitsScheduleRaces>>,
}

fn ShowCircuits<'a>(cx: Scope<'a, ShowCircuitsProps<'a>>) -> Element {
    let circuits = cx.props.circuits;

    cx.render(rsx! {
        table {
            border_collapse: "collapse",
            thead {
                tr {
                    th { "Round" }
                    th { "Name" }
                    th { "Date" }
                    th { "Circuit" }
                }
            }
            tbody {
                for circuit in circuits {
                    if let Some(circuit) = circuit {
                        rsx! {ShowCircuit { circuit: circuit }}
                    }
                }
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct ShowCircuitProps<'a> {
    circuit: &'a circuits::CircuitsScheduleRaces,
}

fn ShowCircuit<'a>(cx: Scope<'a, ShowCircuitProps<'a>>) -> Element {
    let circuit = cx.props.circuit;
    let circuit_details = circuit.circuit.as_ref().expect("no details for circuit");

    cx.render(rsx! {
        tr {
            class: "border-2 hover:bg-gray-100 hover:ring-2 hover:ring-inset",
            td {
                if let Some(round) = &circuit.round {
                    rsx! {render! { round.to_string() }}
                }
            }
            td {
                if let (Some(race_name), Some(url)) = (&circuit.race_name, &circuit.url) {
                    rsx! {
                        a {
                            href: "{url}",
                            target: "_blank",
                            "{race_name}"
                        }
                    }
                }
            }
            td {
                if let Some(date) = &circuit.date {
                    rsx! {render! { date.to_string() }}
                }
            }
            td {
                if let (Some(circuit_name), Some(img_url)) = (&circuit_details.circuit_name, &circuit_details.img) {
                    rsx! {
                        if img_url != "" {
                            render! {
                                a {
                                    class: "group",
                                    href: "#",
                                    "{circuit_name}"
                                    span {
                                        class: "hidden group-hover:block absolute z-99 bg-white",
                                        img {
                                            src: "{img_url}",
                                            alt: "{circuit_name}",
                                            width: "100",
                                        }
                                    }
                                }
                            }
                        } else {
                            render! { circuit_name.to_string() }
                        }
                    }
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
pub struct Circuits;

async fn perform_my_query(
    variables: circuits::Variables,
) -> Result<Vec<Option<circuits::CircuitsScheduleRaces>>, Box<dyn Error>> {
    // this is the important line
    let request_body = Circuits::build_query(variables);

    let response_body: Response<circuits::ResponseData> =
        get_resp_body_from_gql(&request_body).await.json().await?;
    Ok(response_body
        .data
        .ok_or("missing response data")?
        .schedule
        .ok_or("missing schedule")?
        .races
        .ok_or("missing races")?)
}
