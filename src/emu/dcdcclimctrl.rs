#[doc = "Register `DCDCCLIMCTRL` reader"]
pub type R = crate::R<DCDCCLIMCTRLrs>;
#[doc = "Register `DCDCCLIMCTRL` writer"]
pub type W = crate::W<DCDCCLIMCTRLrs>;
#[doc = "Field `CLIMBLANKDLY` reader - Reserved for internal use. Do not change."]
pub type ClimblankdlyR = crate::FieldReader;
#[doc = "Field `CLIMBLANKDLY` writer - Reserved for internal use. Do not change."]
pub type ClimblankdlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BYPLIMEN` reader - Bypass Current Limit Enable"]
pub type ByplimenR = crate::BitReader;
#[doc = "Field `BYPLIMEN` writer - Bypass Current Limit Enable"]
pub type ByplimenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn climblankdly(&self) -> ClimblankdlyR {
        ClimblankdlyR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 13 - Bypass Current Limit Enable"]
    #[inline(always)]
    pub fn byplimen(&self) -> ByplimenR {
        ByplimenR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    #[must_use]
    pub fn climblankdly(&mut self) -> ClimblankdlyW<DCDCCLIMCTRLrs> {
        ClimblankdlyW::new(self, 8)
    }
    #[doc = "Bit 13 - Bypass Current Limit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn byplimen(&mut self) -> ByplimenW<DCDCCLIMCTRLrs> {
        ByplimenW::new(self, 13)
    }
}
#[doc = "DCDC Power Train PFET Current Limiter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdcclimctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdcclimctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCCLIMCTRLrs;
impl crate::RegisterSpec for DCDCCLIMCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcclimctrl::R`](R) reader structure"]
impl crate::Readable for DCDCCLIMCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`dcdcclimctrl::W`](W) writer structure"]
impl crate::Writable for DCDCCLIMCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDCCLIMCTRL to value 0x2100"]
impl crate::Resettable for DCDCCLIMCTRLrs {
    const RESET_VALUE: u32 = 0x2100;
}
