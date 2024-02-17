#[doc = "Register `AUXCNT` reader"]
pub type R = crate::R<AUXCNTrs>;
#[doc = "Field `AUXCNT` reader - Auxiliary Counter Value"]
pub type AUXCNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Auxiliary Counter Value"]
    #[inline(always)]
    pub fn auxcnt(&self) -> AUXCNT_R {
        AUXCNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Auxiliary Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUXCNTrs;
impl crate::RegisterSpec for AUXCNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxcnt::R`](R) reader structure"]
impl crate::Readable for AUXCNTrs {}
#[doc = "`reset()` method sets AUXCNT to value 0"]
impl crate::Resettable for AUXCNTrs {
    const RESET_VALUE: u32 = 0;
}
