#[doc = "Register `SCANDATAP` reader"]
pub type R = crate::R<SCANDATAP_SPEC>;
#[doc = "Field `DATAP` reader - Scan Conversion Result Data Peek"]
pub type DATAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Scan Conversion Result Data Peek"]
    #[inline(always)]
    pub fn datap(&self) -> DATAP_R {
        DATAP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCANDATAP")
            .field("datap", &format_args!("{}", self.datap().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SCANDATAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Scan Sequence Result Data Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scandatap::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANDATAP_SPEC;
impl crate::RegisterSpec for SCANDATAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scandatap::R`](R) reader structure"]
impl crate::Readable for SCANDATAP_SPEC {}
#[doc = "`reset()` method sets SCANDATAP to value 0"]
impl crate::Resettable for SCANDATAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
