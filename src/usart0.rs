#[repr(C)]
#[doc = "Register block"]
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
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - USART Frame Format Register"]
    #[inline(always)]
    pub const fn frame(&self) -> &Frame {
        &self.frame
    }
    #[doc = "0x08 - USART Trigger Control Register"]
    #[inline(always)]
    pub const fn trigctrl(&self) -> &Trigctrl {
        &self.trigctrl
    }
    #[doc = "0x0c - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x10 - USART Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x14 - Clock Control Register"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x18 - RX Buffer Data Extended Register"]
    #[inline(always)]
    pub const fn rxdatax(&self) -> &Rxdatax {
        &self.rxdatax
    }
    #[doc = "0x1c - RX Buffer Data Register"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x20 - RX Buffer Double Data Extended Register"]
    #[inline(always)]
    pub const fn rxdoublex(&self) -> &Rxdoublex {
        &self.rxdoublex
    }
    #[doc = "0x24 - RX FIFO Double Data Register"]
    #[inline(always)]
    pub const fn rxdouble(&self) -> &Rxdouble {
        &self.rxdouble
    }
    #[doc = "0x28 - RX Buffer Data Extended Peek Register"]
    #[inline(always)]
    pub const fn rxdataxp(&self) -> &Rxdataxp {
        &self.rxdataxp
    }
    #[doc = "0x2c - RX Buffer Double Data Extended Peek Register"]
    #[inline(always)]
    pub const fn rxdoublexp(&self) -> &Rxdoublexp {
        &self.rxdoublexp
    }
    #[doc = "0x30 - TX Buffer Data Extended Register"]
    #[inline(always)]
    pub const fn txdatax(&self) -> &Txdatax {
        &self.txdatax
    }
    #[doc = "0x34 - TX Buffer Data Register"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x38 - TX Buffer Double Data Extended Register"]
    #[inline(always)]
    pub const fn txdoublex(&self) -> &Txdoublex {
        &self.txdoublex
    }
    #[doc = "0x3c - TX Buffer Double Data Register"]
    #[inline(always)]
    pub const fn txdouble(&self) -> &Txdouble {
        &self.txdouble
    }
    #[doc = "0x40 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x44 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x48 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x4c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x50 - IrDA Control Register"]
    #[inline(always)]
    pub const fn irctrl(&self) -> &Irctrl {
        &self.irctrl
    }
    #[doc = "0x58 - USART Input Register"]
    #[inline(always)]
    pub const fn input(&self) -> &Input {
        &self.input
    }
    #[doc = "0x5c - I2S Control Register"]
    #[inline(always)]
    pub const fn i2sctrl(&self) -> &I2sctrl {
        &self.i2sctrl
    }
    #[doc = "0x60 - Timing Register"]
    #[inline(always)]
    pub const fn timing(&self) -> &Timing {
        &self.timing
    }
    #[doc = "0x64 - Control Register Extended"]
    #[inline(always)]
    pub const fn ctrlx(&self) -> &Ctrlx {
        &self.ctrlx
    }
    #[doc = "0x68 - Used to Generate Interrupts and Various Delays"]
    #[inline(always)]
    pub const fn timecmp0(&self) -> &Timecmp0 {
        &self.timecmp0
    }
    #[doc = "0x6c - Used to Generate Interrupts and Various Delays"]
    #[inline(always)]
    pub const fn timecmp1(&self) -> &Timecmp1 {
        &self.timecmp1
    }
    #[doc = "0x70 - Used to Generate Interrupts and Various Delays"]
    #[inline(always)]
    pub const fn timecmp2(&self) -> &Timecmp2 {
        &self.timecmp2
    }
    #[doc = "0x74 - I/O Routing Pin Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    #[doc = "0x78 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
    #[doc = "0x7c - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc1(&self) -> &Routeloc1 {
        &self.routeloc1
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CTRLrs>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "FRAME (rw) register accessor: USART Frame Format Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frame::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frame::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame`]
module"]
#[doc(alias = "FRAME")]
pub type Frame = crate::Reg<frame::FRAMErs>;
#[doc = "USART Frame Format Register"]
pub mod frame;
#[doc = "TRIGCTRL (rw) register accessor: USART Trigger Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigctrl`]
module"]
#[doc(alias = "TRIGCTRL")]
pub type Trigctrl = crate::Reg<trigctrl::TRIGCTRLrs>;
#[doc = "USART Trigger Control Register"]
pub mod trigctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CMDrs>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: USART Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::STATUSrs>;
#[doc = "USART Status Register"]
pub mod status;
#[doc = "CLKDIV (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`]
module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::CLKDIVrs>;
#[doc = "Clock Control Register"]
pub mod clkdiv;
#[doc = "RXDATAX (r) register accessor: RX Buffer Data Extended Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdatax::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdatax`]
module"]
#[doc(alias = "RXDATAX")]
pub type Rxdatax = crate::Reg<rxdatax::RXDATAXrs>;
#[doc = "RX Buffer Data Extended Register"]
pub mod rxdatax;
#[doc = "RXDATA (r) register accessor: RX Buffer Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RXDATArs>;
#[doc = "RX Buffer Data Register"]
pub mod rxdata;
#[doc = "RXDOUBLEX (r) register accessor: RX Buffer Double Data Extended Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdoublex::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdoublex`]
module"]
#[doc(alias = "RXDOUBLEX")]
pub type Rxdoublex = crate::Reg<rxdoublex::RXDOUBLEXrs>;
#[doc = "RX Buffer Double Data Extended Register"]
pub mod rxdoublex;
#[doc = "RXDOUBLE (r) register accessor: RX FIFO Double Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdouble::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdouble`]
module"]
#[doc(alias = "RXDOUBLE")]
pub type Rxdouble = crate::Reg<rxdouble::RXDOUBLErs>;
#[doc = "RX FIFO Double Data Register"]
pub mod rxdouble;
#[doc = "RXDATAXP (r) register accessor: RX Buffer Data Extended Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdataxp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdataxp`]
module"]
#[doc(alias = "RXDATAXP")]
pub type Rxdataxp = crate::Reg<rxdataxp::RXDATAXPrs>;
#[doc = "RX Buffer Data Extended Peek Register"]
pub mod rxdataxp;
#[doc = "RXDOUBLEXP (r) register accessor: RX Buffer Double Data Extended Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdoublexp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdoublexp`]
module"]
#[doc(alias = "RXDOUBLEXP")]
pub type Rxdoublexp = crate::Reg<rxdoublexp::RXDOUBLEXPrs>;
#[doc = "RX Buffer Double Data Extended Peek Register"]
pub mod rxdoublexp;
#[doc = "TXDATAX (rw) register accessor: TX Buffer Data Extended Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdatax::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdatax::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdatax`]
module"]
#[doc(alias = "TXDATAX")]
pub type Txdatax = crate::Reg<txdatax::TXDATAXrs>;
#[doc = "TX Buffer Data Extended Register"]
pub mod txdatax;
#[doc = "TXDATA (rw) register accessor: TX Buffer Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TXDATArs>;
#[doc = "TX Buffer Data Register"]
pub mod txdata;
#[doc = "TXDOUBLEX (rw) register accessor: TX Buffer Double Data Extended Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdoublex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdoublex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdoublex`]
module"]
#[doc(alias = "TXDOUBLEX")]
pub type Txdoublex = crate::Reg<txdoublex::TXDOUBLEXrs>;
#[doc = "TX Buffer Double Data Extended Register"]
pub mod txdoublex;
#[doc = "TXDOUBLE (rw) register accessor: TX Buffer Double Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdouble::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdouble::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdouble`]
module"]
#[doc(alias = "TXDOUBLE")]
pub type Txdouble = crate::Reg<txdouble::TXDOUBLErs>;
#[doc = "TX Buffer Double Data Register"]
pub mod txdouble;
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
#[doc = "IRCTRL (rw) register accessor: IrDA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irctrl`]
module"]
#[doc(alias = "IRCTRL")]
pub type Irctrl = crate::Reg<irctrl::IRCTRLrs>;
#[doc = "IrDA Control Register"]
pub mod irctrl;
#[doc = "INPUT (rw) register accessor: USART Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input`]
module"]
#[doc(alias = "INPUT")]
pub type Input = crate::Reg<input::INPUTrs>;
#[doc = "USART Input Register"]
pub mod input;
#[doc = "I2SCTRL (rw) register accessor: I2S Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sctrl`]
module"]
#[doc(alias = "I2SCTRL")]
pub type I2sctrl = crate::Reg<i2sctrl::I2SCTRLrs>;
#[doc = "I2S Control Register"]
pub mod i2sctrl;
#[doc = "TIMING (rw) register accessor: Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timing::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timing::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing`]
module"]
#[doc(alias = "TIMING")]
pub type Timing = crate::Reg<timing::TIMINGrs>;
#[doc = "Timing Register"]
pub mod timing;
#[doc = "CTRLX (rw) register accessor: Control Register Extended\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlx`]
module"]
#[doc(alias = "CTRLX")]
pub type Ctrlx = crate::Reg<ctrlx::CTRLXrs>;
#[doc = "Control Register Extended"]
pub mod ctrlx;
#[doc = "TIMECMP0 (rw) register accessor: Used to Generate Interrupts and Various Delays\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timecmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timecmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timecmp0`]
module"]
#[doc(alias = "TIMECMP0")]
pub type Timecmp0 = crate::Reg<timecmp0::TIMECMP0rs>;
#[doc = "Used to Generate Interrupts and Various Delays"]
pub mod timecmp0;
#[doc = "TIMECMP1 (rw) register accessor: Used to Generate Interrupts and Various Delays\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timecmp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timecmp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timecmp1`]
module"]
#[doc(alias = "TIMECMP1")]
pub type Timecmp1 = crate::Reg<timecmp1::TIMECMP1rs>;
#[doc = "Used to Generate Interrupts and Various Delays"]
pub mod timecmp1;
#[doc = "TIMECMP2 (rw) register accessor: Used to Generate Interrupts and Various Delays\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timecmp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timecmp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timecmp2`]
module"]
#[doc(alias = "TIMECMP2")]
pub type Timecmp2 = crate::Reg<timecmp2::TIMECMP2rs>;
#[doc = "Used to Generate Interrupts and Various Delays"]
pub mod timecmp2;
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
#[doc = "ROUTELOC1 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc1`]
module"]
#[doc(alias = "ROUTELOC1")]
pub type Routeloc1 = crate::Reg<routeloc1::ROUTELOC1rs>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
