///Register `OSCENCMD` writer
pub type W = crate::W<OSCENCMDrs>;
///Field `HFRCOEN` writer - HFRCO Enable
pub type HfrcoenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFRCODIS` writer - HFRCO Disable
pub type HfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXOEN` writer - HFXO Enable
pub type HfxoenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXODIS` writer - HFXO Disable
pub type HfxodisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUXHFRCOEN` writer - AUXHFRCO Enable
pub type AuxhfrcoenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUXHFRCODIS` writer - AUXHFRCO Disable
pub type AuxhfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFRCOEN` writer - LFRCO Enable
pub type LfrcoenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFRCODIS` writer - LFRCO Disable
pub type LfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFXOEN` writer - LFXO Enable
pub type LfxoenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFXODIS` writer - LFXO Disable
pub type LfxodisW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<OSCENCMDrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - HFRCO Enable
    #[inline(always)]
    #[must_use]
    pub fn hfrcoen(&mut self) -> HfrcoenW<OSCENCMDrs> {
        HfrcoenW::new(self, 0)
    }
    ///Bit 1 - HFRCO Disable
    #[inline(always)]
    #[must_use]
    pub fn hfrcodis(&mut self) -> HfrcodisW<OSCENCMDrs> {
        HfrcodisW::new(self, 1)
    }
    ///Bit 2 - HFXO Enable
    #[inline(always)]
    #[must_use]
    pub fn hfxoen(&mut self) -> HfxoenW<OSCENCMDrs> {
        HfxoenW::new(self, 2)
    }
    ///Bit 3 - HFXO Disable
    #[inline(always)]
    #[must_use]
    pub fn hfxodis(&mut self) -> HfxodisW<OSCENCMDrs> {
        HfxodisW::new(self, 3)
    }
    ///Bit 4 - AUXHFRCO Enable
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcoen(&mut self) -> AuxhfrcoenW<OSCENCMDrs> {
        AuxhfrcoenW::new(self, 4)
    }
    ///Bit 5 - AUXHFRCO Disable
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcodis(&mut self) -> AuxhfrcodisW<OSCENCMDrs> {
        AuxhfrcodisW::new(self, 5)
    }
    ///Bit 6 - LFRCO Enable
    #[inline(always)]
    #[must_use]
    pub fn lfrcoen(&mut self) -> LfrcoenW<OSCENCMDrs> {
        LfrcoenW::new(self, 6)
    }
    ///Bit 7 - LFRCO Disable
    #[inline(always)]
    #[must_use]
    pub fn lfrcodis(&mut self) -> LfrcodisW<OSCENCMDrs> {
        LfrcodisW::new(self, 7)
    }
    ///Bit 8 - LFXO Enable
    #[inline(always)]
    #[must_use]
    pub fn lfxoen(&mut self) -> LfxoenW<OSCENCMDrs> {
        LfxoenW::new(self, 8)
    }
    ///Bit 9 - LFXO Disable
    #[inline(always)]
    #[must_use]
    pub fn lfxodis(&mut self) -> LfxodisW<OSCENCMDrs> {
        LfxodisW::new(self, 9)
    }
}
///Oscillator Enable/Disable Command Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscencmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct OSCENCMDrs;
impl crate::RegisterSpec for OSCENCMDrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`oscencmd::W`](W) writer structure
impl crate::Writable for OSCENCMDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OSCENCMD to value 0
impl crate::Resettable for OSCENCMDrs {
    const RESET_VALUE: u32 = 0;
}
