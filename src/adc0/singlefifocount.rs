#[doc = "Register `SINGLEFIFOCOUNT` reader"]
pub type R = crate::R<SINGLEFIFOCOUNT_SPEC>;
#[doc = "Field `SINGLEDC` reader - Single Data Count"]
pub type SINGLEDC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Single Data Count"]
    #[inline(always)]
    pub fn singledc(&self) -> SINGLEDC_R {
        SINGLEDC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SINGLEFIFOCOUNT")
            .field("singledc", &format_args!("{}", self.singledc().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SINGLEFIFOCOUNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Single FIFO Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singlefifocount::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLEFIFOCOUNT_SPEC;
impl crate::RegisterSpec for SINGLEFIFOCOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singlefifocount::R`](R) reader structure"]
impl crate::Readable for SINGLEFIFOCOUNT_SPEC {}
#[doc = "`reset()` method sets SINGLEFIFOCOUNT to value 0"]
impl crate::Resettable for SINGLEFIFOCOUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
