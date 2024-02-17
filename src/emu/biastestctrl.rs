#[doc = "Register `BIASTESTCTRL` reader"]
pub type R = crate::R<BIASTESTCTRLrs>;
#[doc = "Register `BIASTESTCTRL` writer"]
pub type W = crate::W<BIASTESTCTRLrs>;
#[doc = "Field `BIAS_RIP_RESET` reader - Reset Bias Ripple Counter"]
pub type BIAS_RIP_RESET_R = crate::BitReader;
#[doc = "Field `BIAS_RIP_RESET` writer - Reset Bias Ripple Counter"]
pub type BIAS_RIP_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Reset Bias Ripple Counter"]
    #[inline(always)]
    pub fn bias_rip_reset(&self) -> BIAS_RIP_RESET_R {
        BIAS_RIP_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Reset Bias Ripple Counter"]
    #[inline(always)]
    #[must_use]
    pub fn bias_rip_reset(&mut self) -> BIAS_RIP_RESET_W<BIASTESTCTRLrs> {
        BIAS_RIP_RESET_W::new(self, 3)
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
pub struct BIASTESTCTRLrs;
impl crate::RegisterSpec for BIASTESTCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`biastestctrl::R`](R) reader structure"]
impl crate::Readable for BIASTESTCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`biastestctrl::W`](W) writer structure"]
impl crate::Writable for BIASTESTCTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BIASTESTCTRL to value 0"]
impl crate::Resettable for BIASTESTCTRLrs {
    const RESET_VALUE: u32 = 0;
}
