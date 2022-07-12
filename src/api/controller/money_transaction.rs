use crate::api::controller::request::money::Transaction;

pub fn test() {
    let _a = Transaction{
        amount: 100,
        destination: "japan".to_string(),
        source: "america".to_string(),
        nominal: "CIA".to_string(),
        description: "aaa".to_string(),
    };
    println!("aaa");
}