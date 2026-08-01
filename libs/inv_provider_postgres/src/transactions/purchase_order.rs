#![allow(clippy::needless_return)]

use inv_provider::{Item, ItemLocation};
use tracing::{debug, error, info};

use sqlx::{Row, postgres::PgRow, prelude::FromRow};

pub struct PurchaseOrderProviderPostgres {
    dp: database_provider::DatabaseProvider,
}

impl PurchaseOrderProviderPostgres {
    pub fn new(dp: &database_provider::DatabaseProvider) -> Self {
        return Self { dp: dp.clone() };
    }
}

impl inv_provider::transactions::purchase_order::PurchaseOrderProvider
    for PurchaseOrderProviderPostgres
{
    async fn save(
        &self,
        tenant_id: &uuid::Uuid,
        purchase_order: inv_provider::transactions::purchase_order::PurchaseOrder,
    ) -> Result<(), &'static str> {
        info!("save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call mm.purchase_order_save($1,$2);")
                .bind(tenant_id)
                .bind(purchase_order.po_id)
                .bind(purchase_order.version)
                .bind(purchase_order.description)
                .bind(purchase_order.org_id)
                .bind(purchase_order.partner_id)
                .execute(&pool)
                .await
            {
                Err(e) => {
                    error!("Error saving purchase order: {:?}", e);
                    return Err("Error saving purchase order");
                }
                Ok(_) => {
                    return Ok(());
                }
            }
        }

        return Err("No database pool found");
    }
}
