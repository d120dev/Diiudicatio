/*
 * Copyright (c) 2024 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use actix_web::{
    get, post, put,
    web::{Path, ServiceConfig},
    HttpResponse, Responder,
};
use log::info;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(show_current)
        .service(create)
        .service(index)
        .service(open)
        .service(close)
        .service(show)
        .service(submit);
}

#[get("")]
async fn show_current(scope: Path<String>) -> impl Responder {
    info!("{}", scope);
    HttpResponse::Ok()
}

#[post("/create")]
async fn create(scope: Path<String>) -> impl Responder {
    info!("{}", scope);
    HttpResponse::Ok()
}

#[get("/index")]
async fn index(scope: Path<String>) -> impl Responder {
    info!("{}", scope);
    HttpResponse::Ok()
}

#[put("/open/{id}")]
async fn open(path: Path<(String, u16)>) -> impl Responder {
    let (scope, id) = path.into_inner();
    info!("{scope} {id}");
    HttpResponse::Ok()
}

#[put("/close/{id}")]
async fn close(path: Path<(String, u16)>) -> impl Responder {
    let (scope, id) = path.into_inner();
    info!("{scope} {id}");
    HttpResponse::Ok()
}

#[get("/{id}")]
async fn show(path: Path<(String, u16)>) -> impl Responder {
    let (scope, id) = path.into_inner();
    info!("{scope} {id}");
    HttpResponse::Ok()
}

#[post("/{id}")]
async fn submit(path: Path<(String, u16)>) -> impl Responder {
    let (scope, id) = path.into_inner();
    info!("{scope} {id}");
    HttpResponse::Ok()
}
