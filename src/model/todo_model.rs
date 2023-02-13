use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Array, Object, Value};

use crate::prelude::*;
use crate::repository::surrealdb_repo::{Creatable, Patchable, SurrealDBRepo};
use crate::utils::macros::map;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub body: String,
}

// -- Region From<Todo>

impl From<Todo> for Value {
    fn from(val: Todo) -> Self {
        match val.id {
            Some(v) => map![
                    "id".into() => v.into(),
                    "title".into() => val.title.into(),
                    "body".into() => val.body.into(),
            ]
            .into(),
            None => map![
                "title".into() => val.title.into(),
                "body".into() => val.body.into()
            ]
            .into(),
        }
    }
}

impl Creatable for Todo {}

// -- end region

// -- Region From<TodoPatch>

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoPatch {
    pub title: Option<String>,
    pub body: Option<String>,
}

impl From<TodoPatch> for Value {
    fn from(val: TodoPatch) -> Self {
        let mut value: BTreeMap<String, Value> = BTreeMap::new();

        if let Some(t) = val.title {
            value.insert("title".into(), t.into());
        }

        if let Some(b) = val.body {
            value.insert("body".into(), b.into());
        }
        Value::from(value)
    }
}

impl Patchable for TodoPatch {}

// end region

// -- ModelController

pub struct TodoBMC;
impl TodoBMC {
    pub async fn get_all(db: Data<SurrealDBRepo>) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM todo;";

        let res = db.ds.execute(ast, &db.ses, None, true).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }
}
