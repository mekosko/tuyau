use {sea_orm::DatabaseConnection, tokio::net::TcpListener};

use axum::{routing::get, Router};
use ruma::{owned_room_alias_id, owned_user_id};

use crate::{models::DefaultQueryExecutor, router::get_room_information_route, worker::Executor};

mod models;
mod router;
mod setups;
mod worker;

pub type MyResult<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> MyResult<()> {
	let (query_executor, room_id, user_id) = (
		DefaultQueryExecutor::new(DatabaseConnection::Disconnected),
		owned_room_alias_id!("#stokejo:stokejo.com"),
		owned_user_id!("@mekosko:projectyo.network"),
	);
	let (query_executor, room_id, user_id) = (
		Box::leak(Box::new(query_executor)),
		Box::leak(Box::new(room_id)),
		Box::leak(Box::new(user_id)),
	);
	let state = Executor::new(query_executor, room_id, user_id);

	let app = Router::new()
		.route(
			"/_matrix/federation/v1/query/directory",
			get(get_room_information_route),
		)
		.with_state(state);

	let tcp = TcpListener::bind(":::2727").await?;

	axum::serve(tcp, app).await?;

	return Ok(());
}
