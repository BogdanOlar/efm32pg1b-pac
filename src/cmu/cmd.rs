#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMDrs>;
#[doc = "Field `CALSTART` writer - Calibration Start"]
pub type CALSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSTOP` writer - Calibration Stop"]
pub type CALSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOPEAKDETSTART` writer - HFXO Peak Detection Start"]
pub type HFXOPEAKDETSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOSHUNTOPTSTART` writer - HFXO Shunt Current Optimization Start"]
pub type HFXOSHUNTOPTSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Calibration Start"]
    #[inline(always)]
    #[must_use]
    pub fn calstart(&mut self) -> CALSTART_W<CMDrs> {
        CALSTART_W::new(self, 0)
    }
    #[doc = "Bit 1 - Calibration Stop"]
    #[inline(always)]
    #[must_use]
    pub fn calstop(&mut self) -> CALSTOP_W<CMDrs> {
        CALSTOP_W::new(self, 1)
    }
    #[doc = "Bit 4 - HFXO Peak Detection Start"]
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdetstart(&mut self) -> HFXOPEAKDETSTART_W<CMDrs> {
        HFXOPEAKDETSTART_W::new(self, 4)
    }
    #[doc = "Bit 5 - HFXO Shunt Current Optimization Start"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoshuntoptstart(&mut self) -> HFXOSHUNTOPTSTART_W<CMDrs> {
        HFXOSHUNTOPTSTART_W::new(self, 5)
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
#[doc = "Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMDrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMDrs {
    const RESET_VALUE: u32 = 0;
}
