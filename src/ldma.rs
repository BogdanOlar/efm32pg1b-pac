#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - DMA Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - DMA Synchronization Trigger Register (Single-Cycle RMW)"]
    pub sync: SYNC,
    _reserved3: [u8; 20usize],
    #[doc = "0x20 - DMA Channel Enable Register (Single-Cycle RMW)"]
    pub chen: CHEN,
    #[doc = "0x24 - DMA Channel Busy Register"]
    pub chbusy: CHBUSY,
    #[doc = "0x28 - DMA Channel Linking Done Register (Single-Cycle RMW)"]
    pub chdone: CHDONE,
    #[doc = "0x2c - DMA Channel Debug Halt Register"]
    pub dbghalt: DBGHALT,
    #[doc = "0x30 - DMA Channel Software Transfer Request Register"]
    pub swreq: SWREQ,
    #[doc = "0x34 - DMA Channel Request Disable Register"]
    pub reqdis: REQDIS,
    #[doc = "0x38 - DMA Channel Requests Pending Register"]
    pub reqpend: REQPEND,
    #[doc = "0x3c - DMA Channel Link Load Register"]
    pub linkload: LINKLOAD,
    #[doc = "0x40 - DMA Channel Request Clear Register"]
    pub reqclear: REQCLEAR,
    _reserved12: [u8; 28usize],
    #[doc = "0x60 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x64 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x68 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x6c - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved16: [u8; 16usize],
    #[doc = "0x80 - Channel Peripheral Request Select Register"]
    pub ch0_reqsel: CH0_REQSEL,
    #[doc = "0x84 - Channel Configuration Register"]
    pub ch0_cfg: CH0_CFG,
    #[doc = "0x88 - Channel Loop Counter Register"]
    pub ch0_loop: CH0_LOOP,
    #[doc = "0x8c - Channel Descriptor Control Word Register"]
    pub ch0_ctrl: CH0_CTRL,
    #[doc = "0x90 - Channel Descriptor Source Data Address Register"]
    pub ch0_src: CH0_SRC,
    #[doc = "0x94 - Channel Descriptor Destination Data Address Register"]
    pub ch0_dst: CH0_DST,
    #[doc = "0x98 - Channel Descriptor Link Structure Address Register"]
    pub ch0_link: CH0_LINK,
    _reserved23: [u8; 20usize],
    #[doc = "0xb0 - Channel Peripheral Request Select Register"]
    pub ch1_reqsel: CH1_REQSEL,
    #[doc = "0xb4 - Channel Configuration Register"]
    pub ch1_cfg: CH1_CFG,
    #[doc = "0xb8 - Channel Loop Counter Register"]
    pub ch1_loop: CH1_LOOP,
    #[doc = "0xbc - Channel Descriptor Control Word Register"]
    pub ch1_ctrl: CH1_CTRL,
    #[doc = "0xc0 - Channel Descriptor Source Data Address Register"]
    pub ch1_src: CH1_SRC,
    #[doc = "0xc4 - Channel Descriptor Destination Data Address Register"]
    pub ch1_dst: CH1_DST,
    #[doc = "0xc8 - Channel Descriptor Link Structure Address Register"]
    pub ch1_link: CH1_LINK,
    _reserved30: [u8; 20usize],
    #[doc = "0xe0 - Channel Peripheral Request Select Register"]
    pub ch2_reqsel: CH2_REQSEL,
    #[doc = "0xe4 - Channel Configuration Register"]
    pub ch2_cfg: CH2_CFG,
    #[doc = "0xe8 - Channel Loop Counter Register"]
    pub ch2_loop: CH2_LOOP,
    #[doc = "0xec - Channel Descriptor Control Word Register"]
    pub ch2_ctrl: CH2_CTRL,
    #[doc = "0xf0 - Channel Descriptor Source Data Address Register"]
    pub ch2_src: CH2_SRC,
    #[doc = "0xf4 - Channel Descriptor Destination Data Address Register"]
    pub ch2_dst: CH2_DST,
    #[doc = "0xf8 - Channel Descriptor Link Structure Address Register"]
    pub ch2_link: CH2_LINK,
    _reserved37: [u8; 20usize],
    #[doc = "0x110 - Channel Peripheral Request Select Register"]
    pub ch3_reqsel: CH3_REQSEL,
    #[doc = "0x114 - Channel Configuration Register"]
    pub ch3_cfg: CH3_CFG,
    #[doc = "0x118 - Channel Loop Counter Register"]
    pub ch3_loop: CH3_LOOP,
    #[doc = "0x11c - Channel Descriptor Control Word Register"]
    pub ch3_ctrl: CH3_CTRL,
    #[doc = "0x120 - Channel Descriptor Source Data Address Register"]
    pub ch3_src: CH3_SRC,
    #[doc = "0x124 - Channel Descriptor Destination Data Address Register"]
    pub ch3_dst: CH3_DST,
    #[doc = "0x128 - Channel Descriptor Link Structure Address Register"]
    pub ch3_link: CH3_LINK,
    _reserved44: [u8; 20usize],
    #[doc = "0x140 - Channel Peripheral Request Select Register"]
    pub ch4_reqsel: CH4_REQSEL,
    #[doc = "0x144 - Channel Configuration Register"]
    pub ch4_cfg: CH4_CFG,
    #[doc = "0x148 - Channel Loop Counter Register"]
    pub ch4_loop: CH4_LOOP,
    #[doc = "0x14c - Channel Descriptor Control Word Register"]
    pub ch4_ctrl: CH4_CTRL,
    #[doc = "0x150 - Channel Descriptor Source Data Address Register"]
    pub ch4_src: CH4_SRC,
    #[doc = "0x154 - Channel Descriptor Destination Data Address Register"]
    pub ch4_dst: CH4_DST,
    #[doc = "0x158 - Channel Descriptor Link Structure Address Register"]
    pub ch4_link: CH4_LINK,
    _reserved51: [u8; 20usize],
    #[doc = "0x170 - Channel Peripheral Request Select Register"]
    pub ch5_reqsel: CH5_REQSEL,
    #[doc = "0x174 - Channel Configuration Register"]
    pub ch5_cfg: CH5_CFG,
    #[doc = "0x178 - Channel Loop Counter Register"]
    pub ch5_loop: CH5_LOOP,
    #[doc = "0x17c - Channel Descriptor Control Word Register"]
    pub ch5_ctrl: CH5_CTRL,
    #[doc = "0x180 - Channel Descriptor Source Data Address Register"]
    pub ch5_src: CH5_SRC,
    #[doc = "0x184 - Channel Descriptor Destination Data Address Register"]
    pub ch5_dst: CH5_DST,
    #[doc = "0x188 - Channel Descriptor Link Structure Address Register"]
    pub ch5_link: CH5_LINK,
    _reserved58: [u8; 20usize],
    #[doc = "0x1a0 - Channel Peripheral Request Select Register"]
    pub ch6_reqsel: CH6_REQSEL,
    #[doc = "0x1a4 - Channel Configuration Register"]
    pub ch6_cfg: CH6_CFG,
    #[doc = "0x1a8 - Channel Loop Counter Register"]
    pub ch6_loop: CH6_LOOP,
    #[doc = "0x1ac - Channel Descriptor Control Word Register"]
    pub ch6_ctrl: CH6_CTRL,
    #[doc = "0x1b0 - Channel Descriptor Source Data Address Register"]
    pub ch6_src: CH6_SRC,
    #[doc = "0x1b4 - Channel Descriptor Destination Data Address Register"]
    pub ch6_dst: CH6_DST,
    #[doc = "0x1b8 - Channel Descriptor Link Structure Address Register"]
    pub ch6_link: CH6_LINK,
    _reserved65: [u8; 20usize],
    #[doc = "0x1d0 - Channel Peripheral Request Select Register"]
    pub ch7_reqsel: CH7_REQSEL,
    #[doc = "0x1d4 - Channel Configuration Register"]
    pub ch7_cfg: CH7_CFG,
    #[doc = "0x1d8 - Channel Loop Counter Register"]
    pub ch7_loop: CH7_LOOP,
    #[doc = "0x1dc - Channel Descriptor Control Word Register"]
    pub ch7_ctrl: CH7_CTRL,
    #[doc = "0x1e0 - Channel Descriptor Source Data Address Register"]
    pub ch7_src: CH7_SRC,
    #[doc = "0x1e4 - Channel Descriptor Destination Data Address Register"]
    pub ch7_dst: CH7_DST,
    #[doc = "0x1e8 - Channel Descriptor Link Structure Address Register"]
    pub ch7_link: CH7_LINK,
}
#[doc = "DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "DMA Control Register"]
pub mod ctrl;
#[doc = "DMA Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "DMA Status Register"]
pub mod status;
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync](sync) module"]
pub type SYNC = crate::Reg<u32, _SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC;
#[doc = "`read()` method returns [sync::R](sync::R) reader structure"]
impl crate::Readable for SYNC {}
#[doc = "`write(|w| ..)` method takes [sync::W](sync::W) writer structure"]
impl crate::Writable for SYNC {}
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)"]
pub mod sync;
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chen](chen) module"]
pub type CHEN = crate::Reg<u32, _CHEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEN;
#[doc = "`read()` method returns [chen::R](chen::R) reader structure"]
impl crate::Readable for CHEN {}
#[doc = "`write(|w| ..)` method takes [chen::W](chen::W) writer structure"]
impl crate::Writable for CHEN {}
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)"]
pub mod chen;
#[doc = "DMA Channel Busy Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chbusy](chbusy) module"]
pub type CHBUSY = crate::Reg<u32, _CHBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHBUSY;
#[doc = "`read()` method returns [chbusy::R](chbusy::R) reader structure"]
impl crate::Readable for CHBUSY {}
#[doc = "DMA Channel Busy Register"]
pub mod chbusy;
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdone](chdone) module"]
pub type CHDONE = crate::Reg<u32, _CHDONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDONE;
#[doc = "`read()` method returns [chdone::R](chdone::R) reader structure"]
impl crate::Readable for CHDONE {}
#[doc = "`write(|w| ..)` method takes [chdone::W](chdone::W) writer structure"]
impl crate::Writable for CHDONE {}
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)"]
pub mod chdone;
#[doc = "DMA Channel Debug Halt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbghalt](dbghalt) module"]
pub type DBGHALT = crate::Reg<u32, _DBGHALT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGHALT;
#[doc = "`read()` method returns [dbghalt::R](dbghalt::R) reader structure"]
impl crate::Readable for DBGHALT {}
#[doc = "`write(|w| ..)` method takes [dbghalt::W](dbghalt::W) writer structure"]
impl crate::Writable for DBGHALT {}
#[doc = "DMA Channel Debug Halt Register"]
pub mod dbghalt;
#[doc = "DMA Channel Software Transfer Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swreq](swreq) module"]
pub type SWREQ = crate::Reg<u32, _SWREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWREQ;
#[doc = "`write(|w| ..)` method takes [swreq::W](swreq::W) writer structure"]
impl crate::Writable for SWREQ {}
#[doc = "DMA Channel Software Transfer Request Register"]
pub mod swreq;
#[doc = "DMA Channel Request Disable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqdis](reqdis) module"]
pub type REQDIS = crate::Reg<u32, _REQDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQDIS;
#[doc = "`read()` method returns [reqdis::R](reqdis::R) reader structure"]
impl crate::Readable for REQDIS {}
#[doc = "`write(|w| ..)` method takes [reqdis::W](reqdis::W) writer structure"]
impl crate::Writable for REQDIS {}
#[doc = "DMA Channel Request Disable Register"]
pub mod reqdis;
#[doc = "DMA Channel Requests Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqpend](reqpend) module"]
pub type REQPEND = crate::Reg<u32, _REQPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQPEND;
#[doc = "`read()` method returns [reqpend::R](reqpend::R) reader structure"]
impl crate::Readable for REQPEND {}
#[doc = "DMA Channel Requests Pending Register"]
pub mod reqpend;
#[doc = "DMA Channel Link Load Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linkload](linkload) module"]
pub type LINKLOAD = crate::Reg<u32, _LINKLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINKLOAD;
#[doc = "`write(|w| ..)` method takes [linkload::W](linkload::W) writer structure"]
impl crate::Writable for LINKLOAD {}
#[doc = "DMA Channel Link Load Register"]
pub mod linkload;
#[doc = "DMA Channel Request Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqclear](reqclear) module"]
pub type REQCLEAR = crate::Reg<u32, _REQCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQCLEAR;
#[doc = "`write(|w| ..)` method takes [reqclear::W](reqclear::W) writer structure"]
impl crate::Writable for REQCLEAR {}
#[doc = "DMA Channel Request Clear Register"]
pub mod reqclear;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](if_) module"]
pub type IF = crate::Reg<u32, _IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF;
#[doc = "`read()` method returns [if_::R](if_::R) reader structure"]
impl crate::Readable for IF {}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](ifs) module"]
pub type IFS = crate::Reg<u32, _IFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS;
#[doc = "`write(|w| ..)` method takes [ifs::W](ifs::W) writer structure"]
impl crate::Writable for IFS {}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](ifc) module"]
pub type IFC = crate::Reg<u32, _IFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFC;
#[doc = "`write(|w| ..)` method takes [ifc::W](ifc::W) writer structure"]
impl crate::Writable for IFC {}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](ien) module"]
pub type IEN = crate::Reg<u32, _IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEN;
#[doc = "`read()` method returns [ien::R](ien::R) reader structure"]
impl crate::Readable for IEN {}
#[doc = "`write(|w| ..)` method takes [ien::W](ien::W) writer structure"]
impl crate::Writable for IEN {}
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_reqsel](ch0_reqsel) module"]
pub type CH0_REQSEL = crate::Reg<u32, _CH0_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_REQSEL;
#[doc = "`read()` method returns [ch0_reqsel::R](ch0_reqsel::R) reader structure"]
impl crate::Readable for CH0_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch0_reqsel::W](ch0_reqsel::W) writer structure"]
impl crate::Writable for CH0_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch0_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_cfg](ch0_cfg) module"]
pub type CH0_CFG = crate::Reg<u32, _CH0_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_CFG;
#[doc = "`read()` method returns [ch0_cfg::R](ch0_cfg::R) reader structure"]
impl crate::Readable for CH0_CFG {}
#[doc = "`write(|w| ..)` method takes [ch0_cfg::W](ch0_cfg::W) writer structure"]
impl crate::Writable for CH0_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch0_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_loop](ch0_loop) module"]
pub type CH0_LOOP = crate::Reg<u32, _CH0_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_LOOP;
#[doc = "`read()` method returns [ch0_loop::R](ch0_loop::R) reader structure"]
impl crate::Readable for CH0_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch0_loop::W](ch0_loop::W) writer structure"]
impl crate::Writable for CH0_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch0_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_ctrl](ch0_ctrl) module"]
pub type CH0_CTRL = crate::Reg<u32, _CH0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_CTRL;
#[doc = "`read()` method returns [ch0_ctrl::R](ch0_ctrl::R) reader structure"]
impl crate::Readable for CH0_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch0_ctrl::W](ch0_ctrl::W) writer structure"]
impl crate::Writable for CH0_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch0_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_src](ch0_src) module"]
pub type CH0_SRC = crate::Reg<u32, _CH0_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_SRC;
#[doc = "`read()` method returns [ch0_src::R](ch0_src::R) reader structure"]
impl crate::Readable for CH0_SRC {}
#[doc = "`write(|w| ..)` method takes [ch0_src::W](ch0_src::W) writer structure"]
impl crate::Writable for CH0_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch0_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_dst](ch0_dst) module"]
pub type CH0_DST = crate::Reg<u32, _CH0_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_DST;
#[doc = "`read()` method returns [ch0_dst::R](ch0_dst::R) reader structure"]
impl crate::Readable for CH0_DST {}
#[doc = "`write(|w| ..)` method takes [ch0_dst::W](ch0_dst::W) writer structure"]
impl crate::Writable for CH0_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch0_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_link](ch0_link) module"]
pub type CH0_LINK = crate::Reg<u32, _CH0_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_LINK;
#[doc = "`read()` method returns [ch0_link::R](ch0_link::R) reader structure"]
impl crate::Readable for CH0_LINK {}
#[doc = "`write(|w| ..)` method takes [ch0_link::W](ch0_link::W) writer structure"]
impl crate::Writable for CH0_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch0_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_reqsel](ch1_reqsel) module"]
pub type CH1_REQSEL = crate::Reg<u32, _CH1_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_REQSEL;
#[doc = "`read()` method returns [ch1_reqsel::R](ch1_reqsel::R) reader structure"]
impl crate::Readable for CH1_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch1_reqsel::W](ch1_reqsel::W) writer structure"]
impl crate::Writable for CH1_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch1_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_cfg](ch1_cfg) module"]
pub type CH1_CFG = crate::Reg<u32, _CH1_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_CFG;
#[doc = "`read()` method returns [ch1_cfg::R](ch1_cfg::R) reader structure"]
impl crate::Readable for CH1_CFG {}
#[doc = "`write(|w| ..)` method takes [ch1_cfg::W](ch1_cfg::W) writer structure"]
impl crate::Writable for CH1_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch1_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_loop](ch1_loop) module"]
pub type CH1_LOOP = crate::Reg<u32, _CH1_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_LOOP;
#[doc = "`read()` method returns [ch1_loop::R](ch1_loop::R) reader structure"]
impl crate::Readable for CH1_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch1_loop::W](ch1_loop::W) writer structure"]
impl crate::Writable for CH1_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch1_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_ctrl](ch1_ctrl) module"]
pub type CH1_CTRL = crate::Reg<u32, _CH1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_CTRL;
#[doc = "`read()` method returns [ch1_ctrl::R](ch1_ctrl::R) reader structure"]
impl crate::Readable for CH1_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch1_ctrl::W](ch1_ctrl::W) writer structure"]
impl crate::Writable for CH1_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch1_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_src](ch1_src) module"]
pub type CH1_SRC = crate::Reg<u32, _CH1_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_SRC;
#[doc = "`read()` method returns [ch1_src::R](ch1_src::R) reader structure"]
impl crate::Readable for CH1_SRC {}
#[doc = "`write(|w| ..)` method takes [ch1_src::W](ch1_src::W) writer structure"]
impl crate::Writable for CH1_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch1_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_dst](ch1_dst) module"]
pub type CH1_DST = crate::Reg<u32, _CH1_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_DST;
#[doc = "`read()` method returns [ch1_dst::R](ch1_dst::R) reader structure"]
impl crate::Readable for CH1_DST {}
#[doc = "`write(|w| ..)` method takes [ch1_dst::W](ch1_dst::W) writer structure"]
impl crate::Writable for CH1_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch1_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_link](ch1_link) module"]
pub type CH1_LINK = crate::Reg<u32, _CH1_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_LINK;
#[doc = "`read()` method returns [ch1_link::R](ch1_link::R) reader structure"]
impl crate::Readable for CH1_LINK {}
#[doc = "`write(|w| ..)` method takes [ch1_link::W](ch1_link::W) writer structure"]
impl crate::Writable for CH1_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch1_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_reqsel](ch2_reqsel) module"]
pub type CH2_REQSEL = crate::Reg<u32, _CH2_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_REQSEL;
#[doc = "`read()` method returns [ch2_reqsel::R](ch2_reqsel::R) reader structure"]
impl crate::Readable for CH2_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch2_reqsel::W](ch2_reqsel::W) writer structure"]
impl crate::Writable for CH2_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch2_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_cfg](ch2_cfg) module"]
pub type CH2_CFG = crate::Reg<u32, _CH2_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_CFG;
#[doc = "`read()` method returns [ch2_cfg::R](ch2_cfg::R) reader structure"]
impl crate::Readable for CH2_CFG {}
#[doc = "`write(|w| ..)` method takes [ch2_cfg::W](ch2_cfg::W) writer structure"]
impl crate::Writable for CH2_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch2_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_loop](ch2_loop) module"]
pub type CH2_LOOP = crate::Reg<u32, _CH2_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_LOOP;
#[doc = "`read()` method returns [ch2_loop::R](ch2_loop::R) reader structure"]
impl crate::Readable for CH2_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch2_loop::W](ch2_loop::W) writer structure"]
impl crate::Writable for CH2_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch2_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_ctrl](ch2_ctrl) module"]
pub type CH2_CTRL = crate::Reg<u32, _CH2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_CTRL;
#[doc = "`read()` method returns [ch2_ctrl::R](ch2_ctrl::R) reader structure"]
impl crate::Readable for CH2_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch2_ctrl::W](ch2_ctrl::W) writer structure"]
impl crate::Writable for CH2_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch2_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_src](ch2_src) module"]
pub type CH2_SRC = crate::Reg<u32, _CH2_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_SRC;
#[doc = "`read()` method returns [ch2_src::R](ch2_src::R) reader structure"]
impl crate::Readable for CH2_SRC {}
#[doc = "`write(|w| ..)` method takes [ch2_src::W](ch2_src::W) writer structure"]
impl crate::Writable for CH2_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch2_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_dst](ch2_dst) module"]
pub type CH2_DST = crate::Reg<u32, _CH2_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_DST;
#[doc = "`read()` method returns [ch2_dst::R](ch2_dst::R) reader structure"]
impl crate::Readable for CH2_DST {}
#[doc = "`write(|w| ..)` method takes [ch2_dst::W](ch2_dst::W) writer structure"]
impl crate::Writable for CH2_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch2_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_link](ch2_link) module"]
pub type CH2_LINK = crate::Reg<u32, _CH2_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_LINK;
#[doc = "`read()` method returns [ch2_link::R](ch2_link::R) reader structure"]
impl crate::Readable for CH2_LINK {}
#[doc = "`write(|w| ..)` method takes [ch2_link::W](ch2_link::W) writer structure"]
impl crate::Writable for CH2_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch2_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_reqsel](ch3_reqsel) module"]
pub type CH3_REQSEL = crate::Reg<u32, _CH3_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_REQSEL;
#[doc = "`read()` method returns [ch3_reqsel::R](ch3_reqsel::R) reader structure"]
impl crate::Readable for CH3_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch3_reqsel::W](ch3_reqsel::W) writer structure"]
impl crate::Writable for CH3_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch3_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_cfg](ch3_cfg) module"]
pub type CH3_CFG = crate::Reg<u32, _CH3_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_CFG;
#[doc = "`read()` method returns [ch3_cfg::R](ch3_cfg::R) reader structure"]
impl crate::Readable for CH3_CFG {}
#[doc = "`write(|w| ..)` method takes [ch3_cfg::W](ch3_cfg::W) writer structure"]
impl crate::Writable for CH3_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch3_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_loop](ch3_loop) module"]
pub type CH3_LOOP = crate::Reg<u32, _CH3_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_LOOP;
#[doc = "`read()` method returns [ch3_loop::R](ch3_loop::R) reader structure"]
impl crate::Readable for CH3_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch3_loop::W](ch3_loop::W) writer structure"]
impl crate::Writable for CH3_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch3_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_ctrl](ch3_ctrl) module"]
pub type CH3_CTRL = crate::Reg<u32, _CH3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_CTRL;
#[doc = "`read()` method returns [ch3_ctrl::R](ch3_ctrl::R) reader structure"]
impl crate::Readable for CH3_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch3_ctrl::W](ch3_ctrl::W) writer structure"]
impl crate::Writable for CH3_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch3_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_src](ch3_src) module"]
pub type CH3_SRC = crate::Reg<u32, _CH3_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_SRC;
#[doc = "`read()` method returns [ch3_src::R](ch3_src::R) reader structure"]
impl crate::Readable for CH3_SRC {}
#[doc = "`write(|w| ..)` method takes [ch3_src::W](ch3_src::W) writer structure"]
impl crate::Writable for CH3_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch3_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_dst](ch3_dst) module"]
pub type CH3_DST = crate::Reg<u32, _CH3_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_DST;
#[doc = "`read()` method returns [ch3_dst::R](ch3_dst::R) reader structure"]
impl crate::Readable for CH3_DST {}
#[doc = "`write(|w| ..)` method takes [ch3_dst::W](ch3_dst::W) writer structure"]
impl crate::Writable for CH3_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch3_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_link](ch3_link) module"]
pub type CH3_LINK = crate::Reg<u32, _CH3_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_LINK;
#[doc = "`read()` method returns [ch3_link::R](ch3_link::R) reader structure"]
impl crate::Readable for CH3_LINK {}
#[doc = "`write(|w| ..)` method takes [ch3_link::W](ch3_link::W) writer structure"]
impl crate::Writable for CH3_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch3_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_reqsel](ch4_reqsel) module"]
pub type CH4_REQSEL = crate::Reg<u32, _CH4_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_REQSEL;
#[doc = "`read()` method returns [ch4_reqsel::R](ch4_reqsel::R) reader structure"]
impl crate::Readable for CH4_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch4_reqsel::W](ch4_reqsel::W) writer structure"]
impl crate::Writable for CH4_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch4_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_cfg](ch4_cfg) module"]
pub type CH4_CFG = crate::Reg<u32, _CH4_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_CFG;
#[doc = "`read()` method returns [ch4_cfg::R](ch4_cfg::R) reader structure"]
impl crate::Readable for CH4_CFG {}
#[doc = "`write(|w| ..)` method takes [ch4_cfg::W](ch4_cfg::W) writer structure"]
impl crate::Writable for CH4_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch4_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_loop](ch4_loop) module"]
pub type CH4_LOOP = crate::Reg<u32, _CH4_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_LOOP;
#[doc = "`read()` method returns [ch4_loop::R](ch4_loop::R) reader structure"]
impl crate::Readable for CH4_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch4_loop::W](ch4_loop::W) writer structure"]
impl crate::Writable for CH4_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch4_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_ctrl](ch4_ctrl) module"]
pub type CH4_CTRL = crate::Reg<u32, _CH4_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_CTRL;
#[doc = "`read()` method returns [ch4_ctrl::R](ch4_ctrl::R) reader structure"]
impl crate::Readable for CH4_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch4_ctrl::W](ch4_ctrl::W) writer structure"]
impl crate::Writable for CH4_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch4_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_src](ch4_src) module"]
pub type CH4_SRC = crate::Reg<u32, _CH4_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_SRC;
#[doc = "`read()` method returns [ch4_src::R](ch4_src::R) reader structure"]
impl crate::Readable for CH4_SRC {}
#[doc = "`write(|w| ..)` method takes [ch4_src::W](ch4_src::W) writer structure"]
impl crate::Writable for CH4_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch4_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_dst](ch4_dst) module"]
pub type CH4_DST = crate::Reg<u32, _CH4_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_DST;
#[doc = "`read()` method returns [ch4_dst::R](ch4_dst::R) reader structure"]
impl crate::Readable for CH4_DST {}
#[doc = "`write(|w| ..)` method takes [ch4_dst::W](ch4_dst::W) writer structure"]
impl crate::Writable for CH4_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch4_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_link](ch4_link) module"]
pub type CH4_LINK = crate::Reg<u32, _CH4_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_LINK;
#[doc = "`read()` method returns [ch4_link::R](ch4_link::R) reader structure"]
impl crate::Readable for CH4_LINK {}
#[doc = "`write(|w| ..)` method takes [ch4_link::W](ch4_link::W) writer structure"]
impl crate::Writable for CH4_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch4_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_reqsel](ch5_reqsel) module"]
pub type CH5_REQSEL = crate::Reg<u32, _CH5_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_REQSEL;
#[doc = "`read()` method returns [ch5_reqsel::R](ch5_reqsel::R) reader structure"]
impl crate::Readable for CH5_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch5_reqsel::W](ch5_reqsel::W) writer structure"]
impl crate::Writable for CH5_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch5_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_cfg](ch5_cfg) module"]
pub type CH5_CFG = crate::Reg<u32, _CH5_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_CFG;
#[doc = "`read()` method returns [ch5_cfg::R](ch5_cfg::R) reader structure"]
impl crate::Readable for CH5_CFG {}
#[doc = "`write(|w| ..)` method takes [ch5_cfg::W](ch5_cfg::W) writer structure"]
impl crate::Writable for CH5_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch5_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_loop](ch5_loop) module"]
pub type CH5_LOOP = crate::Reg<u32, _CH5_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_LOOP;
#[doc = "`read()` method returns [ch5_loop::R](ch5_loop::R) reader structure"]
impl crate::Readable for CH5_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch5_loop::W](ch5_loop::W) writer structure"]
impl crate::Writable for CH5_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch5_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_ctrl](ch5_ctrl) module"]
pub type CH5_CTRL = crate::Reg<u32, _CH5_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_CTRL;
#[doc = "`read()` method returns [ch5_ctrl::R](ch5_ctrl::R) reader structure"]
impl crate::Readable for CH5_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch5_ctrl::W](ch5_ctrl::W) writer structure"]
impl crate::Writable for CH5_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch5_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_src](ch5_src) module"]
pub type CH5_SRC = crate::Reg<u32, _CH5_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_SRC;
#[doc = "`read()` method returns [ch5_src::R](ch5_src::R) reader structure"]
impl crate::Readable for CH5_SRC {}
#[doc = "`write(|w| ..)` method takes [ch5_src::W](ch5_src::W) writer structure"]
impl crate::Writable for CH5_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch5_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_dst](ch5_dst) module"]
pub type CH5_DST = crate::Reg<u32, _CH5_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_DST;
#[doc = "`read()` method returns [ch5_dst::R](ch5_dst::R) reader structure"]
impl crate::Readable for CH5_DST {}
#[doc = "`write(|w| ..)` method takes [ch5_dst::W](ch5_dst::W) writer structure"]
impl crate::Writable for CH5_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch5_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_link](ch5_link) module"]
pub type CH5_LINK = crate::Reg<u32, _CH5_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_LINK;
#[doc = "`read()` method returns [ch5_link::R](ch5_link::R) reader structure"]
impl crate::Readable for CH5_LINK {}
#[doc = "`write(|w| ..)` method takes [ch5_link::W](ch5_link::W) writer structure"]
impl crate::Writable for CH5_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch5_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_reqsel](ch6_reqsel) module"]
pub type CH6_REQSEL = crate::Reg<u32, _CH6_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_REQSEL;
#[doc = "`read()` method returns [ch6_reqsel::R](ch6_reqsel::R) reader structure"]
impl crate::Readable for CH6_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch6_reqsel::W](ch6_reqsel::W) writer structure"]
impl crate::Writable for CH6_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch6_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_cfg](ch6_cfg) module"]
pub type CH6_CFG = crate::Reg<u32, _CH6_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_CFG;
#[doc = "`read()` method returns [ch6_cfg::R](ch6_cfg::R) reader structure"]
impl crate::Readable for CH6_CFG {}
#[doc = "`write(|w| ..)` method takes [ch6_cfg::W](ch6_cfg::W) writer structure"]
impl crate::Writable for CH6_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch6_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_loop](ch6_loop) module"]
pub type CH6_LOOP = crate::Reg<u32, _CH6_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_LOOP;
#[doc = "`read()` method returns [ch6_loop::R](ch6_loop::R) reader structure"]
impl crate::Readable for CH6_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch6_loop::W](ch6_loop::W) writer structure"]
impl crate::Writable for CH6_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch6_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_ctrl](ch6_ctrl) module"]
pub type CH6_CTRL = crate::Reg<u32, _CH6_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_CTRL;
#[doc = "`read()` method returns [ch6_ctrl::R](ch6_ctrl::R) reader structure"]
impl crate::Readable for CH6_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch6_ctrl::W](ch6_ctrl::W) writer structure"]
impl crate::Writable for CH6_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch6_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_src](ch6_src) module"]
pub type CH6_SRC = crate::Reg<u32, _CH6_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_SRC;
#[doc = "`read()` method returns [ch6_src::R](ch6_src::R) reader structure"]
impl crate::Readable for CH6_SRC {}
#[doc = "`write(|w| ..)` method takes [ch6_src::W](ch6_src::W) writer structure"]
impl crate::Writable for CH6_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch6_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_dst](ch6_dst) module"]
pub type CH6_DST = crate::Reg<u32, _CH6_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_DST;
#[doc = "`read()` method returns [ch6_dst::R](ch6_dst::R) reader structure"]
impl crate::Readable for CH6_DST {}
#[doc = "`write(|w| ..)` method takes [ch6_dst::W](ch6_dst::W) writer structure"]
impl crate::Writable for CH6_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch6_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_link](ch6_link) module"]
pub type CH6_LINK = crate::Reg<u32, _CH6_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_LINK;
#[doc = "`read()` method returns [ch6_link::R](ch6_link::R) reader structure"]
impl crate::Readable for CH6_LINK {}
#[doc = "`write(|w| ..)` method takes [ch6_link::W](ch6_link::W) writer structure"]
impl crate::Writable for CH6_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch6_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_reqsel](ch7_reqsel) module"]
pub type CH7_REQSEL = crate::Reg<u32, _CH7_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_REQSEL;
#[doc = "`read()` method returns [ch7_reqsel::R](ch7_reqsel::R) reader structure"]
impl crate::Readable for CH7_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch7_reqsel::W](ch7_reqsel::W) writer structure"]
impl crate::Writable for CH7_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch7_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_cfg](ch7_cfg) module"]
pub type CH7_CFG = crate::Reg<u32, _CH7_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_CFG;
#[doc = "`read()` method returns [ch7_cfg::R](ch7_cfg::R) reader structure"]
impl crate::Readable for CH7_CFG {}
#[doc = "`write(|w| ..)` method takes [ch7_cfg::W](ch7_cfg::W) writer structure"]
impl crate::Writable for CH7_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch7_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_loop](ch7_loop) module"]
pub type CH7_LOOP = crate::Reg<u32, _CH7_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_LOOP;
#[doc = "`read()` method returns [ch7_loop::R](ch7_loop::R) reader structure"]
impl crate::Readable for CH7_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch7_loop::W](ch7_loop::W) writer structure"]
impl crate::Writable for CH7_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch7_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_ctrl](ch7_ctrl) module"]
pub type CH7_CTRL = crate::Reg<u32, _CH7_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_CTRL;
#[doc = "`read()` method returns [ch7_ctrl::R](ch7_ctrl::R) reader structure"]
impl crate::Readable for CH7_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch7_ctrl::W](ch7_ctrl::W) writer structure"]
impl crate::Writable for CH7_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch7_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_src](ch7_src) module"]
pub type CH7_SRC = crate::Reg<u32, _CH7_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_SRC;
#[doc = "`read()` method returns [ch7_src::R](ch7_src::R) reader structure"]
impl crate::Readable for CH7_SRC {}
#[doc = "`write(|w| ..)` method takes [ch7_src::W](ch7_src::W) writer structure"]
impl crate::Writable for CH7_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch7_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_dst](ch7_dst) module"]
pub type CH7_DST = crate::Reg<u32, _CH7_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_DST;
#[doc = "`read()` method returns [ch7_dst::R](ch7_dst::R) reader structure"]
impl crate::Readable for CH7_DST {}
#[doc = "`write(|w| ..)` method takes [ch7_dst::W](ch7_dst::W) writer structure"]
impl crate::Writable for CH7_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch7_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_link](ch7_link) module"]
pub type CH7_LINK = crate::Reg<u32, _CH7_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_LINK;
#[doc = "`read()` method returns [ch7_link::R](ch7_link::R) reader structure"]
impl crate::Readable for CH7_LINK {}
#[doc = "`write(|w| ..)` method takes [ch7_link::W](ch7_link::W) writer structure"]
impl crate::Writable for CH7_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch7_link;
