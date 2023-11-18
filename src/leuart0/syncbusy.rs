#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSY_SPEC>;
#[doc = "Field `CTRL` reader - CTRL Register Busy"]
pub type CTRL_R = crate::BitReader;
#[doc = "Field `CMD` reader - CMD Register Busy"]
pub type CMD_R = crate::BitReader;
#[doc = "Field `CLKDIV` reader - CLKDIV Register Busy"]
pub type CLKDIV_R = crate::BitReader;
#[doc = "Field `STARTFRAME` reader - STARTFRAME Register Busy"]
pub type STARTFRAME_R = crate::BitReader;
#[doc = "Field `SIGFRAME` reader - SIGFRAME Register Busy"]
pub type SIGFRAME_R = crate::BitReader;
#[doc = "Field `TXDATAX` reader - TXDATAX Register Busy"]
pub type TXDATAX_R = crate::BitReader;
#[doc = "Field `TXDATA` reader - TXDATA Register Busy"]
pub type TXDATA_R = crate::BitReader;
#[doc = "Field `PULSECTRL` reader - PULSECTRL Register Busy"]
pub type PULSECTRL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CTRL Register Busy"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMD Register Busy"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CLKDIV Register Busy"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STARTFRAME Register Busy"]
    #[inline(always)]
    pub fn startframe(&self) -> STARTFRAME_R {
        STARTFRAME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SIGFRAME Register Busy"]
    #[inline(always)]
    pub fn sigframe(&self) -> SIGFRAME_R {
        SIGFRAME_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXDATAX Register Busy"]
    #[inline(always)]
    pub fn txdatax(&self) -> TXDATAX_R {
        TXDATAX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXDATA Register Busy"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PULSECTRL Register Busy"]
    #[inline(always)]
    pub fn pulsectrl(&self) -> PULSECTRL_R {
        PULSECTRL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNCBUSY")
            .field("ctrl", &format_args!("{}", self.ctrl().bit()))
            .field("cmd", &format_args!("{}", self.cmd().bit()))
            .field("clkdiv", &format_args!("{}", self.clkdiv().bit()))
            .field("startframe", &format_args!("{}", self.startframe().bit()))
            .field("sigframe", &format_args!("{}", self.sigframe().bit()))
            .field("txdatax", &format_args!("{}", self.txdatax().bit()))
            .field("txdata", &format_args!("{}", self.txdata().bit()))
            .field("pulsectrl", &format_args!("{}", self.pulsectrl().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SYNCBUSY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Synchronization Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
