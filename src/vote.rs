/*
 * Copyright (c) 2024 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use actix_web::{
    post,
    web::{Path, ServiceConfig},
    HttpResponse, Responder,
};
use log::info;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(create);
}

#[post("/create")]
async fn create(scope: Path<String>) -> impl Responder {
    info!("{}", scope);
    HttpResponse::Ok()
}
