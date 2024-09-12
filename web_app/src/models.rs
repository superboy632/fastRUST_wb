use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Model {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: i32,
    date_created: String,
    oof_shard: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    provider: String,
    amount: i32,
    payment_dt: i64,
    bank: String,
    delivery_cost: i32,
    goods_total: i32,
    custom_fee: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    chrt_id: i32,
    track_number: String,
    price: i32,
    rid: String,
    name: String,
    sale: i32,
    size: String,
    total_price: i32,
    nm_id: i32,
    brand: String,
    status: i32,
}