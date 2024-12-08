#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: Ctrl,
    inputsel: Inputsel,
    status: Status,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    _reserved7: [u8; 0x04],
    aportreq: Aportreq,
    aportconflict: Aportconflict,
    hysteresis0: Hysteresis0,
    hysteresis1: Hysteresis1,
    _reserved11: [u8; 0x10],
    routepen: Routepen,
    routeloc0: Routeloc0,
}
impl RegisterBlock {
    ///0x00 - Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x04 - Input Selection Register
    #[inline(always)]
    pub const fn inputsel(&self) -> &Inputsel {
        &self.inputsel
    }
    ///0x08 - Status Register
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    ///0x0c - Interrupt Flag Register
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    ///0x10 - Interrupt Flag Set Register
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    ///0x14 - Interrupt Flag Clear Register
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    ///0x18 - Interrupt Enable Register
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    ///0x20 - APORT Request Status Register
    #[inline(always)]
    pub const fn aportreq(&self) -> &Aportreq {
        &self.aportreq
    }
    ///0x24 - APORT Conflict Status Register
    #[inline(always)]
    pub const fn aportconflict(&self) -> &Aportconflict {
        &self.aportconflict
    }
    ///0x28 - Hysteresis 0 Register
    #[inline(always)]
    pub const fn hysteresis0(&self) -> &Hysteresis0 {
        &self.hysteresis0
    }
    ///0x2c - Hysteresis 1 Register
    #[inline(always)]
    pub const fn hysteresis1(&self) -> &Hysteresis1 {
        &self.hysteresis1
    }
    ///0x40 - I/O Routing Pine Enable Register
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    ///0x44 - I/O Routing Location Register
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
}
///CTRL (rw) register accessor: Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ctrl`]
///module
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CTRLrs>;
///Control Register
pub mod ctrl;
///INPUTSEL (rw) register accessor: Input Selection Register
///
///You can [`read`](crate::Reg::read) this register and get [`inputsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@inputsel`]
///module
#[doc(alias = "INPUTSEL")]
pub type Inputsel = crate::Reg<inputsel::INPUTSELrs>;
///Input Selection Register
pub mod inputsel;
///STATUS (r) register accessor: Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@status`]
///module
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::STATUSrs>;
///Status Register
pub mod status;
///IF (r) register accessor: Interrupt Flag Register
///
///You can [`read`](crate::Reg::read) this register and get [`if_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@if_`]
///module
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IFrs>;
///Interrupt Flag Register
pub mod if_;
///IFS (w) register accessor: Interrupt Flag Set Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ifs`]
///module
#[doc(alias = "IFS")]
pub type Ifs = crate::Reg<ifs::IFSrs>;
///Interrupt Flag Set Register
pub mod ifs;
///IFC (w) register accessor: Interrupt Flag Clear Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ifc`]
///module
#[doc(alias = "IFC")]
pub type Ifc = crate::Reg<ifc::IFCrs>;
///Interrupt Flag Clear Register
pub mod ifc;
///IEN (rw) register accessor: Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ien`]
///module
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IENrs>;
///Interrupt Enable Register
pub mod ien;
///APORTREQ (r) register accessor: APORT Request Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`aportreq::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@aportreq`]
///module
#[doc(alias = "APORTREQ")]
pub type Aportreq = crate::Reg<aportreq::APORTREQrs>;
///APORT Request Status Register
pub mod aportreq;
///APORTCONFLICT (r) register accessor: APORT Conflict Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`aportconflict::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@aportconflict`]
///module
#[doc(alias = "APORTCONFLICT")]
pub type Aportconflict = crate::Reg<aportconflict::APORTCONFLICTrs>;
///APORT Conflict Status Register
pub mod aportconflict;
///HYSTERESIS0 (rw) register accessor: Hysteresis 0 Register
///
///You can [`read`](crate::Reg::read) this register and get [`hysteresis0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hysteresis0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hysteresis0`]
///module
#[doc(alias = "HYSTERESIS0")]
pub type Hysteresis0 = crate::Reg<hysteresis0::HYSTERESIS0rs>;
///Hysteresis 0 Register
pub mod hysteresis0;
///HYSTERESIS1 (rw) register accessor: Hysteresis 1 Register
///
///You can [`read`](crate::Reg::read) this register and get [`hysteresis1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hysteresis1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hysteresis1`]
///module
#[doc(alias = "HYSTERESIS1")]
pub type Hysteresis1 = crate::Reg<hysteresis1::HYSTERESIS1rs>;
///Hysteresis 1 Register
pub mod hysteresis1;
///ROUTEPEN (rw) register accessor: I/O Routing Pine Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`routepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@routepen`]
///module
#[doc(alias = "ROUTEPEN")]
pub type Routepen = crate::Reg<routepen::ROUTEPENrs>;
///I/O Routing Pine Enable Register
pub mod routepen;
///ROUTELOC0 (rw) register accessor: I/O Routing Location Register
///
///You can [`read`](crate::Reg::read) this register and get [`routeloc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@routeloc0`]
///module
#[doc(alias = "ROUTELOC0")]
pub type Routeloc0 = crate::Reg<routeloc0::ROUTELOC0rs>;
///I/O Routing Location Register
pub mod routeloc0;
