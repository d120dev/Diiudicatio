/*
 * Copyright (c) 2024 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use diiudicatio::diiudicatio_run;

#[actix_web::main]
async fn main() {
    diiudicatio_run().await
}
