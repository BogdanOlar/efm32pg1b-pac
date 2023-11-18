#[doc = "Register `CACHEHITS` reader"]
pub type R = crate::R<CACHEHITS_SPEC>;
#[doc = "Field `CACHEHITS` reader - Cache Hits Since Last Performance Counter Start Command"]
pub type CACHEHITS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Cache Hits Since Last Performance Counter Start Command"]
    #[inline(always)]
    pub fn cachehits(&self) -> CACHEHITS_R {
        CACHEHITS_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHEHITS")
            .field("cachehits", &format_args!("{}", self.cachehits().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CACHEHITS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Cache Hits Performance Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cachehits::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHEHITS_SPEC;
impl crate::RegisterSpec for CACHEHITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cachehits::R`](R) reader structure"]
impl crate::Readable for CACHEHITS_SPEC {}
#[doc = "`reset()` method sets CACHEHITS to value 0"]
impl crate::Resettable for CACHEHITS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
