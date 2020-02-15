#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x0c - Memory Control Register"]
    pub ram0ctrl: RAM0CTRL,
    #[doc = "0x10 - Command Register"]
    pub cmd: CMD,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - EM4 Control Register"]
    pub em4ctrl: EM4CTRL,
    #[doc = "0x1c - Temperature Limits for Interrupt Generation"]
    pub templimits: TEMPLIMITS,
    #[doc = "0x20 - Value of Last Temperature Measurement"]
    pub temp: TEMP,
    #[doc = "0x24 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x28 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x2c - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x30 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x34 - Regulator and Supply Lock Register"]
    pub pwrlock: PWRLOCK,
    #[doc = "0x38 - Power Configuration Register"]
    pub pwrcfg: PWRCFG,
    #[doc = "0x3c - Power Control Register"]
    pub pwrctrl: PWRCTRL,
    #[doc = "0x40 - DCDC Control"]
    pub dcdcctrl: DCDCCTRL,
    _reserved16: [u8; 8usize],
    #[doc = "0x4c - DCDC Miscellaneous Control Register"]
    pub dcdcmiscctrl: DCDCMISCCTRL,
    #[doc = "0x50 - DCDC Power Train NFET Zero Current Detector Control Register"]
    pub dcdczdetctrl: DCDCZDETCTRL,
    #[doc = "0x54 - DCDC Power Train PFET Current Limiter Control Register"]
    pub dcdcclimctrl: DCDCCLIMCTRL,
    #[doc = "0x58 - DCDC Low Noise Compensator Control Register"]
    pub dcdclncompctrl: DCDCLNCOMPCTRL,
    #[doc = "0x5c - DCDC Low Noise Voltage Register"]
    pub dcdclnvctrl: DCDCLNVCTRL,
    #[doc = "0x60 - DCDC Controller Timing Value Register"]
    pub dcdctiming: DCDCTIMING,
    #[doc = "0x64 - DCDC Low Power Voltage Register"]
    pub dcdclpvctrl: DCDCLPVCTRL,
    _reserved23: [u8; 4usize],
    #[doc = "0x6c - DCDC Low Power Control Register"]
    pub dcdclpctrl: DCDCLPCTRL,
    #[doc = "0x70 - DCDC Low Noise Controller Frequency Control"]
    pub dcdclnfreqctrl: DCDCLNFREQCTRL,
    _reserved25: [u8; 4usize],
    #[doc = "0x78 - DCDC Read Status Register"]
    pub dcdcsync: DCDCSYNC,
    _reserved26: [u8; 20usize],
    #[doc = "0x90 - VMON AVDD Channel Control"]
    pub vmonavddctrl: VMONAVDDCTRL,
    #[doc = "0x94 - Alternate VMON AVDD Channel Control"]
    pub vmonaltavddctrl: VMONALTAVDDCTRL,
    #[doc = "0x98 - VMON DVDD Channel Control"]
    pub vmondvddctrl: VMONDVDDCTRL,
    #[doc = "0x9c - VMON IOVDD0 Channel Control"]
    pub vmonio0ctrl: VMONIO0CTRL,
    _reserved30: [u8; 196usize],
    #[doc = "0x164 - Configurations Related to the Bias"]
    pub biasconf: BIASCONF,
    _reserved31: [u8; 40usize],
    #[doc = "0x190 - Test Lock Register"]
    pub testlock: TESTLOCK,
    _reserved32: [u8; 8usize],
    #[doc = "0x19c - Test Control Register for Regulator and BIAS"]
    pub biastestctrl: BIASTESTCTRL,
}
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
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Configuration Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram0ctrl](ram0ctrl) module"]
pub type RAM0CTRL = crate::Reg<u32, _RAM0CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM0CTRL;
#[doc = "`read()` method returns [ram0ctrl::R](ram0ctrl::R) reader structure"]
impl crate::Readable for RAM0CTRL {}
#[doc = "`write(|w| ..)` method takes [ram0ctrl::W](ram0ctrl::W) writer structure"]
impl crate::Writable for RAM0CTRL {}
#[doc = "Memory Control Register"]
pub mod ram0ctrl;
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "EM4 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em4ctrl](em4ctrl) module"]
pub type EM4CTRL = crate::Reg<u32, _EM4CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EM4CTRL;
#[doc = "`read()` method returns [em4ctrl::R](em4ctrl::R) reader structure"]
impl crate::Readable for EM4CTRL {}
#[doc = "`write(|w| ..)` method takes [em4ctrl::W](em4ctrl::W) writer structure"]
impl crate::Writable for EM4CTRL {}
#[doc = "EM4 Control Register"]
pub mod em4ctrl;
#[doc = "Temperature Limits for Interrupt Generation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [templimits](templimits) module"]
pub type TEMPLIMITS = crate::Reg<u32, _TEMPLIMITS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPLIMITS;
#[doc = "`read()` method returns [templimits::R](templimits::R) reader structure"]
impl crate::Readable for TEMPLIMITS {}
#[doc = "`write(|w| ..)` method takes [templimits::W](templimits::W) writer structure"]
impl crate::Writable for TEMPLIMITS {}
#[doc = "Temperature Limits for Interrupt Generation"]
pub mod templimits;
#[doc = "Value of Last Temperature Measurement\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp](temp) module"]
pub type TEMP = crate::Reg<u32, _TEMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMP;
#[doc = "`read()` method returns [temp::R](temp::R) reader structure"]
impl crate::Readable for TEMP {}
#[doc = "Value of Last Temperature Measurement"]
pub mod temp;
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
#[doc = "Regulator and Supply Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrlock](pwrlock) module"]
pub type PWRLOCK = crate::Reg<u32, _PWRLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRLOCK;
#[doc = "`read()` method returns [pwrlock::R](pwrlock::R) reader structure"]
impl crate::Readable for PWRLOCK {}
#[doc = "`write(|w| ..)` method takes [pwrlock::W](pwrlock::W) writer structure"]
impl crate::Writable for PWRLOCK {}
#[doc = "Regulator and Supply Lock Register"]
pub mod pwrlock;
#[doc = "Power Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcfg](pwrcfg) module"]
pub type PWRCFG = crate::Reg<u32, _PWRCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCFG;
#[doc = "`read()` method returns [pwrcfg::R](pwrcfg::R) reader structure"]
impl crate::Readable for PWRCFG {}
#[doc = "`write(|w| ..)` method takes [pwrcfg::W](pwrcfg::W) writer structure"]
impl crate::Writable for PWRCFG {}
#[doc = "Power Configuration Register"]
pub mod pwrcfg;
#[doc = "Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrctrl](pwrctrl) module"]
pub type PWRCTRL = crate::Reg<u32, _PWRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCTRL;
#[doc = "`read()` method returns [pwrctrl::R](pwrctrl::R) reader structure"]
impl crate::Readable for PWRCTRL {}
#[doc = "`write(|w| ..)` method takes [pwrctrl::W](pwrctrl::W) writer structure"]
impl crate::Writable for PWRCTRL {}
#[doc = "Power Control Register"]
pub mod pwrctrl;
#[doc = "DCDC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcctrl](dcdcctrl) module"]
pub type DCDCCTRL = crate::Reg<u32, _DCDCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCCTRL;
#[doc = "`read()` method returns [dcdcctrl::R](dcdcctrl::R) reader structure"]
impl crate::Readable for DCDCCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdcctrl::W](dcdcctrl::W) writer structure"]
impl crate::Writable for DCDCCTRL {}
#[doc = "DCDC Control"]
pub mod dcdcctrl;
#[doc = "DCDC Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcmiscctrl](dcdcmiscctrl) module"]
pub type DCDCMISCCTRL = crate::Reg<u32, _DCDCMISCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCMISCCTRL;
#[doc = "`read()` method returns [dcdcmiscctrl::R](dcdcmiscctrl::R) reader structure"]
impl crate::Readable for DCDCMISCCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdcmiscctrl::W](dcdcmiscctrl::W) writer structure"]
impl crate::Writable for DCDCMISCCTRL {}
#[doc = "DCDC Miscellaneous Control Register"]
pub mod dcdcmiscctrl;
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdczdetctrl](dcdczdetctrl) module"]
pub type DCDCZDETCTRL = crate::Reg<u32, _DCDCZDETCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCZDETCTRL;
#[doc = "`read()` method returns [dcdczdetctrl::R](dcdczdetctrl::R) reader structure"]
impl crate::Readable for DCDCZDETCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdczdetctrl::W](dcdczdetctrl::W) writer structure"]
impl crate::Writable for DCDCZDETCTRL {}
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register"]
pub mod dcdczdetctrl;
#[doc = "DCDC Power Train PFET Current Limiter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcclimctrl](dcdcclimctrl) module"]
pub type DCDCCLIMCTRL = crate::Reg<u32, _DCDCCLIMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCCLIMCTRL;
#[doc = "`read()` method returns [dcdcclimctrl::R](dcdcclimctrl::R) reader structure"]
impl crate::Readable for DCDCCLIMCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdcclimctrl::W](dcdcclimctrl::W) writer structure"]
impl crate::Writable for DCDCCLIMCTRL {}
#[doc = "DCDC Power Train PFET Current Limiter Control Register"]
pub mod dcdcclimctrl;
#[doc = "DCDC Low Noise Compensator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdclncompctrl](dcdclncompctrl) module"]
pub type DCDCLNCOMPCTRL = crate::Reg<u32, _DCDCLNCOMPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCLNCOMPCTRL;
#[doc = "`read()` method returns [dcdclncompctrl::R](dcdclncompctrl::R) reader structure"]
impl crate::Readable for DCDCLNCOMPCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdclncompctrl::W](dcdclncompctrl::W) writer structure"]
impl crate::Writable for DCDCLNCOMPCTRL {}
#[doc = "DCDC Low Noise Compensator Control Register"]
pub mod dcdclncompctrl;
#[doc = "DCDC Low Noise Voltage Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdclnvctrl](dcdclnvctrl) module"]
pub type DCDCLNVCTRL = crate::Reg<u32, _DCDCLNVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCLNVCTRL;
#[doc = "`read()` method returns [dcdclnvctrl::R](dcdclnvctrl::R) reader structure"]
impl crate::Readable for DCDCLNVCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdclnvctrl::W](dcdclnvctrl::W) writer structure"]
impl crate::Writable for DCDCLNVCTRL {}
#[doc = "DCDC Low Noise Voltage Register"]
pub mod dcdclnvctrl;
#[doc = "DCDC Controller Timing Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdctiming](dcdctiming) module"]
pub type DCDCTIMING = crate::Reg<u32, _DCDCTIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCTIMING;
#[doc = "`read()` method returns [dcdctiming::R](dcdctiming::R) reader structure"]
impl crate::Readable for DCDCTIMING {}
#[doc = "`write(|w| ..)` method takes [dcdctiming::W](dcdctiming::W) writer structure"]
impl crate::Writable for DCDCTIMING {}
#[doc = "DCDC Controller Timing Value Register"]
pub mod dcdctiming;
#[doc = "DCDC Low Power Voltage Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdclpvctrl](dcdclpvctrl) module"]
pub type DCDCLPVCTRL = crate::Reg<u32, _DCDCLPVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCLPVCTRL;
#[doc = "`read()` method returns [dcdclpvctrl::R](dcdclpvctrl::R) reader structure"]
impl crate::Readable for DCDCLPVCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdclpvctrl::W](dcdclpvctrl::W) writer structure"]
impl crate::Writable for DCDCLPVCTRL {}
#[doc = "DCDC Low Power Voltage Register"]
pub mod dcdclpvctrl;
#[doc = "DCDC Low Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdclpctrl](dcdclpctrl) module"]
pub type DCDCLPCTRL = crate::Reg<u32, _DCDCLPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCLPCTRL;
#[doc = "`read()` method returns [dcdclpctrl::R](dcdclpctrl::R) reader structure"]
impl crate::Readable for DCDCLPCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdclpctrl::W](dcdclpctrl::W) writer structure"]
impl crate::Writable for DCDCLPCTRL {}
#[doc = "DCDC Low Power Control Register"]
pub mod dcdclpctrl;
#[doc = "DCDC Low Noise Controller Frequency Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdclnfreqctrl](dcdclnfreqctrl) module"]
pub type DCDCLNFREQCTRL = crate::Reg<u32, _DCDCLNFREQCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCLNFREQCTRL;
#[doc = "`read()` method returns [dcdclnfreqctrl::R](dcdclnfreqctrl::R) reader structure"]
impl crate::Readable for DCDCLNFREQCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdclnfreqctrl::W](dcdclnfreqctrl::W) writer structure"]
impl crate::Writable for DCDCLNFREQCTRL {}
#[doc = "DCDC Low Noise Controller Frequency Control"]
pub mod dcdclnfreqctrl;
#[doc = "DCDC Read Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcsync](dcdcsync) module"]
pub type DCDCSYNC = crate::Reg<u32, _DCDCSYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCSYNC;
#[doc = "`read()` method returns [dcdcsync::R](dcdcsync::R) reader structure"]
impl crate::Readable for DCDCSYNC {}
#[doc = "DCDC Read Status Register"]
pub mod dcdcsync;
#[doc = "VMON AVDD Channel Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmonavddctrl](vmonavddctrl) module"]
pub type VMONAVDDCTRL = crate::Reg<u32, _VMONAVDDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VMONAVDDCTRL;
#[doc = "`read()` method returns [vmonavddctrl::R](vmonavddctrl::R) reader structure"]
impl crate::Readable for VMONAVDDCTRL {}
#[doc = "`write(|w| ..)` method takes [vmonavddctrl::W](vmonavddctrl::W) writer structure"]
impl crate::Writable for VMONAVDDCTRL {}
#[doc = "VMON AVDD Channel Control"]
pub mod vmonavddctrl;
#[doc = "Alternate VMON AVDD Channel Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmonaltavddctrl](vmonaltavddctrl) module"]
pub type VMONALTAVDDCTRL = crate::Reg<u32, _VMONALTAVDDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VMONALTAVDDCTRL;
#[doc = "`read()` method returns [vmonaltavddctrl::R](vmonaltavddctrl::R) reader structure"]
impl crate::Readable for VMONALTAVDDCTRL {}
#[doc = "`write(|w| ..)` method takes [vmonaltavddctrl::W](vmonaltavddctrl::W) writer structure"]
impl crate::Writable for VMONALTAVDDCTRL {}
#[doc = "Alternate VMON AVDD Channel Control"]
pub mod vmonaltavddctrl;
#[doc = "VMON DVDD Channel Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmondvddctrl](vmondvddctrl) module"]
pub type VMONDVDDCTRL = crate::Reg<u32, _VMONDVDDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VMONDVDDCTRL;
#[doc = "`read()` method returns [vmondvddctrl::R](vmondvddctrl::R) reader structure"]
impl crate::Readable for VMONDVDDCTRL {}
#[doc = "`write(|w| ..)` method takes [vmondvddctrl::W](vmondvddctrl::W) writer structure"]
impl crate::Writable for VMONDVDDCTRL {}
#[doc = "VMON DVDD Channel Control"]
pub mod vmondvddctrl;
#[doc = "VMON IOVDD0 Channel Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmonio0ctrl](vmonio0ctrl) module"]
pub type VMONIO0CTRL = crate::Reg<u32, _VMONIO0CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VMONIO0CTRL;
#[doc = "`read()` method returns [vmonio0ctrl::R](vmonio0ctrl::R) reader structure"]
impl crate::Readable for VMONIO0CTRL {}
#[doc = "`write(|w| ..)` method takes [vmonio0ctrl::W](vmonio0ctrl::W) writer structure"]
impl crate::Writable for VMONIO0CTRL {}
#[doc = "VMON IOVDD0 Channel Control"]
pub mod vmonio0ctrl;
#[doc = "Configurations Related to the Bias\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biasconf](biasconf) module"]
pub type BIASCONF = crate::Reg<u32, _BIASCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIASCONF;
#[doc = "`read()` method returns [biasconf::R](biasconf::R) reader structure"]
impl crate::Readable for BIASCONF {}
#[doc = "`write(|w| ..)` method takes [biasconf::W](biasconf::W) writer structure"]
impl crate::Writable for BIASCONF {}
#[doc = "Configurations Related to the Bias"]
pub mod biasconf;
#[doc = "Test Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [testlock](testlock) module"]
pub type TESTLOCK = crate::Reg<u32, _TESTLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TESTLOCK;
#[doc = "`read()` method returns [testlock::R](testlock::R) reader structure"]
impl crate::Readable for TESTLOCK {}
#[doc = "`write(|w| ..)` method takes [testlock::W](testlock::W) writer structure"]
impl crate::Writable for TESTLOCK {}
#[doc = "Test Lock Register"]
pub mod testlock;
#[doc = "Test Control Register for Regulator and BIAS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biastestctrl](biastestctrl) module"]
pub type BIASTESTCTRL = crate::Reg<u32, _BIASTESTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIASTESTCTRL;
#[doc = "`read()` method returns [biastestctrl::R](biastestctrl::R) reader structure"]
impl crate::Readable for BIASTESTCTRL {}
#[doc = "`write(|w| ..)` method takes [biastestctrl::W](biastestctrl::W) writer structure"]
impl crate::Writable for BIASTESTCTRL {}
#[doc = "Test Control Register for Regulator and BIAS"]
pub mod biastestctrl;
