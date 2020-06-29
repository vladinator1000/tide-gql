use juniper::{http::graphiql, http::GraphQLRequest, RootNode};
use tide::{http::mime, Body, Response, StatusCode};

use user::User;

pub mod user;

pub struct Context {
    pub db: i32,
}

pub struct RootQuery;

#[juniper::object(Context=Context)]
impl RootQuery {
    fn current_user(_context: &Context) -> User {
        User {
            id: 1,
            name: "Jon".to_string(),
        }
    }
}

pub struct RootMutation;

#[juniper::object(Context=Context)]
impl RootMutation {}

pub type Schema = RootNode<'static, RootQuery, RootMutation>;

pub fn make_schema() -> Schema {
    Schema::new(RootQuery {}, RootMutation {})
}

pub async fn handle_graphql(mut request: crate::Request) -> tide::Result {
    let query: GraphQLRequest = request.body_json().await?;
    let schema = make_schema();
    let gql_response = query.execute(&schema, request.state());

    let status = if gql_response.is_ok() {
        StatusCode::Ok
    } else {
        StatusCode::BadRequest
    };

    let mut response = Response::new(status);
    let body = Body::from_json(&gql_response)?;
    response.set_body(body);

    Ok(response)
}

pub async fn handle_graphiql(_: crate::Request) -> tide::Result {
    let mut response = Response::new(200);
    response.set_body(graphiql::graphiql_source("/graphql"));
    response.set_content_type(mime::HTML);

    Ok(response)
}
