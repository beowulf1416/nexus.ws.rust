use tracing::{
    info,
    debug,
    error
};
use std::sync::Arc;

use serde::{
    Serialize,
    Deserialize
};
use serde_json::json;


use actix_web::{
    http, 
    web, 
    HttpResponse, 
    Responder
};

use crate::endpoints::{
    ApiResponse,
    default_option_response
};

use auth_provider::AuthProvider;
use users_provider::UsersProvider;



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("create")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(users_create_post))
        )
        .service(
            web::resource("set/active")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(users_set_active_post))
        )
        .service(
            web::resource("set/password")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(users_set_password_post))
        )
        .service(
            web::resource("fetch")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(users_fetch_post))
        )
        .service(
            web::resource("assign/tenants")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(users_tenants_assign_post))
        )
    ;
}




#[derive(Debug, Deserialize)]
struct UsersCreatePost {
    user_id: uuid::Uuid,
    email: String,
    pw: String
}

async fn users_create_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UsersCreatePost>
) -> impl Responder {
    info!("users_create_post");

    let authp = auth_provider_postgres::PostgresAuthProvider::new(&dp);
    let up = users_provider_postgres::PostgresUsersProvider::new(&dp);

    if let Err(e) = up.save(
        &params.user_id, 
        &"",
        &"",
        &"",
        &"",
        &""
    ).await {
        error!("unable to save user data: {}", e);
        return HttpResponse::InternalServerError()
            .json(ApiResponse::error("unable to save user data"));
    }

    if let Err(e) = up.add_email(
        &params.user_id,
        &params.email
    ).await {
        error!("unable to save user email: {}", e);
        return HttpResponse::InternalServerError()
            .json(ApiResponse::error("unable to save user email"));
    }

    if let Err(e) = authp.add_user_auth_password(
        &params.user_id, 
        &params.email,
        &params.pw
    ).await {
        error!("unable to add user account: {}", e);
        return HttpResponse::InternalServerError()
            .json(ApiResponse::error("unable to add user account"));
    }

    return HttpResponse::Ok()
        .json(ApiResponse::ok("successfully create user account"));
}


#[derive(Debug, Deserialize)]
struct UserSetActivePost {
    user_id: uuid::Uuid,
    active: bool
}


// #[allow(dead_code)]
async fn users_set_active_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UserSetActivePost>
) -> impl Responder {
    info!("users_set_active_post");

    let up = users_provider_postgres::PostgresUsersProvider::new(&dp);

    if let Err(e) = up.set_active(
        &params.user_id,
        &params.active
    ).await {
        error!("unable to set user active status: {}", e);
        return HttpResponse::InternalServerError()
            .json(ApiResponse::error("unable to set user active status"));
    }

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}




#[derive(Debug, Deserialize)]
struct UsersSetPasswordPost {
    user_id: uuid::Uuid,
    password: String
}



async fn users_set_password_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UsersSetPasswordPost>
) -> impl Responder {
    info!("users_set_password_post");

    let ap = auth_provider_postgres::PostgresAuthProvider::new(&dp);

    


    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}




#[derive(Debug, Deserialize)]
struct UsersFetchPost {
    filter: String
}

async fn users_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UsersFetchPost>
) -> impl Responder {
    info!("users_fetch_post");

    let up = users_provider_postgres::PostgresUsersProvider::new(&dp);
    match up.fetch(
        &params.filter
    ).await {
        Err(e) => {
            error!("unable to fetch users: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch users"));
        }
        Ok(users) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully retrieved users",
                    Some(json!({
                        "users": users
                    }))
                ))
        }
    };
}



#[derive(Debug, Deserialize)]
struct UsersAssignTenantsPost {
    user_ids: Vec<uuid::Uuid>,
    tenant_ids: Vec<uuid::Uuid>
}

async fn users_tenants_assign_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UsersAssignTenantsPost>
) -> impl Responder {
    info!("users_tenants_assign_post");

    return HttpResponse::Ok()
        .json(ApiResponse::ok("//todo users_tenants_assign_post"));
}