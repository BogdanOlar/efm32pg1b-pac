#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSY_SPEC>;
#[doc = "Field `CTRL` reader - CTRL Register Busy"]
pub type CTRL_R = crate::BitReader;
#[doc = "Field `CMD` reader - CMD Register Busy"]
pub type CMD_R = crate::BitReader;
#[doc = "Field `TOPB` reader - TOPB Register Busy"]
pub type TOPB_R = crate::BitReader;
#[doc = "Field `OVSCFG` reader - OVSCFG Register Busy"]
pub type OVSCFG_R = crate::BitReader;
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
    #[doc = "Bit 2 - TOPB Register Busy"]
    #[inline(always)]
    pub fn topb(&self) -> TOPB_R {
        TOPB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OVSCFG Register Busy"]
    #[inline(always)]
    pub fn ovscfg(&self) -> OVSCFG_R {
        OVSCFG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNCBUSY")
            .field("ctrl", &format_args!("{}", self.ctrl().bit()))
            .field("cmd", &format_args!("{}", self.cmd().bit()))
            .field("topb", &format_args!("{}", self.topb().bit()))
            .field("ovscfg", &format_args!("{}", self.ovscfg().bit()))
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
