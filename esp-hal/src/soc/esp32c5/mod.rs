crate::unstable_module! {
    pub mod lp_core;
}
pub mod gpio;
pub(crate) mod regi2c;

pub(crate) use esp32c5 as pac;

#[cfg_attr(not(feature = "unstable"), allow(unused))]
pub(crate) mod registers {
    pub const INTERRUPT_MAP_BASE: u32 = 0x0; // FIXME
}

#[cfg_attr(not(feature = "unstable"), allow(unused))]
pub(crate) mod constants {
    use crate::time::Rate;

    /// RC FAST Clock value (Hertz).
    pub const RC_FAST_CLK: Rate = Rate::from_mhz(20);
}

pub(crate) fn pre_init() {
    todo!()
}
