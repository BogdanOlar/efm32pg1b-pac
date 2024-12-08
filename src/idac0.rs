#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: Ctrl,
    curprog: Curprog,
    _reserved2: [u8; 0x04],
    dutyconfig: Dutyconfig,
    _reserved3: [u8; 0x08],
    status: Status,
    _reserved4: [u8; 0x04],
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    _reserved8: [u8; 0x04],
    aportreq: Aportreq,
    aportconflict: Aportconflict,
}
impl RegisterBlock {
    ///0x00 - Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x04 - Current Programming Register
    #[inline(always)]
    pub const fn curprog(&self) -> &Curprog {
        &self.curprog
    }
    ///0x0c - Duty Cycle Configuration Register
    #[inline(always)]
    pub const fn dutyconfig(&self) -> &Dutyconfig {
        &self.dutyconfig
    }
    ///0x18 - Status Register
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
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
    ///0x34 - APORT Request Status Register
    #[inline(always)]
    pub const fn aportreq(&self) -> &Aportreq {
        &self.aportreq
    }
    ///0x38 - APORT Request Status Register
    #[inline(always)]
    pub const fn aportconflict(&self) -> &Aportconflict {
        &self.aportconflict
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
///CURPROG (rw) register accessor: Current Programming Register
///
///You can [`read`](crate::Reg::read) this register and get [`curprog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`curprog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@curprog`]
///module
#[doc(alias = "CURPROG")]
pub type Curprog = crate::Reg<curprog::CURPROGrs>;
///Current Programming Register
pub mod curprog;
///DUTYCONFIG (rw) register accessor: Duty Cycle Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`dutyconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dutyconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dutyconfig`]
///module
#[doc(alias = "DUTYCONFIG")]
pub type Dutyconfig = crate::Reg<dutyconfig::DUTYCONFIGrs>;
///Duty Cycle Configuration Register
pub mod dutyconfig;
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
///APORTCONFLICT (r) register accessor: APORT Request Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`aportconflict::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@aportconflict`]
///module
#[doc(alias = "APORTCONFLICT")]
pub type Aportconflict = crate::Reg<aportconflict::APORTCONFLICTrs>;
///APORT Request Status Register
pub mod aportconflict;
