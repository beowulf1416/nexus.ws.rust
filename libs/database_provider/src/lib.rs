use tracing::{
    info,
    error
};
use std::collections::HashMap;

use sqlx::{
    Pool,
    Database,
    Any,
    postgres::PgPoolOptions
};


use config::Config;




#[derive(Debug, Clone)]
pub enum DatabaseType {
    Postgres(Pool<sqlx::Postgres>)
}

#[derive(Debug, Clone)]
pub struct DatabaseProvider {
    pools: HashMap<String, DatabaseType>
}


impl DatabaseProvider {
    pub fn new(
        config: &Config
    ) -> Self {
        let mut pools: HashMap<String, DatabaseType> = HashMap::new();
        for (k, v) in config.connections().iter() {
            match v {
                s if s.starts_with("postgres") => {
                    info!("Creating Postgres pool for connection '{}'", k);
                    let pool = PgPoolOptions::new()
                        .max_connections(5)
                        .connect_lazy(s)
                        .unwrap()
                    ;
                    pools.insert(k.clone(), DatabaseType::Postgres(pool));
                }
                _ => {
                    error!("Unsupported database type for connection '{}'", k);
                }   
            }
        }

        return Self {
            pools
        };
    }

    pub fn get_pool(
        &self,
        name: &str
    ) -> Option<DatabaseType> {
        return self.pools.get(name).cloned();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
