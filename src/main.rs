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
    let net:Mutex<Box<(dyn ModuleT + 'static)>> = Mutex::new(Box::new(r