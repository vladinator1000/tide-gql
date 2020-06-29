use tide::{Redirect, Server};

mod graphql;

pub type Request = tide::Request<graphql::Context>;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let context = graphql::Context { db: 1 };
    let mut app = Server::with_state(context);

    app.at("/").get(Redirect::permanent("/graphiql"));
    app.at("/graphiql").get(graphql::handle_graphiql);
    app.at("/graphql").post(graphql::handle_graphql);
    app.listen("127.0.0.1:3131").await?;
    Ok(())
}
