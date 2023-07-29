use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Extension, Router, Server};
use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::signal;
use tracing::{info, span, Instrument, Level};

type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

struct QueryRoot;
#[Object]

impl QueryRoot {
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello, world!"
    }
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

async fn graphql_handler(
    req: GraphQLRequest,
    Extension(schema): Extension<ServiceSchema>,
) -> GraphQLResponse {
    let span = span!(Level::INFO, "graphql_execution");

    info!("Processing GraphQL request");

    let response = async move { schema.execute(req.into_inner()).await }
        .instrument(span.clone())
        .await;

    info!("Processing GraphQL request finished");

    response.into()
}
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    info!("Starting serve");
    let app = Router::new().route("/", get(graphql_playground).post(graphql_handler));

    Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8080)))
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
