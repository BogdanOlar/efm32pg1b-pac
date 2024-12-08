///Register `DCDCCLIMCTRL` reader
pub type R = crate::R<DCDCCLIMCTRLrs>;
///Register `DCDCCLIMCTRL` writer
pub type W = crate::W<DCDCCLIMCTRLrs>;
///Field `CLIMBLANKDLY` reader - Reserved for internal use. Do not change.
pub type ClimblankdlyR = crate::FieldReader;
///Field `CLIMBLANKDLY` writer - Reserved for internal use. Do not change.
pub type ClimblankdlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BYPLIMEN` reader - Bypass Current Limit Enable
pub type ByplimenR = crate::BitReader;
///Field `BYPLIMEN` writer - Bypass Current Limit Enable
pub type ByplimenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 8:9 - Reserved for internal use. Do not change.
    #[inline(always)]
    pub fn climblankdly(&self) -> ClimblankdlyR {
        ClimblankdlyR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 13 - Bypass Current Limit Enable
    #[inline(always)]
    pub fn byplimen(&self) -> ByplimenR {
        ByplimenR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDCCLIMCTRL")
            .field("climblankdly", &self.climblankdly())
            .field("byplimen", &self.byplimen())
            .finish()
    }
}
impl W {
    ///Bits 8:9 - Reserved for internal use. Do not change.
    #[inline(always)]
    #[must_use]
    pub fn climblankdly(&mut self) -> ClimblankdlyW<DCDCCLIMCTRLrs> {
        ClimblankdlyW::new(self, 8)
    }
    ///Bit 13 - Bypass Current Limit Enable
    #[inline(always)]
    #[must_use]
    pub fn byplimen(&mut self) -> ByplimenW<DCDCCLIMCTRLrs> {
        ByplimenW::new(self, 13)
    }
}
///DCDC Power Train PFET Current Limiter Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdcclimctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcclimctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DCDCCLIMCTRLrs;
impl crate::RegisterSpec for DCDCCLIMCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`dcdcclimctrl::R`](R) reader structure
impl crate::Readable for DCDCCLIMCTRLrs {}
///`write(|w| ..)` method takes [`dcdcclimctrl::W`](W) writer structure
impl crate::Writable for DCDCCLIMCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCDCCLIMCTRL to value 0x2100
impl crate::Resettable for DCDCCLIMCTRLrs {
    const RESET_VALUE: u32 = 0x2100;
}
