use tracing::{
    info,
    debug,
    error
};




pub struct Mailer {

}


impl Mailer {

    pub fn new() -> Self {
        return Self {

        };
    }


    pub fn send(&self, msg: String) -> Result<(), &'static str> {
        info!("Sending mail: {}", msg);

        debug!("Message: {}", msg);

        return Ok(());
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
