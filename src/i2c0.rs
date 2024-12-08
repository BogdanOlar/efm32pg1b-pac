#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: Ctrl,
    cmd: Cmd,
    state: State,
    status: Status,
    clkdiv: Clkdiv,
    saddr: Saddr,
    saddrmask: Saddrmask,
    rxdata: Rxdata,
    rxdouble: Rxdouble,
    rxdatap: Rxdatap,
    rxdoublep: Rxdoublep,
    txdata: Txdata,
    txdouble: Txdouble,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    routepen: Routepen,
    routeloc0: Routeloc0,
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
    ///0x08 - State Register
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    ///0x0c - Status Register
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    ///0x10 - Clock Division Register
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    ///0x14 - Slave Address Register
    #[inline(always)]
    pub const fn saddr(&self) -> &Saddr {
        &self.saddr
    }
    ///0x18 - Slave Address Mask Register
    #[inline(always)]
    pub const fn saddrmask(&self) -> &Saddrmask {
        &self.saddrmask
    }
    ///0x1c - Receive Buffer Data Register
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    ///0x20 - Receive Buffer Double Data Register
    #[inline(always)]
    pub const fn rxdouble(&self) -> &Rxdouble {
        &self.rxdouble
    }
    ///0x24 - Receive Buffer Data Peek Register
    #[inline(always)]
    pub const fn rxdatap(&self) -> &Rxdatap {
        &self.rxdatap
    }
    ///0x28 - Receive Buffer Double Data Peek Register
    #[inline(always)]
    pub const fn rxdoublep(&self) -> &Rxdoublep {
        &self.rxdoublep
    }
    ///0x2c - Transmit Buffer Data Register
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    ///0x30 - Transmit Buffer Double Data Register
    #[inline(always)]
    pub const fn txdouble(&self) -> &Txdouble {
        &self.txdouble
    }
    ///0x34 - Interrupt Flag Register
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    ///0x38 - Interrupt Flag Set Register
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    ///0x3c - Interrupt Flag Clear Register
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    ///0x40 - Interrupt Enable Register
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    ///0x44 - I/O Routing Pin Enable Register
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    ///0x48 - I/O Routing Location Register
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
///STATE (r) register accessor: State Register
///
///You can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@state`]
///module
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::STATErs>;
///State Register
pub mod state;
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
///CLKDIV (rw) register accessor: Clock Division Register
///
///You can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@clkdiv`]
///module
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::CLKDIVrs>;
///Clock Division Register
pub mod clkdiv;
///SADDR (rw) register accessor: Slave Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@saddr`]
///module
#[doc(alias = "SADDR")]
pub type Saddr = crate::Reg<saddr::SADDRrs>;
///Slave Address Register
pub mod saddr;
///SADDRMASK (rw) register accessor: Slave Address Mask Register
///
///You can [`read`](crate::Reg::read) this register and get [`saddrmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddrmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@saddrmask`]
///module
#[doc(alias = "SADDRMASK")]
pub type Saddrmask = crate::Reg<saddrmask::SADDRMASKrs>;
///Slave Address Mask Register
pub mod saddrmask;
///RXDATA (r) register accessor: Receive Buffer Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
///
///For information about available fields see [`mod@rxdata`]
///module
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RXDATArs>;
///Receive Buffer Data Register
pub mod rxdata;
///RXDOUBLE (r) register accessor: Receive Buffer Double Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdouble::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
///
///For information about available fields see [`mod@rxdouble`]
///module
#[doc(alias = "RXDOUBLE")]
pub type Rxdouble = crate::Reg<rxdouble::RXDOUBLErs>;
///Receive Buffer Double Data Register
pub mod rxdouble;
///RXDATAP (r) register accessor: Receive Buffer Data Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdatap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rxdatap`]
///module
#[doc(alias = "RXDATAP")]
pub type Rxdatap = crate::Reg<rxdatap::RXDATAPrs>;
///Receive Buffer Data Peek Register
pub mod rxdatap;
///RXDOUBLEP (r) register accessor: Receive Buffer Double Data Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdoublep::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rxdoublep`]
///module
#[doc(alias = "RXDOUBLEP")]
pub type Rxdoublep = crate::Reg<rxdoublep::RXDOUBLEPrs>;
///Receive Buffer Double Data Peek Register
pub mod rxdoublep;
///TXDATA (rw) register accessor: Transmit Buffer Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@txdata`]
///module
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TXDATArs>;
///Transmit Buffer Data Register
pub mod txdata;
///TXDOUBLE (rw) register accessor: Transmit Buffer Double Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`txdouble::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdouble::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@txdouble`]
///module
#[doc(alias = "TXDOUBLE")]
pub type Txdouble = crate::Reg<txdouble::TXDOUBLErs>;
///Transmit Buffer Double Data Register
pub mod txdouble;
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
