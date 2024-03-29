#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    _reserved1: [u8; 0x0c],
    hfrcoctrl: HFRCOCTRL,
    _reserved2: [u8; 0x04],
    auxhfrcoctrl: AUXHFRCOCTRL,
    _reserved3: [u8; 0x04],
    lfrcoctrl: LFRCOCTRL,
    hfxoctrl: HFXOCTRL,
    hfxoctrl1: HFXOCTRL1,
    hfxostartupctrl: HFXOSTARTUPCTRL,
    hfxosteadystatectrl: HFXOSTEADYSTATECTRL,
    hfxotimeoutctrl: HFXOTIMEOUTCTRL,
    lfxoctrl: LFXOCTRL,
    ulfrcoctrl: ULFRCOCTRL,
    _reserved11: [u8; 0x10],
    calctrl: CALCTRL,
    calcnt: CALCNT,
    _reserved13: [u8; 0x08],
    oscencmd: OSCENCMD,
    cmd: CMD,
    _reserved15: [u8; 0x08],
    dbgclksel: DBGCLKSEL,
    hfclksel: HFCLKSEL,
    _reserved17: [u8; 0x08],
    lfaclksel: LFACLKSEL,
    lfbclksel: LFBCLKSEL,
    lfeclksel: LFECLKSEL,
    _reserved20: [u8; 0x04],
    status: STATUS,
    hfclkstatus: HFCLKSTATUS,
    _reserved22: [u8; 0x04],
    hfxotrimstatus: HFXOTRIMSTATUS,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    hfbusclken0: HFBUSCLKEN0,
    _reserved28: [u8; 0x0c],
    hfperclken0: HFPERCLKEN0,
    _reserved29: [u8; 0x1c],
    lfaclken0: LFACLKEN0,
    _reserved30: [u8; 0x04],
    lfbclken0: LFBCLKEN0,
    _reserved31: [u8; 0x04],
    lfeclken0: LFECLKEN0,
    _reserved32: [u8; 0x0c],
    hfpresc: HFPRESC,
    _reserved33: [u8; 0x04],
    hfcorepresc: HFCOREPRESC,
    hfperpresc: HFPERPRESC,
    _reserved35: [u8; 0x04],
    hfexppresc: HFEXPPRESC,
    _reserved36: [u8; 0x08],
    lfapresc0: LFAPRESC0,
    _reserved37: [u8; 0x04],
    lfbpresc0: LFBPRESC0,
    _reserved38: [u8; 0x04],
    lfepresc0: LFEPRESC0,
    _reserved39: [u8; 0x0c],
    syncbusy: SYNCBUSY,
    freeze: FREEZE,
    _reserved41: [u8; 0x08],
    pcntctrl: PCNTCTRL,
    _reserved42: [u8; 0x08],
    adcctrl: ADCCTRL,
    _reserved43: [u8; 0x10],
    routepen: ROUTEPEN,
    routeloc0: ROUTELOC0,
    _reserved45: [u8; 0x08],
    lock: LOCK,
}
impl RegisterBlock {
    #[doc = "0x00 - CMU Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x10 - HFRCO Control Register"]
    #[inline(always)]
    pub const fn hfrcoctrl(&self) -> &HFRCOCTRL {
        &self.hfrcoctrl
    }
    #[doc = "0x18 - AUXHFRCO Control Register"]
    #[inline(always)]
    pub const fn auxhfrcoctrl(&self) -> &AUXHFRCOCTRL {
        &self.auxhfrcoctrl
    }
    #[doc = "0x20 - LFRCO Control Register"]
    #[inline(always)]
    pub const fn lfrcoctrl(&self) -> &LFRCOCTRL {
        &self.lfrcoctrl
    }
    #[doc = "0x24 - HFXO Control Register"]
    #[inline(always)]
    pub const fn hfxoctrl(&self) -> &HFXOCTRL {
        &self.hfxoctrl
    }
    #[doc = "0x28 - HFXO Control 1"]
    #[inline(always)]
    pub const fn hfxoctrl1(&self) -> &HFXOCTRL1 {
        &self.hfxoctrl1
    }
    #[doc = "0x2c - HFXO Startup Control"]
    #[inline(always)]
    pub const fn hfxostartupctrl(&self) -> &HFXOSTARTUPCTRL {
        &self.hfxostartupctrl
    }
    #[doc = "0x30 - HFXO Steady State Control"]
    #[inline(always)]
    pub const fn hfxosteadystatectrl(&self) -> &HFXOSTEADYSTATECTRL {
        &self.hfxosteadystatectrl
    }
    #[doc = "0x34 - HFXO Timeout Control"]
    #[inline(always)]
    pub const fn hfxotimeoutctrl(&self) -> &HFXOTIMEOUTCTRL {
        &self.hfxotimeoutctrl
    }
    #[doc = "0x38 - LFXO Control Register"]
    #[inline(always)]
    pub const fn lfxoctrl(&self) -> &LFXOCTRL {
        &self.lfxoctrl
    }
    #[doc = "0x3c - ULFRCO Control Register"]
    #[inline(always)]
    pub const fn ulfrcoctrl(&self) -> &ULFRCOCTRL {
        &self.ulfrcoctrl
    }
    #[doc = "0x50 - Calibration Control Register"]
    #[inline(always)]
    pub const fn calctrl(&self) -> &CALCTRL {
        &self.calctrl
    }
    #[doc = "0x54 - Calibration Counter Register"]
    #[inline(always)]
    pub const fn calcnt(&self) -> &CALCNT {
        &self.calcnt
    }
    #[doc = "0x60 - Oscillator Enable/Disable Command Register"]
    #[inline(always)]
    pub const fn oscencmd(&self) -> &OSCENCMD {
        &self.oscencmd
    }
    #[doc = "0x64 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x70 - Debug Trace Clock Select"]
    #[inline(always)]
    pub const fn dbgclksel(&self) -> &DBGCLKSEL {
        &self.dbgclksel
    }
    #[doc = "0x74 - High Frequency Clock Select Command Register"]
    #[inline(always)]
    pub const fn hfclksel(&self) -> &HFCLKSEL {
        &self.hfclksel
    }
    #[doc = "0x80 - Low Frequency A Clock Select Register"]
    #[inline(always)]
    pub const fn lfaclksel(&self) -> &LFACLKSEL {
        &self.lfaclksel
    }
    #[doc = "0x84 - Low Frequency B Clock Select Register"]
    #[inline(always)]
    pub const fn lfbclksel(&self) -> &LFBCLKSEL {
        &self.lfbclksel
    }
    #[doc = "0x88 - Low Frequency E Clock Select Register"]
    #[inline(always)]
    pub const fn lfeclksel(&self) -> &LFECLKSEL {
        &self.lfeclksel
    }
    #[doc = "0x90 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x94 - HFCLK Status Register"]
    #[inline(always)]
    pub const fn hfclkstatus(&self) -> &HFCLKSTATUS {
        &self.hfclkstatus
    }
    #[doc = "0x9c - HFXO Trim Status"]
    #[inline(always)]
    pub const fn hfxotrimstatus(&self) -> &HFXOTRIMSTATUS {
        &self.hfxotrimstatus
    }
    #[doc = "0xa0 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0xa4 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0xa8 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0xac - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0xb0 - High Frequency Bus Clock Enable Register 0"]
    #[inline(always)]
    pub const fn hfbusclken0(&self) -> &HFBUSCLKEN0 {
        &self.hfbusclken0
    }
    #[doc = "0xc0 - High Frequency Peripheral Clock Enable Register 0"]
    #[inline(always)]
    pub const fn hfperclken0(&self) -> &HFPERCLKEN0 {
        &self.hfperclken0
    }
    #[doc = "0xe0 - Low Frequency a Clock Enable Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfaclken0(&self) -> &LFACLKEN0 {
        &self.lfaclken0
    }
    #[doc = "0xe8 - Low Frequency B Clock Enable Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfbclken0(&self) -> &LFBCLKEN0 {
        &self.lfbclken0
    }
    #[doc = "0xf0 - Low Frequency E Clock Enable Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfeclken0(&self) -> &LFECLKEN0 {
        &self.lfeclken0
    }
    #[doc = "0x100 - High Frequency Clock Prescaler Register"]
    #[inline(always)]
    pub const fn hfpresc(&self) -> &HFPRESC {
        &self.hfpresc
    }
    #[doc = "0x108 - High Frequency Core Clock Prescaler Register"]
    #[inline(always)]
    pub const fn hfcorepresc(&self) -> &HFCOREPRESC {
        &self.hfcorepresc
    }
    #[doc = "0x10c - High Frequency Peripheral Clock Prescaler Register"]
    #[inline(always)]
    pub const fn hfperpresc(&self) -> &HFPERPRESC {
        &self.hfperpresc
    }
    #[doc = "0x114 - High Frequency Export Clock Prescaler Register"]
    #[inline(always)]
    pub const fn hfexppresc(&self) -> &HFEXPPRESC {
        &self.hfexppresc
    }
    #[doc = "0x120 - Low Frequency a Prescaler Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfapresc0(&self) -> &LFAPRESC0 {
        &self.lfapresc0
    }
    #[doc = "0x128 - Low Frequency B Prescaler Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfbpresc0(&self) -> &LFBPRESC0 {
        &self.lfbpresc0
    }
    #[doc = "0x130 - Low Frequency E Prescaler Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfepresc0(&self) -> &LFEPRESC0 {
        &self.lfepresc0
    }
    #[doc = "0x140 - Synchronization Busy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &SYNCBUSY {
        &self.syncbusy
    }
    #[doc = "0x144 - Freeze Register"]
    #[inline(always)]
    pub const fn freeze(&self) -> &FREEZE {
        &self.freeze
    }
    #[doc = "0x150 - PCNT Control Register"]
    #[inline(always)]
    pub const fn pcntctrl(&self) -> &PCNTCTRL {
        &self.pcntctrl
    }
    #[doc = "0x15c - ADC Control Register"]
    #[inline(always)]
    pub const fn adcctrl(&self) -> &ADCCTRL {
        &self.adcctrl
    }
    #[doc = "0x170 - I/O Routing Pin Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &ROUTEPEN {
        &self.routepen
    }
    #[doc = "0x174 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &ROUTELOC0 {
        &self.routeloc0
    }
    #[doc = "0x180 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &LOCK {
        &self.lock
    }
}
#[doc = "CTRL (rw) register accessor: CMU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRLrs>;
#[doc = "CMU Control Register"]
pub mod ctrl;
#[doc = "HFRCOCTRL (rw) register accessor: HFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfrcoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfrcoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoctrl`]
module"]
pub type HFRCOCTRL = crate::Reg<hfrcoctrl::HFRCOCTRLrs>;
#[doc = "HFRCO Control Register"]
pub mod hfrcoctrl;
#[doc = "AUXHFRCOCTRL (rw) register accessor: AUXHFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxhfrcoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxhfrcoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auxhfrcoctrl`]
module"]
pub type AUXHFRCOCTRL = crate::Reg<auxhfrcoctrl::AUXHFRCOCTRLrs>;
#[doc = "AUXHFRCO Control Register"]
pub mod auxhfrcoctrl;
#[doc = "LFRCOCTRL (rw) register accessor: LFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfrcoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfrcoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfrcoctrl`]
module"]
pub type LFRCOCTRL = crate::Reg<lfrcoctrl::LFRCOCTRLrs>;
#[doc = "LFRCO Control Register"]
pub mod lfrcoctrl;
#[doc = "HFXOCTRL (rw) register accessor: HFXO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxoctrl`]
module"]
pub type HFXOCTRL = crate::Reg<hfxoctrl::HFXOCTRLrs>;
#[doc = "HFXO Control Register"]
pub mod hfxoctrl;
#[doc = "HFXOCTRL1 (rw) register accessor: HFXO Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxoctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxoctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxoctrl1`]
module"]
pub type HFXOCTRL1 = crate::Reg<hfxoctrl1::HFXOCTRL1rs>;
#[doc = "HFXO Control 1"]
pub mod hfxoctrl1;
#[doc = "HFXOSTARTUPCTRL (rw) register accessor: HFXO Startup Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxostartupctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxostartupctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxostartupctrl`]
module"]
pub type HFXOSTARTUPCTRL = crate::Reg<hfxostartupctrl::HFXOSTARTUPCTRLrs>;
#[doc = "HFXO Startup Control"]
pub mod hfxostartupctrl;
#[doc = "HFXOSTEADYSTATECTRL (rw) register accessor: HFXO Steady State Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxosteadystatectrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxosteadystatectrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxosteadystatectrl`]
module"]
pub type HFXOSTEADYSTATECTRL = crate::Reg<hfxosteadystatectrl::HFXOSTEADYSTATECTRLrs>;
#[doc = "HFXO Steady State Control"]
pub mod hfxosteadystatectrl;
#[doc = "HFXOTIMEOUTCTRL (rw) register accessor: HFXO Timeout Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxotimeoutctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxotimeoutctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxotimeoutctrl`]
module"]
pub type HFXOTIMEOUTCTRL = crate::Reg<hfxotimeoutctrl::HFXOTIMEOUTCTRLrs>;
#[doc = "HFXO Timeout Control"]
pub mod hfxotimeoutctrl;
#[doc = "LFXOCTRL (rw) register accessor: LFXO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfxoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfxoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfxoctrl`]
module"]
pub type LFXOCTRL = crate::Reg<lfxoctrl::LFXOCTRLrs>;
#[doc = "LFXO Control Register"]
pub mod lfxoctrl;
#[doc = "ULFRCOCTRL (rw) register accessor: ULFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ulfrcoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulfrcoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ulfrcoctrl`]
module"]
pub type ULFRCOCTRL = crate::Reg<ulfrcoctrl::ULFRCOCTRLrs>;
#[doc = "ULFRCO Control Register"]
pub mod ulfrcoctrl;
#[doc = "CALCTRL (rw) register accessor: Calibration Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calctrl`]
module"]
pub type CALCTRL = crate::Reg<calctrl::CALCTRLrs>;
#[doc = "Calibration Control Register"]
pub mod calctrl;
#[doc = "CALCNT (rw) register accessor: Calibration Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calcnt`]
module"]
pub type CALCNT = crate::Reg<calcnt::CALCNTrs>;
#[doc = "Calibration Counter Register"]
pub mod calcnt;
#[doc = "OSCENCMD (w) register accessor: Oscillator Enable/Disable Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscencmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscencmd`]
module"]
pub type OSCENCMD = crate::Reg<oscencmd::OSCENCMDrs>;
#[doc = "Oscillator Enable/Disable Command Register"]
pub mod oscencmd;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMDrs>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "DBGCLKSEL (rw) register accessor: Debug Trace Clock Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgclksel`]
module"]
pub type DBGCLKSEL = crate::Reg<dbgclksel::DBGCLKSELrs>;
#[doc = "Debug Trace Clock Select"]
pub mod dbgclksel;
#[doc = "HFCLKSEL (w) register accessor: High Frequency Clock Select Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfclksel::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclksel`]
module"]
pub type HFCLKSEL = crate::Reg<hfclksel::HFCLKSELrs>;
#[doc = "High Frequency Clock Select Command Register"]
pub mod hfclksel;
#[doc = "LFACLKSEL (rw) register accessor: Low Frequency A Clock Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfaclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfaclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfaclksel`]
module"]
pub type LFACLKSEL = crate::Reg<lfaclksel::LFACLKSELrs>;
#[doc = "Low Frequency A Clock Select Register"]
pub mod lfaclksel;
#[doc = "LFBCLKSEL (rw) register accessor: Low Frequency B Clock Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfbclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfbclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfbclksel`]
module"]
pub type LFBCLKSEL = crate::Reg<lfbclksel::LFBCLKSELrs>;
#[doc = "Low Frequency B Clock Select Register"]
pub mod lfbclksel;
#[doc = "LFECLKSEL (rw) register accessor: Low Frequency E Clock Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfeclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfeclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfeclksel`]
module"]
pub type LFECLKSEL = crate::Reg<lfeclksel::LFECLKSELrs>;
#[doc = "Low Frequency E Clock Select Register"]
pub mod lfeclksel;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUSrs>;
#[doc = "Status Register"]
pub mod status;
#[doc = "HFCLKSTATUS (r) register accessor: HFCLK Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfclkstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkstatus`]
module"]
pub type HFCLKSTATUS = crate::Reg<hfclkstatus::HFCLKSTATUSrs>;
#[doc = "HFCLK Status Register"]
pub mod hfclkstatus;
#[doc = "HFXOTRIMSTATUS (r) register accessor: HFXO Trim Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxotrimstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxotrimstatus`]
module"]
pub type HFXOTRIMSTATUS = crate::Reg<hfxotrimstatus::HFXOTRIMSTATUSrs>;
#[doc = "HFXO Trim Status"]
pub mod hfxotrimstatus;
#[doc = "IF (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`]
module"]
pub type IF = crate::Reg<if_::IFrs>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs`]
module"]
pub type IFS = crate::Reg<ifs::IFSrs>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifc`]
module"]
pub type IFC = crate::Reg<ifc::IFCrs>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`]
module"]
pub type IEN = crate::Reg<ien::IENrs>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "HFBUSCLKEN0 (rw) register accessor: High Frequency Bus Clock Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfbusclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfbusclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfbusclken0`]
module"]
pub type HFBUSCLKEN0 = crate::Reg<hfbusclken0::HFBUSCLKEN0rs>;
#[doc = "High Frequency Bus Clock Enable Register 0"]
pub mod hfbusclken0;
#[doc = "HFPERCLKEN0 (rw) register accessor: High Frequency Peripheral Clock Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfperclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfperclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfperclken0`]
module"]
pub type HFPERCLKEN0 = crate::Reg<hfperclken0::HFPERCLKEN0rs>;
#[doc = "High Frequency Peripheral Clock Enable Register 0"]
pub mod hfperclken0;
#[doc = "LFACLKEN0 (rw) register accessor: Low Frequency a Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfaclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfaclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfaclken0`]
module"]
pub type LFACLKEN0 = crate::Reg<lfaclken0::LFACLKEN0rs>;
#[doc = "Low Frequency a Clock Enable Register 0 (Async Reg)"]
pub mod lfaclken0;
#[doc = "LFBCLKEN0 (rw) register accessor: Low Frequency B Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfbclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfbclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfbclken0`]
module"]
pub type LFBCLKEN0 = crate::Reg<lfbclken0::LFBCLKEN0rs>;
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)"]
pub mod lfbclken0;
#[doc = "LFECLKEN0 (rw) register accessor: Low Frequency E Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfeclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfeclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfeclken0`]
module"]
pub type LFECLKEN0 = crate::Reg<lfeclken0::LFECLKEN0rs>;
#[doc = "Low Frequency E Clock Enable Register 0 (Async Reg)"]
pub mod lfeclken0;
#[doc = "HFPRESC (rw) register accessor: High Frequency Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfpresc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfpresc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfpresc`]
module"]
pub type HFPRESC = crate::Reg<hfpresc::HFPRESCrs>;
#[doc = "High Frequency Clock Prescaler Register"]
pub mod hfpresc;
#[doc = "HFCOREPRESC (rw) register accessor: High Frequency Core Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfcorepresc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfcorepresc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcorepresc`]
module"]
pub type HFCOREPRESC = crate::Reg<hfcorepresc::HFCOREPRESCrs>;
#[doc = "High Frequency Core Clock Prescaler Register"]
pub mod hfcorepresc;
#[doc = "HFPERPRESC (rw) register accessor: High Frequency Peripheral Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfperpresc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfperpresc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfperpresc`]
module"]
pub type HFPERPRESC = crate::Reg<hfperpresc::HFPERPRESCrs>;
#[doc = "High Frequency Peripheral Clock Prescaler Register"]
pub mod hfperpresc;
#[doc = "HFEXPPRESC (rw) register accessor: High Frequency Export Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfexppresc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfexppresc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfexppresc`]
module"]
pub type HFEXPPRESC = crate::Reg<hfexppresc::HFEXPPRESCrs>;
#[doc = "High Frequency Export Clock Prescaler Register"]
pub mod hfexppresc;
#[doc = "LFAPRESC0 (rw) register accessor: Low Frequency a Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfapresc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfapresc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfapresc0`]
module"]
pub type LFAPRESC0 = crate::Reg<lfapresc0::LFAPRESC0rs>;
#[doc = "Low Frequency a Prescaler Register 0 (Async Reg)"]
pub mod lfapresc0;
#[doc = "LFBPRESC0 (rw) register accessor: Low Frequency B Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfbpresc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfbpresc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfbpresc0`]
module"]
pub type LFBPRESC0 = crate::Reg<lfbpresc0::LFBPRESC0rs>;
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)"]
pub mod lfbpresc0;
#[doc = "LFEPRESC0 (rw) register accessor: Low Frequency E Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfepresc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfepresc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfepresc0`]
module"]
pub type LFEPRESC0 = crate::Reg<lfepresc0::LFEPRESC0rs>;
#[doc = "Low Frequency E Prescaler Register 0 (Async Reg)"]
pub mod lfepresc0;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSYrs>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "FREEZE (rw) register accessor: Freeze Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freeze::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freeze::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freeze`]
module"]
pub type FREEZE = crate::Reg<freeze::FREEZErs>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "PCNTCTRL (rw) register accessor: PCNT Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcntctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcntctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcntctrl`]
module"]
pub type PCNTCTRL = crate::Reg<pcntctrl::PCNTCTRLrs>;
#[doc = "PCNT Control Register"]
pub mod pcntctrl;
#[doc = "ADCCTRL (rw) register accessor: ADC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcctrl`]
module"]
pub type ADCCTRL = crate::Reg<adcctrl::ADCCTRLrs>;
#[doc = "ADC Control Register"]
pub mod adcctrl;
#[doc = "ROUTEPEN (rw) register accessor: I/O Routing Pin Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`]
module"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPENrs>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`]
module"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0rs>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCKrs>;
#[doc = "Configuration Lock Register"]
pub mod lock;
