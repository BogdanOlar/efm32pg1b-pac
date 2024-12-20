#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    swpulse: Swpulse,
    swlevel: Swlevel,
    routepen: Routepen,
    _reserved3: [u8; 0x04],
    routeloc0: Routeloc0,
    routeloc1: Routeloc1,
    routeloc2: Routeloc2,
    _reserved6: [u8; 0x04],
    ctrl: Ctrl,
    dmareq0: Dmareq0,
    dmareq1: Dmareq1,
    _reserved9: [u8; 0x04],
    peek: Peek,
    _reserved10: [u8; 0x0c],
    ch0_ctrl: Ch0Ctrl,
    ch1_ctrl: Ch1Ctrl,
    ch2_ctrl: Ch2Ctrl,
    ch3_ctrl: Ch3Ctrl,
    ch4_ctrl: Ch4Ctrl,
    ch5_ctrl: Ch5Ctrl,
    ch6_ctrl: Ch6Ctrl,
    ch7_ctrl: Ch7Ctrl,
    ch8_ctrl: Ch8Ctrl,
    ch9_ctrl: Ch9Ctrl,
    ch10_ctrl: Ch10Ctrl,
    ch11_ctrl: Ch11Ctrl,
}
impl RegisterBlock {
    ///0x00 - Software Pulse Register
    #[inline(always)]
    pub const fn swpulse(&self) -> &Swpulse {
        &self.swpulse
    }
    ///0x04 - Software Level Register
    #[inline(always)]
    pub const fn swlevel(&self) -> &Swlevel {
        &self.swlevel
    }
    ///0x08 - I/O Routing Pin Enable Register
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    ///0x10 - I/O Routing Location Register
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
    ///0x14 - I/O Routing Location Register
    #[inline(always)]
    pub const fn routeloc1(&self) -> &Routeloc1 {
        &self.routeloc1
    }
    ///0x18 - I/O Routing Location Register
    #[inline(always)]
    pub const fn routeloc2(&self) -> &Routeloc2 {
        &self.routeloc2
    }
    ///0x20 - Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x24 - DMA Request 0 Register
    #[inline(always)]
    pub const fn dmareq0(&self) -> &Dmareq0 {
        &self.dmareq0
    }
    ///0x28 - DMA Request 1 Register
    #[inline(always)]
    pub const fn dmareq1(&self) -> &Dmareq1 {
        &self.dmareq1
    }
    ///0x30 - PRS Channel Values
    #[inline(always)]
    pub const fn peek(&self) -> &Peek {
        &self.peek
    }
    ///0x40 - Channel Control Register
    #[inline(always)]
    pub const fn ch0_ctrl(&self) -> &Ch0Ctrl {
        &self.ch0_ctrl
    }
    ///0x44 - Channel Control Register
    #[inline(always)]
    pub const fn ch1_ctrl(&self) -> &Ch1Ctrl {
        &self.ch1_ctrl
    }
    ///0x48 - Channel Control Register
    #[inline(always)]
    pub const fn ch2_ctrl(&self) -> &Ch2Ctrl {
        &self.ch2_ctrl
    }
    ///0x4c - Channel Control Register
    #[inline(always)]
    pub const fn ch3_ctrl(&self) -> &Ch3Ctrl {
        &self.ch3_ctrl
    }
    ///0x50 - Channel Control Register
    #[inline(always)]
    pub const fn ch4_ctrl(&self) -> &Ch4Ctrl {
        &self.ch4_ctrl
    }
    ///0x54 - Channel Control Register
    #[inline(always)]
    pub const fn ch5_ctrl(&self) -> &Ch5Ctrl {
        &self.ch5_ctrl
    }
    ///0x58 - Channel Control Register
    #[inline(always)]
    pub const fn ch6_ctrl(&self) -> &Ch6Ctrl {
        &self.ch6_ctrl
    }
    ///0x5c - Channel Control Register
    #[inline(always)]
    pub const fn ch7_ctrl(&self) -> &Ch7Ctrl {
        &self.ch7_ctrl
    }
    ///0x60 - Channel Control Register
    #[inline(always)]
    pub const fn ch8_ctrl(&self) -> &Ch8Ctrl {
        &self.ch8_ctrl
    }
    ///0x64 - Channel Control Register
    #[inline(always)]
    pub const fn ch9_ctrl(&self) -> &Ch9Ctrl {
        &self.ch9_ctrl
    }
    ///0x68 - Channel Control Register
    #[inline(always)]
    pub const fn ch10_ctrl(&self) -> &Ch10Ctrl {
        &self.ch10_ctrl
    }
    ///0x6c - Channel Control Register
    #[inline(always)]
    pub const fn ch11_ctrl(&self) -> &Ch11Ctrl {
        &self.ch11_ctrl
    }
}
///SWPULSE (w) register accessor: Software Pulse Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpulse::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@swpulse`]
///module
#[doc(alias = "SWPULSE")]
pub type Swpulse = crate::Reg<swpulse::SWPULSErs>;
///Software Pulse Register
pub mod swpulse;
///SWLEVEL (rw) register accessor: Software Level Register
///
///You can [`read`](crate::Reg::read) this register and get [`swlevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swlevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@swlevel`]
///module
#[doc(alias = "SWLEVEL")]
pub type Swlevel = crate::Reg<swlevel::SWLEVELrs>;
///Software Level Register
pub mod swlevel;
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
///ROUTELOC2 (rw) register accessor: I/O Routing Location Register
///
///You can [`read`](crate::Reg::read) this register and get [`routeloc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@routeloc2`]
///module
#[doc(alias = "ROUTELOC2")]
pub type Routeloc2 = crate::Reg<routeloc2::ROUTELOC2rs>;
///I/O Routing Location Register
pub mod routeloc2;
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
///DMAREQ0 (rw) register accessor: DMA Request 0 Register
///
///You can [`read`](crate::Reg::read) this register and get [`dmareq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmareq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dmareq0`]
///module
#[doc(alias = "DMAREQ0")]
pub type Dmareq0 = crate::Reg<dmareq0::DMAREQ0rs>;
///DMA Request 0 Register
pub mod dmareq0;
///DMAREQ1 (rw) register accessor: DMA Request 1 Register
///
///You can [`read`](crate::Reg::read) this register and get [`dmareq1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmareq1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dmareq1`]
///module
#[doc(alias = "DMAREQ1")]
pub type Dmareq1 = crate::Reg<dmareq1::DMAREQ1rs>;
///DMA Request 1 Register
pub mod dmareq1;
///PEEK (r) register accessor: PRS Channel Values
///
///You can [`read`](crate::Reg::read) this register and get [`peek::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@peek`]
///module
#[doc(alias = "PEEK")]
pub type Peek = crate::Reg<peek::PEEKrs>;
///PRS Channel Values
pub mod peek;
///CH0_CTRL (rw) register accessor: Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ch0_ctrl`]
///module
#[doc(alias = "CH0_CTRL")]
pub type Ch0Ctrl = crate::Reg<ch0_ctrl::CH0_CTRLrs>;
///Channel Control Register
pub mod ch0_ctrl;
///CH1_CTRL (rw) register accessor: Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ch1_ctrl`]
///module
#[doc(alias = "CH1_CTRL")]
pub type Ch1Ctrl = crate::Reg<ch1_ctrl::CH1_CTRLrs>;
///Channel Control Register
pub mod ch1_ctrl;
///CH2_CTRL (rw) register accessor: Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ch2_ctrl`]
///module
#[doc(alias = "CH2_CTRL")]
pub type Ch2Ctrl = crate::Reg<ch2_ctrl::CH2_CTRLrs>;
///Channel Control Register
pub mod ch2_ctrl;
///CH3_CTRL (rw) register accessor: Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ch3_ctrl`]
///module
#[doc(alias = "CH3_CTRL")]
pub type Ch3Ctrl = crate::Reg<ch3_ctrl::CH3_CTRLrs>;
///Channel Control Register
pub mod ch3_ctrl;
///CH4_CTRL (rw) register accessor: Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch4_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ch4_ctrl`]
///module
#[doc(alias = "CH4_CTRL")]
pub type Ch4Ctrl = crate::Reg<ch4_ctrl::CH4_CTRLrs>;
///Channel Control Register
pub mod ch4_ctrl;
///CH5_CTRL (rw) register accessor: Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch5_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ch5_ctrl`]
///module
#[doc(alias = "CH5_CTRL")]
pub type Ch5Ctrl = crate::Reg<ch5_ctrl::CH5_CTRLrs>;
///Channel Control Register
pub mod ch5_ctrl;
///CH6_CTRL (rw) register accessor: Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch6_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ch6_ctrl`]
///module
#[doc(alias = "CH6_CTRL")]
pub type Ch6Ctrl = crate::Reg<ch6_ctrl::CH6_CTRLrs>;
///Channel Control Register
pub mod ch6_ctrl;
///CH7_CTRL (rw) register accessor: Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch7_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ch7_ctrl`]
///module
#[doc(alias = "CH7_CTRL")]
pub type Ch7Ctrl = crate::Reg<ch7_ctrl::CH7_CTRLrs>;
///Channel Control Register
pub mod ch7_ctrl;
///CH8_CTRL (rw) register accessor: Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch8_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ch8_ctrl`]
///module
#[doc(alias = "CH8_CTRL")]
pub type Ch8Ctrl = crate::Reg<ch8_ctrl::CH8_CTRLrs>;
///Channel Control Register
pub mod ch8_ctrl;
///CH9_CTRL (rw) register accessor: Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch9_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ch9_ctrl`]
///module
#[doc(alias = "CH9_CTRL")]
pub type Ch9Ctrl = crate::Reg<ch9_ctrl::CH9_CTRLrs>;
///Channel Control Register
pub mod ch9_ctrl;
///CH10_CTRL (rw) register accessor: Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch10_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ch10_ctrl`]
///module
#[doc(alias = "CH10_CTRL")]
pub type Ch10Ctrl = crate::Reg<ch10_ctrl::CH10_CTRLrs>;
///Channel Control Register
pub mod ch10_ctrl;
///CH11_CTRL (rw) register accessor: Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch11_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ch11_ctrl`]
///module
#[doc(alias = "CH11_CTRL")]
pub type Ch11Ctrl = crate::Reg<ch11_ctrl::CH11_CTRLrs>;
///Channel Control Register
pub mod ch11_ctrl;
