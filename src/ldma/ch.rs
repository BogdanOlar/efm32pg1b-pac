#[repr(C)]
#[derive(Debug)]
///Channel
#[doc(alias = "CH")]
pub struct Ch {
    reqsel: Reqsel,
    cfg: Cfg,
    loop_: Loop,
    ctrl: Ctrl,
    src: Src,
    dst: Dst,
    link: Link,
    _reserved_end: [u8; 0x14],
}
impl Ch {
    ///0x00 - Channel Peripheral Request Select Register
    #[inline(always)]
    pub const fn reqsel(&self) -> &Reqsel {
        &self.reqsel
    }
    ///0x04 - Channel Configuration Register
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    ///0x08 - Channel Loop Counter Register
    #[inline(always)]
    pub const fn loop_(&self) -> &Loop {
        &self.loop_
    }
    ///0x0c - Channel Descriptor Control Word Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x10 - Channel Descriptor Source Data Address Register
    #[inline(always)]
    pub const fn src(&self) -> &Src {
        &self.src
    }
    ///0x14 - Channel Descriptor Destination Data Address Register
    #[inline(always)]
    pub const fn dst(&self) -> &Dst {
        &self.dst
    }
    ///0x18 - Channel Descriptor Link Structure Address Register
    #[inline(always)]
    pub const fn link(&self) -> &Link {
        &self.link
    }
}
///REQSEL (rw) register accessor: Channel Peripheral Request Select Register
///
///You can [`read`](crate::Reg::read) this register and get [`reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@reqsel`] module
#[doc(alias = "REQSEL")]
pub type Reqsel = crate::Reg<reqsel::REQSELrs>;
///Channel Peripheral Request Select Register
pub mod reqsel;
///CFG (rw) register accessor: Channel Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cfg`] module
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CFGrs>;
///Channel Configuration Register
pub mod cfg;
///LOOP (rw) register accessor: Channel Loop Counter Register
///
///You can [`read`](crate::Reg::read) this register and get [`loop_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loop_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@loop_`] module
#[doc(alias = "LOOP")]
pub type Loop = crate::Reg<loop_::LOOPrs>;
///Channel Loop Counter Register
pub mod loop_;
///CTRL (rw) register accessor: Channel Descriptor Control Word Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ctrl`] module
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CTRLrs>;
///Channel Descriptor Control Word Register
pub mod ctrl;
///SRC (rw) register accessor: Channel Descriptor Source Data Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@src`] module
#[doc(alias = "SRC")]
pub type Src = crate::Reg<src::SRCrs>;
///Channel Descriptor Source Data Address Register
pub mod src;
///DST (rw) register accessor: Channel Descriptor Destination Data Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dst`] module
#[doc(alias = "DST")]
pub type Dst = crate::Reg<dst::DSTrs>;
///Channel Descriptor Destination Data Address Register
pub mod dst;
///LINK (rw) register accessor: Channel Descriptor Link Structure Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@link`] module
#[doc(alias = "LINK")]
pub type Link = crate::Reg<link::LINKrs>;
///Channel Descriptor Link Structure Address Register
pub mod link;
