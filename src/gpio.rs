#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pa_ctrl: PaCtrl,
    pa_model: PaModel,
    pa_modeh: PaModeh,
    pa_dout: PaDout,
    _reserved4: [u8; 0x08],
    pa_douttgl: PaDouttgl,
    pa_din: PaDin,
    pa_pinlockn: PaPinlockn,
    _reserved7: [u8; 0x04],
    pa_ovtdis: PaOvtdis,
    _reserved8: [u8; 0x04],
    pb_ctrl: PbCtrl,
    pb_model: PbModel,
    pb_modeh: PbModeh,
    pb_dout: PbDout,
    _reserved12: [u8; 0x08],
    pb_douttgl: PbDouttgl,
    pb_din: PbDin,
    pb_pinlockn: PbPinlockn,
    _reserved15: [u8; 0x04],
    pb_ovtdis: PbOvtdis,
    _reserved16: [u8; 0x04],
    pc_ctrl: PcCtrl,
    pc_model: PcModel,
    pc_modeh: PcModeh,
    pc_dout: PcDout,
    _reserved20: [u8; 0x08],
    pc_douttgl: PcDouttgl,
    pc_din: PcDin,
    pc_pinlockn: PcPinlockn,
    _reserved23: [u8; 0x04],
    pc_ovtdis: PcOvtdis,
    _reserved24: [u8; 0x04],
    pd_ctrl: PdCtrl,
    pd_model: PdModel,
    pd_modeh: PdModeh,
    pd_dout: PdDout,
    _reserved28: [u8; 0x08],
    pd_douttgl: PdDouttgl,
    pd_din: PdDin,
    pd_pinlockn: PdPinlockn,
    _reserved31: [u8; 0x04],
    pd_ovtdis: PdOvtdis,
    _reserved32: [u8; 0x04],
    pe_ctrl: PeCtrl,
    pe_model: PeModel,
    pe_modeh: PeModeh,
    pe_dout: PeDout,
    _reserved36: [u8; 0x08],
    pe_douttgl: PeDouttgl,
    pe_din: PeDin,
    pe_pinlockn: PePinlockn,
    _reserved39: [u8; 0x04],
    pe_ovtdis: PeOvtdis,
    _reserved40: [u8; 0x04],
    pf_ctrl: PfCtrl,
    pf_model: PfModel,
    pf_modeh: PfModeh,
    pf_dout: PfDout,
    _reserved44: [u8; 0x08],
    pf_douttgl: PfDouttgl,
    pf_din: PfDin,
    pf_pinlockn: PfPinlockn,
    _reserved47: [u8; 0x04],
    pf_ovtdis: PfOvtdis,
    _reserved48: [u8; 0x02e4],
    extipsell: Extipsell,
    extipselh: Extipselh,
    extipinsell: Extipinsell,
    extipinselh: Extipinselh,
    extirise: Extirise,
    extifall: Extifall,
    extilevel: Extilevel,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    em4wuen: Em4wuen,
    _reserved60: [u8; 0x10],
    routepen: Routepen,
    routeloc0: Routeloc0,
    _reserved62: [u8; 0x08],
    insense: Insense,
    lock: Lock,
}
impl RegisterBlock {
    #[doc = "0x00 - Port Control Register"]
    #[inline(always)]
    pub const fn pa_ctrl(&self) -> &PaCtrl {
        &self.pa_ctrl
    }
    #[doc = "0x04 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pa_model(&self) -> &PaModel {
        &self.pa_model
    }
    #[doc = "0x08 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pa_modeh(&self) -> &PaModeh {
        &self.pa_modeh
    }
    #[doc = "0x0c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pa_dout(&self) -> &PaDout {
        &self.pa_dout
    }
    #[doc = "0x18 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pa_douttgl(&self) -> &PaDouttgl {
        &self.pa_douttgl
    }
    #[doc = "0x1c - Port Data in Register"]
    #[inline(always)]
    pub const fn pa_din(&self) -> &PaDin {
        &self.pa_din
    }
    #[doc = "0x20 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pa_pinlockn(&self) -> &PaPinlockn {
        &self.pa_pinlockn
    }
    #[doc = "0x28 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pa_ovtdis(&self) -> &PaOvtdis {
        &self.pa_ovtdis
    }
    #[doc = "0x30 - Port Control Register"]
    #[inline(always)]
    pub const fn pb_ctrl(&self) -> &PbCtrl {
        &self.pb_ctrl
    }
    #[doc = "0x34 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pb_model(&self) -> &PbModel {
        &self.pb_model
    }
    #[doc = "0x38 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pb_modeh(&self) -> &PbModeh {
        &self.pb_modeh
    }
    #[doc = "0x3c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pb_dout(&self) -> &PbDout {
        &self.pb_dout
    }
    #[doc = "0x48 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pb_douttgl(&self) -> &PbDouttgl {
        &self.pb_douttgl
    }
    #[doc = "0x4c - Port Data in Register"]
    #[inline(always)]
    pub const fn pb_din(&self) -> &PbDin {
        &self.pb_din
    }
    #[doc = "0x50 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pb_pinlockn(&self) -> &PbPinlockn {
        &self.pb_pinlockn
    }
    #[doc = "0x58 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pb_ovtdis(&self) -> &PbOvtdis {
        &self.pb_ovtdis
    }
    #[doc = "0x60 - Port Control Register"]
    #[inline(always)]
    pub const fn pc_ctrl(&self) -> &PcCtrl {
        &self.pc_ctrl
    }
    #[doc = "0x64 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pc_model(&self) -> &PcModel {
        &self.pc_model
    }
    #[doc = "0x68 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pc_modeh(&self) -> &PcModeh {
        &self.pc_modeh
    }
    #[doc = "0x6c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pc_dout(&self) -> &PcDout {
        &self.pc_dout
    }
    #[doc = "0x78 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pc_douttgl(&self) -> &PcDouttgl {
        &self.pc_douttgl
    }
    #[doc = "0x7c - Port Data in Register"]
    #[inline(always)]
    pub const fn pc_din(&self) -> &PcDin {
        &self.pc_din
    }
    #[doc = "0x80 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pc_pinlockn(&self) -> &PcPinlockn {
        &self.pc_pinlockn
    }
    #[doc = "0x88 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pc_ovtdis(&self) -> &PcOvtdis {
        &self.pc_ovtdis
    }
    #[doc = "0x90 - Port Control Register"]
    #[inline(always)]
    pub const fn pd_ctrl(&self) -> &PdCtrl {
        &self.pd_ctrl
    }
    #[doc = "0x94 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pd_model(&self) -> &PdModel {
        &self.pd_model
    }
    #[doc = "0x98 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pd_modeh(&self) -> &PdModeh {
        &self.pd_modeh
    }
    #[doc = "0x9c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pd_dout(&self) -> &PdDout {
        &self.pd_dout
    }
    #[doc = "0xa8 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pd_douttgl(&self) -> &PdDouttgl {
        &self.pd_douttgl
    }
    #[doc = "0xac - Port Data in Register"]
    #[inline(always)]
    pub const fn pd_din(&self) -> &PdDin {
        &self.pd_din
    }
    #[doc = "0xb0 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pd_pinlockn(&self) -> &PdPinlockn {
        &self.pd_pinlockn
    }
    #[doc = "0xb8 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pd_ovtdis(&self) -> &PdOvtdis {
        &self.pd_ovtdis
    }
    #[doc = "0xc0 - Port Control Register"]
    #[inline(always)]
    pub const fn pe_ctrl(&self) -> &PeCtrl {
        &self.pe_ctrl
    }
    #[doc = "0xc4 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pe_model(&self) -> &PeModel {
        &self.pe_model
    }
    #[doc = "0xc8 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pe_modeh(&self) -> &PeModeh {
        &self.pe_modeh
    }
    #[doc = "0xcc - Port Data Out Register"]
    #[inline(always)]
    pub const fn pe_dout(&self) -> &PeDout {
        &self.pe_dout
    }
    #[doc = "0xd8 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pe_douttgl(&self) -> &PeDouttgl {
        &self.pe_douttgl
    }
    #[doc = "0xdc - Port Data in Register"]
    #[inline(always)]
    pub const fn pe_din(&self) -> &PeDin {
        &self.pe_din
    }
    #[doc = "0xe0 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pe_pinlockn(&self) -> &PePinlockn {
        &self.pe_pinlockn
    }
    #[doc = "0xe8 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pe_ovtdis(&self) -> &PeOvtdis {
        &self.pe_ovtdis
    }
    #[doc = "0xf0 - Port Control Register"]
    #[inline(always)]
    pub const fn pf_ctrl(&self) -> &PfCtrl {
        &self.pf_ctrl
    }
    #[doc = "0xf4 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pf_model(&self) -> &PfModel {
        &self.pf_model
    }
    #[doc = "0xf8 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pf_modeh(&self) -> &PfModeh {
        &self.pf_modeh
    }
    #[doc = "0xfc - Port Data Out Register"]
    #[inline(always)]
    pub const fn pf_dout(&self) -> &PfDout {
        &self.pf_dout
    }
    #[doc = "0x108 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pf_douttgl(&self) -> &PfDouttgl {
        &self.pf_douttgl
    }
    #[doc = "0x10c - Port Data in Register"]
    #[inline(always)]
    pub const fn pf_din(&self) -> &PfDin {
        &self.pf_din
    }
    #[doc = "0x110 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pf_pinlockn(&self) -> &PfPinlockn {
        &self.pf_pinlockn
    }
    #[doc = "0x118 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pf_ovtdis(&self) -> &PfOvtdis {
        &self.pf_ovtdis
    }
    #[doc = "0x400 - External Interrupt Port Select Low Register"]
    #[inline(always)]
    pub const fn extipsell(&self) -> &Extipsell {
        &self.extipsell
    }
    #[doc = "0x404 - External Interrupt Port Select High Register"]
    #[inline(always)]
    pub const fn extipselh(&self) -> &Extipselh {
        &self.extipselh
    }
    #[doc = "0x408 - External Interrupt Pin Select Low Register"]
    #[inline(always)]
    pub const fn extipinsell(&self) -> &Extipinsell {
        &self.extipinsell
    }
    #[doc = "0x40c - External Interrupt Pin Select High Register"]
    #[inline(always)]
    pub const fn extipinselh(&self) -> &Extipinselh {
        &self.extipinselh
    }
    #[doc = "0x410 - External Interrupt Rising Edge Trigger Register"]
    #[inline(always)]
    pub const fn extirise(&self) -> &Extirise {
        &self.extirise
    }
    #[doc = "0x414 - External Interrupt Falling Edge Trigger Register"]
    #[inline(always)]
    pub const fn extifall(&self) -> &Extifall {
        &self.extifall
    }
    #[doc = "0x418 - External Interrupt Level Register"]
    #[inline(always)]
    pub const fn extilevel(&self) -> &Extilevel {
        &self.extilevel
    }
    #[doc = "0x41c - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x420 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x424 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x428 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x42c - EM4 Wake Up Enable Register"]
    #[inline(always)]
    pub const fn em4wuen(&self) -> &Em4wuen {
        &self.em4wuen
    }
    #[doc = "0x440 - I/O Routing Pin Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    #[doc = "0x444 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
    #[doc = "0x450 - Input Sense Register"]
    #[inline(always)]
    pub const fn insense(&self) -> &Insense {
        &self.insense
    }
    #[doc = "0x454 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
}
#[doc = "PA_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_ctrl`]
module"]
#[doc(alias = "PA_CTRL")]
pub type PaCtrl = crate::Reg<pa_ctrl::PA_CTRLrs>;
#[doc = "Port Control Register"]
pub mod pa_ctrl;
#[doc = "PA_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_model`]
module"]
#[doc(alias = "PA_MODEL")]
pub type PaModel = crate::Reg<pa_model::PA_MODELrs>;
#[doc = "Port Pin Mode Low Register"]
pub mod pa_model;
#[doc = "PA_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_modeh`]
module"]
#[doc(alias = "PA_MODEH")]
pub type PaModeh = crate::Reg<pa_modeh::PA_MODEHrs>;
#[doc = "Port Pin Mode High Register"]
pub mod pa_modeh;
#[doc = "PA_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_dout`]
module"]
#[doc(alias = "PA_DOUT")]
pub type PaDout = crate::Reg<pa_dout::PA_DOUTrs>;
#[doc = "Port Data Out Register"]
pub mod pa_dout;
#[doc = "PA_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_douttgl`]
module"]
#[doc(alias = "PA_DOUTTGL")]
pub type PaDouttgl = crate::Reg<pa_douttgl::PA_DOUTTGLrs>;
#[doc = "Port Data Out Toggle Register"]
pub mod pa_douttgl;
#[doc = "PA_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_din`]
module"]
#[doc(alias = "PA_DIN")]
pub type PaDin = crate::Reg<pa_din::PA_DINrs>;
#[doc = "Port Data in Register"]
pub mod pa_din;
#[doc = "PA_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_pinlockn`]
module"]
#[doc(alias = "PA_PINLOCKN")]
pub type PaPinlockn = crate::Reg<pa_pinlockn::PA_PINLOCKNrs>;
#[doc = "Port Unlocked Pins Register"]
pub mod pa_pinlockn;
#[doc = "PA_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_ovtdis`]
module"]
#[doc(alias = "PA_OVTDIS")]
pub type PaOvtdis = crate::Reg<pa_ovtdis::PA_OVTDISrs>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pa_ovtdis;
#[doc = "PB_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_ctrl`]
module"]
#[doc(alias = "PB_CTRL")]
pub type PbCtrl = crate::Reg<pb_ctrl::PB_CTRLrs>;
#[doc = "Port Control Register"]
pub mod pb_ctrl;
#[doc = "PB_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_model`]
module"]
#[doc(alias = "PB_MODEL")]
pub type PbModel = crate::Reg<pb_model::PB_MODELrs>;
#[doc = "Port Pin Mode Low Register"]
pub mod pb_model;
#[doc = "PB_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_modeh`]
module"]
#[doc(alias = "PB_MODEH")]
pub type PbModeh = crate::Reg<pb_modeh::PB_MODEHrs>;
#[doc = "Port Pin Mode High Register"]
pub mod pb_modeh;
#[doc = "PB_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_dout`]
module"]
#[doc(alias = "PB_DOUT")]
pub type PbDout = crate::Reg<pb_dout::PB_DOUTrs>;
#[doc = "Port Data Out Register"]
pub mod pb_dout;
#[doc = "PB_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_douttgl`]
module"]
#[doc(alias = "PB_DOUTTGL")]
pub type PbDouttgl = crate::Reg<pb_douttgl::PB_DOUTTGLrs>;
#[doc = "Port Data Out Toggle Register"]
pub mod pb_douttgl;
#[doc = "PB_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_din`]
module"]
#[doc(alias = "PB_DIN")]
pub type PbDin = crate::Reg<pb_din::PB_DINrs>;
#[doc = "Port Data in Register"]
pub mod pb_din;
#[doc = "PB_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_pinlockn`]
module"]
#[doc(alias = "PB_PINLOCKN")]
pub type PbPinlockn = crate::Reg<pb_pinlockn::PB_PINLOCKNrs>;
#[doc = "Port Unlocked Pins Register"]
pub mod pb_pinlockn;
#[doc = "PB_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_ovtdis`]
module"]
#[doc(alias = "PB_OVTDIS")]
pub type PbOvtdis = crate::Reg<pb_ovtdis::PB_OVTDISrs>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pb_ovtdis;
#[doc = "PC_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_ctrl`]
module"]
#[doc(alias = "PC_CTRL")]
pub type PcCtrl = crate::Reg<pc_ctrl::PC_CTRLrs>;
#[doc = "Port Control Register"]
pub mod pc_ctrl;
#[doc = "PC_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_model`]
module"]
#[doc(alias = "PC_MODEL")]
pub type PcModel = crate::Reg<pc_model::PC_MODELrs>;
#[doc = "Port Pin Mode Low Register"]
pub mod pc_model;
#[doc = "PC_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_modeh`]
module"]
#[doc(alias = "PC_MODEH")]
pub type PcModeh = crate::Reg<pc_modeh::PC_MODEHrs>;
#[doc = "Port Pin Mode High Register"]
pub mod pc_modeh;
#[doc = "PC_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_dout`]
module"]
#[doc(alias = "PC_DOUT")]
pub type PcDout = crate::Reg<pc_dout::PC_DOUTrs>;
#[doc = "Port Data Out Register"]
pub mod pc_dout;
#[doc = "PC_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_douttgl`]
module"]
#[doc(alias = "PC_DOUTTGL")]
pub type PcDouttgl = crate::Reg<pc_douttgl::PC_DOUTTGLrs>;
#[doc = "Port Data Out Toggle Register"]
pub mod pc_douttgl;
#[doc = "PC_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_din`]
module"]
#[doc(alias = "PC_DIN")]
pub type PcDin = crate::Reg<pc_din::PC_DINrs>;
#[doc = "Port Data in Register"]
pub mod pc_din;
#[doc = "PC_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_pinlockn`]
module"]
#[doc(alias = "PC_PINLOCKN")]
pub type PcPinlockn = crate::Reg<pc_pinlockn::PC_PINLOCKNrs>;
#[doc = "Port Unlocked Pins Register"]
pub mod pc_pinlockn;
#[doc = "PC_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_ovtdis`]
module"]
#[doc(alias = "PC_OVTDIS")]
pub type PcOvtdis = crate::Reg<pc_ovtdis::PC_OVTDISrs>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pc_ovtdis;
#[doc = "PD_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_ctrl`]
module"]
#[doc(alias = "PD_CTRL")]
pub type PdCtrl = crate::Reg<pd_ctrl::PD_CTRLrs>;
#[doc = "Port Control Register"]
pub mod pd_ctrl;
#[doc = "PD_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_model`]
module"]
#[doc(alias = "PD_MODEL")]
pub type PdModel = crate::Reg<pd_model::PD_MODELrs>;
#[doc = "Port Pin Mode Low Register"]
pub mod pd_model;
#[doc = "PD_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_modeh`]
module"]
#[doc(alias = "PD_MODEH")]
pub type PdModeh = crate::Reg<pd_modeh::PD_MODEHrs>;
#[doc = "Port Pin Mode High Register"]
pub mod pd_modeh;
#[doc = "PD_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_dout`]
module"]
#[doc(alias = "PD_DOUT")]
pub type PdDout = crate::Reg<pd_dout::PD_DOUTrs>;
#[doc = "Port Data Out Register"]
pub mod pd_dout;
#[doc = "PD_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_douttgl`]
module"]
#[doc(alias = "PD_DOUTTGL")]
pub type PdDouttgl = crate::Reg<pd_douttgl::PD_DOUTTGLrs>;
#[doc = "Port Data Out Toggle Register"]
pub mod pd_douttgl;
#[doc = "PD_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_din`]
module"]
#[doc(alias = "PD_DIN")]
pub type PdDin = crate::Reg<pd_din::PD_DINrs>;
#[doc = "Port Data in Register"]
pub mod pd_din;
#[doc = "PD_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_pinlockn`]
module"]
#[doc(alias = "PD_PINLOCKN")]
pub type PdPinlockn = crate::Reg<pd_pinlockn::PD_PINLOCKNrs>;
#[doc = "Port Unlocked Pins Register"]
pub mod pd_pinlockn;
#[doc = "PD_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_ovtdis`]
module"]
#[doc(alias = "PD_OVTDIS")]
pub type PdOvtdis = crate::Reg<pd_ovtdis::PD_OVTDISrs>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pd_ovtdis;
#[doc = "PE_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_ctrl`]
module"]
#[doc(alias = "PE_CTRL")]
pub type PeCtrl = crate::Reg<pe_ctrl::PE_CTRLrs>;
#[doc = "Port Control Register"]
pub mod pe_ctrl;
#[doc = "PE_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_model`]
module"]
#[doc(alias = "PE_MODEL")]
pub type PeModel = crate::Reg<pe_model::PE_MODELrs>;
#[doc = "Port Pin Mode Low Register"]
pub mod pe_model;
#[doc = "PE_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_modeh`]
module"]
#[doc(alias = "PE_MODEH")]
pub type PeModeh = crate::Reg<pe_modeh::PE_MODEHrs>;
#[doc = "Port Pin Mode High Register"]
pub mod pe_modeh;
#[doc = "PE_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_dout`]
module"]
#[doc(alias = "PE_DOUT")]
pub type PeDout = crate::Reg<pe_dout::PE_DOUTrs>;
#[doc = "Port Data Out Register"]
pub mod pe_dout;
#[doc = "PE_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_douttgl`]
module"]
#[doc(alias = "PE_DOUTTGL")]
pub type PeDouttgl = crate::Reg<pe_douttgl::PE_DOUTTGLrs>;
#[doc = "Port Data Out Toggle Register"]
pub mod pe_douttgl;
#[doc = "PE_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_din`]
module"]
#[doc(alias = "PE_DIN")]
pub type PeDin = crate::Reg<pe_din::PE_DINrs>;
#[doc = "Port Data in Register"]
pub mod pe_din;
#[doc = "PE_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_pinlockn`]
module"]
#[doc(alias = "PE_PINLOCKN")]
pub type PePinlockn = crate::Reg<pe_pinlockn::PE_PINLOCKNrs>;
#[doc = "Port Unlocked Pins Register"]
pub mod pe_pinlockn;
#[doc = "PE_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_ovtdis`]
module"]
#[doc(alias = "PE_OVTDIS")]
pub type PeOvtdis = crate::Reg<pe_ovtdis::PE_OVTDISrs>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pe_ovtdis;
#[doc = "PF_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_ctrl`]
module"]
#[doc(alias = "PF_CTRL")]
pub type PfCtrl = crate::Reg<pf_ctrl::PF_CTRLrs>;
#[doc = "Port Control Register"]
pub mod pf_ctrl;
#[doc = "PF_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_model`]
module"]
#[doc(alias = "PF_MODEL")]
pub type PfModel = crate::Reg<pf_model::PF_MODELrs>;
#[doc = "Port Pin Mode Low Register"]
pub mod pf_model;
#[doc = "PF_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_modeh`]
module"]
#[doc(alias = "PF_MODEH")]
pub type PfModeh = crate::Reg<pf_modeh::PF_MODEHrs>;
#[doc = "Port Pin Mode High Register"]
pub mod pf_modeh;
#[doc = "PF_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_dout`]
module"]
#[doc(alias = "PF_DOUT")]
pub type PfDout = crate::Reg<pf_dout::PF_DOUTrs>;
#[doc = "Port Data Out Register"]
pub mod pf_dout;
#[doc = "PF_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_douttgl`]
module"]
#[doc(alias = "PF_DOUTTGL")]
pub type PfDouttgl = crate::Reg<pf_douttgl::PF_DOUTTGLrs>;
#[doc = "Port Data Out Toggle Register"]
pub mod pf_douttgl;
#[doc = "PF_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_din`]
module"]
#[doc(alias = "PF_DIN")]
pub type PfDin = crate::Reg<pf_din::PF_DINrs>;
#[doc = "Port Data in Register"]
pub mod pf_din;
#[doc = "PF_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_pinlockn`]
module"]
#[doc(alias = "PF_PINLOCKN")]
pub type PfPinlockn = crate::Reg<pf_pinlockn::PF_PINLOCKNrs>;
#[doc = "Port Unlocked Pins Register"]
pub mod pf_pinlockn;
#[doc = "PF_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_ovtdis`]
module"]
#[doc(alias = "PF_OVTDIS")]
pub type PfOvtdis = crate::Reg<pf_ovtdis::PF_OVTDISrs>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pf_ovtdis;
#[doc = "EXTIPSELL (rw) register accessor: External Interrupt Port Select Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extipsell::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extipsell::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipsell`]
module"]
#[doc(alias = "EXTIPSELL")]
pub type Extipsell = crate::Reg<extipsell::EXTIPSELLrs>;
#[doc = "External Interrupt Port Select Low Register"]
pub mod extipsell;
#[doc = "EXTIPSELH (rw) register accessor: External Interrupt Port Select High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extipselh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extipselh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipselh`]
module"]
#[doc(alias = "EXTIPSELH")]
pub type Extipselh = crate::Reg<extipselh::EXTIPSELHrs>;
#[doc = "External Interrupt Port Select High Register"]
pub mod extipselh;
#[doc = "EXTIPINSELL (rw) register accessor: External Interrupt Pin Select Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extipinsell::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extipinsell::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipinsell`]
module"]
#[doc(alias = "EXTIPINSELL")]
pub type Extipinsell = crate::Reg<extipinsell::EXTIPINSELLrs>;
#[doc = "External Interrupt Pin Select Low Register"]
pub mod extipinsell;
#[doc = "EXTIPINSELH (rw) register accessor: External Interrupt Pin Select High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extipinselh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extipinselh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipinselh`]
module"]
#[doc(alias = "EXTIPINSELH")]
pub type Extipinselh = crate::Reg<extipinselh::EXTIPINSELHrs>;
#[doc = "External Interrupt Pin Select High Register"]
pub mod extipinselh;
#[doc = "EXTIRISE (rw) register accessor: External Interrupt Rising Edge Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extirise::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extirise::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extirise`]
module"]
#[doc(alias = "EXTIRISE")]
pub type Extirise = crate::Reg<extirise::EXTIRISErs>;
#[doc = "External Interrupt Rising Edge Trigger Register"]
pub mod extirise;
#[doc = "EXTIFALL (rw) register accessor: External Interrupt Falling Edge Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extifall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extifall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extifall`]
module"]
#[doc(alias = "EXTIFALL")]
pub type Extifall = crate::Reg<extifall::EXTIFALLrs>;
#[doc = "External Interrupt Falling Edge Trigger Register"]
pub mod extifall;
#[doc = "EXTILEVEL (rw) register accessor: External Interrupt Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extilevel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extilevel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extilevel`]
module"]
#[doc(alias = "EXTILEVEL")]
pub type Extilevel = crate::Reg<extilevel::EXTILEVELrs>;
#[doc = "External Interrupt Level Register"]
pub mod extilevel;
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
#[doc = "EM4WUEN (rw) register accessor: EM4 Wake Up Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em4wuen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em4wuen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wuen`]
module"]
#[doc(alias = "EM4WUEN")]
pub type Em4wuen = crate::Reg<em4wuen::EM4WUENrs>;
#[doc = "EM4 Wake Up Enable Register"]
pub mod em4wuen;
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
#[doc = "INSENSE (rw) register accessor: Input Sense Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`insense::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`insense::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@insense`]
module"]
#[doc(alias = "INSENSE")]
pub type Insense = crate::Reg<insense::INSENSErs>;
#[doc = "Input Sense Register"]
pub mod insense;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LOCKrs>;
#[doc = "Configuration Lock Register"]
pub mod lock;
