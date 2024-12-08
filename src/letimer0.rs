#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: Ctrl,
    cmd: Cmd,
    status: Status,
    cnt: Cnt,
    comp0: Comp0,
    comp1: Comp1,
    rep0: Rep0,
    rep1: Rep1,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    _reserved12: [u8; 0x04],
    syncbusy: Syncbusy,
    _reserved13: [u8; 0x08],
    routepen: Routepen,
    routeloc0: Routeloc0,
    _reserved15: [u8; 0x08],
    prssel: Prssel,
}
impl RegisterBlock {
    ///0x00 - Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x04 - Command Register
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    ///0x08 - Status Register
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    ///0x0c - Counter Value Register
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    ///0x10 - Compare Value Register 0
    #[inline(always)]
    pub const fn comp0(&self) -> &Comp0 {
        &self.comp0
    }
    ///0x14 - Compare Value Register 1
    #[inline(always)]
    pub const fn comp1(&self) -> &Comp1 {
        &self.comp1
    }
    ///0x18 - Repeat Counter Register 0
    #[inline(always)]
    pub const fn rep0(&self) -> &Rep0 {
        &self.rep0
    }
    ///0x1c - Repeat Counter Register 1
    #[inline(always)]
    pub const fn rep1(&self) -> &Rep1 {
        &self.rep1
    }
    ///0x20 - Interrupt Flag Register
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    ///0x24 - Interrupt Flag Set Register
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    ///0x28 - Interrupt Flag Clear Register
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    ///0x2c - Interrupt Enable Register
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    ///0x34 - Synchronization Busy Register
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    ///0x40 - I/O Routing Pin Enable Register
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    ///0x44 - I/O Routing Location Register
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
    ///0x50 - PRS Input Select Register
    #[inline(always)]
    pub const fn prssel(&self) -> &Prssel {
        &self.prssel
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
///CMD (w) register accessor: Command Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cmd`]
///module
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CMDrs>;
///Command Register
pub mod cmd;
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
///CNT (rw) register accessor: Counter Value Register
///
///You can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cnt`]
///module
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CNTrs>;
///Counter Value Register
pub mod cnt;
///COMP0 (rw) register accessor: Compare Value Register 0
///
///You can [`read`](crate::Reg::read) this register and get [`comp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@comp0`]
///module
#[doc(alias = "COMP0")]
pub type Comp0 = crate::Reg<comp0::COMP0rs>;
///Compare Value Register 0
pub mod comp0;
///COMP1 (rw) register accessor: Compare Value Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`comp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@comp1`]
///module
#[doc(alias = "COMP1")]
pub type Comp1 = crate::Reg<comp1::COMP1rs>;
///Compare Value Register 1
pub mod comp1;
///REP0 (rw) register accessor: Repeat Counter Register 0
///
///You can [`read`](crate::Reg::read) this register and get [`rep0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rep0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rep0`]
///module
#[doc(alias = "REP0")]
pub type Rep0 = crate::Reg<rep0::REP0rs>;
///Repeat Counter Register 0
pub mod rep0;
///REP1 (rw) register accessor: Repeat Counter Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`rep1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rep1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rep1`]
///module
#[doc(alias = "REP1")]
pub type Rep1 = crate::Reg<rep1::REP1rs>;
///Repeat Counter Register 1
pub mod rep1;
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
///SYNCBUSY (r) register accessor: Synchronization Busy Register
///
///You can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@syncbusy`]
///module
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SYNCBUSYrs>;
///Synchronization Busy Register
pub mod syncbusy;
///ROUTEPEN (rw) register accessor: I/O Routing Pin Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`routepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@routepen`]
///module
#[doc(alias = "ROUTEPEN")]
pub type Routepen = crate::Reg<routepen::ROUTEPENrs>;
///I/O Routing Pin Enable Register
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
///PRSSEL (rw) register accessor: PRS Input Select Register
///
///You can [`read`](crate::Reg::read) this register and get [`prssel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prssel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@prssel`]
///module
#[doc(alias = "PRSSEL")]
pub type Prssel = crate::Reg<prssel::PRSSELrs>;
///PRS Input Select Register
pub mod prssel;
