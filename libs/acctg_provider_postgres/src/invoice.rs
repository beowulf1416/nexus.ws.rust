#![allow(clippy::needless_return)]

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{
    Decode, Encode, Postgres, Row, Type,
    postgres::{PgHasArrayType, PgRow, types::PgMoney},
    prelude::FromRow,
};
use tracing::{debug, error, info};

use acctg_provider::invoice::{Invoice, InvoiceItem, InvoiceProvider, InvoiceType};

// #[derive(Debug, Serialize, Deserialize, Type)]
// #[sqlx(type_name = "acctg.invoice_item_type")]
// struct InvoiceItemDerived(pub InvoiceItem);

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "acctg.invoice_item_type")]
struct InvoiceItemDerived {
    pub invoice_item_id: uuid::Uuid,
    pub version: i32,
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub currency_id: i32,
}

impl<'r> FromRow<'r, PgRow> for InvoiceItemDerived {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        // return Ok(Self(InvoiceItem {
        //     invoice_item_id: row.get("invoice_item_id"),
        //     version: row.get("version"),
        //     description: row.get("description"),
        //     quantity: row.get("quantity"),
        //     unit_price: row.get("unit_price"),
        //     currency_id: row.get("currency_id"),
        // }));
        return Ok(Self {
            invoice_item_id: row.get("invoice_item_id"),
            version: row.get("version"),
            description: row.get("description"),
            quantity: row.get("quantity"),
            unit_price: row.get("unit_price"),
            currency_id: row.get("currency_id"),
        });
    }
}

// impl From<InvoiceItemDerived> for InvoiceItem {
//     fn from(item: InvoiceItemDerived) -> Self {
//         item.0
//     }
// }

struct InvoiceTypeData(pub InvoiceType);

impl<'r> FromRow<'r, PgRow> for InvoiceTypeData {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        return Ok(Self(InvoiceType {
            id: row.get("invoice_type_id"),
            name: row.get("name"),
        }));
    }
}

struct InvoiceData(pub Invoice);

impl<'r> FromRow<'r, PgRow> for InvoiceData {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        debug!("row: {:?}", row);

        // let items_derived: Vec<InvoiceItemDerived> = row.get("items");
        // let items = items_derived
        //     .iter()
        //     .map(|r| InvoiceItem {
        //         item_id: r.0.item_id,
        //         version: r.0.version,
        //         description: r.0.description,
        //         quantity: r.0.quantity,
        //         unit_price: r.0.unit_price,
        //         currency_id: r.0.currency_id,
        //     })
        //     .collect::<Vec<InvoiceItem>>();

        // let items = row
        //     .get::<VecInvoiceItemDerived>("items")
        //     .iter()
        //     .map(|item| InvoiceItem {
        //         item_id: item.0.item_id,
        //         version: item.0.version,
        //         description: item.0.description,
        //         quantity: item.0.quantity,
        //         unit_price: item.0.unit_price,
        //         currency_id: item.0.currency_id,
        //     })
        //     .collect::<Vec<InvoiceItem>>();

        return Ok(Self(Invoice {
            invoice_id: row.get("invoice_id"),
            invoice_type_id: row.get("invoice_type_id"),
            invoice_id_seq: row.get("invoice_id_seq"),
            account_id: row.get("account_id"),
            org_id: row.get("org_id"),
            partner_id: row.get("partner_id"),
            active: row.get("active"),
            version: row.get("version"),
            created: row.get("created_ts"),
            updated: row.get("updated_ts"),
            due_date: row.get("due_date_ts"),
            description: row.get("description"),
            items: Vec::new(),
        }));
    }
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
            match sqlx::query_as::<_, InvoiceTypeData>("select * from acctg.invoice_types_fetch();")
                .fetch_all(&pool)
                .await
            {
                Err(e) => {
                    error!("Error fetching invoice types: {:?}", e);
                    return Err("Error fetching invoice types");
                }
                Ok(rows) => {
                    let types: Vec<InvoiceType> = rows.iter().map(|r| r.0.clone()).collect();
                    return Ok(types);
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
            match sqlx::query_as::<_, InvoiceData>("select * from acctg.invoices_fetch($1,$2);")
                .bind(tenant_id)
                .bind(filter)
                .fetch_all(&pool)
                .await
            {
                Ok(rows) => {
                    let invoices: Vec<Invoice> = rows.iter().map(|r| r.0.clone()).collect();

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
            match sqlx::query_as::<_, InvoiceData>("select * from acctg.invoice_fetch($1);")
                .bind(invoice_id)
                .fetch_one(&pool)
                .await
            {
                Err(e) => {
                    error!("Error fetching invoice: {:?}", e);
                    return Err("Error fetching invoice");
                }
                Ok(row) => {
                    let invoice_items = match sqlx::query_as::<_, InvoiceItemDerived>(
                        "select * from acctg.invoice_items_fetch($1);",
                    )
                    .bind(invoice_id)
                    .fetch_all(&pool)
                    .await
                    {
                        Ok(rows) => {
                            let items = rows
                                .iter()
                                .map(|r| InvoiceItem {
                                    invoice_item_id: r.invoice_item_id,
                                    version: r.version,
                                    description: r.description.clone(),
                                    quantity: r.quantity,
                                    unit_price: r.unit_price,
                                    currency_id: r.currency_id,
                                })
                                .collect::<Vec<InvoiceItem>>();
                            items
                        }
                        Err(e) => {
                            error!("Error fetching invoice: {:?}", e);
                            // return Err("Error fetching invoice");
                            Vec::new()
                        }
                    };

                    return Ok(Invoice {
                        invoice_id: row.0.invoice_id.clone(),
                        invoice_type_id: row.0.invoice_type_id,
                        invoice_id_seq: row.0.invoice_id_seq,
                        account_id: row.0.account_id.clone(),
                        org_id: row.0.org_id.clone(),
                        partner_id: row.0.partner_id.clone(),
                        active: row.0.active,
                        version: row.0.version,
                        created: row.0.created,
                        updated: row.0.updated,
                        due_date: row.0.due_date,
                        description: row.0.description.clone(),
                        items: invoice_items,
                    });
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
            let derived_items = invoice
                .items
                .iter()
                .map(|item| InvoiceItemDerived {
                    invoice_item_id: item.invoice_item_id,
                    version: item.version,
                    description: item.description.clone(),
                    quantity: item.quantity,
                    unit_price: item.unit_price,
                    currency_id: item.currency_id,
                })
                .collect::<Vec<InvoiceItemDerived>>();

            match sqlx::query("call acctg.invoice_save($1,$2,$3,$4,$5,$6,$7,$8,$9);")
                .bind(tenant_id)
                .bind(&invoice.invoice_id)
                .bind(&invoice.invoice_type_id)
                .bind(&invoice.account_id)
                .bind(&invoice.org_id)
                .bind(&invoice.partner_id)
                .bind(&invoice.description)
                .bind(&invoice.due_date)
                // .bind(&derived_items)
                .bind(&invoice.version)
                .execute(&pool)
                .await
            {
                Ok(_) => {
                    match sqlx::query("call acctg.invoice_items_save($1,$2);")
                        .bind(&invoice.invoice_id)
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

    // async fn invoice_items_save(
    //     &self,
    //     invoice_id: &uuid::Uuid,
    //     items: &Vec<InvoiceItem>,
    // ) -> Result<(), &'static str> {
    //     info!("invoice_items_save");

    //     if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
    //         let derived_items = items
    //             .iter()
    //             .map(|item| InvoiceItemDerived {
    //                 item_id: item.item_id,
    //                 description: item.description.clone(),
    //                 quantity: item.quantity,
    //                 // uom_id: item.uom_id,
    //                 unit_price: item.unit_price,
    //                 // total: item.total,
    //                 currency_id: item.currency_id,
    //             })
    //             .collect::<Vec<InvoiceItemDerived>>();

    //         match sqlx::query("call acctg.invoice_items_save($1,$2);")
    //             .bind(&invoice_id)
    //             .bind(&derived_items)
    //             .execute(&pool)
    //             .await
    //         {
    //             Ok(_) => {
    //                 return Ok(());
    //             }
    //             Err(e) => {
    //                 error!("Error saving invoice items: {:?}", e);
    //                 return Err("Error saving invoice items");
    //             }
    //         }
    //     } else {
    //         error!("No Postgres pool found for 'main'");
    //         return Err("Unable to get pool for 'main'");
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    use acctg_provider::{accounts::AccountsProvider, invoice::InvoiceProvider};
    use crm_provider::CrmProvider;
    use tenants_provider::{
        TenantsProvider,
        // organizations::OrganizationsProvider
    };

    #[actix_web::test]
    async fn test_invoice() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
        // let opp = tenants_provider_postgres::organizations::OrganizationsProviderPostgres::new(&dp);

        let cpp = crm_provider_postgres::PostgresCrmProvider::new(&dp);

        let app = crate::accounts::AccountsProviderPostgres::new(&dp);
        let ipp = InvoiceProviderPostgres::new(&dp);

        if let Err(e) = ipp.invoice_types_fetch().await {
            error!(e);
            assert!(false, "unable to fetch invoice types");
        }

        let tenant = tp.tenant_fetch_by_name("tenant_01").await.unwrap();
        let tenant_id = tenant.tenant_id();

        let invoice_id = uuid::Uuid::new_v4();

        let account_id = app
            .account_fetch_by_name(&tenant_id, "asset")
            .await
            .unwrap()
            .account_id;

        let org_id = tenant_id.clone();

        let partner_id = cpp.partners_fetch(&tenant_id, "%").await.unwrap()[0].partner_id;

        let today = chrono::Local::now();
        let due_date = today.checked_add_days(chrono::Days::new(3)).unwrap();

        let invoice = Invoice {
            invoice_id: invoice_id,
            invoice_type_id: 1,
            account_id: account_id,
            org_id: org_id,
            partner_id: partner_id,

            due_date: Some(due_date.to_utc()),
            description: String::from("test invoice 1"),

            invoice_id_seq: 0,
            created: today.to_utc(),
            updated: today.to_utc(),
            active: true,
            version: 0,
            items: vec![
                InvoiceItem {
                    invoice_item_id: uuid::Uuid::new_v4(),
                    description: String::from("test item 1"),
                    quantity: Decimal::new(15, 1),
                    // uom_id: 1,
                    unit_price: Decimal::new(100, 2),
                    // total: Decimal::new(100, 2),
                    currency_id: 1,
                    version: 0,
                },
                InvoiceItem {
                    invoice_item_id: uuid::Uuid::new_v4(),
                    description: String::from("test item 2"),
                    quantity: Decimal::new(25, 1),
                    // uom_id: 1,
                    unit_price: Decimal::new(200, 2),
                    // total: Decimal::new(400, 2),
                    currency_id: 1,
                    version: 0,
                },
            ],
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
    }
}
