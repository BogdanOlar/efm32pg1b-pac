#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    readctrl: READCTRL,
    writectrl: WRITECTRL,
    writecmd: WRITECMD,
    addrb: ADDRB,
    _reserved5: [u8; 0x04],
    wdata: WDATA,
    status: STATUS,
    _reserved7: [u8; 0x10],
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    lock: LOCK,
    cachecmd: CACHECMD,
    cachehits: CACHEHITS,
    cachemisses: CACHEMISSES,
    _reserved15: [u8; 0x04],
    masslock: MASSLOCK,
    _reserved16: [u8; 0x04],
    startup: STARTUP,
    _reserved17: [u8; 0x14],
    cmd: CMD,
}
impl RegisterBlock {
    #[doc = "0x00 - Memory System Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Read Control Register"]
    #[inline(always)]
    pub const fn readctrl(&self) -> &READCTRL {
        &self.readctrl
    }
    #[doc = "0x08 - Write Control Register"]
    #[inline(always)]
    pub const fn writectrl(&self) -> &WRITECTRL {
        &self.writectrl
    }
    #[doc = "0x0c - Write Command Register"]
    #[inline(always)]
    pub const fn writecmd(&self) -> &WRITECMD {
        &self.writecmd
    }
    #[doc = "0x10 - Page Erase/Write Address Buffer"]
    #[inline(always)]
    pub const fn addrb(&self) -> &ADDRB {
        &self.addrb
    }
    #[doc = "0x18 - Write Data Register"]
    #[inline(always)]
    pub const fn wdata(&self) -> &WDATA {
        &self.wdata
    }
    #[doc = "0x1c - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x30 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x34 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x38 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x3c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x40 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &LOCK {
        &self.lock
    }
    #[doc = "0x44 - Flash Cache Command Register"]
    #[inline(always)]
    pub const fn cachecmd(&self) -> &CACHECMD {
        &self.cachecmd
    }
    #[doc = "0x48 - Cache Hits Performance Counter"]
    #[inline(always)]
    pub const fn cachehits(&self) -> &CACHEHITS {
        &self.cachehits
    }
    #[doc = "0x4c - Cache Misses Performance Counter"]
    #[inline(always)]
    pub const fn cachemisses(&self) -> &CACHEMISSES {
        &self.cachemisses
    }
    #[doc = "0x54 - Mass Erase Lock Register"]
    #[inline(always)]
    pub const fn masslock(&self) -> &MASSLOCK {
        &self.masslock
    }
    #[doc = "0x5c - Startup Control"]
    #[inline(always)]
    pub const fn startup(&self) -> &STARTUP {
        &self.startup
    }
    #[doc = "0x74 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
}
#[doc = "CTRL (rw) register accessor: Memory System Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRLrs>;
#[doc = "Memory System Control Register"]
pub mod ctrl;
#[doc = "READCTRL (rw) register accessor: Read Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`readctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readctrl`]
module"]
pub type READCTRL = crate::Reg<readctrl::READCTRLrs>;
#[doc = "Read Control Register"]
pub mod readctrl;
#[doc = "WRITECTRL (rw) register accessor: Write Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`writectrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`writectrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writectrl`]
module"]
pub type WRITECTRL = crate::Reg<writectrl::WRITECTRLrs>;
#[doc = "Write Control Register"]
pub mod writectrl;
#[doc = "WRITECMD (w) register accessor: Write Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`writecmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writecmd`]
module"]
pub type WRITECMD = crate::Reg<writecmd::WRITECMDrs>;
#[doc = "Write Command Register"]
pub mod writecmd;
#[doc = "ADDRB (rw) register accessor: Page Erase/Write Address Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrb`]
module"]
pub type ADDRB = crate::Reg<addrb::ADDRBrs>;
#[doc = "Page Erase/Write Address Buffer"]
pub mod addrb;
#[doc = "WDATA (rw) register accessor: Write Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdata`]
module"]
pub type WDATA = crate::Reg<wdata::WDATArs>;
#[doc = "Write Data Register"]
pub mod wdata;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUSrs>;
#[doc = "Status Register"]
pub mod status;
#[doc = "IF (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`]
module"]
pub type IF = crate::Reg<if_::IFrs>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs`]
module"]
pub type IFS = crate::Reg<ifs::IFSrs>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifc`]
module"]
pub type IFC = crate::Reg<ifc::IFCrs>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`]
module"]
pub type IEN = crate::Reg<ien::IENrs>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCKrs>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "CACHECMD (w) register accessor: Flash Cache Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cachecmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cachecmd`]
module"]
pub type CACHECMD = crate::Reg<cachecmd::CACHECMDrs>;
#[doc = "Flash Cache Command Register"]
pub mod cachecmd;
#[doc = "CACHEHITS (r) register accessor: Cache Hits Performance Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cachehits::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cachehits`]
module"]
pub type CACHEHITS = crate::Reg<cachehits::CACHEHITSrs>;
#[doc = "Cache Hits Performance Counter"]
pub mod cachehits;
#[doc = "CACHEMISSES (r) register accessor: Cache Misses Performance Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cachemisses::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cachemisses`]
module"]
pub type CACHEMISSES = crate::Reg<cachemisses::CACHEMISSESrs>;
#[doc = "Cache Misses Performance Counter"]
pub mod cachemisses;
#[doc = "MASSLOCK (rw) register accessor: Mass Erase Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`masslock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`masslock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@masslock`]
module"]
pub type MASSLOCK = crate::Reg<masslock::MASSLOCKrs>;
#[doc = "Mass Erase Lock Register"]
pub mod masslock;
#[doc = "STARTUP (rw) register accessor: Startup Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`startup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`startup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@startup`]
module"]
pub type STARTUP = crate::Reg<startup::STARTUPrs>;
#[doc = "Startup Control"]
pub mod startup;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMDrs>;
#[doc = "Command Register"]
pub mod cmd;
