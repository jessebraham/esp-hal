//! Async Delay Test
//!
//! Specifically tests the various implementations of the
//! `embedded_hal_async::delay::DelayNs` trait.

//% CHIPS: esp32 esp32c2 esp32c3 esp32c6 esp32s2 esp32s3

#![no_std]
#![no_main]

use embedded_hal_async::delay::DelayNs;
use esp_hal::{
    peripherals::Peripherals,
    timer::{
        systimer::{Alarm, FrozenUnit, SystemTimer},
        timg::TimerGroup,
    },
};
use hil_test as _;

struct Context {
    peripherals: Peripherals,
}

async fn test_async_delay_ns(mut timer: impl DelayNs, duration: u32) {
    let t1 = esp_hal::time::current_time();
    timer.delay_ns(duration).await;
    let t2 = esp_hal::time::current_time();

    assert!(t2 > t1);
    assert!(
        (t2 - t1).to_nanos() >= duration as u64,
        "diff: {:?}",
        (t2 - t1).to_nanos()
    );
}

#[cfg(test)]
#[embedded_test::tests(executor = esp_hal_embassy::Executor::new())]
mod tests {
    use super::*;

    #[init]
    fn init() -> Context {
        Context {
            peripherals: esp_hal::init(esp_hal::Config::default()),
        }
    }

    #[test]
    #[timeout(2)]
    async fn test_systimer_async_delay_ns(ctx: Context) {
        let mut alarms = SystemTimer::new(ctx.peripherals.SYSTIMER);
        let unit = FrozenUnit::new(&mut alarms.unit0);
        let alarm0 = Alarm::new_async(alarms.comparator0, &unit).into_periodic();

        test_async_delay_ns(alarm0, 600_000_000).await;
    }

    #[test]
    #[timeout(2)]
    async fn test_timg0_async_delay_ns(ctx: Context) {
        let timg0 = TimerGroup::new_async(ctx.peripherals.TIMG0);

        test_async_delay_ns(timg0.timer0, 600_000_000).await;
        #[cfg(timg_timer1)]
        test_async_delay_ns(timg0.timer1, 600_000_000).await;
    }

    #[cfg(timg1)]
    #[test]
    #[timeout(2)]
    async fn test_timg1_async_delay_ns(ctx: Context) {
        let timg1 = TimerGroup::new_async(ctx.peripherals.TIMG1);

        test_async_delay_ns(timg1.timer0, 600_000_000).await;
        #[cfg(timg_timer1)]
        test_async_delay_ns(timg1.timer1, 600_000_000).await;
    }
}
