#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: Ctrl,
    cmd: Cmd,
    status: Status,
    clkdiv: Clkdiv,
    startframe: Startframe,
    sigframe: Sigframe,
    rxdatax: Rxdatax,
    rxdata: Rxdata,
    rxdataxp: Rxdataxp,
    txdatax: Txdatax,
    txdata: Txdata,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    pulsectrl: Pulsectrl,
    freeze: Freeze,
    syncbusy: Syncbusy,
    _reserved18: [u8; 0x0c],
    routepen: Routepen,
    routeloc0: Routeloc0,
    _reserved20: [u8; 0x08],
    input: Input,
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
    ///0x0c - Clock Control Register
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    ///0x10 - Start Frame Register
    #[inline(always)]
    pub const fn startframe(&self) -> &Startframe {
        &self.startframe
    }
    ///0x14 - Signal Frame Register
    #[inline(always)]
    pub const fn sigframe(&self) -> &Sigframe {
        &self.sigframe
    }
    ///0x18 - Receive Buffer Data Extended Register
    #[inline(always)]
    pub const fn rxdatax(&self) -> &Rxdatax {
        &self.rxdatax
    }
    ///0x1c - Receive Buffer Data Register
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    ///0x20 - Receive Buffer Data Extended Peek Register
    #[inline(always)]
    pub const fn rxdataxp(&self) -> &Rxdataxp {
        &self.rxdataxp
    }
    ///0x24 - Transmit Buffer Data Extended Register
    #[inline(always)]
    pub const fn txdatax(&self) -> &Txdatax {
        &self.txdatax
    }
    ///0x28 - Transmit Buffer Data Register
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    ///0x2c - Interrupt Flag Register
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    ///0x30 - Interrupt Flag Set Register
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    ///0x34 - Interrupt Flag Clear Register
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    ///0x38 - Interrupt Enable Register
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    ///0x3c - Pulse Control Register
    #[inline(always)]
    pub const fn pulsectrl(&self) -> &Pulsectrl {
        &self.pulsectrl
    }
    ///0x40 - Freeze Register
    #[inline(always)]
    pub const fn freeze(&self) -> &Freeze {
        &self.freeze
    }
    ///0x44 - Synchronization Busy Register
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    ///0x54 - I/O Routing Pin Enable Register
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    ///0x58 - I/O Routing Location Register
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
    ///0x64 - LEUART Input Register
    #[inline(always)]
    pub const fn input(&self) -> &Input {
        &self.input
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
///CLKDIV (rw) register accessor: Clock Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@clkdiv`]
///module
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::CLKDIVrs>;
///Clock Control Register
pub mod clkdiv;
///STARTFRAME (rw) register accessor: Start Frame Register
///
///You can [`read`](crate::Reg::read) this register and get [`startframe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`startframe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@startframe`]
///module
#[doc(alias = "STARTFRAME")]
pub type Startframe = crate::Reg<startframe::STARTFRAMErs>;
///Start Frame Register
pub mod startframe;
///SIGFRAME (rw) register accessor: Signal Frame Register
///
///You can [`read`](crate::Reg::read) this register and get [`sigframe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigframe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@sigframe`]
///module
#[doc(alias = "SIGFRAME")]
pub type Sigframe = crate::Reg<sigframe::SIGFRAMErs>;
///Signal Frame Register
pub mod sigframe;
///RXDATAX (r) register accessor: Receive Buffer Data Extended Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdatax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
///
///For information about available fields see [`mod@rxdatax`]
///module
#[doc(alias = "RXDATAX")]
pub type Rxdatax = crate::Reg<rxdatax::RXDATAXrs>;
///Receive Buffer Data Extended Register
pub mod rxdatax;
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
///RXDATAXP (r) register accessor: Receive Buffer Data Extended Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdataxp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rxdataxp`]
///module
#[doc(alias = "RXDATAXP")]
pub type Rxdataxp = crate::Reg<rxdataxp::RXDATAXPrs>;
///Receive Buffer Data Extended Peek Register
pub mod rxdataxp;
///TXDATAX (rw) register accessor: Transmit Buffer Data Extended Register
///
///You can [`read`](crate::Reg::read) this register and get [`txdatax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdatax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@txdatax`]
///module
#[doc(alias = "TXDATAX")]
pub type Txdatax = crate::Reg<txdatax::TXDATAXrs>;
///Transmit Buffer Data Extended Register
pub mod txdatax;
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
///PULSECTRL (rw) register accessor: Pulse Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`pulsectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pulsectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pulsectrl`]
///module
#[doc(alias = "PULSECTRL")]
pub type Pulsectrl = crate::Reg<pulsectrl::PULSECTRLrs>;
///Pulse Control Register
pub mod pulsectrl;
///FREEZE (rw) register accessor: Freeze Register
///
///You can [`read`](crate::Reg::read) this register and get [`freeze::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freeze::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@freeze`]
///module
#[doc(alias = "FREEZE")]
pub type Freeze = crate::Reg<freeze::FREEZErs>;
///Freeze Register
pub mod freeze;
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
///INPUT (rw) register accessor: LEUART Input Register
///
///You can [`read`](crate::Reg::read) this register and get [`input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@input`]
///module
#[doc(alias = "INPUT")]
pub type Input = crate::Reg<input::INPUTrs>;
///LEUART Input Register
pub mod input;
