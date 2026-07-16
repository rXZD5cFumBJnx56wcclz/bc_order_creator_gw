use bc_utils_lg::structs::signals::Signal;
use bc_utils_lg::structs::trade::{Order, StateValues, Trigger};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct OrderCreator {
    pub signal_long: f64,
    pub signal_short: f64,
    pub leverage: f64,
}

impl Default for OrderCreator {
    fn default() -> Self {
        Self { signal_long: 1., signal_short: -1., leverage: 1. }
    }
}

impl OrderCreator {
    pub fn create_order(
        &self,
        symbol: &str,
        type_: &str,
        signal: &Signal,
        qty: f64,
        is_reduce: bool,
        include_in_storage: bool,
        price: Option<f64>,
        trigger: Option<Trigger>,
        state_values: Option<StateValues>,
    ) -> (Order, bool, Option<Trigger>) {
        (
            Order::new(
                symbol.to_string(),
                if signal.signal == self.signal_long {
                    "buy".to_string()
                } else if signal.signal == self.signal_short {
                    "sell".to_string()
                } else {
                    "hold".to_string()
                },
                qty,
                self.leverage,
                price,
                type_.to_string(),
                is_reduce,
                Uuid::new_v4().to_string(),
                if signal.signal == self.signal_long {
                    1
                } else {
                    2
                },
                true,
                state_values,
            ),
            include_in_storage,
            trigger,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[test]
    fn create_order_res_1() {
        assert_eq_pr!(
            {
                let mut bind = OrderCreator::default()
                    .create_order(
                        "",
                        "limit",
                        &Signal::new(1., 1.),
                        1.,
                        false,
                        false,
                        Some(1.9),
                        None,
                        None,
                    )
                    .0;
                bind.order_link_id = "".to_string();
                bind
            },
            Order::new(
                "".to_string(),
                "buy".to_string(),
                1.,
                1.,
                Some(1.9),
                "limit".to_string(),
                false,
                "".to_string(),
                1,
                true,
                None
            )
        );
    }
}
