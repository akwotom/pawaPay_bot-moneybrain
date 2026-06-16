/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module is focused on the logic of checking to see if a cheque has already been paid for.
*/

use crate::{
    azaman,
    cheque::{Cheque, db},
};

/// This method finds the full data of the cheque, and refreshes its payment if necessary.
pub async fn find_and_poll(id: String) -> Option<Cheque> {
    let cheque = db::find_by_id(&id).await;
    let cheque = match cheque {
        Option::Some(v) => v,
        Option::None => return Option::None,
    };

    let cheque_copy = cheque.clone();

    if !cheque.funded {
        // If the cheque is not funded, let's query the payment provider to see its status.

        let mut nw_cheque = cheque_copy.clone();

        tokio::spawn(async move {
            let txn = azaman::fund::find_by_service_id(&nw_cheque.id).await;

            let txn = match txn {
                Option::None => {
                    eprintln!(
                        "Could not refresh cheque with id {}, because no collect transaction was found for it.\n",
                        nw_cheque.id
                    );
                    return;
                }
                Option::Some(v) => v,
            };

            let txn = azaman::fund::refresh_with_provider(txn.id).await;

            let txn = match txn {
                Result::Err(e) => {
                    eprintln!(
                        "Could not refresh the funding transaction status of a cheque\n{}\n",
                        e
                    );
                    return;
                }
                Result::Ok(v) => v,
            };

            let txn = match txn {
                Option::None => return,
                Option::Some(v) => v,
            };

            if txn.status >= 30 {
                nw_cheque.funded = true;
                db::update(&nw_cheque).await;
                // The next time the cheque is fetched, we'll just return this copy, without the need to refresh.
                nw_cheque.on_status_change();
            }
        });
    } else {
        if !cheque.liquidated {
            // Now, let's use this chance to catch dead locks at the liquidation side.

            let cheque_id = cheque.id.clone();

            let mut nw_cheque = cheque.clone();

            tokio::spawn(async move {
                let payout = &match azaman::payout::find_by_service_id(&cheque_id).await {
                    Option::None => return,
                    Option::Some(v) => v,
                };

                azaman::payout::refresh_with_provider(payout.id.clone()).await.map_err(|e| format!("Could not refresh the payout {payout:?}\n\nIt is associated to cheque with id {cheque_id}\nError:\n{e}\n")).unwrap();

                if payout.status >= 30 {
                    nw_cheque.liquidated = true;
                    db::update(&nw_cheque).await;
                    nw_cheque.on_status_change();
                }
            });
        }
    }

    return Option::Some(cheque);
}
