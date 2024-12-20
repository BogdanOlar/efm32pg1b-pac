#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    lock: Lock,
    ram0ctrl: Ram0ctrl,
    cmd: Cmd,
    _reserved5: [u8; 0x04],
    em4ctrl: Em4ctrl,
    templimits: Templimits,
    temp: Temp,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    pwrlock: Pwrlock,
    pwrcfg: Pwrcfg,
    pwrctrl: Pwrctrl,
    dcdcctrl: Dcdcctrl,
    _reserved16: [u8; 0x08],
    dcdcmiscctrl: Dcdcmiscctrl,
    dcdczdetctrl: Dcdczdetctrl,
    dcdcclimctrl: Dcdcclimctrl,
    dcdclncompctrl: Dcdclncompctrl,
    dcdclnvctrl: Dcdclnvctrl,
    dcdctiming: Dcdctiming,
    dcdclpvctrl: Dcdclpvctrl,
    _reserved23: [u8; 0x04],
    dcdclpctrl: Dcdclpctrl,
    dcdclnfreqctrl: Dcdclnfreqctrl,
    _reserved25: [u8; 0x04],
    dcdcsync: Dcdcsync,
    _reserved26: [u8; 0x14],
    vmonavddctrl: Vmonavddctrl,
    vmonaltavddctrl: Vmonaltavddctrl,
    vmondvddctrl: Vmondvddctrl,
    vmonio0ctrl: Vmonio0ctrl,
    _reserved30: [u8; 0xc4],
    biasconf: Biasconf,
    _reserved31: [u8; 0x28],
    testlock: Testlock,
    _reserved32: [u8; 0x08],
    biastestctrl: Biastestctrl,
}
impl RegisterBlock {
    ///0x00 - Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x04 - Status Register
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    ///0x08 - Configuration Lock Register
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    ///0x0c - Memory Control Register
    #[inline(always)]
    pub const fn ram0ctrl(&self) -> &Ram0ctrl {
        &self.ram0ctrl
    }
    ///0x10 - Command Register
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    ///0x18 - EM4 Control Register
    #[inline(always)]
    pub const fn em4ctrl(&self) -> &Em4ctrl {
        &self.em4ctrl
    }
    ///0x1c - Temperature Limits for Interrupt Generation
    #[inline(always)]
    pub const fn templimits(&self) -> &Templimits {
        &self.templimits
    }
    ///0x20 - Value of Last Temperature Measurement
    #[inline(always)]
    pub const fn temp(&self) -> &Temp {
        &self.temp
    }
    ///0x24 - Interrupt Flag Register
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    ///0x28 - Interrupt Flag Set Register
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    ///0x2c - Interrupt Flag Clear Register
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    ///0x30 - Interrupt Enable Register
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    ///0x34 - Regulator and Supply Lock Register
    #[inline(always)]
    pub const fn pwrlock(&self) -> &Pwrlock {
        &self.pwrlock
    }
    ///0x38 - Power Configuration Register
    #[inline(always)]
    pub const fn pwrcfg(&self) -> &Pwrcfg {
        &self.pwrcfg
    }
    ///0x3c - Power Control Register
    #[inline(always)]
    pub const fn pwrctrl(&self) -> &Pwrctrl {
        &self.pwrctrl
    }
    ///0x40 - DCDC Control
    #[inline(always)]
    pub const fn dcdcctrl(&self) -> &Dcdcctrl {
        &self.dcdcctrl
    }
    ///0x4c - DCDC Miscellaneous Control Register
    #[inline(always)]
    pub const fn dcdcmiscctrl(&self) -> &Dcdcmiscctrl {
        &self.dcdcmiscctrl
    }
    ///0x50 - DCDC Power Train NFET Zero Current Detector Control Register
    #[inline(always)]
    pub const fn dcdczdetctrl(&self) -> &Dcdczdetctrl {
        &self.dcdczdetctrl
    }
    ///0x54 - DCDC Power Train PFET Current Limiter Control Register
    #[inline(always)]
    pub const fn dcdcclimctrl(&self) -> &Dcdcclimctrl {
        &self.dcdcclimctrl
    }
    ///0x58 - DCDC Low Noise Compensator Control Register
    #[inline(always)]
    pub const fn dcdclncompctrl(&self) -> &Dcdclncompctrl {
        &self.dcdclncompctrl
    }
    ///0x5c - DCDC Low Noise Voltage Register
    #[inline(always)]
    pub const fn dcdclnvctrl(&self) -> &Dcdclnvctrl {
        &self.dcdclnvctrl
    }
    ///0x60 - DCDC Controller Timing Value Register
    #[inline(always)]
    pub const fn dcdctiming(&self) -> &Dcdctiming {
        &self.dcdctiming
    }
    ///0x64 - DCDC Low Power Voltage Register
    #[inline(always)]
    pub const fn dcdclpvctrl(&self) -> &Dcdclpvctrl {
        &self.dcdclpvctrl
    }
    ///0x6c - DCDC Low Power Control Register
    #[inline(always)]
    pub const fn dcdclpctrl(&self) -> &Dcdclpctrl {
        &self.dcdclpctrl
    }
    ///0x70 - DCDC Low Noise Controller Frequency Control
    #[inline(always)]
    pub const fn dcdclnfreqctrl(&self) -> &Dcdclnfreqctrl {
        &self.dcdclnfreqctrl
    }
    ///0x78 - DCDC Read Status Register
    #[inline(always)]
    pub const fn dcdcsync(&self) -> &Dcdcsync {
        &self.dcdcsync
    }
    ///0x90 - VMON AVDD Channel Control
    #[inline(always)]
    pub const fn vmonavddctrl(&self) -> &Vmonavddctrl {
        &self.vmonavddctrl
    }
    ///0x94 - Alternate VMON AVDD Channel Control
    #[inline(always)]
    pub const fn vmonaltavddctrl(&self) -> &Vmonaltavddctrl {
        &self.vmonaltavddctrl
    }
    ///0x98 - VMON DVDD Channel Control
    #[inline(always)]
    pub const fn vmondvddctrl(&self) -> &Vmondvddctrl {
        &self.vmondvddctrl
    }
    ///0x9c - VMON IOVDD0 Channel Control
    #[inline(always)]
    pub const fn vmonio0ctrl(&self) -> &Vmonio0ctrl {
        &self.vmonio0ctrl
    }
    ///0x164 - Configurations Related to the Bias
    #[inline(always)]
    pub const fn biasconf(&self) -> &Biasconf {
        &self.biasconf
    }
    ///0x190 - Test Lock Register
    #[inline(always)]
    pub const fn testlock(&self) -> &Testlock {
        &self.testlock
    }
    ///0x19c - Test Control Register for Regulator and BIAS
    #[inline(always)]
    pub const fn biastestctrl(&self) -> &Biastestctrl {
        &self.biastestctrl
    }
}
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
///STATUS (r) register accessor: Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@status`]
///module
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::STATUSrs>;
///Status Register
pub mod status;
///LOCK (rw) register accessor: Configuration Lock Register
///
///You can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lock`]
///module
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LOCKrs>;
///Configuration Lock Register
pub mod lock;
///RAM0CTRL (rw) register accessor: Memory Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ram0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ram0ctrl`]
///module
#[doc(alias = "RAM0CTRL")]
pub type Ram0ctrl = crate::Reg<ram0ctrl::RAM0CTRLrs>;
///Memory Control Register
pub mod ram0ctrl;
///CMD (w) register accessor: Command Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cmd`]
///module
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CMDrs>;
///Command Register
pub mod cmd;
///EM4CTRL (rw) register accessor: EM4 Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`em4ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@em4ctrl`]
///module
#[doc(alias = "EM4CTRL")]
pub type Em4ctrl = crate::Reg<em4ctrl::EM4CTRLrs>;
///EM4 Control Register
pub mod em4ctrl;
///TEMPLIMITS (rw) register accessor: Temperature Limits for Interrupt Generation
///
///You can [`read`](crate::Reg::read) this register and get [`templimits::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`templimits::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@templimits`]
///module
#[doc(alias = "TEMPLIMITS")]
pub type Templimits = crate::Reg<templimits::TEMPLIMITSrs>;
///Temperature Limits for Interrupt Generation
pub mod templimits;
///TEMP (r) register accessor: Value of Last Temperature Measurement
///
///You can [`read`](crate::Reg::read) this register and get [`temp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@temp`]
///module
#[doc(alias = "TEMP")]
pub type Temp = crate::Reg<temp::TEMPrs>;
///Value of Last Temperature Measurement
pub mod temp;
///IF (r) register accessor: Interrupt Flag Register
///
///You can [`read`](crate::Reg::read) this register and get [`if_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@if_`]
///module
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IFrs>;
///Interrupt Flag Register
pub mod if_;
///IFS (w) register accessor: Interrupt Flag Set Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ifs`]
///module
#[doc(alias = "IFS")]
pub type Ifs = crate::Reg<ifs::IFSrs>;
///Interrupt Flag Set Register
pub mod ifs;
///IFC (w) register accessor: Interrupt Flag Clear Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ifc`]
///module
#[doc(alias = "IFC")]
pub type Ifc = crate::Reg<ifc::IFCrs>;
///Interrupt Flag Clear Register
pub mod ifc;
///IEN (rw) register accessor: Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ien`]
///module
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IENrs>;
///Interrupt Enable Register
pub mod ien;
///PWRLOCK (rw) register accessor: Regulator and Supply Lock Register
///
///You can [`read`](crate::Reg::read) this register and get [`pwrlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pwrlock`]
///module
#[doc(alias = "PWRLOCK")]
pub type Pwrlock = crate::Reg<pwrlock::PWRLOCKrs>;
///Regulator and Supply Lock Register
pub mod pwrlock;
///PWRCFG (rw) register accessor: Power Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`pwrcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pwrcfg`]
///module
#[doc(alias = "PWRCFG")]
pub type Pwrcfg = crate::Reg<pwrcfg::PWRCFGrs>;
///Power Configuration Register
pub mod pwrcfg;
///PWRCTRL (rw) register accessor: Power Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`pwrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pwrctrl`]
///module
#[doc(alias = "PWRCTRL")]
pub type Pwrctrl = crate::Reg<pwrctrl::PWRCTRLrs>;
///Power Control Register
pub mod pwrctrl;
///DCDCCTRL (rw) register accessor: DCDC Control
///
///You can [`read`](crate::Reg::read) this register and get [`dcdcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dcdcctrl`]
///module
#[doc(alias = "DCDCCTRL")]
pub type Dcdcctrl = crate::Reg<dcdcctrl::DCDCCTRLrs>;
///DCDC Control
pub mod dcdcctrl;
///DCDCMISCCTRL (rw) register accessor: DCDC Miscellaneous Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdcmiscctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcmiscctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dcdcmiscctrl`]
///module
#[doc(alias = "DCDCMISCCTRL")]
pub type Dcdcmiscctrl = crate::Reg<dcdcmiscctrl::DCDCMISCCTRLrs>;
///DCDC Miscellaneous Control Register
pub mod dcdcmiscctrl;
///DCDCZDETCTRL (rw) register accessor: DCDC Power Train NFET Zero Current Detector Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdczdetctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdczdetctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dcdczdetctrl`]
///module
#[doc(alias = "DCDCZDETCTRL")]
pub type Dcdczdetctrl = crate::Reg<dcdczdetctrl::DCDCZDETCTRLrs>;
///DCDC Power Train NFET Zero Current Detector Control Register
pub mod dcdczdetctrl;
///DCDCCLIMCTRL (rw) register accessor: DCDC Power Train PFET Current Limiter Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdcclimctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcclimctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dcdcclimctrl`]
///module
#[doc(alias = "DCDCCLIMCTRL")]
pub type Dcdcclimctrl = crate::Reg<dcdcclimctrl::DCDCCLIMCTRLrs>;
///DCDC Power Train PFET Current Limiter Control Register
pub mod dcdcclimctrl;
///DCDCLNCOMPCTRL (rw) register accessor: DCDC Low Noise Compensator Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdclncompctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclncompctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dcdclncompctrl`]
///module
#[doc(alias = "DCDCLNCOMPCTRL")]
pub type Dcdclncompctrl = crate::Reg<dcdclncompctrl::DCDCLNCOMPCTRLrs>;
///DCDC Low Noise Compensator Control Register
pub mod dcdclncompctrl;
///DCDCLNVCTRL (rw) register accessor: DCDC Low Noise Voltage Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdclnvctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclnvctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dcdclnvctrl`]
///module
#[doc(alias = "DCDCLNVCTRL")]
pub type Dcdclnvctrl = crate::Reg<dcdclnvctrl::DCDCLNVCTRLrs>;
///DCDC Low Noise Voltage Register
pub mod dcdclnvctrl;
///DCDCTIMING (rw) register accessor: DCDC Controller Timing Value Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdctiming::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdctiming::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dcdctiming`]
///module
#[doc(alias = "DCDCTIMING")]
pub type Dcdctiming = crate::Reg<dcdctiming::DCDCTIMINGrs>;
///DCDC Controller Timing Value Register
pub mod dcdctiming;
///DCDCLPVCTRL (rw) register accessor: DCDC Low Power Voltage Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdclpvctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclpvctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dcdclpvctrl`]
///module
#[doc(alias = "DCDCLPVCTRL")]
pub type Dcdclpvctrl = crate::Reg<dcdclpvctrl::DCDCLPVCTRLrs>;
///DCDC Low Power Voltage Register
pub mod dcdclpvctrl;
///DCDCLPCTRL (rw) register accessor: DCDC Low Power Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdclpctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclpctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dcdclpctrl`]
///module
#[doc(alias = "DCDCLPCTRL")]
pub type Dcdclpctrl = crate::Reg<dcdclpctrl::DCDCLPCTRLrs>;
///DCDC Low Power Control Register
pub mod dcdclpctrl;
///DCDCLNFREQCTRL (rw) register accessor: DCDC Low Noise Controller Frequency Control
///
///You can [`read`](crate::Reg::read) this register and get [`dcdclnfreqctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclnfreqctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dcdclnfreqctrl`]
///module
#[doc(alias = "DCDCLNFREQCTRL")]
pub type Dcdclnfreqctrl = crate::Reg<dcdclnfreqctrl::DCDCLNFREQCTRLrs>;
///DCDC Low Noise Controller Frequency Control
pub mod dcdclnfreqctrl;
///DCDCSYNC (r) register accessor: DCDC Read Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdcsync::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dcdcsync`]
///module
#[doc(alias = "DCDCSYNC")]
pub type Dcdcsync = crate::Reg<dcdcsync::DCDCSYNCrs>;
///DCDC Read Status Register
pub mod dcdcsync;
///VMONAVDDCTRL (rw) register accessor: VMON AVDD Channel Control
///
///You can [`read`](crate::Reg::read) this register and get [`vmonavddctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmonavddctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vmonavddctrl`]
///module
#[doc(alias = "VMONAVDDCTRL")]
pub type Vmonavddctrl = crate::Reg<vmonavddctrl::VMONAVDDCTRLrs>;
///VMON AVDD Channel Control
pub mod vmonavddctrl;
///VMONALTAVDDCTRL (rw) register accessor: Alternate VMON AVDD Channel Control
///
///You can [`read`](crate::Reg::read) this register and get [`vmonaltavddctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmonaltavddctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vmonaltavddctrl`]
///module
#[doc(alias = "VMONALTAVDDCTRL")]
pub type Vmonaltavddctrl = crate::Reg<vmonaltavddctrl::VMONALTAVDDCTRLrs>;
///Alternate VMON AVDD Channel Control
pub mod vmonaltavddctrl;
///VMONDVDDCTRL (rw) register accessor: VMON DVDD Channel Control
///
///You can [`read`](crate::Reg::read) this register and get [`vmondvddctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmondvddctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vmondvddctrl`]
///module
#[doc(alias = "VMONDVDDCTRL")]
pub type Vmondvddctrl = crate::Reg<vmondvddctrl::VMONDVDDCTRLrs>;
///VMON DVDD Channel Control
pub mod vmondvddctrl;
///VMONIO0CTRL (rw) register accessor: VMON IOVDD0 Channel Control
///
///You can [`read`](crate::Reg::read) this register and get [`vmonio0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmonio0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vmonio0ctrl`]
///module
#[doc(alias = "VMONIO0CTRL")]
pub type Vmonio0ctrl = crate::Reg<vmonio0ctrl::VMONIO0CTRLrs>;
///VMON IOVDD0 Channel Control
pub mod vmonio0ctrl;
///BIASCONF (rw) register accessor: Configurations Related to the Bias
///
///You can [`read`](crate::Reg::read) this register and get [`biasconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biasconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@biasconf`]
///module
#[doc(alias = "BIASCONF")]
pub type Biasconf = crate::Reg<biasconf::BIASCONFrs>;
///Configurations Related to the Bias
pub mod biasconf;
///TESTLOCK (rw) register accessor: Test Lock Register
///
///You can [`read`](crate::Reg::read) this register and get [`testlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@testlock`]
///module
#[doc(alias = "TESTLOCK")]
pub type Testlock = crate::Reg<testlock::TESTLOCKrs>;
///Test Lock Register
pub mod testlock;
///BIASTESTCTRL (rw) register accessor: Test Control Register for Regulator and BIAS
///
///You can [`read`](crate::Reg::read) this register and get [`biastestctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biastestctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@biastestctrl`]
///module
#[doc(alias = "BIASTESTCTRL")]
pub type Biastestctrl = crate::Reg<biastestctrl::BIASTESTCTRLrs>;
///Test Control Register for Regulator and BIAS
pub mod biastestctrl;
