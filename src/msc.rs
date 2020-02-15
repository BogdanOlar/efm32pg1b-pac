#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Memory System Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Read Control Register"]
    pub readctrl: READCTRL,
    #[doc = "0x08 - Write Control Register"]
    pub writectrl: WRITECTRL,
    #[doc = "0x0c - Write Command Register"]
    pub writecmd: WRITECMD,
    #[doc = "0x10 - Page Erase/Write Address Buffer"]
    pub addrb: ADDRB,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - Write Data Register"]
    pub wdata: WDATA,
    #[doc = "0x1c - Status Register"]
    pub status: STATUS,
    _reserved7: [u8; 16usize],
    #[doc = "0x30 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x34 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x38 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x3c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x40 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x44 - Flash Cache Command Register"]
    pub cachecmd: CACHECMD,
    #[doc = "0x48 - Cache Hits Performance Counter"]
    pub cachehits: CACHEHITS,
    #[doc = "0x4c - Cache Misses Performance Counter"]
    pub cachemisses: CACHEMISSES,
    _reserved15: [u8; 4usize],
    #[doc = "0x54 - Mass Erase Lock Register"]
    pub masslock: MASSLOCK,
    _reserved16: [u8; 4usize],
    #[doc = "0x5c - Startup Control"]
    pub startup: STARTUP,
    _reserved17: [u8; 20usize],
    #[doc = "0x74 - Command Register"]
    pub cmd: CMD,
}
#[doc = "Memory System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Memory System Control Register"]
pub mod ctrl;
#[doc = "Read Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readctrl](readctrl) module"]
pub type READCTRL = crate::Reg<u32, _READCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READCTRL;
#[doc = "`read()` method returns [readctrl::R](readctrl::R) reader structure"]
impl crate::Readable for READCTRL {}
#[doc = "`write(|w| ..)` method takes [readctrl::W](readctrl::W) writer structure"]
impl crate::Writable for READCTRL {}
#[doc = "Read Control Register"]
pub mod readctrl;
#[doc = "Write Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writectrl](writectrl) module"]
pub type WRITECTRL = crate::Reg<u32, _WRITECTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRITECTRL;
#[doc = "`read()` method returns [writectrl::R](writectrl::R) reader structure"]
impl crate::Readable for WRITECTRL {}
#[doc = "`write(|w| ..)` method takes [writectrl::W](writectrl::W) writer structure"]
impl crate::Writable for WRITECTRL {}
#[doc = "Write Control Register"]
pub mod writectrl;
#[doc = "Write Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writecmd](writecmd) module"]
pub type WRITECMD = crate::Reg<u32, _WRITECMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRITECMD;
#[doc = "`write(|w| ..)` method takes [writecmd::W](writecmd::W) writer structure"]
impl crate::Writable for WRITECMD {}
#[doc = "Write Command Register"]
pub mod writecmd;
#[doc = "Page Erase/Write Address Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrb](addrb) module"]
pub type ADDRB = crate::Reg<u32, _ADDRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRB;
#[doc = "`read()` method returns [addrb::R](addrb::R) reader structure"]
impl crate::Readable for ADDRB {}
#[doc = "`write(|w| ..)` method takes [addrb::W](addrb::W) writer structure"]
impl crate::Writable for ADDRB {}
#[doc = "Page Erase/Write Address Buffer"]
pub mod addrb;
#[doc = "Write Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdata](wdata) module"]
pub type WDATA = crate::Reg<u32, _WDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDATA;
#[doc = "`read()` method returns [wdata::R](wdata::R) reader structure"]
impl crate::Readable for WDATA {}
#[doc = "`write(|w| ..)` method takes [wdata::W](wdata::W) writer structure"]
impl crate::Writable for WDATA {}
#[doc = "Write Data Register"]
pub mod wdata;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
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
#[doc = "Flash Cache Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cachecmd](cachecmd) module"]
pub type CACHECMD = crate::Reg<u32, _CACHECMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACHECMD;
#[doc = "`write(|w| ..)` method takes [cachecmd::W](cachecmd::W) writer structure"]
impl crate::Writable for CACHECMD {}
#[doc = "Flash Cache Command Register"]
pub mod cachecmd;
#[doc = "Cache Hits Performance Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cachehits](cachehits) module"]
pub type CACHEHITS = crate::Reg<u32, _CACHEHITS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACHEHITS;
#[doc = "`read()` method returns [cachehits::R](cachehits::R) reader structure"]
impl crate::Readable for CACHEHITS {}
#[doc = "Cache Hits Performance Counter"]
pub mod cachehits;
#[doc = "Cache Misses Performance Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cachemisses](cachemisses) module"]
pub type CACHEMISSES = crate::Reg<u32, _CACHEMISSES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACHEMISSES;
#[doc = "`read()` method returns [cachemisses::R](cachemisses::R) reader structure"]
impl crate::Readable for CACHEMISSES {}
#[doc = "Cache Misses Performance Counter"]
pub mod cachemisses;
#[doc = "Mass Erase Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [masslock](masslock) module"]
pub type MASSLOCK = crate::Reg<u32, _MASSLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASSLOCK;
#[doc = "`read()` method returns [masslock::R](masslock::R) reader structure"]
impl crate::Readable for MASSLOCK {}
#[doc = "`write(|w| ..)` method takes [masslock::W](masslock::W) writer structure"]
impl crate::Writable for MASSLOCK {}
#[doc = "Mass Erase Lock Register"]
pub mod masslock;
#[doc = "Startup Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [startup](startup) module"]
pub type STARTUP = crate::Reg<u32, _STARTUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTUP;
#[doc = "`read()` method returns [startup::R](startup::R) reader structure"]
impl crate::Readable for STARTUP {}
#[doc = "`write(|w| ..)` method takes [startup::W](startup::W) writer structure"]
impl crate::Writable for STARTUP {}
#[doc = "Startup Control"]
pub mod startup;
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
