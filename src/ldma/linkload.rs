#[doc = "Register `LINKLOAD` writer"]
pub type W = crate::W<LINKLOAD_SPEC>;
#[doc = "Field `LINKLOAD` writer - DMA Link Loads"]
pub type LINKLOAD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl core::fmt::Debug for crate::generic::Reg<LINKLOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Link Loads"]
    #[inline(always)]
    #[must_use]
    pub fn linkload(&mut self) -> LINKLOAD_W<LINKLOAD_SPEC, 0> {
        LINKLOAD_W::new(self)
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
#[doc = "DMA Channel Link Load Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linkload::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINKLOAD_SPEC;
impl crate::RegisterSpec for LINKLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`linkload::W`](W) writer structure"]
impl crate::Writable for LINKLOAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINKLOAD to value 0"]
impl crate::Resettable for LINKLOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
