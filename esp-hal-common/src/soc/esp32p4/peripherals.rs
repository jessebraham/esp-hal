//! # Peripheral instance singletons (ESP32-P4)
//!
//! ## Overview
//!
//! The `Peripherals` module provides singleton instances of various peripherals
//! and allows users to access and use them in their applications.
//!
//! These peripherals provide various functionalities and interfaces for
//! interacting with different hardware components on the `ESP32-P4` chip, such
//! as timers, `GPIO` pins, `I2C`, `SPI`, `UART`, and more. Users can access and
//! utilize these peripherals by importing the respective singleton instances
//! from this module.
//!
//! It's important to note that the module also exports the `Interrupt` enum
//! from the `ESP32-P4` `PAC (Peripheral Access Crate)` for users to handle
//! interrupts associated with these peripherals.
//!
//! ⚠️ NOTE: notice that `radio` and `lp_core` are marked with `false` in the
//! `peripherals!` macro. Basically, that means that there's no real peripheral
//! (no `RADIO` nor `LP_CORE` peripheral in the PACs) but we're
//! creating "virtual peripherals" for them in order to ensure the uniqueness of
//! the instances (Singletons).

use esp32p4 as pac;
// We need to export this for users to use
pub use pac::Interrupt;

// We need to export this in the hal for the drivers to use
pub(crate) use self::peripherals::*;

crate::peripherals! {
    ADC <= ADC,
    AES <= AES,
    ASSIST_DEBUG <= ASSIST_DEBUG,
    AXI_DMA <= AXI_DMA,
    AXI_ICM <= AXI_ICM,
    BITSCRAMBLER <= BITSCRAMBLER,
    CACHE <= CACHE,
    DMA <= DMA,
    DS <= DS,
    ECC <= ECC,
    ECDSA <= ECDSA,
    EFUSE <= EFUSE,
    GPIO <= GPIO,
    GPIO_SD <= GPIO_SD,
    H264 <= H264,
    H264_DMA <= H264_DMA,
    HMAC <= HMAC,
    HP_SYS <= HP_SYS,
    HP_SYS_CLKRST <= HP_SYS_CLKRST,
    I2C0 <= I2C0,
    I2C1 <= I2C1,
    I2S0 <= I2S0,
    I2S1 <= I2S1,
    I2S2 <= I2S2,
    I3C_MST <= I3C_MST,
    I3C_MST_MEM <= I3C_MST_MEM,
    I3C_SLV <= I3C_SLV,
    INTERRUPT_CORE0 <= INTERRUPT_CORE0,
    INTERRUPT_CORE1 <= INTERRUPT_CORE1,
    IO_MUX <= IO_MUX,
    ISP <= ISP,
    JPEG <= JPEG,
    LCD_CAM <= LCD_CAM,
    LEDC <= LEDC,
    LP_ADC <= LP_ADC,
    LP_ANA_PERI <= LP_ANA_PERI,
    LP_AON_CLKRST <= LP_AON_CLKRST,
    LP_GPIO <= LP_GPIO,
    LP_HUK <= LP_HUK,
    LP_I2C_ANA_MST <= LP_I2C_ANA_MST,
    LP_I2C0 <= LP_I2C0,
    LP_I2S0 <= LP_I2S0,
    LP_INTR <= LP_INTR,
    LP_IO_MUX <= LP_IO_MUX,
    LP_PERI <= LP_PERI,
    LP_SYS <= LP_SYS,
    LP_TIMER <= LP_TIMER,
    LP_TOUCH <= LP_TOUCH,
    LP_TSENS <= LP_TSENS,
    LP_UART <= LP_UART,
    LP_WDT <= LP_WDT,
    MCPWM0 <= MCPWM0,
    MCPWM1 <= MCPWM1,
    MIPI_CSI_BRIDGE <= MIPI_CSI_BRIDGE,
    MIPI_CSI_HOST <= MIPI_CSI_HOST,
    MIPI_DSI_BRIDGE <= MIPI_DSI_BRIDGE,
    MIPI_DSI_HOST <= MIPI_DSI_HOST,
    PARL_IO <= PARL_IO,
    PAU <= PAU,
    PCNT <= PCNT,
    PMU <= PMU,
    PPA <= PPA,
    PVT <= PVT,
    RMT <= RMT,
    RSA <= RSA,
    SDHOST <= SDHOST,
    SHA <= SHA,
    SOC_ETM <= SOC_ETM,
    SPI0 <= SPI0,
    SPI1 <= SPI1,
    SPI2 <= SPI2,
    SPI3 <= SPI3,
    SYSTIMER <= SYSTIMER,
    TIMG0 <= TIMG0,
    TIMG1 <= TIMG1,
    TRACE0 <= TRACE0,
    TRACE1 <= TRACE1,
    TWAI0 <= TWAI0,
    TWAI1 <= TWAI1,
    TWAI2 <= TWAI2,
    UART0 <= UART0,
    UHCI0 <= UHCI0,
    USB_DEVICE <= USB_DEVICE,
    USB_WRAP <= USB_WRAP,
}
