#[doc = "Register `CHEN` reader"]
pub type R = crate::R<CHEN_SPEC>;
#[doc = "Register `CHEN` writer"]
pub type W = crate::W<CHEN_SPEC>;
#[doc = "Field `CHEN` reader - Channel Enables"]
pub type CHEN_R = crate::FieldReader;
#[doc = "Field `CHEN` writer - Channel Enables"]
pub type CHEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Channel Enables"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHEN")
            .field("chen", &format_args!("{}", self.chen().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CHEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel Enables"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<CHEN_SPEC, 0> {
        CHEN_W::new(self)
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
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHEN_SPEC;
impl crate::RegisterSpec for CHEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chen::R`](R) reader structure"]
impl crate::Readable for CHEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chen::W`](W) writer structure"]
impl crate::Writable for CHEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHEN to value 0"]
impl crate::Resettable for CHEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
