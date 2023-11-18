#[doc = "Register `BIASTESTCTRL` reader"]
pub type R = crate::R<BIASTESTCTRL_SPEC>;
#[doc = "Register `BIASTESTCTRL` writer"]
pub type W = crate::W<BIASTESTCTRL_SPEC>;
#[doc = "Field `BIAS_RIP_RESET` reader - Reset Bias Ripple Counter"]
pub type BIAS_RIP_RESET_R = crate::BitReader;
#[doc = "Field `BIAS_RIP_RESET` writer - Reset Bias Ripple Counter"]
pub type BIAS_RIP_RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - Reset Bias Ripple Counter"]
    #[inline(always)]
    pub fn bias_rip_reset(&self) -> BIAS_RIP_RESET_R {
        BIAS_RIP_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BIASTESTCTRL")
            .field(
                "bias_rip_reset",
                &format_args!("{}", self.bias_rip_reset().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BIASTESTCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - Reset Bias Ripple Counter"]
    #[inline(always)]
    #[must_use]
    pub fn bias_rip_reset(&mut self) -> BIAS_RIP_RESET_W<BIASTESTCTRL_SPEC, 3> {
        BIAS_RIP_RESET_W::new(self)
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
#[doc = "Test Control Register for Regulator and BIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`biastestctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`biastestctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIASTESTCTRL_SPEC;
impl crate::RegisterSpec for BIASTESTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`biastestctrl::R`](R) reader structure"]
impl crate::Readable for BIASTESTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`biastestctrl::W`](W) writer structure"]
impl crate::Writable for BIASTESTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIASTESTCTRL to value 0"]
impl crate::Resettable for BIASTESTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
