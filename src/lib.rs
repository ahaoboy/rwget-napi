#![deny(clippy::all)]
use clap::Parser;
use napi_derive::napi;
use rwget::{rwget, Args};

#[tokio::main]
pub async fn start(v: Vec<String>) {
  let args = Args::parse_from(v);
  rwget(args).await;
}

#[napi]
pub fn rwget_start(v: Vec<String>) {
  start(v);
}
