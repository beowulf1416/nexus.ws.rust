#![allow(clippy::needless_return)]

use tracing::{debug, error, info};

use serde::{Deserialize, Serialize};
use sqlx::{
    Row,
    postgres::PgRow,
    prelude::{FromRow, Type},
};

use inv_provider::{
    Item, ItemLocation,
    transactions::purchase_order::{PurchaseOrder, PurchaseOrderItem},
};

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "mm.purchase_order_item_type")]
struct PurchaseOrderItemDerived {
    pub item_id: uuid::Uuid,
    pub quantity: rust_decimal::Decimal,
    pub uom_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PurchaseOrderDerived(pub PurchaseOrder);

impl<'r> FromRow<'r, PgRow> for PurchaseOrderDerived {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        return Ok(Self(PurchaseOrder {
            po_id: row.get("po_id"),
            active: row.get("active"),
            version: row.get("version"),
            description: row.get("description"),
            org_id: row.get("org_id"),
            partner_id: row.get("partner_id"),
            items: vec![],
        }));
    }
}

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
        purchase_order: &inv_provider::transactions::purchase_order::PurchaseOrder,
    ) -> Result<(), &'static str> {
        info!("save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            let derived_items = purchase_order
                .items
                .iter()
                .map(|item| PurchaseOrderItemDerived {
                    item_id: item.item_id.clone(),
                    quantity: item.quantity.clone(),
                    uom_id: item.uom_id,
                })
                .collect::<Vec<PurchaseOrderItemDerived>>();

            match pool.begin().await {
                Err(e) => {
                    error!("Error starting transaction: {:?}", e);
                    return Err("Error starting transaction");
                }
                Ok(mut tx) => {
                    match sqlx::query("call mm.purchase_order_save($1,$2,$3,$4,$5,$6);")
                        .bind(tenant_id)
                        .bind(purchase_order.po_id)
                        .bind(purchase_order.version)
                        .bind(purchase_order.description.clone())
                        .bind(purchase_order.org_id)
                        .bind(purchase_order.partner_id)
                        .execute(&mut *tx)
                        .await
                    {
                        Err(e) => {
                            error!("Error saving purchase order: {:?}", e);
                            return Err("Error saving purchase order");
                        }
                        Ok(_) => {
                            match sqlx::query("call mm.purchase_order_items_save($1,$2,$3);")
                                .bind(tenant_id)
                                .bind(purchase_order.po_id)
                                .bind(derived_items)
                                .execute(&mut *tx)
                                .await
                            {
                                Err(e) => {
                                    error!("Error saving purchase order items: {:?}", e);
                                    return Err("Error saving purchase order items");
                                }
                                Ok(_) => {
                                    if let Err(e) = tx.commit().await {
                                        error!("Error committing transaction: {:?}", e);
                                        return Err("Error committing transaction");
                                    }
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
        }

        return Err("No database pool found");
    }

    async fn fetch_by_id(&self, po_id: uuid::Uuid) -> Result<PurchaseOrder, &'static str> {
        info!("save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, PurchaseOrderDerived>(
                "select * from mm.purchase_order_fetch_by_id($1);",
            )
            .bind(po_id)
            .fetch_one(&pool)
            .await
            {
                Err(e) => {
                    error!("Error fetching purchase order: {:?}", e);
                    return Err("Error fetching purchase order");
                }
                Ok(row) => {
                    return Ok(row.0.clone());
                }
            }
        }

        return Err("No database pool found");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::item::ItemProviderPostgres;
    use crm_provider::CrmProvider;
    use inv_provider::{ItemProvider, transactions::purchase_order::PurchaseOrderProvider};
    use tenants_provider::{TenantsProvider, organizations::OrganizationsProvider};

    #[actix_web::test]
    async fn test_inventory_purchase_order() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
        let tenant_id = tp.tenant_fetch_by_name("tenant_01").await.unwrap().id;

        let opp = tenants_provider_postgres::organizations::OrganizationsProviderPostgres::new(&dp);
        let org_id = opp
            .fetch_by_name(&tenant_id, &"child_01")
            .await
            .unwrap()
            .org_id;

        let item_01_id = uuid::Uuid::new_v4();
        let item_02_id = uuid::Uuid::new_v4();

        let cpp = crm_provider_postgres::CrmProviderPostgres::new(&dp);
        let partner_id = cpp
            .partner_fetch_by_name(&tenant_id, &"partner_01")
            .await
            .unwrap()
            .partner_id;

        let ipp = ItemProviderPostgres::new(&dp);
        let ppp = PurchaseOrderProviderPostgres::new(&dp);

        let offset = rand::random::<u16>();

        let item_01 = Item {
            item_id: item_01_id,
            active: true,
            version: 1,
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
            name: format!("item_01_{}", offset),
            description: format!("item_01_{}", offset),
            sku: String::from(""),
            upc: String::from(""),
            perishable: false,
            hazardous: false,
            flammable: false,
            esd_sensitive: false,
        };

        let item_02 = Item {
            item_id: item_02_id,
            active: true,
            version: 1,
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
            name: format!("item_02_{}", offset),
            description: format!("item_02_{}", offset),
            sku: String::from(""),
            upc: String::from(""),
            perishable: false,
            hazardous: false,
            flammable: false,
            esd_sensitive: false,
        };

        if let Err(e) = ipp.item_save(&tenant_id, &item_01).await {
            error!("unable to create inventory item: {:?}", e);
            assert!(false, "unable to create inventory item");
        }

        if let Err(e) = ipp.item_save(&tenant_id, &item_02).await {
            error!("unable to create inventory item: {:?}", e);
            assert!(false, "unable to create inventory item");
        }

        let po_id = uuid::Uuid::new_v4();

        let mut po = PurchaseOrder {
            po_id: po_id,
            active: true,
            version: 1,
            description: String::from(""),
            org_id: org_id,
            partner_id: partner_id,
            items: vec![
                PurchaseOrderItem {
                    item_id: item_01_id,
                    quantity: rust_decimal::Decimal::new(1, 0),
                    uom_id: 1,
                },
                PurchaseOrderItem {
                    item_id: item_02_id,
                    quantity: rust_decimal::Decimal::new(2, 0),
                    uom_id: 1,
                },
            ],
        };

        if let Err(e) = ppp.save(&tenant_id, &po).await {
            error!("unable to save purchase order: {:?}", e);
            assert!(false, "unable to save purchase order");
        }

        // attempt to remove one item
        po.items.remove(0);

        if let Err(e) = ppp.save(&tenant_id, &po).await {
            error!("unable to update purchase order: {:?}", e);
            assert!(false, "unable to update purchase order");
        }
    }
}
