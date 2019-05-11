mod message;
pub use message::*;

mod connection;
pub use connection::*;

mod variant;
pub use variant::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let connection = crate::Connection::new_session()
            .map_err(|e| {
                println!("error: {}", e);

                e
            })
            .unwrap();
    }
}
