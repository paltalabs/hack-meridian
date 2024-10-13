use crate::models::PaymentPeriod;

pub fn calculate_periods_since(
    start_timestamp: u64,
    current_timestamp: u64,
    payment_period: PaymentPeriod,
) -> u64 {
    let period_seconds = match payment_period {
        PaymentPeriod::Weekly => 7 * 24 * 60 * 60,
        PaymentPeriod::Monthly => 30 * 24 * 60 * 60,
        PaymentPeriod::Annually => 365 * 24 * 60 * 60,
        // EverySecond
        PaymentPeriod::EverySecond => 1,
    };

    let elapsed_seconds = current_timestamp - start_timestamp;
    elapsed_seconds / period_seconds
}

pub fn calculate_periods_amounts_in_seconds(
    periods: u64,
    payment_period: PaymentPeriod,
) -> u64 {
    let period_seconds = match payment_period {
        PaymentPeriod::Weekly => 7 * 24 * 60 * 60,
        PaymentPeriod::Monthly => 30 * 24 * 60 * 60,
        PaymentPeriod::Annually => 365 * 24 * 60 * 60,
        PaymentPeriod::EverySecond => 1,
    };

    periods * period_seconds
}