use axum::{handler::post, Router, Json, AddExtensionLayer, extract::Extension};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tch::nn::ModuleT;
use tch::vision::{resnet, imagenet};

extern crate tch;
extern crate base64;
extern crate image;

struct DnnModel {
    net: Mutex<Box<dyn ModuleT>>
}

#[tokio::main]
async fn main() {
    let weights = std::path::Path::new("/resnet18.ot"); 
    let mut vs = tch::nn::VarStore::new(tch::Device::Cpu);
    let net:Mutex<Box<(dyn ModuleT + 'static)>> = Mutex::new(Box::new(resnet::resnet18(&vs.root(), imagenet::CLASS_COUNT)));
    let _ = vs.load(weights);
    let state = Arc::new(DnnModel { net });

    let app = Router::new()
        .route("/", post(proc))
        .layer(AddExtensionLayer::new(state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct RequestJson {
    img: String,
}

#[derive(Serialize)]
struct ResponseJson {
    result: Vec<String>,
}

async fn proc(Json(payload): Json<RequestJson>, Extension(state): Extension<Arc<DnnModel>>) -> Json<Value> {
    let net = state.net.lock().await;

    let img_buffer = base64: