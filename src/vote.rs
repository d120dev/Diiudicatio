/*
 * Copyright (c) 2024 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use actix_web::{
    get, post, put,
    web::{Path, ServiceConfig},
    HttpResponse, Responder,
};
use chrono::{DateTime, Utc};
use log::info;
use uuid::Uuid;

struct Vote {
    uuid: Uuid,
    scope_id: u32,
    short_id: Option<String>,
    name: String,
    text: String,
    vote_type: VoteTypes,
    required_majority: Option<Majorities>,
    vote_state: VoteStates,
    vote_access: VoteAccess,
    remainder_time: DateTime<Utc>,
    remainder_open: DateTime<Utc>,
    remainder_close: DateTime<Utc>,
    remainder_archive: DateTime<Utc>,
    remainder_redact: DateTime<Utc>,
}

enum VoteTypes {
    YesNo,
    Archiving,
    Redacting,
    Options,
    Alternative,
}

enum Majorities {
    Relative,
    Simple,
    SimpleQualified { numerator: u16, denominator: u16 },
    AbsoluteQualified { numerator: u16, denominator: u16 },
}

enum VoteStates {
    Created,
    Open,
    Accepted,
    Rejected,
    Archived,
    Redacted,
    Deleted,
}

enum VoteAccess {
    Open,
    Protected(String),
    Closed,
}

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
