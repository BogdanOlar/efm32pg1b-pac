#[doc = "Register `DATAREV` reader"]
pub type R = crate::R<DATAREV_SPEC>;
#[doc = "Field `DATAREV` reader - Data Reverse Value"]
pub type DATAREV_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data Reverse Value"]
    #[inline(always)]
    pub fn datarev(&self) -> DATAREV_R {
        DATAREV_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATAREV")
            .field("datarev", &format_args!("{}", self.datarev().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DATAREV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "CRC Data Reverse Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datarev::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATAREV_SPEC;
impl crate::RegisterSpec for DATAREV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datarev::R`](R) reader structure"]
impl crate::Readable for DATAREV_SPEC {}
#[doc = "`reset()` method sets DATAREV to value 0"]
impl crate::Resettable for DATAREV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
