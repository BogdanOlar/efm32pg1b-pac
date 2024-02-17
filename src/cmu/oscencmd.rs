#[doc = "Register `OSCENCMD` writer"]
pub type W = crate::W<OSCENCMDrs>;
#[doc = "Field `HFRCOEN` writer - HFRCO Enable"]
pub type HFRCOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRCODIS` writer - HFRCO Disable"]
pub type HFRCODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOEN` writer - HFXO Enable"]
pub type HFXOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXODIS` writer - HFXO Disable"]
pub type HFXODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCOEN` writer - AUXHFRCO Enable"]
pub type AUXHFRCOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCODIS` writer - AUXHFRCO Disable"]
pub type AUXHFRCODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCOEN` writer - LFRCO Enable"]
pub type LFRCOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCODIS` writer - LFRCO Disable"]
pub type LFRCODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXOEN` writer - LFXO Enable"]
pub type LFXOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXODIS` writer - LFXO Disable"]
pub type LFXODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - HFRCO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcoen(&mut self) -> HFRCOEN_W<OSCENCMDrs> {
        HFRCOEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - HFRCO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W<OSCENCMDrs> {
        HFRCODIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - HFXO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoen(&mut self) -> HFXOEN_W<OSCENCMDrs> {
        HFXOEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - HFXO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxodis(&mut self) -> HFXODIS_W<OSCENCMDrs> {
        HFXODIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcoen(&mut self) -> AUXHFRCOEN_W<OSCENCMDrs> {
        AUXHFRCOEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - AUXHFRCO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcodis(&mut self) -> AUXHFRCODIS_W<OSCENCMDrs> {
        AUXHFRCODIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - LFRCO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcoen(&mut self) -> LFRCOEN_W<OSCENCMDrs> {
        LFRCOEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - LFRCO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcodis(&mut self) -> LFRCODIS_W<OSCENCMDrs> {
        LFRCODIS_W::new(self, 7)
    }
    #[doc = "Bit 8 - LFXO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfxoen(&mut self) -> LFXOEN_W<OSCENCMDrs> {
        LFXOEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - LFXO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lfxodis(&mut self) -> LFXODIS_W<OSCENCMDrs> {
        LFXODIS_W::new(self, 9)
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
#[doc = "Oscillator Enable/Disable Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscencmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCENCMDrs;
impl crate::RegisterSpec for OSCENCMDrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`oscencmd::W`](W) writer structure"]
impl crate::Writable for OSCENCMDrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCENCMD to value 0"]
impl crate::Resettable for OSCENCMDrs {
    const RESET_VALUE: u32 = 0;
}
