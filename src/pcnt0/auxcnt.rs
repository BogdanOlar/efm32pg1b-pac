#[doc = "Register `AUXCNT` reader"]
pub type R = crate::R<AUXCNT_SPEC>;
#[doc = "Field `AUXCNT` reader - Auxiliary Counter Value"]
pub type AUXCNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Auxiliary Counter Value"]
    #[inline(always)]
    pub fn auxcnt(&self) -> AUXCNT_R {
        AUXCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUXCNT")
            .field("auxcnt", &format_args!("{}", self.auxcnt().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<AUXCNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Auxiliary Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUXCNT_SPEC;
impl crate::RegisterSpec for AUXCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxcnt::R`](R) reader structure"]
impl crate::Readable for AUXCNT_SPEC {}
#[doc = "`reset()` method sets AUXCNT to value 0"]
impl crate::Resettable for AUXCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
