use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpRequest, HttpResponse,
};

//https://stackoverflow.com/questions/54406029/how-can-i-parse-query-strings-in-actix-web

use crate::model::user_model::{Role, User, UserBMC, UserPatch};
use crate::repository::surrealdb_repo::SurrealDBRepo;

#[post("/users")]
pub async fn create_user(db: Data<SurrealDBRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        username: new_user.username.to_owned(),
        password: new_user.password.to_owned(),
        role: match new_user.role {
            Role::Admin => Role::Admin,
            Role::User => Role::User,
        },
    };

    let user_detail = UserBMC::create(db, "user", data).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users/{id}")]
pub async fn get_user(db: Data<SurrealDBRepo>, path: Path<String>) -> HttpResponse {
    println!("path:id: {:?}", path);
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let user_detail = UserBMC::get(db, &id).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/users/{id}")]
pub async fn update_user(
    db: Data<SurrealDBRepo>,
    path: Path<String>,
    user_patch: Json<UserPatch>,
) -> HttpResponse {
    println!("path: {:?}", path);
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let data = UserPatch {
        username: user_patch.username.to_owned(),
        password: user_patch.password.to_owned(),
        role: user_patch.role.to_owned(),
    };

    let update_result = UserBMC::update(db, &id, data).await;

    match update_result {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(db: Data<SurrealDBRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let delete_result = UserBMC::delete(db, &id).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//https://actix.rs/docs/extractors/
#[get("/users")]
pub async fn get_users(db: Data<SurrealDBRepo>) -> HttpResponse {
    let users = UserBMC::get_all(db).await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// WE DO NOT WANT TO LOAD ALL USERS AND FILTER HERE IN MAP,
// WE WANT TO FILTER IN THE DB and the SQL only returns the filtered results
#[get("/users/{role:role}")]
pub async fn get_users_by_role(
    path: Path<String>,
    db: Data<SurrealDBRepo>,
    role: Path<Role>,
) -> HttpResponse {
    let path = path.into_inner();
    println!("pathGETUSRrolecallee: {:?}", path);
    // if role.is_empty() {
    //     return HttpResponse::BadRequest().body("invalid role");
    // }
    // let role = match role.as_str() {
    //     "admin" => Role::Admin,
    //     "user" => Role::User,
    //     _ => Role::User,
    // };

    //println!("call GETALL with role_arg: {:?}", role);

    let users = UserBMC::get_all_by_role(db, Role::User).await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
