///Register `DCDCLPCTRL` reader
pub type R = crate::R<DCDCLPCTRLrs>;
///Register `DCDCLPCTRL` writer
pub type W = crate::W<DCDCLPCTRLrs>;
///Field `LPCMPHYSSEL` reader - LP Mode Hysteresis Selection
pub type LpcmphysselR = crate::FieldReader;
///Field `LPCMPHYSSEL` writer - LP Mode Hysteresis Selection
pub type LpcmphysselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LPVREFDUTYEN` reader - LP Mode Duty Cycling Enable
pub type LpvrefdutyenR = crate::BitReader;
///Field `LPVREFDUTYEN` writer - LP Mode Duty Cycling Enable
pub type LpvrefdutyenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPBLANK` reader - Reserved for internal use. Do not change.
pub type LpblankR = crate::FieldReader;
///Field `LPBLANK` writer - Reserved for internal use. Do not change.
pub type LpblankW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 12:15 - LP Mode Hysteresis Selection
    #[inline(always)]
    pub fn lpcmphyssel(&self) -> LpcmphysselR {
        LpcmphysselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 24 - LP Mode Duty Cycling Enable
    #[inline(always)]
    pub fn lpvrefdutyen(&self) -> LpvrefdutyenR {
        LpvrefdutyenR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - Reserved for internal use. Do not change.
    #[inline(always)]
    pub fn lpblank(&self) -> LpblankR {
        LpblankR::new(((self.bits >> 25) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDCLPCTRL")
            .field("lpcmphyssel", &self.lpcmphyssel())
            .field("lpvrefdutyen", &self.lpvrefdutyen())
            .field("lpblank", &self.lpblank())
            .finish()
    }
}
impl W {
    ///Bits 12:15 - LP Mode Hysteresis Selection
    #[inline(always)]
    #[must_use]
    pub fn lpcmphyssel(&mut self) -> LpcmphysselW<DCDCLPCTRLrs> {
        LpcmphysselW::new(self, 12)
    }
    ///Bit 24 - LP Mode Duty Cycling Enable
    #[inline(always)]
    #[must_use]
    pub fn lpvrefdutyen(&mut self) -> LpvrefdutyenW<DCDCLPCTRLrs> {
        LpvrefdutyenW::new(self, 24)
    }
    ///Bits 25:26 - Reserved for internal use. Do not change.
    #[inline(always)]
    #[must_use]
    pub fn lpblank(&mut self) -> LpblankW<DCDCLPCTRLrs> {
        LpblankW::new(self, 25)
    }
}
///DCDC Low Power Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdclpctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclpctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DCDCLPCTRLrs;
impl crate::RegisterSpec for DCDCLPCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`dcdclpctrl::R`](R) reader structure
impl crate::Readable for DCDCLPCTRLrs {}
///`write(|w| ..)` method takes [`dcdclpctrl::W`](W) writer structure
impl crate::Writable for DCDCLPCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCDCLPCTRL to value 0x7000
impl crate::Resettable for DCDCLPCTRLrs {
    const RESET_VALUE: u32 = 0x7000;
}
