use bc_utils_lg::{
    structs::{
        settings::SETTINGS_ORDER_CREATORS,
        signals::Signal,
        trade::{Order, StateValues, Trigger},
    },
    types::maps::MAP,
};

use crate::order_creator::OrderCreator;

pub struct OrderCreatorsGateway<'a> {
    pub order_creator: *const MAP<&'a str, OrderCreator>,
    s: &'a SETTINGS_ORDER_CREATORS,
}

impl<'a> OrderCreatorsGateway<'a> {
    pub fn new(
        order_creator: *const MAP<&'a str, OrderCreator>,
        s: &'a SETTINGS_ORDER_CREATORS,
    ) -> Self {
        Self { s, order_creator }
    }
}

pub fn get_map(s: &SETTINGS_ORDER_CREATORS) -> MAP<&str, OrderCreator> {
    s.iter()
        .map(|(k, v)| {
            (
                k.as_str(),
                OrderCreator {
                    signal_long: v.signal_long,
                    signal_short: v.signal_short,
                    leverage: v.leverage,
                },
            )
        })
        .collect()
}

impl OrderCreatorsGateway<'_> {
    pub fn series<'a>(
        &'a self,
        symbol: &str,
        signals: &'a MAP<&'a str, Signal>,
        indications: &'a MAP<&'a str, f64>,
        res_utils_state: &'a MAP<&'a str, f64>,
    ) -> MAP<&'a str, (Order, bool, Option<Trigger>)> {
        self.s
            .iter()
            .map(|(k, setting)| {
                let qty = res_utils_state[setting.used_util_state.as_str()];
                (
                    k.as_str(),
                    unsafe { &*self.order_creator }[k.as_str()].create_order(
                        symbol,
                        &setting.type_,
                        &signals[setting.used_signal.as_str()],
                        qty,
                        qty * setting.commission,
                        setting.is_reduce,
                        setting.include_in_storage,
                        &setting.type_price_cross,
                        if setting.used_ind.is_some() {
                            Some(indications[setting.used_ind.as_ref().unwrap().as_str()])
                        } else {
                            None
                        },
                        if setting.trigger.is_some() {
                            Some({
                                let setting_trigger = setting.trigger.as_ref().unwrap();
                                Trigger {
                                    price: indications[setting_trigger.used_ind.as_str()],
                                    trigger_by: setting_trigger.trigger_by.to_string(),
                                    direction: res_utils_state
                                        [setting_trigger.used_util_state.as_str()]
                                        as usize,
                                }
                            })
                        } else {
                            None
                        },
                        if setting.state_values.is_some() {
                            let stat_values = setting.state_values.as_ref().unwrap();
                            Some(StateValues {
                                qty_percent_of_position: stat_values.qty_percent_of_position,
                            })
                        } else {
                            None
                        },
                    ),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;

    use bc_utils_lg::structs::settings::SETTINGS_ORDER_CREATOR;
    use pretty_assertions::assert_eq as assert_eq_pr;

    static S: LazyLock<SETTINGS_ORDER_CREATORS> = LazyLock::new(|| {
        SETTINGS_ORDER_CREATORS::from_iter([(
            "order_creator_1".to_string(),
            SETTINGS_ORDER_CREATOR {
                used_signal: "signal_1".to_string(),
                used_util_state: "qty_1".to_string(),
                ..Default::default()
            },
        )])
    });

    #[test]
    fn series_res_1() {
        let map = get_map(&S);
        let bind1 = MAP::from_iter([("signal_1", Signal::new(1., 1.))]);
        let bind2 = Default::default();
        let bind3 = MAP::from_iter([("qty_1", 1.)]);
        let bind4 = OrderCreatorsGateway::new(&map, &S);
        assert_eq_pr!(
            {
                let mut bind = bind4.series("", &bind1, &bind2, &bind3);
                bind.get_mut("order_creator_1").unwrap().0.order_link_id = "".to_string();
                bind
            },
            MAP::from_iter([(
                "order_creator_1",
                (
                    Order {
                        qty: 1.,
                        side: "buy".to_string(),
                        leverage: 1.,
                        position_idx: 1,
                        is_active: true,
                        type_price_cross: "last".to_string(),
                        commission: 0.001,
                        ..Default::default()
                    },
                    Default::default(),
                    Default::default()
                )
            )])
        );
    }
}
