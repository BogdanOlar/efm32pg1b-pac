#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    sync: Sync,
    _reserved3: [u8; 0x14],
    chen: Chen,
    chbusy: Chbusy,
    chdone: Chdone,
    dbghalt: Dbghalt,
    swreq: Swreq,
    reqdis: Reqdis,
    reqpend: Reqpend,
    linkload: Linkload,
    reqclear: Reqclear,
    _reserved12: [u8; 0x1c],
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    _reserved16: [u8; 0x10],
    ch0: Ch0,
    _reserved17: [u8; 0x14],
    ch1: Ch1,
    _reserved18: [u8; 0x14],
    ch2: Ch2,
    _reserved19: [u8; 0x14],
    ch3: Ch3,
    _reserved20: [u8; 0x14],
    ch4: Ch4,
    _reserved21: [u8; 0x14],
    ch5: Ch5,
    _reserved22: [u8; 0x14],
    ch6: Ch6,
    _reserved23: [u8; 0x14],
    ch7: Ch7,
}
impl RegisterBlock {
    ///0x00 - DMA Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x04 - DMA Status Register
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    ///0x08 - DMA Synchronization Trigger Register (Single-Cycle RMW)
    #[inline(always)]
    pub const fn sync(&self) -> &Sync {
        &self.sync
    }
    ///0x20 - DMA Channel Enable Register (Single-Cycle RMW)
    #[inline(always)]
    pub const fn chen(&self) -> &Chen {
        &self.chen
    }
    ///0x24 - DMA Channel Busy Register
    #[inline(always)]
    pub const fn chbusy(&self) -> &Chbusy {
        &self.chbusy
    }
    ///0x28 - DMA Channel Linking Done Register (Single-Cycle RMW)
    #[inline(always)]
    pub const fn chdone(&self) -> &Chdone {
        &self.chdone
    }
    ///0x2c - DMA Channel Debug Halt Register
    #[inline(always)]
    pub const fn dbghalt(&self) -> &Dbghalt {
        &self.dbghalt
    }
    ///0x30 - DMA Channel Software Transfer Request Register
    #[inline(always)]
    pub const fn swreq(&self) -> &Swreq {
        &self.swreq
    }
    ///0x34 - DMA Channel Request Disable Register
    #[inline(always)]
    pub const fn reqdis(&self) -> &Reqdis {
        &self.reqdis
    }
    ///0x38 - DMA Channel Requests Pending Register
    #[inline(always)]
    pub const fn reqpend(&self) -> &Reqpend {
        &self.reqpend
    }
    ///0x3c - DMA Channel Link Load Register
    #[inline(always)]
    pub const fn linkload(&self) -> &Linkload {
        &self.linkload
    }
    ///0x40 - DMA Channel Request Clear Register
    #[inline(always)]
    pub const fn reqclear(&self) -> &Reqclear {
        &self.reqclear
    }
    ///0x60 - Interrupt Flag Register
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    ///0x64 - Interrupt Flag Set Register
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    ///0x68 - Interrupt Flag Clear Register
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    ///0x6c - Interrupt Enable Register
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    ///0x80..0x9c - Channel 0
    #[inline(always)]
    pub const fn ch0(&self) -> &Ch0 {
        &self.ch0
    }
    ///0xb0..0xcc - Channel 1
    #[inline(always)]
    pub const fn ch1(&self) -> &Ch1 {
        &self.ch1
    }
    ///0xe0..0xfc - Channel 2
    #[inline(always)]
    pub const fn ch2(&self) -> &Ch2 {
        &self.ch2
    }
    ///0x110..0x12c - Channel 3
    #[inline(always)]
    pub const fn ch3(&self) -> &Ch3 {
        &self.ch3
    }
    ///0x140..0x15c - Channel 4
    #[inline(always)]
    pub const fn ch4(&self) -> &Ch4 {
        &self.ch4
    }
    ///0x170..0x18c - Channel 5
    #[inline(always)]
    pub const fn ch5(&self) -> &Ch5 {
        &self.ch5
    }
    ///0x1a0..0x1bc - Channel 6
    #[inline(always)]
    pub const fn ch6(&self) -> &Ch6 {
        &self.ch6
    }
    ///0x1d0..0x1ec - Channel 7
    #[inline(always)]
    pub const fn ch7(&self) -> &Ch7 {
        &self.ch7
    }
}
///CTRL (rw) register accessor: DMA Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ctrl`] module
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CTRLrs>;
///DMA Control Register
pub mod ctrl;
///STATUS (r) register accessor: DMA Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@status`] module
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::STATUSrs>;
///DMA Status Register
pub mod status;
///SYNC (rw) register accessor: DMA Synchronization Trigger Register (Single-Cycle RMW)
///
///You can [`read`](crate::Reg::read) this register and get [`sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@sync`] module
#[doc(alias = "SYNC")]
pub type Sync = crate::Reg<sync::SYNCrs>;
///DMA Synchronization Trigger Register (Single-Cycle RMW)
pub mod sync;
///CHEN (rw) register accessor: DMA Channel Enable Register (Single-Cycle RMW)
///
///You can [`read`](crate::Reg::read) this register and get [`chen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@chen`] module
#[doc(alias = "CHEN")]
pub type Chen = crate::Reg<chen::CHENrs>;
///DMA Channel Enable Register (Single-Cycle RMW)
pub mod chen;
///CHBUSY (r) register accessor: DMA Channel Busy Register
///
///You can [`read`](crate::Reg::read) this register and get [`chbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@chbusy`] module
#[doc(alias = "CHBUSY")]
pub type Chbusy = crate::Reg<chbusy::CHBUSYrs>;
///DMA Channel Busy Register
pub mod chbusy;
///CHDONE (rw) register accessor: DMA Channel Linking Done Register (Single-Cycle RMW)
///
///You can [`read`](crate::Reg::read) this register and get [`chdone::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdone::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@chdone`] module
#[doc(alias = "CHDONE")]
pub type Chdone = crate::Reg<chdone::CHDONErs>;
///DMA Channel Linking Done Register (Single-Cycle RMW)
pub mod chdone;
///DBGHALT (rw) register accessor: DMA Channel Debug Halt Register
///
///You can [`read`](crate::Reg::read) this register and get [`dbghalt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbghalt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dbghalt`] module
#[doc(alias = "DBGHALT")]
pub type Dbghalt = crate::Reg<dbghalt::DBGHALTrs>;
///DMA Channel Debug Halt Register
pub mod dbghalt;
///SWREQ (w) register accessor: DMA Channel Software Transfer Request Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@swreq`] module
#[doc(alias = "SWREQ")]
pub type Swreq = crate::Reg<swreq::SWREQrs>;
///DMA Channel Software Transfer Request Register
pub mod swreq;
///REQDIS (rw) register accessor: DMA Channel Request Disable Register
///
///You can [`read`](crate::Reg::read) this register and get [`reqdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@reqdis`] module
#[doc(alias = "REQDIS")]
pub type Reqdis = crate::Reg<reqdis::REQDISrs>;
///DMA Channel Request Disable Register
pub mod reqdis;
///REQPEND (r) register accessor: DMA Channel Requests Pending Register
///
///You can [`read`](crate::Reg::read) this register and get [`reqpend::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@reqpend`] module
#[doc(alias = "REQPEND")]
pub type Reqpend = crate::Reg<reqpend::REQPENDrs>;
///DMA Channel Requests Pending Register
pub mod reqpend;
///LINKLOAD (w) register accessor: DMA Channel Link Load Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linkload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@linkload`] module
#[doc(alias = "LINKLOAD")]
pub type Linkload = crate::Reg<linkload::LINKLOADrs>;
///DMA Channel Link Load Register
pub mod linkload;
///REQCLEAR (w) register accessor: DMA Channel Request Clear Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@reqclear`] module
#[doc(alias = "REQCLEAR")]
pub type Reqclear = crate::Reg<reqclear::REQCLEARrs>;
///DMA Channel Request Clear Register
pub mod reqclear;
///IF (r) register accessor: Interrupt Flag Register
///
///You can [`read`](crate::Reg::read) this register and get [`if_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@if_`] module
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IFrs>;
///Interrupt Flag Register
pub mod if_;
///IFS (w) register accessor: Interrupt Flag Set Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ifs`] module
#[doc(alias = "IFS")]
pub type Ifs = crate::Reg<ifs::IFSrs>;
///Interrupt Flag Set Register
pub mod ifs;
///IFC (w) register accessor: Interrupt Flag Clear Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ifc`] module
#[doc(alias = "IFC")]
pub type Ifc = crate::Reg<ifc::IFCrs>;
///Interrupt Flag Clear Register
pub mod ifc;
///IEN (rw) register accessor: Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ien`] module
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IENrs>;
///Interrupt Enable Register
pub mod ien;
///Channel 0
pub use self::ch0::Ch0;
///Cluster
///Channel 0
pub mod ch0;
pub use ch0 as ch1;
pub use ch0 as ch2;
pub use ch0 as ch3;
pub use ch0 as ch4;
pub use ch0 as ch5;
pub use ch0 as ch6;
pub use ch0 as ch7;
///Channel 1
pub use Ch0 as Ch1;
///Channel 2
pub use Ch0 as Ch2;
///Channel 3
pub use Ch0 as Ch3;
///Channel 4
pub use Ch0 as Ch4;
///Channel 5
pub use Ch0 as Ch5;
///Channel 6
pub use Ch0 as Ch6;
///Channel 7
pub use Ch0 as Ch7;
