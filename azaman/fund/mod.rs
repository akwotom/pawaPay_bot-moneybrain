/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This file is the entry point for the azaman fund module.
    The module is concerned with taking money from the user.
*/

mod collect_money;
pub use collect_money::*;
mod db;
mod poll_fund;

pub use poll_fund::*;

pub use db::find_by_service_id;

mod on_callback_received;

pub use on_callback_received::on_callback_received;
