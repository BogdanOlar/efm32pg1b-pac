#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software Pulse Register"]
    pub swpulse: SWPULSE,
    #[doc = "0x04 - Software Level Register"]
    pub swlevel: SWLEVEL,
    #[doc = "0x08 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    #[doc = "0x14 - I/O Routing Location Register"]
    pub routeloc1: ROUTELOC1,
    #[doc = "0x18 - I/O Routing Location Register"]
    pub routeloc2: ROUTELOC2,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x24 - DMA Request 0 Register"]
    pub dmareq0: DMAREQ0,
    #[doc = "0x28 - DMA Request 1 Register"]
    pub dmareq1: DMAREQ1,
    _reserved9: [u8; 0x04],
    #[doc = "0x30 - PRS Channel Values"]
    pub peek: PEEK,
    _reserved10: [u8; 0x0c],
    #[doc = "0x40 - Channel Control Register"]
    pub ch0_ctrl: CH0_CTRL,
    #[doc = "0x44 - Channel Control Register"]
    pub ch1_ctrl: CH1_CTRL,
    #[doc = "0x48 - Channel Control Register"]
    pub ch2_ctrl: CH2_CTRL,
    #[doc = "0x4c - Channel Control Register"]
    pub ch3_ctrl: CH3_CTRL,
    #[doc = "0x50 - Channel Control Register"]
    pub ch4_ctrl: CH4_CTRL,
    #[doc = "0x54 - Channel Control Register"]
    pub ch5_ctrl: CH5_CTRL,
    #[doc = "0x58 - Channel Control Register"]
    pub ch6_ctrl: CH6_CTRL,
    #[doc = "0x5c - Channel Control Register"]
    pub ch7_ctrl: CH7_CTRL,
    #[doc = "0x60 - Channel Control Register"]
    pub ch8_ctrl: CH8_CTRL,
    #[doc = "0x64 - Channel Control Register"]
    pub ch9_ctrl: CH9_CTRL,
    #[doc = "0x68 - Channel Control Register"]
    pub ch10_ctrl: CH10_CTRL,
    #[doc = "0x6c - Channel Control Register"]
    pub ch11_ctrl: CH11_CTRL,
}
#[doc = "SWPULSE (w) register accessor: Software Pulse Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpulse::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swpulse`]
module"]
pub type SWPULSE = crate::Reg<swpulse::SWPULSE_SPEC>;
#[doc = "Software Pulse Register"]
pub mod swpulse;
#[doc = "SWLEVEL (rw) register accessor: Software Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swlevel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swlevel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swlevel`]
module"]
pub type SWLEVEL = crate::Reg<swlevel::SWLEVEL_SPEC>;
#[doc = "Software Level Register"]
pub mod swlevel;
#[doc = "ROUTEPEN (rw) register accessor: I/O Routing Pin Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`]
module"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`]
module"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "ROUTELOC1 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc1`]
module"]
pub type ROUTELOC1 = crate::Reg<routeloc1::ROUTELOC1_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
#[doc = "ROUTELOC2 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc2`]
module"]
pub type ROUTELOC2 = crate::Reg<routeloc2::ROUTELOC2_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc2;
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "DMAREQ0 (rw) register accessor: DMA Request 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmareq0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmareq0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmareq0`]
module"]
pub type DMAREQ0 = crate::Reg<dmareq0::DMAREQ0_SPEC>;
#[doc = "DMA Request 0 Register"]
pub mod dmareq0;
#[doc = "DMAREQ1 (rw) register accessor: DMA Request 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmareq1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmareq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmareq1`]
module"]
pub type DMAREQ1 = crate::Reg<dmareq1::DMAREQ1_SPEC>;
#[doc = "DMA Request 1 Register"]
pub mod dmareq1;
#[doc = "PEEK (r) register accessor: PRS Channel Values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek`]
module"]
pub type PEEK = crate::Reg<peek::PEEK_SPEC>;
#[doc = "PRS Channel Values"]
pub mod peek;
#[doc = "CH0_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_ctrl`]
module"]
pub type CH0_CTRL = crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "CH1_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ctrl`]
module"]
pub type CH1_CTRL = crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "CH2_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_ctrl`]
module"]
pub type CH2_CTRL = crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "CH3_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_ctrl`]
module"]
pub type CH3_CTRL = crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "CH4_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_ctrl`]
module"]
pub type CH4_CTRL = crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "CH5_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_ctrl`]
module"]
pub type CH5_CTRL = crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;
#[doc = "CH6_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_ctrl`]
module"]
pub type CH6_CTRL = crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch6_ctrl;
#[doc = "CH7_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_ctrl`]
module"]
pub type CH7_CTRL = crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch7_ctrl;
#[doc = "CH8_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_ctrl`]
module"]
pub type CH8_CTRL = crate::Reg<ch8_ctrl::CH8_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch8_ctrl;
#[doc = "CH9_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch9_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_ctrl`]
module"]
pub type CH9_CTRL = crate::Reg<ch9_ctrl::CH9_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch9_ctrl;
#[doc = "CH10_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch10_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_ctrl`]
module"]
pub type CH10_CTRL = crate::Reg<ch10_ctrl::CH10_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch10_ctrl;
#[doc = "CH11_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch11_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_ctrl`]
module"]
pub type CH11_CTRL = crate::Reg<ch11_ctrl::CH11_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch11_ctrl;
