/*
 * Copyright (c) 2024 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use actix_web::{
    get, post, put,
    web::{Json, Path, ServiceConfig},
    HttpResponse, Responder,
};
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
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
    #[serde(with = "ts_seconds_option")]
    remainder_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    remainder_open: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    remainder_close: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    remainder_archive: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    remainder_redact: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
enum VoteTypes {
    YesNo,
    Archiving,
    Redacting,
    Options,
    Alternative,
}

#[derive(Serialize)]
enum Majorities {
    Relative,
    Simple,
    SimpleQualified { numerator: u16, denominator: u16 },
    AbsoluteQualified { numerator: u16, denominator: u16 },
}

#[derive(Serialize)]
enum VoteStates {
    Created,
    Open,
    Accepted,
    Rejected,
    Archived,
    Redacted,
    Deleted,
}

#[derive(Serialize)]
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
    Json(Vote {
        name: String::from("Hello"),
        uuid: Uuid::nil(),
        scope_id: 0,
        short_id: None,
        text: String::from("Very important Beschluss"),
        vote_type: VoteTypes::YesNo,
        required_majority: None,
        vote_state: VoteStates::Open,
        vote_access: VoteAccess::Open,
        remainder_time: Some(DateTime::default()),
        remainder_open: Some(DateTime::default()),
        remainder_close: Some(DateTime::default()),
        remainder_archive: Some(DateTime::default()),
        remainder_redact: Some(DateTime::default()),
    })
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
