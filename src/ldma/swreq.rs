#[doc = "Register `SWREQ` writer"]
pub type W = crate::W<SWREQ_SPEC>;
#[doc = "Field `SWREQ` writer - Software Transfer Requests"]
pub type SWREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl core::fmt::Debug for crate::generic::Reg<SWREQ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Software Transfer Requests"]
    #[inline(always)]
    #[must_use]
    pub fn swreq(&mut self) -> SWREQ_W<SWREQ_SPEC, 0> {
        SWREQ_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Channel Software Transfer Request Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWREQ_SPEC;
impl crate::RegisterSpec for SWREQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swreq::W`](W) writer structure"]
impl crate::Writable for SWREQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWREQ to value 0"]
impl crate::Resettable for SWREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
