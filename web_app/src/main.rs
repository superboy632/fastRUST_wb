use axum::{
    response::IntoResponse,
    routing::{get},
    Router,
};

mod models;


use std::fs;
use models::Model;
use serde::{Deserialize, Serialize};

async fn get_model() -> impl IntoResponse {
    let res: String = fs::read_to_string("model.json").unwrap();
    let test: Model = serde_json::from_str(&res).expect("Failed to parse JSON");

    // Возвращаем данные в формате JSON
    axum::Json(test)
}

#[tokio::main]
async fn main() {
    // let mut data = DashMap::new();
    // data.insert(1, res);
    // let Some(value) = data.get(&1);
    /*
    создаем новый экземпляр маршрутизатора, который обрабатывает входящие http запросы
    метод route добавляет маршрут к маршрутизатору
    get указывает что для get запроса к этому маршруту будет использоваться get_model в качестве обработчика
    это значит, что как только мы заходим на главную страницу сервиса, будет сделан get запрос и обработчик этого запроса
     */
    let app = Router::new().route("/", get(get_model));

    // задаем адрес сервера 0.0.0.0 с портом 3000
    let addr = "0.0.0.0:3000";
    println!("Запуск сервера на http://{}", addr);


    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}