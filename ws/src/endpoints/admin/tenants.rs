use tracing::{
    info,
    debug,
    error
};
use std::sync::Arc;
use serde::Deserialize;
use serde_json::json;
use actix_web::{
    http, 
    web, 
    guard,
    HttpResponse, 
    Responder
};


use crate::endpoints::{
    ApiResponse,
    default_option_response
};
use crate::middleware::permissions::Permission;

use tenants_provider::TenantsProvider;
use users_provider::UsersProvider;
use roles_provider::{ 
    Role,
    RolesProvider
};




pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("fetch/id")
                .wrap(Permission::new("tenant.fetch"))
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(admin_tenants_fetch_id))
        )
        .service(
            web::resource("save")
                .wrap(Permission::new("tenant.save"))
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(admin_tenants_save)
                )
        )
        .service(
            web::resource("fetch")
                .wrap(Permission::new("tenant.list"))
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(admin_tenants_fetch))
        )
        .service(
            web::resource("fetch/users")
                .wrap(Permission::new("tenant.users.list"))
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(admin_tenants_fetch_users))
        )
        .service(
            web::resource("set/active")
                .wrap(Permission::new("tenant.set.active"))
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(admin_tenants_set_active))
        )
        .service(
            web::resource("role/save")
                .wrap(Permission::new("tenant.roles.save"))
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(admin_role_save_post))
        )
        .service(
            web::resource("roles/fetch")
                .wrap(Permission::new("tenant.roles.list"))
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(admin_roles_fetch_post))
        )
        .service(
            web::resource("role/assign/users")
                .wrap(Permission::new("tenant.role.assign.users"))
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(admin_role_assign_users_post))
        )
        .service(
            web::resource("role/revoke/users")
                .wrap(Permission::new("tenant.role.assign.users"))
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(admin_role_revoke_users_post))
        )
        .service(
            web::resource("role/assign/permissions")
                .wrap(Permission::new("tenant.role.assign.permission"))
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(admin_role_assign_permissions_post))
        )
        .service(
            web::resource("role/revoke/permissions")
                .wrap(Permission::new("tenant.role.assign.permission"))
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(admin_role_revoke_permissions_post))
        )
        // .service(
        //     web::resource("users/fetch")
        //         .wrap(Permission::new("tenant.users.list"))
        //         .route(web::method(http::Method::OPTIONS).to(default_option_response))
        //         .route(web::post().guard(guard::Header("content-type", "application/json")).to(tenant_users_fetch_post))
        // )
    ;
}



#[derive(Debug, Deserialize)]
struct AdminTenantFetchById {
    tenant_id: uuid::Uuid
}

async fn admin_tenants_fetch_id(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<AdminTenantFetchById>
) -> impl Responder {
    info!("admin_tenants_fetch_id");

    let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);

    match tp.tenants_fetch_by_id(&params.tenant_id).await {
        Err(e) => {
            error!("unable to fetch tenant by id: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch tenant by id"));
        }
        Ok(tenant) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    &"successfully retrieved tenant by id",
                    Some(json!({
                        "tenant": tenant
                    }))
                ));
        }
    }
}



#[derive(Debug, Deserialize)]
struct AdminTenantSavePost {
    tenant_id: uuid::Uuid,
    name: String,
    description: String
}


async fn admin_tenants_save(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<AdminTenantSavePost>
) -> impl Responder {
    info!("admin_tenants_save");

    let atp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);

    if let Err(e) = atp.tenant_save(
        &params.tenant_id, 
        &params.name, 
        &params.description
    ).await {
        error!("unable to save tenant: {}", e);
        return HttpResponse::InternalServerError()
            .json(ApiResponse::error("unable to save tenant"));
    }

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}




#[derive(Debug, Deserialize)]
struct AdminTenantsFetchPost {
    filter: String
}

async fn admin_tenants_fetch(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<AdminTenantsFetchPost>
) -> impl Responder {
    info!("admin_tenants_fetch");

    let atp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);

    let filter = format!("%{}%", params.filter);

    match atp.tenants_fetch(
        filter.as_str()
    ).await {
        Err(e) => {
            error!("unable to fetch tenant records: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch tenant records"));
        }
        Ok(tenants) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully retrieved tenant records",
                    Some(json!({
                        "tenants": tenants
                    }))
                ));
        }
    }
}




#[derive(Debug, Deserialize)]
struct AdminTenantUsersPost {
    tenant_id: uuid::Uuid,
    filter: String
}

async fn admin_tenants_fetch_users(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<AdminTenantUsersPost>
) -> impl Responder {
    info!("admin_tenants_fetch_users");

    let up = users_provider_postgres::PostgresUsersProvider::new(&dp);

    match up.tenant_users_fetch(
        &params.tenant_id,
        format!("%{}%", params.filter).as_str()
    ).await {
        Err(e) => {
            error!("unable to fetch tenant users: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch tenant users"));
        }
        Ok(users) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully retrieved tenant users",
                    Some(json!({
                        "users": users
                    }))
                ));
        }
    }
}




#[derive(Debug, Deserialize)]
struct RoleSavePost {
    tenant_id: uuid::Uuid,
    role_id: uuid::Uuid,
    name: String,
    description: String
}

async fn admin_role_save_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<RoleSavePost>
) -> impl Responder {
    info!("admin_role_save_post");

    let rp = roles_provider_postgres::PostgresRolesProvider::new(&dp);

    let role = roles_provider::Role {
        role_id: params.role_id,
        name: params.name.clone(),
        description: params.description.clone(),
        active: true,
        created: chrono::Utc::now()
    };

    match rp.save(
        &params.tenant_id,
        &role
    ).await {
        Err(e) => {
            error!("unable to add role: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to add role"));
        }
        Ok(_) => {
            return HttpResponse::Ok()
                .json(ApiResponse::ok("successfully added role"));
        }
    }
}



#[derive(Debug, Deserialize)]
struct RolesFetchPost {
    tenant_id: uuid::Uuid,
    filter: String
}

async fn admin_roles_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<RolesFetchPost>
) -> impl Responder {
    info!("admin_roles_fetch_post");

    let rp = roles_provider_postgres::PostgresRolesProvider::new(&dp);

    match rp.fetch(
        &params.tenant_id,
        // &params.filter
        format!("%{}%", params.filter).as_str()
    ).await {
        Err(e) => {
            error!("unable to fetch roles: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch roles"));
        }
        Ok(roles) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully fetched roles",
                    Some(json!({
                        "roles": roles
                    }))
                ));
        }
    }
}



#[derive(Debug, Deserialize)]
struct RoleUserAssignmentPost {
    role_ids: Vec<uuid::Uuid>,
    user_ids: Vec<uuid::Uuid>
}

async fn admin_role_assign_users_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<RoleUserAssignmentPost>
) -> impl Responder {
    info!("admin_role_assign_post");

    let rp = roles_provider_postgres::PostgresRolesProvider::new(&dp);

    match rp.assign_users(
        &params.role_ids,
        &params.user_ids
    ).await {
        Err(e) => {
            error!("unable to assign role to users: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to assign role to users"));
        }
        Ok(_) => {
            return HttpResponse::Ok()
                .json(ApiResponse::ok("successfully assigned role to users"));
        }
    }
}

async fn admin_role_revoke_users_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<RoleUserAssignmentPost>
) -> impl Responder {
    info!("admin_role_revoke_post");

    let rp = roles_provider_postgres::PostgresRolesProvider::new(&dp);

    match rp.revoke_users(
        &params.role_ids,
        &params.user_ids
    ).await {
        Err(e) => {
            error!("unable to revoke role from users: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to revoke role from users"));
        }
        Ok(_) => {
            return HttpResponse::Ok()
                .json(ApiResponse::ok("successfully revoked role from users"));
        }
    }
}



#[derive(Debug, Deserialize)]
struct RolePermissionsAssignmentPost {
    role_ids: Vec<uuid::Uuid>,
    permission_ids: Vec<i32>
}

async fn admin_role_assign_permissions_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<RolePermissionsAssignmentPost>
) -> impl Responder {
    info!("admin_role_assign_permissions_post");

    let rp = roles_provider_postgres::PostgresRolesProvider::new(&dp);

    match rp.assign_permissions(
        &params.role_ids,
        &params.permission_ids
    ).await {
        Err(e) => {
            error!("unable to assign permissions to role: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to assign permissions to role"));
        }
        Ok(_) => {
            return HttpResponse::Ok()
                .json(ApiResponse::ok("successfully assigned permissions to role"));
        }
    }
}


async fn admin_role_revoke_permissions_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<RolePermissionsAssignmentPost>
) -> impl Responder {
    info!("admin_role_revoke_permissions_post");

    let rp = roles_provider_postgres::PostgresRolesProvider::new(&dp);

    match rp.revoke_permissions(
        &params.role_ids,
        &params.permission_ids
    ).await {
        Err(e) => {
            error!("unable to revoke permissions from role: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to revoke permissions from role"));
        }
        Ok(_) => {
            return HttpResponse::Ok()
                .json(ApiResponse::ok("successfully revoke permissions from role"));
        }
    }
}


#[derive(Debug, Deserialize)]
struct AdminTenantSetActive {
    tenant_ids: Vec<uuid::Uuid>,
    active: bool
}

async fn admin_tenants_set_active(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<AdminTenantSetActive>
) -> impl Responder {
    info!("admin_tenants_set_active");

    let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);

    match tp.tenants_set_active(
        &params.tenant_ids,
        &params.active
    ).await {
        Err(e) => {
            error!("unable to set tenants active state: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to set tenants active state"));
        }
        Ok(_) => {
            return HttpResponse::Ok()
                .json(ApiResponse::ok("successfully set tenants active state"));
        }
    }
}



/*
#[derive(Debug, Deserialize)]
struct TenantUsersFetchPost {
    tenant_id: uuid::Uuid,
    filter: String
}


async fn tenant_users_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<TenantUsersFetchPost>
) -> impl Responder {
    info!("tenant_users_fetch_post");
    debug!("params: {:?}", params);

    let up = users_provider_postgres::PostgresUsersProvider::new(&dp);

    match up.tenant_users_fetch(
        &params.tenant_id,
        format!("%{}%", params.filter).as_str()
    ).await {
        Err(e) => {
            error!("unable to fetch tenant users: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch tenant users"));
        }
        Ok(users) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully fetch tenant users",
                    Some(json!({
                        "users": users
                    }))
                ));
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use tracing::error;
    use users_provider::UsersProvider;
    use tenants_provider::TenantsProvider;
    use roles_provider::RolesProvider;


    #[actix_web::test]
    async fn test_create_test_accounts() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        // tenant
        let tenant_id = uuid::Uuid::new_v4();
        let tenant_name = format!("tenant_{}", rand::random::<u16>());
        let tenant_description = "test_tenant";

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);

        if let Err(e) = tp.tenant_save(&tenant_id, &tenant_name, &tenant_description).await {
            error!(e);
            assert!(false, "unable to save tenant record");
        }

        if let Err(e) = tp.tenant_set_active(&tenant_id, &true).await {
            error!(e);
            assert!(false, "unable to set tenant active state");
        }

        // user
        let user_id = uuid::Uuid::new_v4();
        let user_email = format!("test_{}@test.com", rand::random::<u16>());

        let user_first_name = "test_first";
        let user_middle_name = "test_middle";
        let user_last_name = "test_last";
        let user_prefix = "test_prefix";
        let user_suffix = "test_suffix";

        let up = users_provider_postgres::PostgresUsersProvider::new(&dp);

        if let Err(e) = up.save(&user_id, &user_first_name, &user_middle_name, &user_last_name, &user_prefix, &user_suffix).await {
            error!(e);
            assert!(false, "unable to save user");
        }

        if let Err(e) = up.set_active(&user_id, &true).await {
            error!(e);
            assert!(false, "unable to set user active state");
        }

        if let Err(e) = up.add_email(&user_id, &user_email).await {
            error!(e);
            assert!(false, "unable to add user email");
        }

        // assign user to tenant
        if let Err(e) = up.tenant_assign(
            &vec![user_id], 
            &vec![tenant_id]
        ).await {
            error!(e);
            assert!(false, "unable to fetch assign users to tenants");
        }


        // roles
        let role_id = uuid::Uuid::new_v4();
        let role_name = format!("role_{}", rand::random::<u16>());
        let role_description = "roles_provider_postgres_test";

        let role = roles_provider::Role {
            role_id,
            name: role_name,
            description: String::from(role_description),
            active: true,
            created: chrono::Utc::now()
        };

        let rp = roles_provider_postgres::PostgresRolesProvider::new(&dp);

        if let Err(e) = rp.save(&tenant_id, &role).await {
            error!("unable to create role: {:?}", e);
            assert!(false, "unable to create role");
        }
        
        if let Err(e) = rp.assign_permissions(
            &vec!(role_id),
            &vec!(1)
        ).await {
            error!("unable to assign permission to role: {}", e);
            assert!(false, "unable to assign permission to role");
        }
    }
}