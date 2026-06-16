/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains logic for managing cheques.
    A cheque is what is created by a user, in order that money be sent to another user.
    After a cheque is created, it is paid for. This is called funding the cheque.
    After that, the recipient can collect the money. This is called liquidating.
*/

mod fund_new_cheque;
mod models;
pub use fund_new_cheque::*;
pub use models::*;
mod begin_funding;
mod db;

mod find_and_poll;

pub use find_and_poll::*;
mod on_payment_complete;
pub use on_payment_complete::on_payment_complete;
mod liquidate_cheque;
pub use liquidate_cheque::liquidate_cheque_by_id;
