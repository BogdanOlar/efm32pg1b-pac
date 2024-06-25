#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSYrs>;
#[doc = "Field `CTRL` reader - CTRL Register Busy"]
pub type CtrlR = crate::BitReader;
#[doc = "Field `CMD` reader - CMD Register Busy"]
pub type CmdR = crate::BitReader;
#[doc = "Field `CLKDIV` reader - CLKDIV Register Busy"]
pub type ClkdivR = crate::BitReader;
#[doc = "Field `STARTFRAME` reader - STARTFRAME Register Busy"]
pub type StartframeR = crate::BitReader;
#[doc = "Field `SIGFRAME` reader - SIGFRAME Register Busy"]
pub type SigframeR = crate::BitReader;
#[doc = "Field `TXDATAX` reader - TXDATAX Register Busy"]
pub type TxdataxR = crate::BitReader;
#[doc = "Field `TXDATA` reader - TXDATA Register Busy"]
pub type TxdataR = crate::BitReader;
#[doc = "Field `PULSECTRL` reader - PULSECTRL Register Busy"]
pub type PulsectrlR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CTRL Register Busy"]
    #[inline(always)]
    pub fn ctrl(&self) -> CtrlR {
        CtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMD Register Busy"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CLKDIV Register Busy"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STARTFRAME Register Busy"]
    #[inline(always)]
    pub fn startframe(&self) -> StartframeR {
        StartframeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SIGFRAME Register Busy"]
    #[inline(always)]
    pub fn sigframe(&self) -> SigframeR {
        SigframeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXDATAX Register Busy"]
    #[inline(always)]
    pub fn txdatax(&self) -> TxdataxR {
        TxdataxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXDATA Register Busy"]
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PULSECTRL Register Busy"]
    #[inline(always)]
    pub fn pulsectrl(&self) -> PulsectrlR {
        PulsectrlR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNCBUSY")
            .field("ctrl", &self.ctrl())
            .field("cmd", &self.cmd())
            .field("clkdiv", &self.clkdiv())
            .field("startframe", &self.startframe())
            .field("sigframe", &self.sigframe())
            .field("txdatax", &self.txdatax())
            .field("txdata", &self.txdata())
            .field("pulsectrl", &self.pulsectrl())
            .finish()
    }
}
#[doc = "Synchronization Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCBUSYrs;
impl crate::RegisterSpec for SYNCBUSYrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SYNCBUSYrs {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSYrs {
    const RESET_VALUE: u32 = 0;
}
