#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x04],
    cmd: Cmd,
    status: Status,
    singlectrl: Singlectrl,
    singlectrlx: Singlectrlx,
    scanctrl: Scanctrl,
    scanctrlx: Scanctrlx,
    scanmask: Scanmask,
    scaninputsel: Scaninputsel,
    scannegsel: Scannegsel,
    cmpthr: Cmpthr,
    biasprog: Biasprog,
    cal: Cal,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    singledata: Singledata,
    scandata: Scandata,
    singledatap: Singledatap,
    scandatap: Scandatap,
    _reserved21: [u8; 0x10],
    scandatax: Scandatax,
    scandataxp: Scandataxp,
    _reserved23: [u8; 0x0c],
    aportreq: Aportreq,
    aportconflict: Aportconflict,
    singlefifocount: Singlefifocount,
    scanfifocount: Scanfifocount,
    singlefifoclear: Singlefifoclear,
    scanfifoclear: Scanfifoclear,
    aportmasterdis: Aportmasterdis,
}
impl RegisterBlock {
    ///0x00 - Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x08 - Command Register
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    ///0x0c - Status Register
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    ///0x10 - Single Channel Control Register
    #[inline(always)]
    pub const fn singlectrl(&self) -> &Singlectrl {
        &self.singlectrl
    }
    ///0x14 - Single Channel Control Register Continued
    #[inline(always)]
    pub const fn singlectrlx(&self) -> &Singlectrlx {
        &self.singlectrlx
    }
    ///0x18 - Scan Control Register
    #[inline(always)]
    pub const fn scanctrl(&self) -> &Scanctrl {
        &self.scanctrl
    }
    ///0x1c - Scan Control Register Continued
    #[inline(always)]
    pub const fn scanctrlx(&self) -> &Scanctrlx {
        &self.scanctrlx
    }
    ///0x20 - Scan Sequence Input Mask Register
    #[inline(always)]
    pub const fn scanmask(&self) -> &Scanmask {
        &self.scanmask
    }
    ///0x24 - Input Selection Register for Scan Mode
    #[inline(always)]
    pub const fn scaninputsel(&self) -> &Scaninputsel {
        &self.scaninputsel
    }
    ///0x28 - Negative Input Select Register for Scan
    #[inline(always)]
    pub const fn scannegsel(&self) -> &Scannegsel {
        &self.scannegsel
    }
    ///0x2c - Compare Threshold Register
    #[inline(always)]
    pub const fn cmpthr(&self) -> &Cmpthr {
        &self.cmpthr
    }
    ///0x30 - Bias Programming Register for Various Analog Blocks Used in ADC Operation
    #[inline(always)]
    pub const fn biasprog(&self) -> &Biasprog {
        &self.biasprog
    }
    ///0x34 - Calibration Register
    #[inline(always)]
    pub const fn cal(&self) -> &Cal {
        &self.cal
    }
    ///0x38 - Interrupt Flag Register
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    ///0x3c - Interrupt Flag Set Register
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    ///0x40 - Interrupt Flag Clear Register
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    ///0x44 - Interrupt Enable Register
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    ///0x48 - Single Conversion Result Data
    #[inline(always)]
    pub const fn singledata(&self) -> &Singledata {
        &self.singledata
    }
    ///0x4c - Scan Conversion Result Data
    #[inline(always)]
    pub const fn scandata(&self) -> &Scandata {
        &self.scandata
    }
    ///0x50 - Single Conversion Result Data Peek Register
    #[inline(always)]
    pub const fn singledatap(&self) -> &Singledatap {
        &self.singledatap
    }
    ///0x54 - Scan Sequence Result Data Peek Register
    #[inline(always)]
    pub const fn scandatap(&self) -> &Scandatap {
        &self.scandatap
    }
    ///0x68 - Scan Sequence Result Data + Data Source Register
    #[inline(always)]
    pub const fn scandatax(&self) -> &Scandatax {
        &self.scandatax
    }
    ///0x6c - Scan Sequence Result Data + Data Source Peek Register
    #[inline(always)]
    pub const fn scandataxp(&self) -> &Scandataxp {
        &self.scandataxp
    }
    ///0x7c - APORT Request Status Register
    #[inline(always)]
    pub const fn aportreq(&self) -> &Aportreq {
        &self.aportreq
    }
    ///0x80 - APORT Conflict Status Register
    #[inline(always)]
    pub const fn aportconflict(&self) -> &Aportconflict {
        &self.aportconflict
    }
    ///0x84 - Single FIFO Count Register
    #[inline(always)]
    pub const fn singlefifocount(&self) -> &Singlefifocount {
        &self.singlefifocount
    }
    ///0x88 - Scan FIFO Count Register
    #[inline(always)]
    pub const fn scanfifocount(&self) -> &Scanfifocount {
        &self.scanfifocount
    }
    ///0x8c - Single FIFO Clear Register
    #[inline(always)]
    pub const fn singlefifoclear(&self) -> &Singlefifoclear {
        &self.singlefifoclear
    }
    ///0x90 - Scan FIFO Clear Register
    #[inline(always)]
    pub const fn scanfifoclear(&self) -> &Scanfifoclear {
        &self.scanfifoclear
    }
    ///0x94 - APORT Bus Master Disable Register
    #[inline(always)]
    pub const fn aportmasterdis(&self) -> &Aportmasterdis {
        &self.aportmasterdis
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
///SINGLECTRL (rw) register accessor: Single Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`singlectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@singlectrl`]
///module
#[doc(alias = "SINGLECTRL")]
pub type Singlectrl = crate::Reg<singlectrl::SINGLECTRLrs>;
///Single Channel Control Register
pub mod singlectrl;
///SINGLECTRLX (rw) register accessor: Single Channel Control Register Continued
///
///You can [`read`](crate::Reg::read) this register and get [`singlectrlx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlectrlx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@singlectrlx`]
///module
#[doc(alias = "SINGLECTRLX")]
pub type Singlectrlx = crate::Reg<singlectrlx::SINGLECTRLXrs>;
///Single Channel Control Register Continued
pub mod singlectrlx;
///SCANCTRL (rw) register accessor: Scan Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`scanctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@scanctrl`]
///module
#[doc(alias = "SCANCTRL")]
pub type Scanctrl = crate::Reg<scanctrl::SCANCTRLrs>;
///Scan Control Register
pub mod scanctrl;
///SCANCTRLX (rw) register accessor: Scan Control Register Continued
///
///You can [`read`](crate::Reg::read) this register and get [`scanctrlx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanctrlx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@scanctrlx`]
///module
#[doc(alias = "SCANCTRLX")]
pub type Scanctrlx = crate::Reg<scanctrlx::SCANCTRLXrs>;
///Scan Control Register Continued
pub mod scanctrlx;
///SCANMASK (rw) register accessor: Scan Sequence Input Mask Register
///
///You can [`read`](crate::Reg::read) this register and get [`scanmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@scanmask`]
///module
#[doc(alias = "SCANMASK")]
pub type Scanmask = crate::Reg<scanmask::SCANMASKrs>;
///Scan Sequence Input Mask Register
pub mod scanmask;
///SCANINPUTSEL (rw) register accessor: Input Selection Register for Scan Mode
///
///You can [`read`](crate::Reg::read) this register and get [`scaninputsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scaninputsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@scaninputsel`]
///module
#[doc(alias = "SCANINPUTSEL")]
pub type Scaninputsel = crate::Reg<scaninputsel::SCANINPUTSELrs>;
///Input Selection Register for Scan Mode
pub mod scaninputsel;
///SCANNEGSEL (rw) register accessor: Negative Input Select Register for Scan
///
///You can [`read`](crate::Reg::read) this register and get [`scannegsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scannegsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@scannegsel`]
///module
#[doc(alias = "SCANNEGSEL")]
pub type Scannegsel = crate::Reg<scannegsel::SCANNEGSELrs>;
///Negative Input Select Register for Scan
pub mod scannegsel;
///CMPTHR (rw) register accessor: Compare Threshold Register
///
///You can [`read`](crate::Reg::read) this register and get [`cmpthr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cmpthr`]
///module
#[doc(alias = "CMPTHR")]
pub type Cmpthr = crate::Reg<cmpthr::CMPTHRrs>;
///Compare Threshold Register
pub mod cmpthr;
///BIASPROG (rw) register accessor: Bias Programming Register for Various Analog Blocks Used in ADC Operation
///
///You can [`read`](crate::Reg::read) this register and get [`biasprog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biasprog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@biasprog`]
///module
#[doc(alias = "BIASPROG")]
pub type Biasprog = crate::Reg<biasprog::BIASPROGrs>;
///Bias Programming Register for Various Analog Blocks Used in ADC Operation
pub mod biasprog;
///CAL (rw) register accessor: Calibration Register
///
///You can [`read`](crate::Reg::read) this register and get [`cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cal`]
///module
#[doc(alias = "CAL")]
pub type Cal = crate::Reg<cal::CALrs>;
///Calibration Register
pub mod cal;
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
///SINGLEDATA (r) register accessor: Single Conversion Result Data
///
///You can [`read`](crate::Reg::read) this register and get [`singledata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
///
///For information about available fields see [`mod@singledata`]
///module
#[doc(alias = "SINGLEDATA")]
pub type Singledata = crate::Reg<singledata::SINGLEDATArs>;
///Single Conversion Result Data
pub mod singledata;
///SCANDATA (r) register accessor: Scan Conversion Result Data
///
///You can [`read`](crate::Reg::read) this register and get [`scandata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
///
///For information about available fields see [`mod@scandata`]
///module
#[doc(alias = "SCANDATA")]
pub type Scandata = crate::Reg<scandata::SCANDATArs>;
///Scan Conversion Result Data
pub mod scandata;
///SINGLEDATAP (r) register accessor: Single Conversion Result Data Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`singledatap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@singledatap`]
///module
#[doc(alias = "SINGLEDATAP")]
pub type Singledatap = crate::Reg<singledatap::SINGLEDATAPrs>;
///Single Conversion Result Data Peek Register
pub mod singledatap;
///SCANDATAP (r) register accessor: Scan Sequence Result Data Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`scandatap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@scandatap`]
///module
#[doc(alias = "SCANDATAP")]
pub type Scandatap = crate::Reg<scandatap::SCANDATAPrs>;
///Scan Sequence Result Data Peek Register
pub mod scandatap;
///SCANDATAX (r) register accessor: Scan Sequence Result Data + Data Source Register
///
///You can [`read`](crate::Reg::read) this register and get [`scandatax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
///
///For information about available fields see [`mod@scandatax`]
///module
#[doc(alias = "SCANDATAX")]
pub type Scandatax = crate::Reg<scandatax::SCANDATAXrs>;
///Scan Sequence Result Data + Data Source Register
pub mod scandatax;
///SCANDATAXP (r) register accessor: Scan Sequence Result Data + Data Source Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`scandataxp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@scandataxp`]
///module
#[doc(alias = "SCANDATAXP")]
pub type Scandataxp = crate::Reg<scandataxp::SCANDATAXPrs>;
///Scan Sequence Result Data + Data Source Peek Register
pub mod scandataxp;
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
///SINGLEFIFOCOUNT (r) register accessor: Single FIFO Count Register
///
///You can [`read`](crate::Reg::read) this register and get [`singlefifocount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@singlefifocount`]
///module
#[doc(alias = "SINGLEFIFOCOUNT")]
pub type Singlefifocount = crate::Reg<singlefifocount::SINGLEFIFOCOUNTrs>;
///Single FIFO Count Register
pub mod singlefifocount;
///SCANFIFOCOUNT (r) register accessor: Scan FIFO Count Register
///
///You can [`read`](crate::Reg::read) this register and get [`scanfifocount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@scanfifocount`]
///module
#[doc(alias = "SCANFIFOCOUNT")]
pub type Scanfifocount = crate::Reg<scanfifocount::SCANFIFOCOUNTrs>;
///Scan FIFO Count Register
pub mod scanfifocount;
///SINGLEFIFOCLEAR (w) register accessor: Single FIFO Clear Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlefifoclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@singlefifoclear`]
///module
#[doc(alias = "SINGLEFIFOCLEAR")]
pub type Singlefifoclear = crate::Reg<singlefifoclear::SINGLEFIFOCLEARrs>;
///Single FIFO Clear Register
pub mod singlefifoclear;
///SCANFIFOCLEAR (w) register accessor: Scan FIFO Clear Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanfifoclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@scanfifoclear`]
///module
#[doc(alias = "SCANFIFOCLEAR")]
pub type Scanfifoclear = crate::Reg<scanfifoclear::SCANFIFOCLEARrs>;
///Scan FIFO Clear Register
pub mod scanfifoclear;
///APORTMASTERDIS (rw) register accessor: APORT Bus Master Disable Register
///
///You can [`read`](crate::Reg::read) this register and get [`aportmasterdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aportmasterdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@aportmasterdis`]
///module
#[doc(alias = "APORTMASTERDIS")]
pub type Aportmasterdis = crate::Reg<aportmasterdis::APORTMASTERDISrs>;
///APORT Bus Master Disable Register
pub mod aportmasterdis;
