#[doc = "Register `CNT` reader"]
pub type R = crate::R<CNT_SPEC>;
#[doc = "Field `CNT` reader - Counter Value"]
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT")
            .field("cnt", &format_args!("{}", self.cnt().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CNT_SPEC {}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
