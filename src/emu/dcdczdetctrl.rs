///Register `DCDCZDETCTRL` reader
pub type R = crate::R<DCDCZDETCTRLrs>;
///Register `DCDCZDETCTRL` writer
pub type W = crate::W<DCDCZDETCTRLrs>;
///Field `ZDETILIMSEL` reader - Reverse Current Limit Level Selection for Zero Detector
pub type ZdetilimselR = crate::FieldReader;
///Field `ZDETILIMSEL` writer - Reverse Current Limit Level Selection for Zero Detector
pub type ZdetilimselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ZDETBLANKDLY` reader - Reserved for internal use. Do not change.
pub type ZdetblankdlyR = crate::FieldReader;
///Field `ZDETBLANKDLY` writer - Reserved for internal use. Do not change.
pub type ZdetblankdlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 4:6 - Reverse Current Limit Level Selection for Zero Detector
    #[inline(always)]
    pub fn zdetilimsel(&self) -> ZdetilimselR {
        ZdetilimselR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:9 - Reserved for internal use. Do not change.
    #[inline(always)]
    pub fn zdetblankdly(&self) -> ZdetblankdlyR {
        ZdetblankdlyR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDCZDETCTRL")
            .field("zdetilimsel", &self.zdetilimsel())
            .field("zdetblankdly", &self.zdetblankdly())
            .finish()
    }
}
impl W {
    ///Bits 4:6 - Reverse Current Limit Level Selection for Zero Detector
    #[inline(always)]
    #[must_use]
    pub fn zdetilimsel(&mut self) -> ZdetilimselW<DCDCZDETCTRLrs> {
        ZdetilimselW::new(self, 4)
    }
    ///Bits 8:9 - Reserved for internal use. Do not change.
    #[inline(always)]
    #[must_use]
    pub fn zdetblankdly(&mut self) -> ZdetblankdlyW<DCDCZDETCTRLrs> {
        ZdetblankdlyW::new(self, 8)
    }
}
///DCDC Power Train NFET Zero Current Detector Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdczdetctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdczdetctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DCDCZDETCTRLrs;
impl crate::RegisterSpec for DCDCZDETCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`dcdczdetctrl::R`](R) reader structure
impl crate::Readable for DCDCZDETCTRLrs {}
///`write(|w| ..)` method takes [`dcdczdetctrl::W`](W) writer structure
impl crate::Writable for DCDCZDETCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCDCZDETCTRL to value 0x0130
impl crate::Resettable for DCDCZDETCTRLrs {
    const RESET_VALUE: u32 = 0x0130;
}
