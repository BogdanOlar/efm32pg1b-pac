#[doc = "Register `OSCENCMD` writer"]
pub type W = crate::W<OSCENCMDrs>;
#[doc = "Field `HFRCOEN` writer - HFRCO Enable"]
pub type HfrcoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRCODIS` writer - HFRCO Disable"]
pub type HfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOEN` writer - HFXO Enable"]
pub type HfxoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXODIS` writer - HFXO Disable"]
pub type HfxodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCOEN` writer - AUXHFRCO Enable"]
pub type AuxhfrcoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCODIS` writer - AUXHFRCO Disable"]
pub type AuxhfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCOEN` writer - LFRCO Enable"]
pub type LfrcoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCODIS` writer - LFRCO Disable"]
pub type LfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXOEN` writer - LFXO Enable"]
pub type LfxoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXODIS` writer - LFXO Disable"]
pub type LfxodisW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<OSCENCMDrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - HFRCO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcoen(&mut self) -> HfrcoenW<OSCENCMDrs> {
        HfrcoenW::new(self, 0)
    }
    #[doc = "Bit 1 - HFRCO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcodis(&mut self) -> HfrcodisW<OSCENCMDrs> {
        HfrcodisW::new(self, 1)
    }
    #[doc = "Bit 2 - HFXO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoen(&mut self) -> HfxoenW<OSCENCMDrs> {
        HfxoenW::new(self, 2)
    }
    #[doc = "Bit 3 - HFXO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxodis(&mut self) -> HfxodisW<OSCENCMDrs> {
        HfxodisW::new(self, 3)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcoen(&mut self) -> AuxhfrcoenW<OSCENCMDrs> {
        AuxhfrcoenW::new(self, 4)
    }
    #[doc = "Bit 5 - AUXHFRCO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcodis(&mut self) -> AuxhfrcodisW<OSCENCMDrs> {
        AuxhfrcodisW::new(self, 5)
    }
    #[doc = "Bit 6 - LFRCO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcoen(&mut self) -> LfrcoenW<OSCENCMDrs> {
        LfrcoenW::new(self, 6)
    }
    #[doc = "Bit 7 - LFRCO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcodis(&mut self) -> LfrcodisW<OSCENCMDrs> {
        LfrcodisW::new(self, 7)
    }
    #[doc = "Bit 8 - LFXO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfxoen(&mut self) -> LfxoenW<OSCENCMDrs> {
        LfxoenW::new(self, 8)
    }
    #[doc = "Bit 9 - LFXO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lfxodis(&mut self) -> LfxodisW<OSCENCMDrs> {
        LfxodisW::new(self, 9)
    }
}
#[doc = "Oscillator Enable/Disable Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscencmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCENCMDrs;
impl crate::RegisterSpec for OSCENCMDrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`oscencmd::W`](W) writer structure"]
impl crate::Writable for OSCENCMDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCENCMD to value 0"]
impl crate::Resettable for OSCENCMDrs {
    const RESET_VALUE: u32 = 0;
}
