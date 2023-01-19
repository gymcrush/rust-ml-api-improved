use axum::{handler::post, Router, Json, AddExtensionLayer, extract::Extension};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};