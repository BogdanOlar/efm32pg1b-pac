#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: Ctrl,
    frame: Frame,
    trigctrl: Trigctrl,
    cmd: Cmd,
    status: Status,
    clkdiv: Clkdiv,
    rxdatax: Rxdatax,
    rxdata: Rxdata,
    rxdoublex: Rxdoublex,
    rxdouble: Rxdouble,
    rxdataxp: Rxdataxp,
    rxdoublexp: Rxdoublexp,
    txdatax: Txdatax,
    txdata: Txdata,
    txdoublex: Txdoublex,
    txdouble: Txdouble,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    irctrl: Irctrl,
    _reserved21: [u8; 0x04],
    input: Input,
    i2sctrl: I2sctrl,
    timing: Timing,
    ctrlx: Ctrlx,
    timecmp0: Timecmp0,
    timecmp1: Timecmp1,
    timecmp2: Timecmp2,
    routepen: Routepen,
    routeloc0: Routeloc0,
    routeloc1: Routeloc1,
}
impl RegisterBlock {
    ///0x00 - Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x04 - USART Frame Format Register
    #[inline(always)]
    pub const fn frame(&self) -> &Frame {
        &self.frame
    }
    ///0x08 - USART Trigger Control Register
    #[inline(always)]
    pub const fn trigctrl(&self) -> &Trigctrl {
        &self.trigctrl
    }
    ///0x0c - Command Register
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    ///0x10 - USART Status Register
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    ///0x14 - Clock Control Register
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    ///0x18 - RX Buffer Data Extended Register
    #[inline(always)]
    pub const fn rxdatax(&self) -> &Rxdatax {
        &self.rxdatax
    }
    ///0x1c - RX Buffer Data Register
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    ///0x20 - RX Buffer Double Data Extended Register
    #[inline(always)]
    pub const fn rxdoublex(&self) -> &Rxdoublex {
        &self.rxdoublex
    }
    ///0x24 - RX FIFO Double Data Register
    #[inline(always)]
    pub const fn rxdouble(&self) -> &Rxdouble {
        &self.rxdouble
    }
    ///0x28 - RX Buffer Data Extended Peek Register
    #[inline(always)]
    pub const fn rxdataxp(&self) -> &Rxdataxp {
        &self.rxdataxp
    }
    ///0x2c - RX Buffer Double Data Extended Peek Register
    #[inline(always)]
    pub const fn rxdoublexp(&self) -> &Rxdoublexp {
        &self.rxdoublexp
    }
    ///0x30 - TX Buffer Data Extended Register
    #[inline(always)]
    pub const fn txdatax(&self) -> &Txdatax {
        &self.txdatax
    }
    ///0x34 - TX Buffer Data Register
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    ///0x38 - TX Buffer Double Data Extended Register
    #[inline(always)]
    pub const fn txdoublex(&self) -> &Txdoublex {
        &self.txdoublex
    }
    ///0x3c - TX Buffer Double Data Register
    #[inline(always)]
    pub const fn txdouble(&self) -> &Txdouble {
        &self.txdouble
    }
    ///0x40 - Interrupt Flag Register
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    ///0x44 - Interrupt Flag Set Register
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    ///0x48 - Interrupt Flag Clear Register
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    ///0x4c - Interrupt Enable Register
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    ///0x50 - IrDA Control Register
    #[inline(always)]
    pub const fn irctrl(&self) -> &Irctrl {
        &self.irctrl
    }
    ///0x58 - USART Input Register
    #[inline(always)]
    pub const fn input(&self) -> &Input {
        &self.input
    }
    ///0x5c - I2S Control Register
    #[inline(always)]
    pub const fn i2sctrl(&self) -> &I2sctrl {
        &self.i2sctrl
    }
    ///0x60 - Timing Register
    #[inline(always)]
    pub const fn timing(&self) -> &Timing {
        &self.timing
    }
    ///0x64 - Control Register Extended
    #[inline(always)]
    pub const fn ctrlx(&self) -> &Ctrlx {
        &self.ctrlx
    }
    ///0x68 - Used to Generate Interrupts and Various Delays
    #[inline(always)]
    pub const fn timecmp0(&self) -> &Timecmp0 {
        &self.timecmp0
    }
    ///0x6c - Used to Generate Interrupts and Various Delays
    #[inline(always)]
    pub const fn timecmp1(&self) -> &Timecmp1 {
        &self.timecmp1
    }
    ///0x70 - Used to Generate Interrupts and Various Delays
    #[inline(always)]
    pub const fn timecmp2(&self) -> &Timecmp2 {
        &self.timecmp2
    }
    ///0x74 - I/O Routing Pin Enable Register
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    ///0x78 - I/O Routing Location Register
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
    ///0x7c - I/O Routing Location Register
    #[inline(always)]
    pub const fn routeloc1(&self) -> &Routeloc1 {
        &self.routeloc1
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
///FRAME (rw) register accessor: USART Frame Format Register
///
///You can [`read`](crate::Reg::read) this register and get [`frame::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@frame`]
///module
#[doc(alias = "FRAME")]
pub type Frame = crate::Reg<frame::FRAMErs>;
///USART Frame Format Register
pub mod frame;
///TRIGCTRL (rw) register accessor: USART Trigger Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`trigctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@trigctrl`]
///module
#[doc(alias = "TRIGCTRL")]
pub type Trigctrl = crate::Reg<trigctrl::TRIGCTRLrs>;
///USART Trigger Control Register
pub mod trigctrl;
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
///STATUS (r) register accessor: USART Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@status`]
///module
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::STATUSrs>;
///USART Status Register
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
///RXDATAX (r) register accessor: RX Buffer Data Extended Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdatax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
///
///For information about available fields see [`mod@rxdatax`]
///module
#[doc(alias = "RXDATAX")]
pub type Rxdatax = crate::Reg<rxdatax::RXDATAXrs>;
///RX Buffer Data Extended Register
pub mod rxdatax;
///RXDATA (r) register accessor: RX Buffer Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
///
///For information about available fields see [`mod@rxdata`]
///module
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RXDATArs>;
///RX Buffer Data Register
pub mod rxdata;
///RXDOUBLEX (r) register accessor: RX Buffer Double Data Extended Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdoublex::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
///
///For information about available fields see [`mod@rxdoublex`]
///module
#[doc(alias = "RXDOUBLEX")]
pub type Rxdoublex = crate::Reg<rxdoublex::RXDOUBLEXrs>;
///RX Buffer Double Data Extended Register
pub mod rxdoublex;
///RXDOUBLE (r) register accessor: RX FIFO Double Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdouble::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
///
///For information about available fields see [`mod@rxdouble`]
///module
#[doc(alias = "RXDOUBLE")]
pub type Rxdouble = crate::Reg<rxdouble::RXDOUBLErs>;
///RX FIFO Double Data Register
pub mod rxdouble;
///RXDATAXP (r) register accessor: RX Buffer Data Extended Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdataxp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rxdataxp`]
///module
#[doc(alias = "RXDATAXP")]
pub type Rxdataxp = crate::Reg<rxdataxp::RXDATAXPrs>;
///RX Buffer Data Extended Peek Register
pub mod rxdataxp;
///RXDOUBLEXP (r) register accessor: RX Buffer Double Data Extended Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdoublexp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rxdoublexp`]
///module
#[doc(alias = "RXDOUBLEXP")]
pub type Rxdoublexp = crate::Reg<rxdoublexp::RXDOUBLEXPrs>;
///RX Buffer Double Data Extended Peek Register
pub mod rxdoublexp;
///TXDATAX (rw) register accessor: TX Buffer Data Extended Register
///
///You can [`read`](crate::Reg::read) this register and get [`txdatax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdatax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@txdatax`]
///module
#[doc(alias = "TXDATAX")]
pub type Txdatax = crate::Reg<txdatax::TXDATAXrs>;
///TX Buffer Data Extended Register
pub mod txdatax;
///TXDATA (rw) register accessor: TX Buffer Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@txdata`]
///module
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TXDATArs>;
///TX Buffer Data Register
pub mod txdata;
///TXDOUBLEX (rw) register accessor: TX Buffer Double Data Extended Register
///
///You can [`read`](crate::Reg::read) this register and get [`txdoublex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdoublex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@txdoublex`]
///module
#[doc(alias = "TXDOUBLEX")]
pub type Txdoublex = crate::Reg<txdoublex::TXDOUBLEXrs>;
///TX Buffer Double Data Extended Register
pub mod txdoublex;
///TXDOUBLE (rw) register accessor: TX Buffer Double Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`txdouble::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdouble::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@txdouble`]
///module
#[doc(alias = "TXDOUBLE")]
pub type Txdouble = crate::Reg<txdouble::TXDOUBLErs>;
///TX Buffer Double Data Register
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
///IRCTRL (rw) register accessor: IrDA Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`irctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@irctrl`]
///module
#[doc(alias = "IRCTRL")]
pub type Irctrl = crate::Reg<irctrl::IRCTRLrs>;
///IrDA Control Register
pub mod irctrl;
///INPUT (rw) register accessor: USART Input Register
///
///You can [`read`](crate::Reg::read) this register and get [`input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@input`]
///module
#[doc(alias = "INPUT")]
pub type Input = crate::Reg<input::INPUTrs>;
///USART Input Register
pub mod input;
///I2SCTRL (rw) register accessor: I2S Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`i2sctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@i2sctrl`]
///module
#[doc(alias = "I2SCTRL")]
pub type I2sctrl = crate::Reg<i2sctrl::I2SCTRLrs>;
///I2S Control Register
pub mod i2sctrl;
///TIMING (rw) register accessor: Timing Register
///
///You can [`read`](crate::Reg::read) this register and get [`timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@timing`]
///module
#[doc(alias = "TIMING")]
pub type Timing = crate::Reg<timing::TIMINGrs>;
///Timing Register
pub mod timing;
///CTRLX (rw) register accessor: Control Register Extended
///
///You can [`read`](crate::Reg::read) this register and get [`ctrlx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ctrlx`]
///module
#[doc(alias = "CTRLX")]
pub type Ctrlx = crate::Reg<ctrlx::CTRLXrs>;
///Control Register Extended
pub mod ctrlx;
///TIMECMP0 (rw) register accessor: Used to Generate Interrupts and Various Delays
///
///You can [`read`](crate::Reg::read) this register and get [`timecmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timecmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@timecmp0`]
///module
#[doc(alias = "TIMECMP0")]
pub type Timecmp0 = crate::Reg<timecmp0::TIMECMP0rs>;
///Used to Generate Interrupts and Various Delays
pub mod timecmp0;
///TIMECMP1 (rw) register accessor: Used to Generate Interrupts and Various Delays
///
///You can [`read`](crate::Reg::read) this register and get [`timecmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timecmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@timecmp1`]
///module
#[doc(alias = "TIMECMP1")]
pub type Timecmp1 = crate::Reg<timecmp1::TIMECMP1rs>;
///Used to Generate Interrupts and Various Delays
pub mod timecmp1;
///TIMECMP2 (rw) register accessor: Used to Generate Interrupts and Various Delays
///
///You can [`read`](crate::Reg::read) this register and get [`timecmp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timecmp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@timecmp2`]
///module
#[doc(alias = "TIMECMP2")]
pub type Timecmp2 = crate::Reg<timecmp2::TIMECMP2rs>;
///Used to Generate Interrupts and Various Delays
pub mod timecmp2;
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
///ROUTELOC1 (rw) register accessor: I/O Routing Location Register
///
///You can [`read`](crate::Reg::read) this register and get [`routeloc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@routeloc1`]
///module
#[doc(alias = "ROUTELOC1")]
pub type Routeloc1 = crate::Reg<routeloc1::ROUTELOC1rs>;
///I/O Routing Location Register
pub mod routeloc1;
