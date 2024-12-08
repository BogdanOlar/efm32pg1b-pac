///Register `DCDCLPVCTRL` reader
pub type R = crate::R<DCDCLPVCTRLrs>;
///Register `DCDCLPVCTRL` writer
pub type W = crate::W<DCDCLPVCTRLrs>;
///Field `LPATT` reader - Low Power Feedback Attenuation
pub type LpattR = crate::BitReader;
///Field `LPATT` writer - Low Power Feedback Attenuation
pub type LpattW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPVREF` reader - LP Mode Reference Selection for EM23 and EM4H
pub type LpvrefR = crate::FieldReader;
///Field `LPVREF` writer - LP Mode Reference Selection for EM23 and EM4H
pub type LpvrefW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Low Power Feedback Attenuation
    #[inline(always)]
    pub fn lpatt(&self) -> LpattR {
        LpattR::new((self.bits & 1) != 0)
    }
    ///Bits 1:8 - LP Mode Reference Selection for EM23 and EM4H
    #[inline(always)]
    pub fn lpvref(&self) -> LpvrefR {
        LpvrefR::new(((self.bits >> 1) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDCLPVCTRL")
            .field("lpatt", &self.lpatt())
            .field("lpvref", &self.lpvref())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low Power Feedback Attenuation
    #[inline(always)]
    #[must_use]
    pub fn lpatt(&mut self) -> LpattW<DCDCLPVCTRLrs> {
        LpattW::new(self, 0)
    }
    ///Bits 1:8 - LP Mode Reference Selection for EM23 and EM4H
    #[inline(always)]
    #[must_use]
    pub fn lpvref(&mut self) -> LpvrefW<DCDCLPVCTRLrs> {
        LpvrefW::new(self, 1)
    }
}
///DCDC Low Power Voltage Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdclpvctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclpvctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DCDCLPVCTRLrs;
impl crate::RegisterSpec for DCDCLPVCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`dcdclpvctrl::R`](R) reader structure
impl crate::Readable for DCDCLPVCTRLrs {}
///`write(|w| ..)` method takes [`dcdclpvctrl::W`](W) writer structure
impl crate::Writable for DCDCLPVCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCDCLPVCTRL to value 0x0168
impl crate::Resettable for DCDCLPVCTRLrs {
    const RESET_VALUE: u32 = 0x0168;
}
