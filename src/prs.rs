#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software Pulse Register"]
    pub swpulse: SWPULSE,
    #[doc = "0x04 - Software Level Register"]
    pub swlevel: SWLEVEL,
    #[doc = "0x08 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    #[doc = "0x14 - I/O Routing Location Register"]
    pub routeloc1: ROUTELOC1,
    #[doc = "0x18 - I/O Routing Location Register"]
    pub routeloc2: ROUTELOC2,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x24 - DMA Request 0 Register"]
    pub dmareq0: DMAREQ0,
    #[doc = "0x28 - DMA Request 1 Register"]
    pub dmareq1: DMAREQ1,
    _reserved9: [u8; 4usize],
    #[doc = "0x30 - PRS Channel Values"]
    pub peek: PEEK,
    _reserved10: [u8; 12usize],
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
#[doc = "Software Pulse Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpulse](swpulse) module"]
pub type SWPULSE = crate::Reg<u32, _SWPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWPULSE;
#[doc = "`write(|w| ..)` method takes [swpulse::W](swpulse::W) writer structure"]
impl crate::Writable for SWPULSE {}
#[doc = "Software Pulse Register"]
pub mod swpulse;
#[doc = "Software Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swlevel](swlevel) module"]
pub type SWLEVEL = crate::Reg<u32, _SWLEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWLEVEL;
#[doc = "`read()` method returns [swlevel::R](swlevel::R) reader structure"]
impl crate::Readable for SWLEVEL {}
#[doc = "`write(|w| ..)` method takes [swlevel::W](swlevel::W) writer structure"]
impl crate::Writable for SWLEVEL {}
#[doc = "Software Level Register"]
pub mod swlevel;
#[doc = "I/O Routing Pin Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](routepen) module"]
pub type ROUTEPEN = crate::Reg<u32, _ROUTEPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTEPEN;
#[doc = "`read()` method returns [routepen::R](routepen::R) reader structure"]
impl crate::Readable for ROUTEPEN {}
#[doc = "`write(|w| ..)` method takes [routepen::W](routepen::W) writer structure"]
impl crate::Writable for ROUTEPEN {}
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc0](routeloc0) module"]
pub type ROUTELOC0 = crate::Reg<u32, _ROUTELOC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTELOC0;
#[doc = "`read()` method returns [routeloc0::R](routeloc0::R) reader structure"]
impl crate::Readable for ROUTELOC0 {}
#[doc = "`write(|w| ..)` method takes [routeloc0::W](routeloc0::W) writer structure"]
impl crate::Writable for ROUTELOC0 {}
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc1](routeloc1) module"]
pub type ROUTELOC1 = crate::Reg<u32, _ROUTELOC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTELOC1;
#[doc = "`read()` method returns [routeloc1::R](routeloc1::R) reader structure"]
impl crate::Readable for ROUTELOC1 {}
#[doc = "`write(|w| ..)` method takes [routeloc1::W](routeloc1::W) writer structure"]
impl crate::Writable for ROUTELOC1 {}
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc2](routeloc2) module"]
pub type ROUTELOC2 = crate::Reg<u32, _ROUTELOC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTELOC2;
#[doc = "`read()` method returns [routeloc2::R](routeloc2::R) reader structure"]
impl crate::Readable for ROUTELOC2 {}
#[doc = "`write(|w| ..)` method takes [routeloc2::W](routeloc2::W) writer structure"]
impl crate::Writable for ROUTELOC2 {}
#[doc = "I/O Routing Location Register"]
pub mod routeloc2;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "DMA Request 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmareq0](dmareq0) module"]
pub type DMAREQ0 = crate::Reg<u32, _DMAREQ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAREQ0;
#[doc = "`read()` method returns [dmareq0::R](dmareq0::R) reader structure"]
impl crate::Readable for DMAREQ0 {}
#[doc = "`write(|w| ..)` method takes [dmareq0::W](dmareq0::W) writer structure"]
impl crate::Writable for DMAREQ0 {}
#[doc = "DMA Request 0 Register"]
pub mod dmareq0;
#[doc = "DMA Request 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmareq1](dmareq1) module"]
pub type DMAREQ1 = crate::Reg<u32, _DMAREQ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAREQ1;
#[doc = "`read()` method returns [dmareq1::R](dmareq1::R) reader structure"]
impl crate::Readable for DMAREQ1 {}
#[doc = "`write(|w| ..)` method takes [dmareq1::W](dmareq1::W) writer structure"]
impl crate::Writable for DMAREQ1 {}
#[doc = "DMA Request 1 Register"]
pub mod dmareq1;
#[doc = "PRS Channel Values\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek](peek) module"]
pub type PEEK = crate::Reg<u32, _PEEK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK;
#[doc = "`read()` method returns [peek::R](peek::R) reader structure"]
impl crate::Readable for PEEK {}
#[doc = "PRS Channel Values"]
pub mod peek;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_ctrl](ch0_ctrl) module"]
pub type CH0_CTRL = crate::Reg<u32, _CH0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_CTRL;
#[doc = "`read()` method returns [ch0_ctrl::R](ch0_ctrl::R) reader structure"]
impl crate::Readable for CH0_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch0_ctrl::W](ch0_ctrl::W) writer structure"]
impl crate::Writable for CH0_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_ctrl](ch1_ctrl) module"]
pub type CH1_CTRL = crate::Reg<u32, _CH1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_CTRL;
#[doc = "`read()` method returns [ch1_ctrl::R](ch1_ctrl::R) reader structure"]
impl crate::Readable for CH1_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch1_ctrl::W](ch1_ctrl::W) writer structure"]
impl crate::Writable for CH1_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_ctrl](ch2_ctrl) module"]
pub type CH2_CTRL = crate::Reg<u32, _CH2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_CTRL;
#[doc = "`read()` method returns [ch2_ctrl::R](ch2_ctrl::R) reader structure"]
impl crate::Readable for CH2_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch2_ctrl::W](ch2_ctrl::W) writer structure"]
impl crate::Writable for CH2_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_ctrl](ch3_ctrl) module"]
pub type CH3_CTRL = crate::Reg<u32, _CH3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_CTRL;
#[doc = "`read()` method returns [ch3_ctrl::R](ch3_ctrl::R) reader structure"]
impl crate::Readable for CH3_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch3_ctrl::W](ch3_ctrl::W) writer structure"]
impl crate::Writable for CH3_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_ctrl](ch4_ctrl) module"]
pub type CH4_CTRL = crate::Reg<u32, _CH4_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_CTRL;
#[doc = "`read()` method returns [ch4_ctrl::R](ch4_ctrl::R) reader structure"]
impl crate::Readable for CH4_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch4_ctrl::W](ch4_ctrl::W) writer structure"]
impl crate::Writable for CH4_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_ctrl](ch5_ctrl) module"]
pub type CH5_CTRL = crate::Reg<u32, _CH5_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_CTRL;
#[doc = "`read()` method returns [ch5_ctrl::R](ch5_ctrl::R) reader structure"]
impl crate::Readable for CH5_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch5_ctrl::W](ch5_ctrl::W) writer structure"]
impl crate::Writable for CH5_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_ctrl](ch6_ctrl) module"]
pub type CH6_CTRL = crate::Reg<u32, _CH6_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_CTRL;
#[doc = "`read()` method returns [ch6_ctrl::R](ch6_ctrl::R) reader structure"]
impl crate::Readable for CH6_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch6_ctrl::W](ch6_ctrl::W) writer structure"]
impl crate::Writable for CH6_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch6_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_ctrl](ch7_ctrl) module"]
pub type CH7_CTRL = crate::Reg<u32, _CH7_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_CTRL;
#[doc = "`read()` method returns [ch7_ctrl::R](ch7_ctrl::R) reader structure"]
impl crate::Readable for CH7_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch7_ctrl::W](ch7_ctrl::W) writer structure"]
impl crate::Writable for CH7_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch7_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_ctrl](ch8_ctrl) module"]
pub type CH8_CTRL = crate::Reg<u32, _CH8_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_CTRL;
#[doc = "`read()` method returns [ch8_ctrl::R](ch8_ctrl::R) reader structure"]
impl crate::Readable for CH8_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch8_ctrl::W](ch8_ctrl::W) writer structure"]
impl crate::Writable for CH8_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch8_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_ctrl](ch9_ctrl) module"]
pub type CH9_CTRL = crate::Reg<u32, _CH9_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_CTRL;
#[doc = "`read()` method returns [ch9_ctrl::R](ch9_ctrl::R) reader structure"]
impl crate::Readable for CH9_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch9_ctrl::W](ch9_ctrl::W) writer structure"]
impl crate::Writable for CH9_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch9_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_ctrl](ch10_ctrl) module"]
pub type CH10_CTRL = crate::Reg<u32, _CH10_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_CTRL;
#[doc = "`read()` method returns [ch10_ctrl::R](ch10_ctrl::R) reader structure"]
impl crate::Readable for CH10_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch10_ctrl::W](ch10_ctrl::W) writer structure"]
impl crate::Writable for CH10_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch10_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_ctrl](ch11_ctrl) module"]
pub type CH11_CTRL = crate::Reg<u32, _CH11_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_CTRL;
#[doc = "`read()` method returns [ch11_ctrl::R](ch11_ctrl::R) reader structure"]
impl crate::Readable for CH11_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch11_ctrl::W](ch11_ctrl::W) writer structure"]
impl crate::Writable for CH11_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch11_ctrl;
