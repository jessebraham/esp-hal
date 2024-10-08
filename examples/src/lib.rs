#![no_std]

pub fn cycles() -> u64 {
    #[cfg(feature = "esp32")]
    {
        esp_hal::xtensa_lx::timer::get_cycle_count() as u64
    }

    // FIXME
    #[cfg(not(any(feature = "esp32", feature = "esp32p4")))]
    {
        esp_hal::timer::systimer::SystemTimer::now()
    }

    // FIXME
    #[cfg(feature = "esp32p4")]
    {
        0
    }
}
