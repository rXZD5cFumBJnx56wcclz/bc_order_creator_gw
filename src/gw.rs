use bc_utils_lg::structs::settings::{SETTINGS_ORDER_CREATOR, SETTINGS_TRADE};
use bc_utils_lg::{
    structs::{
        settings::{SETTINGS_ORDER_CREATORS, SETTINGS_TRIGGER_OUT_OF_STORAGE},
        signals::Signal,
        trade::{Order, Trigger},
    },
    types::maps::MAP,
};
// use uuid::Uuid;

fn create_trigger(
    s: &SETTINGS_TRIGGER_OUT_OF_STORAGE,
    ind: &MAP<&str, f64>,
    utils: &MAP<&str, f64>,
) -> Option<Trigger> {
    Some(Trigger {
        price: ind[s.used_ind.as_str()],
        trigger_by: s.trigger_by.clone(),
        direction: utils[s.used_util_state.as_str()] as usize,
    })
}

pub fn create_order(
    symbol: &str,
    signals: &MAP<&str, Signal>,
    ind: &MAP<&str, f64>,
    utils: &MAP<&str, f64>,
    s_order_creator: &SETTINGS_ORDER_CREATOR,
    s_trade: &SETTINGS_TRADE,
) -> (Order, bool, Option<Trigger>) {
    let signal = signals[s_order_creator.used_signal.as_str()];
    let price = ind
        .get(
            s_order_creator
                .used_ind
                .as_ref()
                .unwrap_or(&"".to_string())
                .as_str(),
        )
        .copied();
    let qty = utils[s_order_creator.used_util_state.as_str()];
    let position_idx = if signal.signal == s_trade.signal_long {
        1
    } else {
        2
    };
    (
        Order::new(
            symbol.to_string(),
            if signal.signal == s_trade.signal_long {
                "buy".to_string()
            } else if signal.signal == s_trade.signal_short {
                "sell".to_string()
            } else {
                "hold".to_string()
            },
            qty,
            qty * if s_order_creator.type_ == "market" {
                s_trade.commission_market
            } else {
                s_trade.commission_limit
            },
            s_order_creator.leverage,
            price,
            s_order_creator.type_.clone(),
            s_order_creator.is_reduce,
            s_order_creator.type_price_cross.to_string(),
            // Uuid::new_v4().to_string(),
            String::default(),
            position_idx,
            true,
        ),
        s_order_creator.include_in_storage,
        if s_order_creator.include_in_storage {
            create_trigger(s_order_creator.trigger.as_ref().unwrap(), ind, utils)
        } else {
            None
        },
    )
}

pub fn series<'a>(
    s: &'a SETTINGS_ORDER_CREATORS,
    s_trade: &SETTINGS_TRADE,
    symbol: &str,
    signals: &MAP<&str, Signal>,
    indications: &MAP<&str, f64>,
    res_utils_state: &MAP<&str, f64>,
) -> MAP<&'a str, (Order, bool, Option<Trigger>)> {
    s.iter()
        .map(|(k, setting)| {
            (
                k.as_str(),
                create_order(
                    symbol,
                    signals,
                    indications,
                    res_utils_state,
                    setting,
                    s_trade,
                ),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bc_test_kit::prelude::*;

    #[test]
    fn series_res_1() {
        assert_eq_pr!(
            &series(
                &ORDER_CREATORS,
                &TRADE,
                "",
                &SIGNALS_STATE,
                &INDICATIONS_STATE,
                &UTILS_STATE_STATE,
            ),
            &*ORDER_CREATOR_STATE
        )
    }
}
