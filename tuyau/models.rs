use ruma::events::pdu::RoomV3Pdu;
use sea_orm::{
	ActiveModelBehavior, ConnectionTrait, DatabaseConnection, DeriveEntityModel, DerivePrimaryKey,
	DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Schema,
};

use crate::{
	models,
	worker::{QueryExecutor, SetupBundle},
	Maybe, MyResult,
};

pub mod keyserver;
pub mod state;
pub mod timeline;

pub type MaybePdu = Maybe<RoomV3Pdu>;

// =========================================================================

#[derive(Clone)]
pub struct DefaultQueryExecutor {
	inner: DatabaseConnection,
}

impl DefaultQueryExecutor {
	pub async fn new(inner: DatabaseConnection) -> MyResult<Self> {
		let backend = inner.get_database_backend();
		let query = Schema::new(backend);

		for mut statement in [
			query.create_table_from_entity(models::keyserver::Entity),
			query.create_table_from_entity(models::timeline::Entity),
			query.create_table_from_entity(models::Entity),
		] {
			let statement = backend.build(statement.if_not_exists());
			inner.execute(statement).await?;
		}
		Ok(DefaultQueryExecutor { inner })
	}
}

// =========================================================================

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "setup")]
pub struct Model {
	#[sea_orm(primary_key, unique, auto_increment = false)]
	alias: String,
	admin: String,
	ident: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// =========================================================================

impl QueryExecutor for DefaultQueryExecutor {
	async fn new(&self, setup: SetupBundle) -> MyResult<()> {
		todo!()
	}

	async fn get(&self) -> MyResult<Maybe<SetupBundle>> {
		todo!()
	}
}
