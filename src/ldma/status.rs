#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `ANYBUSY` reader - Any DMA Channel Busy"]
pub type ANYBUSY_R = crate::BitReader;
#[doc = "Field `ANYREQ` reader - Any DMA Channel Request Pending"]
pub type ANYREQ_R = crate::BitReader;
#[doc = "Field `CHGRANT` reader - Granted Channel Number"]
pub type CHGRANT_R = crate::FieldReader;
#[doc = "Field `CHERROR` reader - Errant Channel Number"]
pub type CHERROR_R = crate::FieldReader;
#[doc = "Field `FIFOLEVEL` reader - FIFO Level"]
pub type FIFOLEVEL_R = crate::FieldReader;
#[doc = "Field `CHNUM` reader - Number of Channels"]
pub type CHNUM_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Any DMA Channel Busy"]
    #[inline(always)]
    pub fn anybusy(&self) -> ANYBUSY_R {
        ANYBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Any DMA Channel Request Pending"]
    #[inline(always)]
    pub fn anyreq(&self) -> ANYREQ_R {
        ANYREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Granted Channel Number"]
    #[inline(always)]
    pub fn chgrant(&self) -> CHGRANT_R {
        CHGRANT_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Errant Channel Number"]
    #[inline(always)]
    pub fn cherror(&self) -> CHERROR_R {
        CHERROR_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:20 - FIFO Level"]
    #[inline(always)]
    pub fn fifolevel(&self) -> FIFOLEVEL_R {
        FIFOLEVEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Number of Channels"]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("anybusy", &format_args!("{}", self.anybusy().bit()))
            .field("anyreq", &format_args!("{}", self.anyreq().bit()))
            .field("chgrant", &format_args!("{}", self.chgrant().bits()))
            .field("cherror", &format_args!("{}", self.cherror().bits()))
            .field("fifolevel", &format_args!("{}", self.fifolevel().bits()))
            .field("chnum", &format_args!("{}", self.chnum().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0x0810_0000"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0810_0000;
}
