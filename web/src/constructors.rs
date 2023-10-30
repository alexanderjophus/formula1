use dioxus::prelude::*;
use graphql_client::{GraphQLQuery, Response};
use std::error::Error;

use crate::{footer, get_resp_body_from_gql};

pub fn ConstructorsComponent(cx: Scope) -> Element {
    let year = use_state(cx, || "current".to_string());

    let future = use_future(cx, year, |year| async move {
        let variables = constructors::Variables {
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
                b { "Constructors Standings" }
            }
            input {
                r#type: "text",
                placeholder: "current",
                oninput: move |event| {
                    year.set(event.value.to_string());
                }
            }
            match future.value() {
                Some(constructors) if constructors.len() > 0 => rsx! {ShowConstructors { constructors: constructors }},
                _ => rsx! {render! { "loading" }}
            }
        }
        footer::Footer {}
    })
}

#[derive(PartialEq, Props)]
struct ShowConstructorsProps<'a> {
    constructors: &'a Vec<Option<constructors::ConstructorsConstructorStandingsTeams>>,
}

fn ShowConstructors<'a>(cx: Scope<'a, ShowConstructorsProps<'a>>) -> Element {
    let constructors = cx.props.constructors;

    cx.render(rsx! {
        table {
            border_collapse: "collapse",
            thead {
                tr {
                    th { "Position" }
                    th { "Team" }
                    th { "Points" }
                }
            }
            tbody {
                for constructor in constructors {
                    if let Some(team) = constructor {
                        rsx! {ShowConstructor { constructor: team }}
                    }
                }
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct ShowConstructorProps<'a> {
    constructor: &'a constructors::ConstructorsConstructorStandingsTeams,
}

fn ShowConstructor<'a>(cx: Scope<'a, ShowConstructorProps<'a>>) -> Element {
    let constructor = cx.props.constructor;
    let team = constructor.team.as_ref().expect("no team");

    cx.render(rsx! {
        tr {
            class: "border-2 hover:bg-gray-100 hover:ring-2 hover:ring-inset",
            td {
                if let Some(position) = &constructor.position {
                    match position.as_str() {
                        "1" => rsx! {render! { "🥇" }},
                        "2" => rsx! {render! { "🥈" }},
                        "3" => rsx! {render! { "🥉" }},
                        _ => rsx! {render! { position.to_string() }}
                    }
                }
            }
            td {
                if let (Some(name), Some(url)) = (&team.name, &team.url) {
                    rsx! {render! {
                        a {
                            href: "{url}",
                            target: "_blank",
                            "{name}"
                        }
                    }}
                }
            }
            td {
                if let Some(points) = &constructor.points {
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
pub struct Constructors;

async fn perform_my_query(
    variables: constructors::Variables,
) -> Result<Vec<Option<constructors::ConstructorsConstructorStandingsTeams>>, Box<dyn Error>> {
    let request_body = Constructors::build_query(variables);

    let response_body: Response<constructors::ResponseData> =
        get_resp_body_from_gql(&request_body).await.json().await?;
    Ok(response_body
        .data
        .ok_or("missing response data")?
        .constructor_standings
        .ok_or("missing constructor standings")?
        .teams
        .ok_or("missing teams")?)
}
