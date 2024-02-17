#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    status: STATUS,
    sync: SYNC,
    _reserved3: [u8; 0x14],
    chen: CHEN,
    chbusy: CHBUSY,
    chdone: CHDONE,
    dbghalt: DBGHALT,
    swreq: SWREQ,
    reqdis: REQDIS,
    reqpend: REQPEND,
    linkload: LINKLOAD,
    reqclear: REQCLEAR,
    _reserved12: [u8; 0x1c],
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    _reserved16: [u8; 0x10],
    ch0_reqsel: CH0_REQSEL,
    ch0_cfg: CH0_CFG,
    ch0_loop: CH0_LOOP,
    ch0_ctrl: CH0_CTRL,
    ch0_src: CH0_SRC,
    ch0_dst: CH0_DST,
    ch0_link: CH0_LINK,
    _reserved23: [u8; 0x14],
    ch1_reqsel: CH1_REQSEL,
    ch1_cfg: CH1_CFG,
    ch1_loop: CH1_LOOP,
    ch1_ctrl: CH1_CTRL,
    ch1_src: CH1_SRC,
    ch1_dst: CH1_DST,
    ch1_link: CH1_LINK,
    _reserved30: [u8; 0x14],
    ch2_reqsel: CH2_REQSEL,
    ch2_cfg: CH2_CFG,
    ch2_loop: CH2_LOOP,
    ch2_ctrl: CH2_CTRL,
    ch2_src: CH2_SRC,
    ch2_dst: CH2_DST,
    ch2_link: CH2_LINK,
    _reserved37: [u8; 0x14],
    ch3_reqsel: CH3_REQSEL,
    ch3_cfg: CH3_CFG,
    ch3_loop: CH3_LOOP,
    ch3_ctrl: CH3_CTRL,
    ch3_src: CH3_SRC,
    ch3_dst: CH3_DST,
    ch3_link: CH3_LINK,
    _reserved44: [u8; 0x14],
    ch4_reqsel: CH4_REQSEL,
    ch4_cfg: CH4_CFG,
    ch4_loop: CH4_LOOP,
    ch4_ctrl: CH4_CTRL,
    ch4_src: CH4_SRC,
    ch4_dst: CH4_DST,
    ch4_link: CH4_LINK,
    _reserved51: [u8; 0x14],
    ch5_reqsel: CH5_REQSEL,
    ch5_cfg: CH5_CFG,
    ch5_loop: CH5_LOOP,
    ch5_ctrl: CH5_CTRL,
    ch5_src: CH5_SRC,
    ch5_dst: CH5_DST,
    ch5_link: CH5_LINK,
    _reserved58: [u8; 0x14],
    ch6_reqsel: CH6_REQSEL,
    ch6_cfg: CH6_CFG,
    ch6_loop: CH6_LOOP,
    ch6_ctrl: CH6_CTRL,
    ch6_src: CH6_SRC,
    ch6_dst: CH6_DST,
    ch6_link: CH6_LINK,
    _reserved65: [u8; 0x14],
    ch7_reqsel: CH7_REQSEL,
    ch7_cfg: CH7_CFG,
    ch7_loop: CH7_LOOP,
    ch7_ctrl: CH7_CTRL,
    ch7_src: CH7_SRC,
    ch7_dst: CH7_DST,
    ch7_link: CH7_LINK,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - DMA Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - DMA Synchronization Trigger Register (Single-Cycle RMW)"]
    #[inline(always)]
    pub const fn sync(&self) -> &SYNC {
        &self.sync
    }
    #[doc = "0x20 - DMA Channel Enable Register (Single-Cycle RMW)"]
    #[inline(always)]
    pub const fn chen(&self) -> &CHEN {
        &self.chen
    }
    #[doc = "0x24 - DMA Channel Busy Register"]
    #[inline(always)]
    pub const fn chbusy(&self) -> &CHBUSY {
        &self.chbusy
    }
    #[doc = "0x28 - DMA Channel Linking Done Register (Single-Cycle RMW)"]
    #[inline(always)]
    pub const fn chdone(&self) -> &CHDONE {
        &self.chdone
    }
    #[doc = "0x2c - DMA Channel Debug Halt Register"]
    #[inline(always)]
    pub const fn dbghalt(&self) -> &DBGHALT {
        &self.dbghalt
    }
    #[doc = "0x30 - DMA Channel Software Transfer Request Register"]
    #[inline(always)]
    pub const fn swreq(&self) -> &SWREQ {
        &self.swreq
    }
    #[doc = "0x34 - DMA Channel Request Disable Register"]
    #[inline(always)]
    pub const fn reqdis(&self) -> &REQDIS {
        &self.reqdis
    }
    #[doc = "0x38 - DMA Channel Requests Pending Register"]
    #[inline(always)]
    pub const fn reqpend(&self) -> &REQPEND {
        &self.reqpend
    }
    #[doc = "0x3c - DMA Channel Link Load Register"]
    #[inline(always)]
    pub const fn linkload(&self) -> &LINKLOAD {
        &self.linkload
    }
    #[doc = "0x40 - DMA Channel Request Clear Register"]
    #[inline(always)]
    pub const fn reqclear(&self) -> &REQCLEAR {
        &self.reqclear
    }
    #[doc = "0x60 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x64 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x68 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x6c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x80 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch0_reqsel(&self) -> &CH0_REQSEL {
        &self.ch0_reqsel
    }
    #[doc = "0x84 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch0_cfg(&self) -> &CH0_CFG {
        &self.ch0_cfg
    }
    #[doc = "0x88 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch0_loop(&self) -> &CH0_LOOP {
        &self.ch0_loop
    }
    #[doc = "0x8c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch0_ctrl(&self) -> &CH0_CTRL {
        &self.ch0_ctrl
    }
    #[doc = "0x90 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch0_src(&self) -> &CH0_SRC {
        &self.ch0_src
    }
    #[doc = "0x94 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch0_dst(&self) -> &CH0_DST {
        &self.ch0_dst
    }
    #[doc = "0x98 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch0_link(&self) -> &CH0_LINK {
        &self.ch0_link
    }
    #[doc = "0xb0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch1_reqsel(&self) -> &CH1_REQSEL {
        &self.ch1_reqsel
    }
    #[doc = "0xb4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch1_cfg(&self) -> &CH1_CFG {
        &self.ch1_cfg
    }
    #[doc = "0xb8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch1_loop(&self) -> &CH1_LOOP {
        &self.ch1_loop
    }
    #[doc = "0xbc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch1_ctrl(&self) -> &CH1_CTRL {
        &self.ch1_ctrl
    }
    #[doc = "0xc0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch1_src(&self) -> &CH1_SRC {
        &self.ch1_src
    }
    #[doc = "0xc4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch1_dst(&self) -> &CH1_DST {
        &self.ch1_dst
    }
    #[doc = "0xc8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch1_link(&self) -> &CH1_LINK {
        &self.ch1_link
    }
    #[doc = "0xe0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch2_reqsel(&self) -> &CH2_REQSEL {
        &self.ch2_reqsel
    }
    #[doc = "0xe4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch2_cfg(&self) -> &CH2_CFG {
        &self.ch2_cfg
    }
    #[doc = "0xe8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch2_loop(&self) -> &CH2_LOOP {
        &self.ch2_loop
    }
    #[doc = "0xec - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch2_ctrl(&self) -> &CH2_CTRL {
        &self.ch2_ctrl
    }
    #[doc = "0xf0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch2_src(&self) -> &CH2_SRC {
        &self.ch2_src
    }
    #[doc = "0xf4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch2_dst(&self) -> &CH2_DST {
        &self.ch2_dst
    }
    #[doc = "0xf8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch2_link(&self) -> &CH2_LINK {
        &self.ch2_link
    }
    #[doc = "0x110 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch3_reqsel(&self) -> &CH3_REQSEL {
        &self.ch3_reqsel
    }
    #[doc = "0x114 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch3_cfg(&self) -> &CH3_CFG {
        &self.ch3_cfg
    }
    #[doc = "0x118 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch3_loop(&self) -> &CH3_LOOP {
        &self.ch3_loop
    }
    #[doc = "0x11c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch3_ctrl(&self) -> &CH3_CTRL {
        &self.ch3_ctrl
    }
    #[doc = "0x120 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch3_src(&self) -> &CH3_SRC {
        &self.ch3_src
    }
    #[doc = "0x124 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch3_dst(&self) -> &CH3_DST {
        &self.ch3_dst
    }
    #[doc = "0x128 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch3_link(&self) -> &CH3_LINK {
        &self.ch3_link
    }
    #[doc = "0x140 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch4_reqsel(&self) -> &CH4_REQSEL {
        &self.ch4_reqsel
    }
    #[doc = "0x144 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch4_cfg(&self) -> &CH4_CFG {
        &self.ch4_cfg
    }
    #[doc = "0x148 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch4_loop(&self) -> &CH4_LOOP {
        &self.ch4_loop
    }
    #[doc = "0x14c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch4_ctrl(&self) -> &CH4_CTRL {
        &self.ch4_ctrl
    }
    #[doc = "0x150 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch4_src(&self) -> &CH4_SRC {
        &self.ch4_src
    }
    #[doc = "0x154 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch4_dst(&self) -> &CH4_DST {
        &self.ch4_dst
    }
    #[doc = "0x158 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch4_link(&self) -> &CH4_LINK {
        &self.ch4_link
    }
    #[doc = "0x170 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch5_reqsel(&self) -> &CH5_REQSEL {
        &self.ch5_reqsel
    }
    #[doc = "0x174 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch5_cfg(&self) -> &CH5_CFG {
        &self.ch5_cfg
    }
    #[doc = "0x178 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch5_loop(&self) -> &CH5_LOOP {
        &self.ch5_loop
    }
    #[doc = "0x17c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch5_ctrl(&self) -> &CH5_CTRL {
        &self.ch5_ctrl
    }
    #[doc = "0x180 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch5_src(&self) -> &CH5_SRC {
        &self.ch5_src
    }
    #[doc = "0x184 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch5_dst(&self) -> &CH5_DST {
        &self.ch5_dst
    }
    #[doc = "0x188 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch5_link(&self) -> &CH5_LINK {
        &self.ch5_link
    }
    #[doc = "0x1a0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch6_reqsel(&self) -> &CH6_REQSEL {
        &self.ch6_reqsel
    }
    #[doc = "0x1a4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch6_cfg(&self) -> &CH6_CFG {
        &self.ch6_cfg
    }
    #[doc = "0x1a8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch6_loop(&self) -> &CH6_LOOP {
        &self.ch6_loop
    }
    #[doc = "0x1ac - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch6_ctrl(&self) -> &CH6_CTRL {
        &self.ch6_ctrl
    }
    #[doc = "0x1b0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch6_src(&self) -> &CH6_SRC {
        &self.ch6_src
    }
    #[doc = "0x1b4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch6_dst(&self) -> &CH6_DST {
        &self.ch6_dst
    }
    #[doc = "0x1b8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch6_link(&self) -> &CH6_LINK {
        &self.ch6_link
    }
    #[doc = "0x1d0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch7_reqsel(&self) -> &CH7_REQSEL {
        &self.ch7_reqsel
    }
    #[doc = "0x1d4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch7_cfg(&self) -> &CH7_CFG {
        &self.ch7_cfg
    }
    #[doc = "0x1d8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch7_loop(&self) -> &CH7_LOOP {
        &self.ch7_loop
    }
    #[doc = "0x1dc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch7_ctrl(&self) -> &CH7_CTRL {
        &self.ch7_ctrl
    }
    #[doc = "0x1e0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch7_src(&self) -> &CH7_SRC {
        &self.ch7_src
    }
    #[doc = "0x1e4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch7_dst(&self) -> &CH7_DST {
        &self.ch7_dst
    }
    #[doc = "0x1e8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch7_link(&self) -> &CH7_LINK {
        &self.ch7_link
    }
}
#[doc = "CTRL (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRLrs>;
#[doc = "DMA Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: DMA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUSrs>;
#[doc = "DMA Status Register"]
pub mod status;
#[doc = "SYNC (rw) register accessor: DMA Synchronization Trigger Register (Single-Cycle RMW)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync`]
module"]
pub type SYNC = crate::Reg<sync::SYNCrs>;
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)"]
pub mod sync;
#[doc = "CHEN (rw) register accessor: DMA Channel Enable Register (Single-Cycle RMW)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen`]
module"]
pub type CHEN = crate::Reg<chen::CHENrs>;
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)"]
pub mod chen;
#[doc = "CHBUSY (r) register accessor: DMA Channel Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chbusy`]
module"]
pub type CHBUSY = crate::Reg<chbusy::CHBUSYrs>;
#[doc = "DMA Channel Busy Register"]
pub mod chbusy;
#[doc = "CHDONE (rw) register accessor: DMA Channel Linking Done Register (Single-Cycle RMW)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chdone::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdone::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdone`]
module"]
pub type CHDONE = crate::Reg<chdone::CHDONErs>;
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)"]
pub mod chdone;
#[doc = "DBGHALT (rw) register accessor: DMA Channel Debug Halt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbghalt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbghalt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbghalt`]
module"]
pub type DBGHALT = crate::Reg<dbghalt::DBGHALTrs>;
#[doc = "DMA Channel Debug Halt Register"]
pub mod dbghalt;
#[doc = "SWREQ (w) register accessor: DMA Channel Software Transfer Request Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreq`]
module"]
pub type SWREQ = crate::Reg<swreq::SWREQrs>;
#[doc = "DMA Channel Software Transfer Request Register"]
pub mod swreq;
#[doc = "REQDIS (rw) register accessor: DMA Channel Request Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqdis`]
module"]
pub type REQDIS = crate::Reg<reqdis::REQDISrs>;
#[doc = "DMA Channel Request Disable Register"]
pub mod reqdis;
#[doc = "REQPEND (r) register accessor: DMA Channel Requests Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqpend::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqpend`]
module"]
pub type REQPEND = crate::Reg<reqpend::REQPENDrs>;
#[doc = "DMA Channel Requests Pending Register"]
pub mod reqpend;
#[doc = "LINKLOAD (w) register accessor: DMA Channel Link Load Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linkload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linkload`]
module"]
pub type LINKLOAD = crate::Reg<linkload::LINKLOADrs>;
#[doc = "DMA Channel Link Load Register"]
pub mod linkload;
#[doc = "REQCLEAR (w) register accessor: DMA Channel Request Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqclear`]
module"]
pub type REQCLEAR = crate::Reg<reqclear::REQCLEARrs>;
#[doc = "DMA Channel Request Clear Register"]
pub mod reqclear;
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
#[doc = "CH0_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_reqsel`]
module"]
pub type CH0_REQSEL = crate::Reg<ch0_reqsel::CH0_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch0_reqsel;
#[doc = "CH0_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_cfg`]
module"]
pub type CH0_CFG = crate::Reg<ch0_cfg::CH0_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch0_cfg;
#[doc = "CH0_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_loop`]
module"]
pub type CH0_LOOP = crate::Reg<ch0_loop::CH0_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch0_loop;
#[doc = "CH0_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_ctrl`]
module"]
pub type CH0_CTRL = crate::Reg<ch0_ctrl::CH0_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch0_ctrl;
#[doc = "CH0_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_src`]
module"]
pub type CH0_SRC = crate::Reg<ch0_src::CH0_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch0_src;
#[doc = "CH0_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_dst`]
module"]
pub type CH0_DST = crate::Reg<ch0_dst::CH0_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch0_dst;
#[doc = "CH0_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_link`]
module"]
pub type CH0_LINK = crate::Reg<ch0_link::CH0_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch0_link;
#[doc = "CH1_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_reqsel`]
module"]
pub type CH1_REQSEL = crate::Reg<ch1_reqsel::CH1_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch1_reqsel;
#[doc = "CH1_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_cfg`]
module"]
pub type CH1_CFG = crate::Reg<ch1_cfg::CH1_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch1_cfg;
#[doc = "CH1_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_loop`]
module"]
pub type CH1_LOOP = crate::Reg<ch1_loop::CH1_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch1_loop;
#[doc = "CH1_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ctrl`]
module"]
pub type CH1_CTRL = crate::Reg<ch1_ctrl::CH1_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch1_ctrl;
#[doc = "CH1_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_src`]
module"]
pub type CH1_SRC = crate::Reg<ch1_src::CH1_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch1_src;
#[doc = "CH1_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dst`]
module"]
pub type CH1_DST = crate::Reg<ch1_dst::CH1_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch1_dst;
#[doc = "CH1_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_link`]
module"]
pub type CH1_LINK = crate::Reg<ch1_link::CH1_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch1_link;
#[doc = "CH2_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_reqsel`]
module"]
pub type CH2_REQSEL = crate::Reg<ch2_reqsel::CH2_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch2_reqsel;
#[doc = "CH2_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_cfg`]
module"]
pub type CH2_CFG = crate::Reg<ch2_cfg::CH2_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch2_cfg;
#[doc = "CH2_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_loop`]
module"]
pub type CH2_LOOP = crate::Reg<ch2_loop::CH2_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch2_loop;
#[doc = "CH2_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_ctrl`]
module"]
pub type CH2_CTRL = crate::Reg<ch2_ctrl::CH2_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch2_ctrl;
#[doc = "CH2_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_src`]
module"]
pub type CH2_SRC = crate::Reg<ch2_src::CH2_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch2_src;
#[doc = "CH2_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_dst`]
module"]
pub type CH2_DST = crate::Reg<ch2_dst::CH2_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch2_dst;
#[doc = "CH2_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_link`]
module"]
pub type CH2_LINK = crate::Reg<ch2_link::CH2_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch2_link;
#[doc = "CH3_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_reqsel`]
module"]
pub type CH3_REQSEL = crate::Reg<ch3_reqsel::CH3_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch3_reqsel;
#[doc = "CH3_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_cfg`]
module"]
pub type CH3_CFG = crate::Reg<ch3_cfg::CH3_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch3_cfg;
#[doc = "CH3_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_loop`]
module"]
pub type CH3_LOOP = crate::Reg<ch3_loop::CH3_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch3_loop;
#[doc = "CH3_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_ctrl`]
module"]
pub type CH3_CTRL = crate::Reg<ch3_ctrl::CH3_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch3_ctrl;
#[doc = "CH3_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_src`]
module"]
pub type CH3_SRC = crate::Reg<ch3_src::CH3_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch3_src;
#[doc = "CH3_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_dst`]
module"]
pub type CH3_DST = crate::Reg<ch3_dst::CH3_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch3_dst;
#[doc = "CH3_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_link`]
module"]
pub type CH3_LINK = crate::Reg<ch3_link::CH3_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch3_link;
#[doc = "CH4_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_reqsel`]
module"]
pub type CH4_REQSEL = crate::Reg<ch4_reqsel::CH4_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch4_reqsel;
#[doc = "CH4_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_cfg`]
module"]
pub type CH4_CFG = crate::Reg<ch4_cfg::CH4_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch4_cfg;
#[doc = "CH4_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_loop`]
module"]
pub type CH4_LOOP = crate::Reg<ch4_loop::CH4_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch4_loop;
#[doc = "CH4_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_ctrl`]
module"]
pub type CH4_CTRL = crate::Reg<ch4_ctrl::CH4_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch4_ctrl;
#[doc = "CH4_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_src`]
module"]
pub type CH4_SRC = crate::Reg<ch4_src::CH4_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch4_src;
#[doc = "CH4_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_dst`]
module"]
pub type CH4_DST = crate::Reg<ch4_dst::CH4_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch4_dst;
#[doc = "CH4_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_link`]
module"]
pub type CH4_LINK = crate::Reg<ch4_link::CH4_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch4_link;
#[doc = "CH5_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_reqsel`]
module"]
pub type CH5_REQSEL = crate::Reg<ch5_reqsel::CH5_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch5_reqsel;
#[doc = "CH5_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_cfg`]
module"]
pub type CH5_CFG = crate::Reg<ch5_cfg::CH5_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch5_cfg;
#[doc = "CH5_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_loop`]
module"]
pub type CH5_LOOP = crate::Reg<ch5_loop::CH5_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch5_loop;
#[doc = "CH5_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_ctrl`]
module"]
pub type CH5_CTRL = crate::Reg<ch5_ctrl::CH5_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch5_ctrl;
#[doc = "CH5_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_src`]
module"]
pub type CH5_SRC = crate::Reg<ch5_src::CH5_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch5_src;
#[doc = "CH5_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_dst`]
module"]
pub type CH5_DST = crate::Reg<ch5_dst::CH5_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch5_dst;
#[doc = "CH5_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_link`]
module"]
pub type CH5_LINK = crate::Reg<ch5_link::CH5_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch5_link;
#[doc = "CH6_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_reqsel`]
module"]
pub type CH6_REQSEL = crate::Reg<ch6_reqsel::CH6_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch6_reqsel;
#[doc = "CH6_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_cfg`]
module"]
pub type CH6_CFG = crate::Reg<ch6_cfg::CH6_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch6_cfg;
#[doc = "CH6_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_loop`]
module"]
pub type CH6_LOOP = crate::Reg<ch6_loop::CH6_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch6_loop;
#[doc = "CH6_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_ctrl`]
module"]
pub type CH6_CTRL = crate::Reg<ch6_ctrl::CH6_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch6_ctrl;
#[doc = "CH6_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_src`]
module"]
pub type CH6_SRC = crate::Reg<ch6_src::CH6_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch6_src;
#[doc = "CH6_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_dst`]
module"]
pub type CH6_DST = crate::Reg<ch6_dst::CH6_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch6_dst;
#[doc = "CH6_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_link`]
module"]
pub type CH6_LINK = crate::Reg<ch6_link::CH6_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch6_link;
#[doc = "CH7_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_reqsel`]
module"]
pub type CH7_REQSEL = crate::Reg<ch7_reqsel::CH7_REQSELrs>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch7_reqsel;
#[doc = "CH7_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_cfg`]
module"]
pub type CH7_CFG = crate::Reg<ch7_cfg::CH7_CFGrs>;
#[doc = "Channel Configuration Register"]
pub mod ch7_cfg;
#[doc = "CH7_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_loop`]
module"]
pub type CH7_LOOP = crate::Reg<ch7_loop::CH7_LOOPrs>;
#[doc = "Channel Loop Counter Register"]
pub mod ch7_loop;
#[doc = "CH7_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_ctrl`]
module"]
pub type CH7_CTRL = crate::Reg<ch7_ctrl::CH7_CTRLrs>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch7_ctrl;
#[doc = "CH7_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_src`]
module"]
pub type CH7_SRC = crate::Reg<ch7_src::CH7_SRCrs>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch7_src;
#[doc = "CH7_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_dst`]
module"]
pub type CH7_DST = crate::Reg<ch7_dst::CH7_DSTrs>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch7_dst;
#[doc = "CH7_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_link`]
module"]
pub type CH7_LINK = crate::Reg<ch7_link::CH7_LINKrs>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch7_link;
