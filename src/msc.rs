#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: Ctrl,
    readctrl: Readctrl,
    writectrl: Writectrl,
    writecmd: Writecmd,
    addrb: Addrb,
    _reserved5: [u8; 0x04],
    wdata: Wdata,
    status: Status,
    _reserved7: [u8; 0x10],
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    lock: Lock,
    cachecmd: Cachecmd,
    cachehits: Cachehits,
    cachemisses: Cachemisses,
    _reserved15: [u8; 0x04],
    masslock: Masslock,
    _reserved16: [u8; 0x04],
    startup: Startup,
    _reserved17: [u8; 0x14],
    cmd: Cmd,
}
impl RegisterBlock {
    ///0x00 - Memory System Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x04 - Read Control Register
    #[inline(always)]
    pub const fn readctrl(&self) -> &Readctrl {
        &self.readctrl
    }
    ///0x08 - Write Control Register
    #[inline(always)]
    pub const fn writectrl(&self) -> &Writectrl {
        &self.writectrl
    }
    ///0x0c - Write Command Register
    #[inline(always)]
    pub const fn writecmd(&self) -> &Writecmd {
        &self.writecmd
    }
    ///0x10 - Page Erase/Write Address Buffer
    #[inline(always)]
    pub const fn addrb(&self) -> &Addrb {
        &self.addrb
    }
    ///0x18 - Write Data Register
    #[inline(always)]
    pub const fn wdata(&self) -> &Wdata {
        &self.wdata
    }
    ///0x1c - Status Register
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    ///0x30 - Interrupt Flag Register
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    ///0x34 - Interrupt Flag Set Register
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    ///0x38 - Interrupt Flag Clear Register
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    ///0x3c - Interrupt Enable Register
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    ///0x40 - Configuration Lock Register
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    ///0x44 - Flash Cache Command Register
    #[inline(always)]
    pub const fn cachecmd(&self) -> &Cachecmd {
        &self.cachecmd
    }
    ///0x48 - Cache Hits Performance Counter
    #[inline(always)]
    pub const fn cachehits(&self) -> &Cachehits {
        &self.cachehits
    }
    ///0x4c - Cache Misses Performance Counter
    #[inline(always)]
    pub const fn cachemisses(&self) -> &Cachemisses {
        &self.cachemisses
    }
    ///0x54 - Mass Erase Lock Register
    #[inline(always)]
    pub const fn masslock(&self) -> &Masslock {
        &self.masslock
    }
    ///0x5c - Startup Control
    #[inline(always)]
    pub const fn startup(&self) -> &Startup {
        &self.startup
    }
    ///0x74 - Command Register
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
}
///CTRL (rw) register accessor: Memory System Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ctrl`]
///module
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CTRLrs>;
///Memory System Control Register
pub mod ctrl;
///READCTRL (rw) register accessor: Read Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`readctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@readctrl`]
///module
#[doc(alias = "READCTRL")]
pub type Readctrl = crate::Reg<readctrl::READCTRLrs>;
///Read Control Register
pub mod readctrl;
///WRITECTRL (rw) register accessor: Write Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`writectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@writectrl`]
///module
#[doc(alias = "WRITECTRL")]
pub type Writectrl = crate::Reg<writectrl::WRITECTRLrs>;
///Write Control Register
pub mod writectrl;
///WRITECMD (w) register accessor: Write Command Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writecmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@writecmd`]
///module
#[doc(alias = "WRITECMD")]
pub type Writecmd = crate::Reg<writecmd::WRITECMDrs>;
///Write Command Register
pub mod writecmd;
///ADDRB (rw) register accessor: Page Erase/Write Address Buffer
///
///You can [`read`](crate::Reg::read) this register and get [`addrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@addrb`]
///module
#[doc(alias = "ADDRB")]
pub type Addrb = crate::Reg<addrb::ADDRBrs>;
///Page Erase/Write Address Buffer
pub mod addrb;
///WDATA (rw) register accessor: Write Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wdata`]
///module
#[doc(alias = "WDATA")]
pub type Wdata = crate::Reg<wdata::WDATArs>;
///Write Data Register
pub mod wdata;
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
///LOCK (rw) register accessor: Configuration Lock Register
///
///You can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lock`]
///module
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LOCKrs>;
///Configuration Lock Register
pub mod lock;
///CACHECMD (w) register accessor: Flash Cache Command Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cachecmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cachecmd`]
///module
#[doc(alias = "CACHECMD")]
pub type Cachecmd = crate::Reg<cachecmd::CACHECMDrs>;
///Flash Cache Command Register
pub mod cachecmd;
///CACHEHITS (r) register accessor: Cache Hits Performance Counter
///
///You can [`read`](crate::Reg::read) this register and get [`cachehits::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cachehits`]
///module
#[doc(alias = "CACHEHITS")]
pub type Cachehits = crate::Reg<cachehits::CACHEHITSrs>;
///Cache Hits Performance Counter
pub mod cachehits;
///CACHEMISSES (r) register accessor: Cache Misses Performance Counter
///
///You can [`read`](crate::Reg::read) this register and get [`cachemisses::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cachemisses`]
///module
#[doc(alias = "CACHEMISSES")]
pub type Cachemisses = crate::Reg<cachemisses::CACHEMISSESrs>;
///Cache Misses Performance Counter
pub mod cachemisses;
///MASSLOCK (rw) register accessor: Mass Erase Lock Register
///
///You can [`read`](crate::Reg::read) this register and get [`masslock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masslock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@masslock`]
///module
#[doc(alias = "MASSLOCK")]
pub type Masslock = crate::Reg<masslock::MASSLOCKrs>;
///Mass Erase Lock Register
pub mod masslock;
///STARTUP (rw) register accessor: Startup Control
///
///You can [`read`](crate::Reg::read) this register and get [`startup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`startup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@startup`]
///module
#[doc(alias = "STARTUP")]
pub type Startup = crate::Reg<startup::STARTUPrs>;
///Startup Control
pub mod startup;
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
