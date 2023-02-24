use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::sql::statements::CreateStatement;
use surrealdb::sql::{thing, Array, Object, Value};

use crate::prelude::*;
use crate::repository::surrealdb_repo::{Creatable, Patchable, SurrealDBRepo};
use crate::utils::macros::map;

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub username: String,
    pub password: String,
    pub role: String,
}

// -- Region From<User>
impl From<User> for Value {
    fn from(user: User) -> Self {
        match user.id {
            Some(u) => map![
              "id".into() => u.into(),
              "username".into() => user.username.into(),
              "password".into() => user.password.into(),
              "role".into() => user.role.into(), // how to cover this as Role Type
            ]
            .into(),
            None => map![],
        }
    }
}

impl Creatable for User {}

//-- end region

// -- Region From<UserPatch>

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPatch {
    pub username: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
}

impl From<UserPatch> for Value {
    fn from(user: UserPatch) -> Self {
        let mut value: BTreeMap<String, Value> = BTreeMap::new();
        if let Some(u) = user.username {
            value.insert("username".into(), u.into());
        }
        if let Some(p) = user.password {
            value.insert("password".into(), p.into());
        }
        if let Some(r) = user.role {
            value.insert("role".into(), r.into());
        }

        Value::from(value)
    }
}

impl Patchable for UserPatch {}

// -- end region

// -- Modelcontroller
pub struct UserBMC;
impl UserBMC {
    pub async fn get_all(db: Data<SurrealDBRepo>) -> Result<Vec<Object>, Error> {
        let sql = "SELECT * FROM user;";
        let res = db.ds.execute(sql, &db.ses, None, true).await?;
        let first_res = res.into_iter().next().expect("Did not get a User response");
        let array: Array = W(first_res.result?).try_into()?;
        array.into_iter().map(|v| W(v).try_into()).collect() //
    }

    pub async fn create<T: Creatable>(
        db: Data<SurrealDBRepo>,
        tb: &str,
        data: T,
    ) -> Result<Object, Error> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN *";
        let data: Object = W(data.into()).try_into()?;
        let vars: BTreeMap<String, Value> = map![
            "tb".into() => tb.into(),
            "data".into() => Value::from(data),
        ];
        let ress = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

        let first_val = ress
            .into_iter()
            .next()
            .map(|r| r.result)
            .expect("id not returned")?;

        W(first_val.first()).try_into()
    }
}
