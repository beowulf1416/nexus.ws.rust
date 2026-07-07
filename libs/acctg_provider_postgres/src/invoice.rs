#![allow(clippy::needless_return)]

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{Encode, Row, Type, postgres::PgHasArrayType, postgres::types::PgMoney};
use tracing::{debug, error, info};

use acctg_provider::invoice::{Invoice, InvoiceItem, InvoiceProvider, InvoiceType};

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "acctg.invoice_item_type")]
pub struct InvoiceItemDerived {
    pub item_id: uuid::Uuid,
    pub description: String,
    pub quantity: Decimal,
    // pub uom_id: i32,
    // #[serde(with = "money_format")]
    pub unit_price: Decimal,
    // pub total: Decimal,
    pub currency_id: i32,
}

pub struct InvoiceProviderPostgres {
    dp: database_provider::DatabaseProvider,
}

impl InvoiceProviderPostgres {
    pub fn new(dp: &database_provider::DatabaseProvider) -> Self {
        return Self { dp: dp.clone() };
    }
}

impl InvoiceProvider for InvoiceProviderPostgres {
    async fn invoice_types_fetch(&self) -> Result<Vec<InvoiceType>, &'static str> {
        info!("invoice_types_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from acctg.invoice_types_fetch();")
                .fetch_all(&pool)
                .await
            {
                Ok(rows) => {
                    let types: Vec<InvoiceType> = rows
                        .iter()
                        .map(|r| {
                            let invoice_type_id: i32 = r.get("invoice_type_id");
                            let name: String = r.get("name");
                            return InvoiceType {
                                id: invoice_type_id,
                                name,
                            };
                        })
                        .collect();

                    return Ok(types);
                }
                Err(e) => {
                    error!("Error fetching invoice types: {:?}", e);
                    return Err("Error fetching invoice types");
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn invoices_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str,
    ) -> Result<Vec<Invoice>, &'static str> {
        info!("invoices_fetch");
        // debug!("tenant_id: {:?}, filter: {}", tenant_id, filter);

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from acctg.invoices_fetch($1,$2);")
                .bind(tenant_id)
                .bind(filter)
                .fetch_all(&pool)
                .await
            {
                Ok(rows) => {
                    let invoices: Vec<Invoice> = rows
                        .iter()
                        .map(|r| {
                            let invoice_id: uuid::Uuid = r.get("invoice_id");
                            let invoice_type_id: i32 = r.get("invoice_type_id");
                            let invoice_id_seq: i32 = r.get("invoice_id_seq");
                            let active: bool = r.get("active");
                            let created_at: chrono::DateTime<chrono::Utc> = r.get("created_ts");
                            let due_date: Option<chrono::DateTime<chrono::Utc>> =
                                r.get("due_date_ts");
                            let description: String = r.get("description");
                            return Invoice {
                                invoice_id: invoice_id,
                                invoice_type_id: invoice_type_id,
                                invoice_id_seq: invoice_id_seq,
                                active: active,
                                created_at: created_at,
                                due_date: due_date,
                                description: description,
                                items: Vec::new(),
                            };
                        })
                        .collect();

                    return Ok(invoices);
                }
                Err(e) => {
                    error!("Error fetching invoices: {:?}", e);
                    return Err("Error fetching invoices");
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn invoice_fetch(&self, invoice_id: &uuid::Uuid) -> Result<Invoice, &'static str> {
        info!("invoice_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from acctg.invoice_fetch($1);")
                .bind(invoice_id)
                .fetch_one(&pool)
                .await
            {
                Ok(row) => {
                    let invoice_id: uuid::Uuid = row.get("invoice_id");
                    let invoice_type_id: i32 = row.get("invoice_type_id");
                    let invoice_id_seq: i32 = row.get("invoice_id_seq");
                    let active: bool = row.get("active");
                    let created_at: chrono::DateTime<chrono::Utc> = row.get("created_ts");
                    let due_date: Option<chrono::DateTime<chrono::Utc>> = row.get("due_date_ts");
                    let description: String = row.get("description");
                    return Ok(Invoice {
                        invoice_id: invoice_id,
                        invoice_type_id: invoice_type_id,
                        invoice_id_seq: invoice_id_seq,
                        active: active,
                        created_at: created_at,
                        due_date: due_date,
                        description: description,
                        items: Vec::new(),
                    });
                }
                Err(e) => {
                    error!("Error fetching invoices: {:?}", e);
                    return Err("Error fetching invoices");
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn invoice_save(
        &self,
        tenant_id: &uuid::Uuid,
        invoice: &Invoice,
    ) -> Result<(), &'static str> {
        info!("invoice_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call acctg.invoice_save($1,$2,$3,$4,$5);")
                .bind(tenant_id)
                .bind(&invoice.invoice_id)
                .bind(&invoice.invoice_type_id)
                .bind(&invoice.description)
                .bind(&invoice.due_date)
                // .bind(&invoice.currency_id)
                .execute(&pool)
                .await
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    error!("Error saving invoice: {:?}", e);
                    return Err("Error saving invoice");
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn invoice_items_save(
        &self,
        invoice_id: &uuid::Uuid,
        items: &Vec<InvoiceItem>,
    ) -> Result<(), &'static str> {
        info!("invoice_items_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            let derived_items = items
                .iter()
                .map(|item| InvoiceItemDerived {
                    item_id: item.item_id,
                    description: item.description.clone(),
                    quantity: item.quantity,
                    // uom_id: item.uom_id,
                    unit_price: item.unit_price,
                    // total: item.total,
                    currency_id: item.currency_id,
                })
                .collect::<Vec<InvoiceItemDerived>>();

            match sqlx::query("call acctg.invoice_items_save($1,$2);")
                .bind(&invoice_id)
                .bind(&derived_items)
                .execute(&pool)
                .await
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    error!("Error saving invoice items: {:?}", e);
                    return Err("Error saving invoice items");
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use acctg_provider::invoice::InvoiceProvider;
    use tenants_provider::TenantsProvider;

    #[actix_web::test]
    async fn test_invoice() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
        let ipp = InvoiceProviderPostgres::new(&dp);

        if let Err(e) = ipp.invoice_types_fetch().await {
            error!(e);
            assert!(false, "unable to fetch invoice types");
        }

        let tenant = tp.tenant_fetch_by_name("tenant_01").await.unwrap();
        let tenant_id = tenant.tenant_id();

        let invoice_id = uuid::Uuid::new_v4();

        let today = chrono::Local::now();
        let due_date = today.checked_add_days(chrono::Days::new(3)).unwrap();

        let invoice = Invoice {
            invoice_id: invoice_id,
            invoice_type_id: 1,
            due_date: Some(due_date.to_utc()),
            description: String::from("test invoice 1"),

            invoice_id_seq: 0,
            created_at: today.to_utc(),
            active: true,
            items: Vec::new(),
        };

        if let Err(e) = ipp.invoice_save(&tenant_id, &invoice).await {
            error!(e);
            assert!(false, "unable to save invoice");
        }

        if let Err(e) = ipp.invoice_fetch(&invoice_id).await {
            error!(e);
            assert!(false, "unable to fetch invoice");
        }

        if let Err(e) = ipp.invoices_fetch(&tenant_id, &"%").await {
            error!(e);
            assert!(false, "unable to fetch invoices");
        }

        let invoice_items = vec![
            InvoiceItem {
                item_id: uuid::Uuid::new_v4(),
                description: String::from("test item 1"),
                quantity: Decimal::new(15, 1),
                uom_id: 1,
                unit_price: Decimal::new(100, 2),
                total: Decimal::new(100, 2),
                currency_id: 1,
            },
            InvoiceItem {
                item_id: uuid::Uuid::new_v4(),
                description: String::from("test item 2"),
                quantity: Decimal::new(25, 1),
                uom_id: 1,
                unit_price: Decimal::new(200, 2),
                total: Decimal::new(400, 2),
                currency_id: 1,
            },
        ];
        if let Err(e) = ipp.invoice_items_save(&invoice_id, &invoice_items).await {
            error!(e);
            assert!(false, "unable to save invoice items");
        }
    }
}
