#[repr(C)]
#[doc = "Register block"]
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
    ch0_reqsel: Ch0Reqsel,
    ch0_cfg: Ch0Cfg,
    ch0_loop: Ch0Loop,
    ch0_ctrl: Ch0Ctrl,
    ch0_src: Ch0Src,
    ch0_dst: Ch0Dst,
    ch0_link: Ch0Link,
    _reserved23: [u8; 0x14],
    ch1_reqsel: Ch1Reqsel,
    ch1_cfg: Ch1Cfg,
    ch1_loop: Ch1Loop,
    ch1_ctrl: Ch1Ctrl,
    ch1_src: Ch1Src,
    ch1_dst: Ch1Dst,
    ch1_link: Ch1Link,
    _reserved30: [u8; 0x14],
    ch2_reqsel: Ch2Reqsel,
    ch2_cfg: Ch2Cfg,
    ch2_loop: Ch2Loop,
    ch2_ctrl: Ch2Ctrl,
    ch2_src: Ch2Src,
    ch2_dst: Ch2Dst,
    ch2_link: Ch2Link,
    _reserved37: [u8; 0x14],
    ch3_reqsel: Ch3Reqsel,
    ch3_cfg: Ch3Cfg,
    ch3_loop: Ch3Loop,
    ch3_ctrl: Ch3Ctrl,
    ch3_src: Ch3Src,
    ch3_dst: Ch3Dst,
    ch3_link: Ch3Link,
    _reserved44: [u8; 0x14],
    ch4_reqsel: Ch4Reqsel,
    ch4_cfg: Ch4Cfg,
    ch4_loop: Ch4Loop,
    ch4_ctrl: Ch4Ctrl,
    ch4_src: Ch4Src,
    ch4_dst: Ch4Dst,
    ch4_link: Ch4Link,
    _reserved51: [u8; 0x14],
    ch5_reqsel: Ch5Reqsel,
    ch5_cfg: Ch5Cfg,
    ch5_loop: Ch5Loop,
    ch5_ctrl: Ch5Ctrl,
    ch5_src: Ch5Src,
    ch5_dst: Ch5Dst,
    ch5_link: Ch5Link,
    _reserved58: [u8; 0x14],
    ch6_reqsel: Ch6Reqsel,
    ch6_cfg: Ch6Cfg,
    ch6_loop: Ch6Loop,
    ch6_ctrl: Ch6Ctrl,
    ch6_src: Ch6Src,
    ch6_dst: Ch6Dst,
    ch6_link: Ch6Link,
    _reserved65: [u8; 0x14],
    ch7_reqsel: Ch7Reqsel,
    ch7_cfg: Ch7Cfg,
    ch7_loop: Ch7Loop,
    ch7_ctrl: Ch7Ctrl,
    ch7_src: Ch7Src,
    ch7_dst: Ch7Dst,
    ch7_link: Ch7Link,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - DMA Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - DMA Synchronization Trigger Register (Single-Cycle RMW)"]
    #[inline(always)]
    pub const fn sync(&self) -> &Sync {
        &self.sync
    }
    #[doc = "0x20 - DMA Channel Enable Register (Single-Cycle RMW)"]
    #[inline(always)]
    pub const fn chen(&self) -> &Chen {
        &self.chen
    }
    #[doc = "0x24 - DMA Channel Busy Register"]
    #[inline(always)]
    pub const fn chbusy(&self) -> &Chbusy {
        &self.chbusy
    }
    #[doc = "0x28 - DMA Channel Linking Done Register (Single-Cycle RMW)"]
    #[inline(always)]
    pub const fn chdone(&self) -> &Chdone {
        &self.chdone
    }
    #[doc = "0x2c - DMA Channel Debug Halt Register"]
    #[inline(always)]
    pub const fn dbghalt(&self) -> &Dbghalt {
        &self.dbghalt
    }
    #[doc = "0x30 - DMA Channel Software Transfer Request Register"]
    #[inline(always)]
    pub const fn swreq(&self) -> &Swreq {
        &self.swreq
    }
    #[doc = "0x34 - DMA Channel Request Disable Register"]
    #[inline(always)]
    pub const fn reqdis(&self) -> &Reqdis {
        &self.reqdis
    }
    #[doc = "0x38 - DMA Channel Requests Pending Register"]
    #[inline(always)]
    pub const fn reqpend(&self) -> &Reqpend {
        &self.reqpend
    }
    #[doc = "0x3c - DMA Channel Link Load Register"]
    #[inline(always)]
    pub const fn linkload(&self) -> &Linkload {
        &self.linkload
    }
    #[doc = "0x40 - DMA Channel Request Clear Register"]
    #[inline(always)]
    pub const fn reqclear(&self) -> &Reqclear {
        &self.reqclear
    }
    #[doc = "0x60 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x64 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x68 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x6c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x80 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch0_reqsel(&self) -> &Ch0Reqsel {
        &self.ch0_reqsel
    }
    #[doc = "0x84 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch0_cfg(&self) -> &Ch0Cfg {
        &self.ch0_cfg
    }
    #[doc = "0x88 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch0_loop(&self) -> &Ch0Loop {
        &self.ch0_loop
    }
    #[doc = "0x8c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch0_ctrl(&self) -> &Ch0Ctrl {
        &self.ch0_ctrl
    }
    #[doc = "0x90 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch0_src(&self) -> &Ch0Src {
        &self.ch0_src
    }
    #[doc = "0x94 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch0_dst(&self) -> &Ch0Dst {
        &self.ch0_dst
    }
    #[doc = "0x98 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch0_link(&self) -> &Ch0Link {
        &self.ch0_link
    }
    #[doc = "0xb0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch1_reqsel(&self) -> &Ch1Reqsel {
        &self.ch1_reqsel
    }
    #[doc = "0xb4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch1_cfg(&self) -> &Ch1Cfg {
        &self.ch1_cfg
    }
    #[doc = "0xb8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch1_loop(&self) -> &Ch1Loop {
        &self.ch1_loop
    }
    #[doc = "0xbc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch1_ctrl(&self) -> &Ch1Ctrl {
        &self.ch1_ctrl
    }
    #[doc = "0xc0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch1_src(&self) -> &Ch1Src {
        &self.ch1_src
    }
    #[doc = "0xc4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch1_dst(&self) -> &Ch1Dst {
        &self.ch1_dst
    }
    #[doc = "0xc8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch1_link(&self) -> &Ch1Link {
        &self.ch1_link
    }
    #[doc = "0xe0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch2_reqsel(&self) -> &Ch2Reqsel {
        &self.ch2_reqsel
    }
    #[doc = "0xe4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch2_cfg(&self) -> &Ch2Cfg {
        &self.ch2_cfg
    }
    #[doc = "0xe8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch2_loop(&self) -> &Ch2Loop {
        &self.ch2_loop
    }
    #[doc = "0xec - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch2_ctrl(&self) -> &Ch2Ctrl {
        &self.ch2_ctrl
    }
    #[doc = "0xf0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch2_src(&self) -> &Ch2Src {
        &self.ch2_src
    }
    #[doc = "0xf4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch2_dst(&self) -> &Ch2Dst {
        &self.ch2_dst
    }
    #[doc = "0xf8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch2_link(&self) -> &Ch2Link {
        &self.ch2_link
    }
    #[doc = "0x110 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch3_reqsel(&self) -> &Ch3Reqsel {
        &self.ch3_reqsel
    }
    #[doc = "0x114 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch3_cfg(&self) -> &Ch3Cfg {
        &self.ch3_cfg
    }
    #[doc = "0x118 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch3_loop(&self) -> &Ch3Loop {
        &self.ch3_loop
    }
    #[doc = "0x11c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch3_ctrl(&self) -> &Ch3Ctrl {
        &self.ch3_ctrl
    }
    #[doc = "0x120 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch3_src(&self) -> &Ch3Src {
        &self.ch3_src
    }
    #[doc = "0x124 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch3_dst(&self) -> &Ch3Dst {
        &self.ch3_dst
    }
    #[doc = "0x128 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch3_link(&self) -> &Ch3Link {
        &self.ch3_link
    }
    #[doc = "0x140 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch4_reqsel(&self) -> &Ch4Reqsel {
        &self.ch4_reqsel
    }
    #[doc = "0x144 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch4_cfg(&self) -> &Ch4Cfg {
        &self.ch4_cfg
    }
    #[doc = "0x148 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch4_loop(&self) -> &Ch4Loop {
        &self.ch4_loop
    }
    #[doc = "0x14c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch4_ctrl(&self) -> &Ch4Ctrl {
        &self.ch4_ctrl
    }
    #[doc = "0x150 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch4_src(&self) -> &Ch4Src {
        &self.ch4_src
    }
    #[doc = "0x154 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch4_dst(&self) -> &Ch4Dst {
        &self.ch4_dst
    }
    #[doc = "0x158 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch4_link(&self) -> &Ch4Link {
        &self.ch4_link
    }
    #[doc = "0x170 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch5_reqsel(&self) -> &Ch5Reqsel {
        &self.ch5_reqsel
    }
    #[doc = "0x174 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch5_cfg(&self) -> &Ch5Cfg {
        &self.ch5_cfg
    }
    #[doc = "0x178 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch5_loop(&self) -> &Ch5Loop {
        &self.ch5_loop
    }
    #[doc = "0x17c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch5_ctrl(&self) -> &Ch5Ctrl {
        &self.ch5_ctrl
    }
    #[doc = "0x180 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch5_src(&self) -> &Ch5Src {
        &self.ch5_src
    }
    #[doc = "0x184 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch5_dst(&self) -> &Ch5Dst {
        &self.ch5_dst
    }
    #[doc = "0x188 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch5_link(&self) -> &Ch5Link {
        &self.ch5_link
    }
    #[doc = "0x1a0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch6_reqsel(&self) -> &Ch6Reqsel {
        &self.ch6_reqsel
    }
    #[doc = "0x1a4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch6_cfg(&self) -> &Ch6Cfg {
        &self.ch6_cfg
    }
    #[doc = "0x1a8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch6_loop(&self) -> &Ch6Loop {
        &self.ch6_loop
    }
    #[doc = "0x1ac - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch6_ctrl(&self) -> &Ch6Ctrl {
        &self.ch6_ctrl
    }
    #[doc = "0x1b0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch6_src(&self) -> &Ch6Src {
        &self.ch6_src
    }
    #[doc = "0x1b4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch6_dst(&self) -> &Ch6Dst {
        &self.ch6_dst
    }
    #[doc = "0x1b8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch6_link(&self) -> &Ch6Link {
        &self.ch6_link
    }
    #[doc = "0x1d0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch7_reqsel(&self) -> &Ch7Reqsel {
        &self.ch7_reqsel
    }
    #[doc = "0x1d4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch7_cfg(&self) -> &Ch7Cfg {
        &self.ch7_cfg
    }
    #[doc = "0x1d8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch7_loop(&self) -> &Ch7Loop {
        &self.ch7_loop
    }
    #[doc = "0x1dc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch7_ctrl(&self) -> &Ch7Ctrl {
        &self.ch7_ctrl
    }
    #[doc = "0x1e0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch7_src(&self) -> &Ch7Src {
        &self.ch7_src
    }
    #[doc = "0x1e4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch7_dst(&self) -> &Ch7Dst {
        &self.ch7_dst
    }
    #[doc = "0x1e8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch7_link(&self) -> &Ch7Link {
        &self.ch7_link
    }
}
#[doc = "CTRL (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CTRLrs>;
#[doc = "DMA Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: DMA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::STATUSrs>;
#[doc = "DMA Status Register"]
pub mod status;
#[doc = "SYNC (rw) register accessor: DMA Synchronization Trigger Register (Single-Cycle RMW)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync`]
module"]
#[doc(alias = "SYNC")]
pub type Sync = crate::Reg<sync::SYNCrs>;
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)"]
pub mod sync;
#[doc = "CHEN (rw) register accessor: DMA Channel Enable Register (Single-Cycle RMW)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen`]
module"]
#[doc(alias = "CHEN")]
pub type Chen = crate::Reg<chen::CHENrs>;
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)"]
pub mod chen;
#[doc = "CHBUSY (r) register accessor: DMA Channel Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chbusy`]
module"]
#[doc(alias = "CHBUSY")]
pub type Chbusy = crate::Reg<chbusy::CHBUSYrs>;
#[doc = "DMA Channel Busy Register"]
pub mod chbusy;
#[doc = "CHDONE (rw) register accessor: DMA Channel Linking Done Register (Single-Cycle RMW)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chdone::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdone::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdone`]
module"]
#[doc(alias = "CHDONE")]
pub type Chdone = crate::Reg<chdone::CHDONErs>;
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)"]
pub mod chdone;
#[doc = "DBGHALT (rw) register accessor: DMA Channel Debug Halt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbghalt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbghalt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbghalt`]
module"]
#[doc(alias = "DBGHALT")]
pub type Dbghalt = crate::Reg<dbghalt::DBGHALTrs>;
#[doc = "DMA Channel Debug Halt Register"]
pub mod dbghalt;
#[doc = "SWREQ (w) register accessor: DMA Channel Software Transfer Request Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreq`]
module"]
#[doc(alias = "SWREQ")]
pub type Swreq = crate::Reg<swreq::SWREQrs>;
#[doc = "DMA Channel Software Transfer Request Register"]
pub mod swreq;
#[doc = "REQDIS (rw) register accessor: DMA Channel Request Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqdis`]
module"]
#[doc(alias = "REQDIS")]
pub type Reqdis = crate::Reg<reqdis::REQDISrs>;
#[doc = "DMA Channel Request Disable Register"]
pub mod reqdis;
#[doc = "REQPEND (r) register accessor: DMA Channel Requests Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqpend::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqpend`]
module"]
#[doc(alias = "REQPEND")]
pub type Reqpend = crate::Reg<reqpend::REQPENDrs>;
#[doc = "DMA Channel Requests Pending Register"]
pub mod reqpend;
#[doc = "LINKLOAD (w) register accessor: DMA Channel Link Load Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linkload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linkload`]
module"]
#[doc(alias = "LINKLOAD")]
pub type Linkload = crate::Reg<linkload::LINKLOADrs>;
#[doc = "DMA Channel Link Load Register"]
pub mod linkload;
#[doc = "REQCLEAR (w) register accessor: DMA Channel Request Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqclear`]
module"]
#[doc(alias = "REQCLEAR")]
pub type Reqclear = crate::Reg<reqclear::REQCLEARrs>;
#[doc = "DMA Channel Request Clear Register"]
pub mod reqclear;
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
#[doc = "CH0_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_reqsel`]
module"]
#[doc(alias = "CH0_REQSEL")]
pub type Ch0Reqsel = crate::Reg<ch0_reqsel::CH0_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch0_reqsel;
#[doc = "CH0_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_cfg`]
module"]
#[doc(alias = "CH0_CFG")]
pub type Ch0Cfg = crate::Reg<ch0_cfg::CH0_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch0_cfg;
#[doc = "CH0_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_loop`]
module"]
#[doc(alias = "CH0_LOOP")]
pub type Ch0Loop = crate::Reg<ch0_loop::CH0_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch0_loop;
#[doc = "CH0_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_ctrl`]
module"]
#[doc(alias = "CH0_CTRL")]
pub type Ch0Ctrl = crate::Reg<ch0_ctrl::CH0_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch0_ctrl;
#[doc = "CH0_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_src`]
module"]
#[doc(alias = "CH0_SRC")]
pub type Ch0Src = crate::Reg<ch0_src::CH0_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch0_src;
#[doc = "CH0_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_dst`]
module"]
#[doc(alias = "CH0_DST")]
pub type Ch0Dst = crate::Reg<ch0_dst::CH0_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch0_dst;
#[doc = "CH0_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_link`]
module"]
#[doc(alias = "CH0_LINK")]
pub type Ch0Link = crate::Reg<ch0_link::CH0_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch0_link;
#[doc = "CH1_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_reqsel`]
module"]
#[doc(alias = "CH1_REQSEL")]
pub type Ch1Reqsel = crate::Reg<ch1_reqsel::CH1_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch1_reqsel;
#[doc = "CH1_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_cfg`]
module"]
#[doc(alias = "CH1_CFG")]
pub type Ch1Cfg = crate::Reg<ch1_cfg::CH1_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch1_cfg;
#[doc = "CH1_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_loop`]
module"]
#[doc(alias = "CH1_LOOP")]
pub type Ch1Loop = crate::Reg<ch1_loop::CH1_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch1_loop;
#[doc = "CH1_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ctrl`]
module"]
#[doc(alias = "CH1_CTRL")]
pub type Ch1Ctrl = crate::Reg<ch1_ctrl::CH1_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch1_ctrl;
#[doc = "CH1_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_src`]
module"]
#[doc(alias = "CH1_SRC")]
pub type Ch1Src = crate::Reg<ch1_src::CH1_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch1_src;
#[doc = "CH1_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dst`]
module"]
#[doc(alias = "CH1_DST")]
pub type Ch1Dst = crate::Reg<ch1_dst::CH1_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch1_dst;
#[doc = "CH1_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_link`]
module"]
#[doc(alias = "CH1_LINK")]
pub type Ch1Link = crate::Reg<ch1_link::CH1_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch1_link;
#[doc = "CH2_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_reqsel`]
module"]
#[doc(alias = "CH2_REQSEL")]
pub type Ch2Reqsel = crate::Reg<ch2_reqsel::CH2_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch2_reqsel;
#[doc = "CH2_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_cfg`]
module"]
#[doc(alias = "CH2_CFG")]
pub type Ch2Cfg = crate::Reg<ch2_cfg::CH2_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch2_cfg;
#[doc = "CH2_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_loop`]
module"]
#[doc(alias = "CH2_LOOP")]
pub type Ch2Loop = crate::Reg<ch2_loop::CH2_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch2_loop;
#[doc = "CH2_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_ctrl`]
module"]
#[doc(alias = "CH2_CTRL")]
pub type Ch2Ctrl = crate::Reg<ch2_ctrl::CH2_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch2_ctrl;
#[doc = "CH2_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_src`]
module"]
#[doc(alias = "CH2_SRC")]
pub type Ch2Src = crate::Reg<ch2_src::CH2_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch2_src;
#[doc = "CH2_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_dst`]
module"]
#[doc(alias = "CH2_DST")]
pub type Ch2Dst = crate::Reg<ch2_dst::CH2_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch2_dst;
#[doc = "CH2_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_link`]
module"]
#[doc(alias = "CH2_LINK")]
pub type Ch2Link = crate::Reg<ch2_link::CH2_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch2_link;
#[doc = "CH3_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_reqsel`]
module"]
#[doc(alias = "CH3_REQSEL")]
pub type Ch3Reqsel = crate::Reg<ch3_reqsel::CH3_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch3_reqsel;
#[doc = "CH3_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_cfg`]
module"]
#[doc(alias = "CH3_CFG")]
pub type Ch3Cfg = crate::Reg<ch3_cfg::CH3_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch3_cfg;
#[doc = "CH3_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_loop`]
module"]
#[doc(alias = "CH3_LOOP")]
pub type Ch3Loop = crate::Reg<ch3_loop::CH3_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch3_loop;
#[doc = "CH3_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_ctrl`]
module"]
#[doc(alias = "CH3_CTRL")]
pub type Ch3Ctrl = crate::Reg<ch3_ctrl::CH3_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch3_ctrl;
#[doc = "CH3_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_src`]
module"]
#[doc(alias = "CH3_SRC")]
pub type Ch3Src = crate::Reg<ch3_src::CH3_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch3_src;
#[doc = "CH3_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_dst`]
module"]
#[doc(alias = "CH3_DST")]
pub type Ch3Dst = crate::Reg<ch3_dst::CH3_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch3_dst;
#[doc = "CH3_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_link`]
module"]
#[doc(alias = "CH3_LINK")]
pub type Ch3Link = crate::Reg<ch3_link::CH3_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch3_link;
#[doc = "CH4_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_reqsel`]
module"]
#[doc(alias = "CH4_REQSEL")]
pub type Ch4Reqsel = crate::Reg<ch4_reqsel::CH4_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch4_reqsel;
#[doc = "CH4_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_cfg`]
module"]
#[doc(alias = "CH4_CFG")]
pub type Ch4Cfg = crate::Reg<ch4_cfg::CH4_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch4_cfg;
#[doc = "CH4_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_loop`]
module"]
#[doc(alias = "CH4_LOOP")]
pub type Ch4Loop = crate::Reg<ch4_loop::CH4_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch4_loop;
#[doc = "CH4_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_ctrl`]
module"]
#[doc(alias = "CH4_CTRL")]
pub type Ch4Ctrl = crate::Reg<ch4_ctrl::CH4_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch4_ctrl;
#[doc = "CH4_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_src`]
module"]
#[doc(alias = "CH4_SRC")]
pub type Ch4Src = crate::Reg<ch4_src::CH4_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch4_src;
#[doc = "CH4_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_dst`]
module"]
#[doc(alias = "CH4_DST")]
pub type Ch4Dst = crate::Reg<ch4_dst::CH4_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch4_dst;
#[doc = "CH4_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_link`]
module"]
#[doc(alias = "CH4_LINK")]
pub type Ch4Link = crate::Reg<ch4_link::CH4_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch4_link;
#[doc = "CH5_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_reqsel`]
module"]
#[doc(alias = "CH5_REQSEL")]
pub type Ch5Reqsel = crate::Reg<ch5_reqsel::CH5_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch5_reqsel;
#[doc = "CH5_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_cfg`]
module"]
#[doc(alias = "CH5_CFG")]
pub type Ch5Cfg = crate::Reg<ch5_cfg::CH5_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch5_cfg;
#[doc = "CH5_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_loop`]
module"]
#[doc(alias = "CH5_LOOP")]
pub type Ch5Loop = crate::Reg<ch5_loop::CH5_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch5_loop;
#[doc = "CH5_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_ctrl`]
module"]
#[doc(alias = "CH5_CTRL")]
pub type Ch5Ctrl = crate::Reg<ch5_ctrl::CH5_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch5_ctrl;
#[doc = "CH5_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_src`]
module"]
#[doc(alias = "CH5_SRC")]
pub type Ch5Src = crate::Reg<ch5_src::CH5_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch5_src;
#[doc = "CH5_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_dst`]
module"]
#[doc(alias = "CH5_DST")]
pub type Ch5Dst = crate::Reg<ch5_dst::CH5_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch5_dst;
#[doc = "CH5_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_link`]
module"]
#[doc(alias = "CH5_LINK")]
pub type Ch5Link = crate::Reg<ch5_link::CH5_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch5_link;
#[doc = "CH6_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_reqsel`]
module"]
#[doc(alias = "CH6_REQSEL")]
pub type Ch6Reqsel = crate::Reg<ch6_reqsel::CH6_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch6_reqsel;
#[doc = "CH6_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_cfg`]
module"]
#[doc(alias = "CH6_CFG")]
pub type Ch6Cfg = crate::Reg<ch6_cfg::CH6_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch6_cfg;
#[doc = "CH6_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_loop`]
module"]
#[doc(alias = "CH6_LOOP")]
pub type Ch6Loop = crate::Reg<ch6_loop::CH6_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch6_loop;
#[doc = "CH6_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_ctrl`]
module"]
#[doc(alias = "CH6_CTRL")]
pub type Ch6Ctrl = crate::Reg<ch6_ctrl::CH6_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch6_ctrl;
#[doc = "CH6_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_src`]
module"]
#[doc(alias = "CH6_SRC")]
pub type Ch6Src = crate::Reg<ch6_src::CH6_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch6_src;
#[doc = "CH6_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_dst`]
module"]
#[doc(alias = "CH6_DST")]
pub type Ch6Dst = crate::Reg<ch6_dst::CH6_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch6_dst;
#[doc = "CH6_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_link`]
module"]
#[doc(alias = "CH6_LINK")]
pub type Ch6Link = crate::Reg<ch6_link::CH6_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch6_link;
#[doc = "CH7_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_reqsel`]
module"]
#[doc(alias = "CH7_REQSEL")]
pub type Ch7Reqsel = crate::Reg<ch7_reqsel::CH7_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch7_reqsel;
#[doc = "CH7_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_cfg`]
module"]
#[doc(alias = "CH7_CFG")]
pub type Ch7Cfg = crate::Reg<ch7_cfg::CH7_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch7_cfg;
#[doc = "CH7_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_loop`]
module"]
#[doc(alias = "CH7_LOOP")]
pub type Ch7Loop = crate::Reg<ch7_loop::CH7_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch7_loop;
#[doc = "CH7_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_ctrl`]
module"]
#[doc(alias = "CH7_CTRL")]
pub type Ch7Ctrl = crate::Reg<ch7_ctrl::CH7_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch7_ctrl;
#[doc = "CH7_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_src`]
module"]
#[doc(alias = "CH7_SRC")]
pub type Ch7Src = crate::Reg<ch7_src::CH7_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch7_src;
#[doc = "CH7_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_dst`]
module"]
#[doc(alias = "CH7_DST")]
pub type Ch7Dst = crate::Reg<ch7_dst::CH7_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch7_dst;
#[doc = "CH7_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_link`]
module"]
#[doc(alias = "CH7_LINK")]
pub type Ch7Link = crate::Reg<ch7_link::CH7_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch7_link;
