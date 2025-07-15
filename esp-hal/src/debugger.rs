//! Debugger utilities

/// Checks if a debugger is connected.
pub fn debugger_connected() -> bool {
    #[cfg(xtensa)]
    {
        xtensa_lx::is_debugger_attached()
    }

    #[cfg(soc_has_assist_debug)]
    {
        crate::peripherals::ASSIST_DEBUG::regs()
            .core_0_debug_mode()
            .read()
            .core_0_debug_module_active()
            .bit_is_set()
    }

    #[cfg(not(any(xtensa, soc_has_assist_debug)))]
    {
        false
    }
}
