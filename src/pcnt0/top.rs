#[doc = "Register `TOP` reader"]
pub type R = crate::R<TOP_SPEC>;
#[doc = "Field `TOP` reader - Counter Top Value"]
pub type TOP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Top Value"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOP")
            .field("top", &format_args!("{}", self.top().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TOP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Top Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`top::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOP_SPEC;
impl crate::RegisterSpec for TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`top::R`](R) reader structure"]
impl crate::Readable for TOP_SPEC {}
#[doc = "`reset()` method sets TOP to value 0xff"]
impl crate::Resettable for TOP_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
