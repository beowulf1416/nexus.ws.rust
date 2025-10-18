
use std::collections::HashMap;

use sqlx::{
    Pool,
    Database,
    Any
};


pub struct DatabaseProvider {
    pools: HashMap<String, Pool<Any>>
}


impl DatabaseProvider {
    pub fn new(
    ) -> Self {
        return Self {
            pools: HashMap::new()
        };
    }

    pub fn add_pool(
        &mut self,
        name: String,
        pool: Pool<Any>
    ) {
        self.pools.insert(name, pool);
    }

    pub fn get_pool(
        &self,
        name: &str
    ) -> Option<&Pool<Any>> {
        return self.pools.get(name);
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
