#[doc = "Register `REQCLEAR` writer"]
pub type W = crate::W<REQCLEARrs>;
#[doc = "Field `REQCLEAR` writer - DMA Request Clear"]
pub type REQCLEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - DMA Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn reqclear(&mut self) -> REQCLEAR_W<REQCLEARrs> {
        REQCLEAR_W::new(self, 0)
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
#[doc = "DMA Channel Request Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REQCLEARrs;
impl crate::RegisterSpec for REQCLEARrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`reqclear::W`](W) writer structure"]
impl crate::Writable for REQCLEARrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REQCLEAR to value 0"]
impl crate::Resettable for REQCLEARrs {
    const RESET_VALUE: u32 = 0;
}
