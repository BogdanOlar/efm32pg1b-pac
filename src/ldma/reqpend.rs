#[doc = "Register `REQPEND` reader"]
pub type R = crate::R<REQPEND_SPEC>;
#[doc = "Field `REQPEND` reader - DMA Requests Pending"]
pub type REQPEND_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - DMA Requests Pending"]
    #[inline(always)]
    pub fn reqpend(&self) -> REQPEND_R {
        REQPEND_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REQPEND")
            .field("reqpend", &format_args!("{}", self.reqpend().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REQPEND_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA Channel Requests Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqpend::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REQPEND_SPEC;
impl crate::RegisterSpec for REQPEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reqpend::R`](R) reader structure"]
impl crate::Readable for REQPEND_SPEC {}
#[doc = "`reset()` method sets REQPEND to value 0"]
impl crate::Resettable for REQPEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
