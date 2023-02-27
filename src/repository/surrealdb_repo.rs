#[allow(dead_code)] //TODO: remove this
use std::sync::Arc;
use surrealdb::sql::Value;
use surrealdb::{Datastore, Error, Session};

pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}

#[derive(Clone)]
pub struct SurrealDBRepo {
    pub ds: Arc<Datastore>,
    pub ses: Session,
}

impl SurrealDBRepo {
    pub async fn init() -> Result<Self, Error> {
        let ds = Arc::new(Datastore::new("file:://surrealdb").await?);
        let ses = Session::for_kv().with_ns("test").with_db("test");
        Ok(SurrealDBRepo { ses, ds })
    }

    pub async fn init_with_name(name: &str) -> Result<Self, Error> {
        let file_name = format!("file:://surrealdb/{}", name);
        let ds = Arc::new(Datastore::new(&file_name).await?);
        let ses = Session::for_kv().with_ns("test").with_db("test");
        Ok(SurrealDBRepo { ses, ds })
    }

    pub async fn init_with_name_and_ns_and_db(
        name: &str,
        ns: &str,
        db: &str,
    ) -> Result<Self, Error> {
        let file_name = format!("file:://surrealdb/{}", name);
        let ds = Arc::new(Datastore::new(&file_name).await?);
        let ses = Session::for_kv().with_ns(ns).with_db(db);
        Ok(SurrealDBRepo { ses, ds })
    }
}
