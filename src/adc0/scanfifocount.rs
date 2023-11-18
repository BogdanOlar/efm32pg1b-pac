#[doc = "Register `SCANFIFOCOUNT` reader"]
pub type R = crate::R<SCANFIFOCOUNT_SPEC>;
#[doc = "Field `SCANDC` reader - Scan Data Count"]
pub type SCANDC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Scan Data Count"]
    #[inline(always)]
    pub fn scandc(&self) -> SCANDC_R {
        SCANDC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCANFIFOCOUNT")
            .field("scandc", &format_args!("{}", self.scandc().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SCANFIFOCOUNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Scan FIFO Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanfifocount::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANFIFOCOUNT_SPEC;
impl crate::RegisterSpec for SCANFIFOCOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanfifocount::R`](R) reader structure"]
impl crate::Readable for SCANFIFOCOUNT_SPEC {}
#[doc = "`reset()` method sets SCANFIFOCOUNT to value 0"]
impl crate::Resettable for SCANFIFOCOUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
