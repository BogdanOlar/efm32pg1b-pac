#[doc = "Register `CHBUSY` reader"]
pub type R = crate::R<CHBUSY_SPEC>;
#[doc = "Field `BUSY` reader - Channels Busy"]
pub type BUSY_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Channels Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHBUSY")
            .field("busy", &format_args!("{}", self.busy().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CHBUSY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA Channel Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHBUSY_SPEC;
impl crate::RegisterSpec for CHBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chbusy::R`](R) reader structure"]
impl crate::Readable for CHBUSY_SPEC {}
#[doc = "`reset()` method sets CHBUSY to value 0"]
impl crate::Resettable for CHBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
