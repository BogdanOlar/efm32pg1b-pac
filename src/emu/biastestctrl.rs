#[doc = "Register `BIASTESTCTRL` reader"]
pub type R = crate::R<BIASTESTCTRLrs>;
#[doc = "Register `BIASTESTCTRL` writer"]
pub type W = crate::W<BIASTESTCTRLrs>;
#[doc = "Field `BIAS_RIP_RESET` reader - Reset Bias Ripple Counter"]
pub type BiasRipResetR = crate::BitReader;
#[doc = "Field `BIAS_RIP_RESET` writer - Reset Bias Ripple Counter"]
pub type BiasRipResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Reset Bias Ripple Counter"]
    #[inline(always)]
    pub fn bias_rip_reset(&self) -> BiasRipResetR {
        BiasRipResetR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BIASTESTCTRL")
            .field("bias_rip_reset", &self.bias_rip_reset())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Reset Bias Ripple Counter"]
    #[inline(always)]
    #[must_use]
    pub fn bias_rip_reset(&mut self) -> BiasRipResetW<BIASTESTCTRLrs> {
        BiasRipResetW::new(self, 3)
    }
}
#[doc = "Test Control Register for Regulator and BIAS\n\nYou can [`read`](crate::Reg::read) this register and get [`biastestctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biastestctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIASTESTCTRLrs;
impl crate::RegisterSpec for BIASTESTCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`biastestctrl::R`](R) reader structure"]
impl crate::Readable for BIASTESTCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`biastestctrl::W`](W) writer structure"]
impl crate::Writable for BIASTESTCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BIASTESTCTRL to value 0"]
impl crate::Resettable for BIASTESTCTRLrs {
    const RESET_VALUE: u32 = 0;
}
