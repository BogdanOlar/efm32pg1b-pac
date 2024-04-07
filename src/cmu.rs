#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x0c],
    hfrcoctrl: Hfrcoctrl,
    _reserved2: [u8; 0x04],
    auxhfrcoctrl: Auxhfrcoctrl,
    _reserved3: [u8; 0x04],
    lfrcoctrl: Lfrcoctrl,
    hfxoctrl: Hfxoctrl,
    hfxoctrl1: Hfxoctrl1,
    hfxostartupctrl: Hfxostartupctrl,
    hfxosteadystatectrl: Hfxosteadystatectrl,
    hfxotimeoutctrl: Hfxotimeoutctrl,
    lfxoctrl: Lfxoctrl,
    ulfrcoctrl: Ulfrcoctrl,
    _reserved11: [u8; 0x10],
    calctrl: Calctrl,
    calcnt: Calcnt,
    _reserved13: [u8; 0x08],
    oscencmd: Oscencmd,
    cmd: Cmd,
    _reserved15: [u8; 0x08],
    dbgclksel: Dbgclksel,
    hfclksel: Hfclksel,
    _reserved17: [u8; 0x08],
    lfaclksel: Lfaclksel,
    lfbclksel: Lfbclksel,
    lfeclksel: Lfeclksel,
    _reserved20: [u8; 0x04],
    status: Status,
    hfclkstatus: Hfclkstatus,
    _reserved22: [u8; 0x04],
    hfxotrimstatus: Hfxotrimstatus,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    hfbusclken0: Hfbusclken0,
    _reserved28: [u8; 0x0c],
    hfperclken0: Hfperclken0,
    _reserved29: [u8; 0x1c],
    lfaclken0: Lfaclken0,
    _reserved30: [u8; 0x04],
    lfbclken0: Lfbclken0,
    _reserved31: [u8; 0x04],
    lfeclken0: Lfeclken0,
    _reserved32: [u8; 0x0c],
    hfpresc: Hfpresc,
    _reserved33: [u8; 0x04],
    hfcorepresc: Hfcorepresc,
    hfperpresc: Hfperpresc,
    _reserved35: [u8; 0x04],
    hfexppresc: Hfexppresc,
    _reserved36: [u8; 0x08],
    lfapresc0: Lfapresc0,
    _reserved37: [u8; 0x04],
    lfbpresc0: Lfbpresc0,
    _reserved38: [u8; 0x04],
    lfepresc0: Lfepresc0,
    _reserved39: [u8; 0x0c],
    syncbusy: Syncbusy,
    freeze: Freeze,
    _reserved41: [u8; 0x08],
    pcntctrl: Pcntctrl,
    _reserved42: [u8; 0x08],
    adcctrl: Adcctrl,
    _reserved43: [u8; 0x10],
    routepen: Routepen,
    routeloc0: Routeloc0,
    _reserved45: [u8; 0x08],
    lock: Lock,
}
impl RegisterBlock {
    #[doc = "0x00 - CMU Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x10 - HFRCO Control Register"]
    #[inline(always)]
    pub const fn hfrcoctrl(&self) -> &Hfrcoctrl {
        &self.hfrcoctrl
    }
    #[doc = "0x18 - AUXHFRCO Control Register"]
    #[inline(always)]
    pub const fn auxhfrcoctrl(&self) -> &Auxhfrcoctrl {
        &self.auxhfrcoctrl
    }
    #[doc = "0x20 - LFRCO Control Register"]
    #[inline(always)]
    pub const fn lfrcoctrl(&self) -> &Lfrcoctrl {
        &self.lfrcoctrl
    }
    #[doc = "0x24 - HFXO Control Register"]
    #[inline(always)]
    pub const fn hfxoctrl(&self) -> &Hfxoctrl {
        &self.hfxoctrl
    }
    #[doc = "0x28 - HFXO Control 1"]
    #[inline(always)]
    pub const fn hfxoctrl1(&self) -> &Hfxoctrl1 {
        &self.hfxoctrl1
    }
    #[doc = "0x2c - HFXO Startup Control"]
    #[inline(always)]
    pub const fn hfxostartupctrl(&self) -> &Hfxostartupctrl {
        &self.hfxostartupctrl
    }
    #[doc = "0x30 - HFXO Steady State Control"]
    #[inline(always)]
    pub const fn hfxosteadystatectrl(&self) -> &Hfxosteadystatectrl {
        &self.hfxosteadystatectrl
    }
    #[doc = "0x34 - HFXO Timeout Control"]
    #[inline(always)]
    pub const fn hfxotimeoutctrl(&self) -> &Hfxotimeoutctrl {
        &self.hfxotimeoutctrl
    }
    #[doc = "0x38 - LFXO Control Register"]
    #[inline(always)]
    pub const fn lfxoctrl(&self) -> &Lfxoctrl {
        &self.lfxoctrl
    }
    #[doc = "0x3c - ULFRCO Control Register"]
    #[inline(always)]
    pub const fn ulfrcoctrl(&self) -> &Ulfrcoctrl {
        &self.ulfrcoctrl
    }
    #[doc = "0x50 - Calibration Control Register"]
    #[inline(always)]
    pub const fn calctrl(&self) -> &Calctrl {
        &self.calctrl
    }
    #[doc = "0x54 - Calibration Counter Register"]
    #[inline(always)]
    pub const fn calcnt(&self) -> &Calcnt {
        &self.calcnt
    }
    #[doc = "0x60 - Oscillator Enable/Disable Command Register"]
    #[inline(always)]
    pub const fn oscencmd(&self) -> &Oscencmd {
        &self.oscencmd
    }
    #[doc = "0x64 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x70 - Debug Trace Clock Select"]
    #[inline(always)]
    pub const fn dbgclksel(&self) -> &Dbgclksel {
        &self.dbgclksel
    }
    #[doc = "0x74 - High Frequency Clock Select Command Register"]
    #[inline(always)]
    pub const fn hfclksel(&self) -> &Hfclksel {
        &self.hfclksel
    }
    #[doc = "0x80 - Low Frequency A Clock Select Register"]
    #[inline(always)]
    pub const fn lfaclksel(&self) -> &Lfaclksel {
        &self.lfaclksel
    }
    #[doc = "0x84 - Low Frequency B Clock Select Register"]
    #[inline(always)]
    pub const fn lfbclksel(&self) -> &Lfbclksel {
        &self.lfbclksel
    }
    #[doc = "0x88 - Low Frequency E Clock Select Register"]
    #[inline(always)]
    pub const fn lfeclksel(&self) -> &Lfeclksel {
        &self.lfeclksel
    }
    #[doc = "0x90 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x94 - HFCLK Status Register"]
    #[inline(always)]
    pub const fn hfclkstatus(&self) -> &Hfclkstatus {
        &self.hfclkstatus
    }
    #[doc = "0x9c - HFXO Trim Status"]
    #[inline(always)]
    pub const fn hfxotrimstatus(&self) -> &Hfxotrimstatus {
        &self.hfxotrimstatus
    }
    #[doc = "0xa0 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0xa4 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0xa8 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0xac - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0xb0 - High Frequency Bus Clock Enable Register 0"]
    #[inline(always)]
    pub const fn hfbusclken0(&self) -> &Hfbusclken0 {
        &self.hfbusclken0
    }
    #[doc = "0xc0 - High Frequency Peripheral Clock Enable Register 0"]
    #[inline(always)]
    pub const fn hfperclken0(&self) -> &Hfperclken0 {
        &self.hfperclken0
    }
    #[doc = "0xe0 - Low Frequency a Clock Enable Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfaclken0(&self) -> &Lfaclken0 {
        &self.lfaclken0
    }
    #[doc = "0xe8 - Low Frequency B Clock Enable Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfbclken0(&self) -> &Lfbclken0 {
        &self.lfbclken0
    }
    #[doc = "0xf0 - Low Frequency E Clock Enable Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfeclken0(&self) -> &Lfeclken0 {
        &self.lfeclken0
    }
    #[doc = "0x100 - High Frequency Clock Prescaler Register"]
    #[inline(always)]
    pub const fn hfpresc(&self) -> &Hfpresc {
        &self.hfpresc
    }
    #[doc = "0x108 - High Frequency Core Clock Prescaler Register"]
    #[inline(always)]
    pub const fn hfcorepresc(&self) -> &Hfcorepresc {
        &self.hfcorepresc
    }
    #[doc = "0x10c - High Frequency Peripheral Clock Prescaler Register"]
    #[inline(always)]
    pub const fn hfperpresc(&self) -> &Hfperpresc {
        &self.hfperpresc
    }
    #[doc = "0x114 - High Frequency Export Clock Prescaler Register"]
    #[inline(always)]
    pub const fn hfexppresc(&self) -> &Hfexppresc {
        &self.hfexppresc
    }
    #[doc = "0x120 - Low Frequency a Prescaler Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfapresc0(&self) -> &Lfapresc0 {
        &self.lfapresc0
    }
    #[doc = "0x128 - Low Frequency B Prescaler Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfbpresc0(&self) -> &Lfbpresc0 {
        &self.lfbpresc0
    }
    #[doc = "0x130 - Low Frequency E Prescaler Register 0 (Async Reg)"]
    #[inline(always)]
    pub const fn lfepresc0(&self) -> &Lfepresc0 {
        &self.lfepresc0
    }
    #[doc = "0x140 - Synchronization Busy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x144 - Freeze Register"]
    #[inline(always)]
    pub const fn freeze(&self) -> &Freeze {
        &self.freeze
    }
    #[doc = "0x150 - PCNT Control Register"]
    #[inline(always)]
    pub const fn pcntctrl(&self) -> &Pcntctrl {
        &self.pcntctrl
    }
    #[doc = "0x15c - ADC Control Register"]
    #[inline(always)]
    pub const fn adcctrl(&self) -> &Adcctrl {
        &self.adcctrl
    }
    #[doc = "0x170 - I/O Routing Pin Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    #[doc = "0x174 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
    #[doc = "0x180 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
}
#[doc = "CTRL (rw) register accessor: CMU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CTRLrs>;
#[doc = "CMU Control Register"]
pub mod ctrl;
#[doc = "HFRCOCTRL (rw) register accessor: HFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfrcoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfrcoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoctrl`]
module"]
#[doc(alias = "HFRCOCTRL")]
pub type Hfrcoctrl = crate::Reg<hfrcoctrl::HFRCOCTRLrs>;
#[doc = "HFRCO Control Register"]
pub mod hfrcoctrl;
#[doc = "AUXHFRCOCTRL (rw) register accessor: AUXHFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxhfrcoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxhfrcoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auxhfrcoctrl`]
module"]
#[doc(alias = "AUXHFRCOCTRL")]
pub type Auxhfrcoctrl = crate::Reg<auxhfrcoctrl::AUXHFRCOCTRLrs>;
#[doc = "AUXHFRCO Control Register"]
pub mod auxhfrcoctrl;
#[doc = "LFRCOCTRL (rw) register accessor: LFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfrcoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfrcoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfrcoctrl`]
module"]
#[doc(alias = "LFRCOCTRL")]
pub type Lfrcoctrl = crate::Reg<lfrcoctrl::LFRCOCTRLrs>;
#[doc = "LFRCO Control Register"]
pub mod lfrcoctrl;
#[doc = "HFXOCTRL (rw) register accessor: HFXO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxoctrl`]
module"]
#[doc(alias = "HFXOCTRL")]
pub type Hfxoctrl = crate::Reg<hfxoctrl::HFXOCTRLrs>;
#[doc = "HFXO Control Register"]
pub mod hfxoctrl;
#[doc = "HFXOCTRL1 (rw) register accessor: HFXO Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxoctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxoctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxoctrl1`]
module"]
#[doc(alias = "HFXOCTRL1")]
pub type Hfxoctrl1 = crate::Reg<hfxoctrl1::HFXOCTRL1rs>;
#[doc = "HFXO Control 1"]
pub mod hfxoctrl1;
#[doc = "HFXOSTARTUPCTRL (rw) register accessor: HFXO Startup Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxostartupctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxostartupctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxostartupctrl`]
module"]
#[doc(alias = "HFXOSTARTUPCTRL")]
pub type Hfxostartupctrl = crate::Reg<hfxostartupctrl::HFXOSTARTUPCTRLrs>;
#[doc = "HFXO Startup Control"]
pub mod hfxostartupctrl;
#[doc = "HFXOSTEADYSTATECTRL (rw) register accessor: HFXO Steady State Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxosteadystatectrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxosteadystatectrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxosteadystatectrl`]
module"]
#[doc(alias = "HFXOSTEADYSTATECTRL")]
pub type Hfxosteadystatectrl = crate::Reg<hfxosteadystatectrl::HFXOSTEADYSTATECTRLrs>;
#[doc = "HFXO Steady State Control"]
pub mod hfxosteadystatectrl;
#[doc = "HFXOTIMEOUTCTRL (rw) register accessor: HFXO Timeout Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxotimeoutctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxotimeoutctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxotimeoutctrl`]
module"]
#[doc(alias = "HFXOTIMEOUTCTRL")]
pub type Hfxotimeoutctrl = crate::Reg<hfxotimeoutctrl::HFXOTIMEOUTCTRLrs>;
#[doc = "HFXO Timeout Control"]
pub mod hfxotimeoutctrl;
#[doc = "LFXOCTRL (rw) register accessor: LFXO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfxoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfxoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfxoctrl`]
module"]
#[doc(alias = "LFXOCTRL")]
pub type Lfxoctrl = crate::Reg<lfxoctrl::LFXOCTRLrs>;
#[doc = "LFXO Control Register"]
pub mod lfxoctrl;
#[doc = "ULFRCOCTRL (rw) register accessor: ULFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ulfrcoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulfrcoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ulfrcoctrl`]
module"]
#[doc(alias = "ULFRCOCTRL")]
pub type Ulfrcoctrl = crate::Reg<ulfrcoctrl::ULFRCOCTRLrs>;
#[doc = "ULFRCO Control Register"]
pub mod ulfrcoctrl;
#[doc = "CALCTRL (rw) register accessor: Calibration Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calctrl`]
module"]
#[doc(alias = "CALCTRL")]
pub type Calctrl = crate::Reg<calctrl::CALCTRLrs>;
#[doc = "Calibration Control Register"]
pub mod calctrl;
#[doc = "CALCNT (rw) register accessor: Calibration Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calcnt`]
module"]
#[doc(alias = "CALCNT")]
pub type Calcnt = crate::Reg<calcnt::CALCNTrs>;
#[doc = "Calibration Counter Register"]
pub mod calcnt;
#[doc = "OSCENCMD (w) register accessor: Oscillator Enable/Disable Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscencmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscencmd`]
module"]
#[doc(alias = "OSCENCMD")]
pub type Oscencmd = crate::Reg<oscencmd::OSCENCMDrs>;
#[doc = "Oscillator Enable/Disable Command Register"]
pub mod oscencmd;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CMDrs>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "DBGCLKSEL (rw) register accessor: Debug Trace Clock Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgclksel`]
module"]
#[doc(alias = "DBGCLKSEL")]
pub type Dbgclksel = crate::Reg<dbgclksel::DBGCLKSELrs>;
#[doc = "Debug Trace Clock Select"]
pub mod dbgclksel;
#[doc = "HFCLKSEL (w) register accessor: High Frequency Clock Select Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfclksel::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclksel`]
module"]
#[doc(alias = "HFCLKSEL")]
pub type Hfclksel = crate::Reg<hfclksel::HFCLKSELrs>;
#[doc = "High Frequency Clock Select Command Register"]
pub mod hfclksel;
#[doc = "LFACLKSEL (rw) register accessor: Low Frequency A Clock Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfaclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfaclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfaclksel`]
module"]
#[doc(alias = "LFACLKSEL")]
pub type Lfaclksel = crate::Reg<lfaclksel::LFACLKSELrs>;
#[doc = "Low Frequency A Clock Select Register"]
pub mod lfaclksel;
#[doc = "LFBCLKSEL (rw) register accessor: Low Frequency B Clock Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfbclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfbclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfbclksel`]
module"]
#[doc(alias = "LFBCLKSEL")]
pub type Lfbclksel = crate::Reg<lfbclksel::LFBCLKSELrs>;
#[doc = "Low Frequency B Clock Select Register"]
pub mod lfbclksel;
#[doc = "LFECLKSEL (rw) register accessor: Low Frequency E Clock Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfeclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfeclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfeclksel`]
module"]
#[doc(alias = "LFECLKSEL")]
pub type Lfeclksel = crate::Reg<lfeclksel::LFECLKSELrs>;
#[doc = "Low Frequency E Clock Select Register"]
pub mod lfeclksel;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::STATUSrs>;
#[doc = "Status Register"]
pub mod status;
#[doc = "HFCLKSTATUS (r) register accessor: HFCLK Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfclkstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkstatus`]
module"]
#[doc(alias = "HFCLKSTATUS")]
pub type Hfclkstatus = crate::Reg<hfclkstatus::HFCLKSTATUSrs>;
#[doc = "HFCLK Status Register"]
pub mod hfclkstatus;
#[doc = "HFXOTRIMSTATUS (r) register accessor: HFXO Trim Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxotrimstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxotrimstatus`]
module"]
#[doc(alias = "HFXOTRIMSTATUS")]
pub type Hfxotrimstatus = crate::Reg<hfxotrimstatus::HFXOTRIMSTATUSrs>;
#[doc = "HFXO Trim Status"]
pub mod hfxotrimstatus;
#[doc = "IF (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`]
module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IFrs>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs`]
module"]
#[doc(alias = "IFS")]
pub type Ifs = crate::Reg<ifs::IFSrs>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifc`]
module"]
#[doc(alias = "IFC")]
pub type Ifc = crate::Reg<ifc::IFCrs>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`]
module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IENrs>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "HFBUSCLKEN0 (rw) register accessor: High Frequency Bus Clock Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfbusclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfbusclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfbusclken0`]
module"]
#[doc(alias = "HFBUSCLKEN0")]
pub type Hfbusclken0 = crate::Reg<hfbusclken0::HFBUSCLKEN0rs>;
#[doc = "High Frequency Bus Clock Enable Register 0"]
pub mod hfbusclken0;
#[doc = "HFPERCLKEN0 (rw) register accessor: High Frequency Peripheral Clock Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfperclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfperclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfperclken0`]
module"]
#[doc(alias = "HFPERCLKEN0")]
pub type Hfperclken0 = crate::Reg<hfperclken0::HFPERCLKEN0rs>;
#[doc = "High Frequency Peripheral Clock Enable Register 0"]
pub mod hfperclken0;
#[doc = "LFACLKEN0 (rw) register accessor: Low Frequency a Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfaclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfaclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfaclken0`]
module"]
#[doc(alias = "LFACLKEN0")]
pub type Lfaclken0 = crate::Reg<lfaclken0::LFACLKEN0rs>;
#[doc = "Low Frequency a Clock Enable Register 0 (Async Reg)"]
pub mod lfaclken0;
#[doc = "LFBCLKEN0 (rw) register accessor: Low Frequency B Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfbclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfbclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfbclken0`]
module"]
#[doc(alias = "LFBCLKEN0")]
pub type Lfbclken0 = crate::Reg<lfbclken0::LFBCLKEN0rs>;
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)"]
pub mod lfbclken0;
#[doc = "LFECLKEN0 (rw) register accessor: Low Frequency E Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfeclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfeclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfeclken0`]
module"]
#[doc(alias = "LFECLKEN0")]
pub type Lfeclken0 = crate::Reg<lfeclken0::LFECLKEN0rs>;
#[doc = "Low Frequency E Clock Enable Register 0 (Async Reg)"]
pub mod lfeclken0;
#[doc = "HFPRESC (rw) register accessor: High Frequency Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfpresc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfpresc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfpresc`]
module"]
#[doc(alias = "HFPRESC")]
pub type Hfpresc = crate::Reg<hfpresc::HFPRESCrs>;
#[doc = "High Frequency Clock Prescaler Register"]
pub mod hfpresc;
#[doc = "HFCOREPRESC (rw) register accessor: High Frequency Core Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfcorepresc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfcorepresc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcorepresc`]
module"]
#[doc(alias = "HFCOREPRESC")]
pub type Hfcorepresc = crate::Reg<hfcorepresc::HFCOREPRESCrs>;
#[doc = "High Frequency Core Clock Prescaler Register"]
pub mod hfcorepresc;
#[doc = "HFPERPRESC (rw) register accessor: High Frequency Peripheral Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfperpresc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfperpresc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfperpresc`]
module"]
#[doc(alias = "HFPERPRESC")]
pub type Hfperpresc = crate::Reg<hfperpresc::HFPERPRESCrs>;
#[doc = "High Frequency Peripheral Clock Prescaler Register"]
pub mod hfperpresc;
#[doc = "HFEXPPRESC (rw) register accessor: High Frequency Export Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfexppresc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfexppresc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfexppresc`]
module"]
#[doc(alias = "HFEXPPRESC")]
pub type Hfexppresc = crate::Reg<hfexppresc::HFEXPPRESCrs>;
#[doc = "High Frequency Export Clock Prescaler Register"]
pub mod hfexppresc;
#[doc = "LFAPRESC0 (rw) register accessor: Low Frequency a Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfapresc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfapresc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfapresc0`]
module"]
#[doc(alias = "LFAPRESC0")]
pub type Lfapresc0 = crate::Reg<lfapresc0::LFAPRESC0rs>;
#[doc = "Low Frequency a Prescaler Register 0 (Async Reg)"]
pub mod lfapresc0;
#[doc = "LFBPRESC0 (rw) register accessor: Low Frequency B Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfbpresc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfbpresc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfbpresc0`]
module"]
#[doc(alias = "LFBPRESC0")]
pub type Lfbpresc0 = crate::Reg<lfbpresc0::LFBPRESC0rs>;
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)"]
pub mod lfbpresc0;
#[doc = "LFEPRESC0 (rw) register accessor: Low Frequency E Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfepresc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfepresc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfepresc0`]
module"]
#[doc(alias = "LFEPRESC0")]
pub type Lfepresc0 = crate::Reg<lfepresc0::LFEPRESC0rs>;
#[doc = "Low Frequency E Prescaler Register 0 (Async Reg)"]
pub mod lfepresc0;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SYNCBUSYrs>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "FREEZE (rw) register accessor: Freeze Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freeze::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freeze::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freeze`]
module"]
#[doc(alias = "FREEZE")]
pub type Freeze = crate::Reg<freeze::FREEZErs>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "PCNTCTRL (rw) register accessor: PCNT Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcntctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcntctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcntctrl`]
module"]
#[doc(alias = "PCNTCTRL")]
pub type Pcntctrl = crate::Reg<pcntctrl::PCNTCTRLrs>;
#[doc = "PCNT Control Register"]
pub mod pcntctrl;
#[doc = "ADCCTRL (rw) register accessor: ADC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcctrl`]
module"]
#[doc(alias = "ADCCTRL")]
pub type Adcctrl = crate::Reg<adcctrl::ADCCTRLrs>;
#[doc = "ADC Control Register"]
pub mod adcctrl;
#[doc = "ROUTEPEN (rw) register accessor: I/O Routing Pin Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`]
module"]
#[doc(alias = "ROUTEPEN")]
pub type Routepen = crate::Reg<routepen::ROUTEPENrs>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`]
module"]
#[doc(alias = "ROUTELOC0")]
pub type Routeloc0 = crate::Reg<routeloc0::ROUTELOC0rs>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LOCKrs>;
#[doc = "Configuration Lock Register"]
pub mod lock;
