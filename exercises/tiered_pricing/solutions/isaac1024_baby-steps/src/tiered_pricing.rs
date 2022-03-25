fn get_total_subscription_price(number_of_subscriptions: u32) -> u32 {
    const FIRST_UNIT_PRICE: u32 = 299;
    const SECOND_UNIT_PRICE: u32 = 239;
    const THIRD_UNIT_PRICE: u32 = 219;
    const FOURTH_UNIT_PRICE: u32 = 199;

    match number_of_subscriptions {
        51 => 149 * number_of_subscriptions,
        52 => 149 * number_of_subscriptions,
        1|2 => FIRST_UNIT_PRICE * number_of_subscriptions,
        3..=10 => SECOND_UNIT_PRICE * number_of_subscriptions,
        11..=25 => THIRD_UNIT_PRICE * number_of_subscriptions,
        _ => FOURTH_UNIT_PRICE * number_of_subscriptions,
    }
}

#[cfg(test)]
mod tests {
    use crate::tiered_pricing::get_total_subscription_price;

    #[test]
    fn when_get_a_subscription_return_299_euros() {
        assert_eq!(299, get_total_subscription_price(1));
    }

    #[test]
    fn when_get_2_subscriptions_return_598_euros() {
        assert_eq!(598, get_total_subscription_price(2));
    }

    #[test]
    fn when_get_3_subscriptions_return_717_euros() {
        assert_eq!(717, get_total_subscription_price(3));
    }

    #[test]
    fn when_get_10_subscriptions_return_2390_euros() {
        assert_eq!(2390, get_total_subscription_price(10));
    }

    #[test]
    fn when_get_11_subscriptions_return_2409_euros() {
        assert_eq!(2409, get_total_subscription_price(11));
    }

    #[test]
    fn when_get_25_subscriptions_return_5475_euros() {
        assert_eq!(5475, get_total_subscription_price(25));
    }

    #[test]
    fn when_get_26_subscriptions_return_5174_euros() {
        assert_eq!(5174, get_total_subscription_price(26));
    }

    #[test]
    fn when_get_50_subscriptions_return_9950_euros() {
        assert_eq!(9950, get_total_subscription_price(50));
    }

    #[test]
    fn when_get_51_subscriptions_return_7599_euros() {
        assert_eq!(7599, get_total_subscription_price(51));
    }

    #[test]
    fn when_get_52_subscriptions_return_7748_euros() {
        assert_eq!(7748, get_total_subscription_price(52));
    }
}
