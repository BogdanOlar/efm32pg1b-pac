///Register `CMD` writer
pub type W = crate::W<CMDrs>;
///Field `CALSTART` writer - Calibration Start
pub type CalstartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALSTOP` writer - Calibration Stop
pub type CalstopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXOPEAKDETSTART` writer - HFXO Peak Detection Start
pub type HfxopeakdetstartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXOSHUNTOPTSTART` writer - HFXO Shunt Current Optimization Start
pub type HfxoshuntoptstartW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CMDrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Calibration Start
    #[inline(always)]
    #[must_use]
    pub fn calstart(&mut self) -> CalstartW<CMDrs> {
        CalstartW::new(self, 0)
    }
    ///Bit 1 - Calibration Stop
    #[inline(always)]
    #[must_use]
    pub fn calstop(&mut self) -> CalstopW<CMDrs> {
        CalstopW::new(self, 1)
    }
    ///Bit 4 - HFXO Peak Detection Start
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdetstart(&mut self) -> HfxopeakdetstartW<CMDrs> {
        HfxopeakdetstartW::new(self, 4)
    }
    ///Bit 5 - HFXO Shunt Current Optimization Start
    #[inline(always)]
    #[must_use]
    pub fn hfxoshuntoptstart(&mut self) -> HfxoshuntoptstartW<CMDrs> {
        HfxoshuntoptstartW::new(self, 5)
    }
}
///Command Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cmd::W`](W) writer structure
impl crate::Writable for CMDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMD to value 0
impl crate::Resettable for CMDrs {
    const RESET_VALUE: u32 = 0;
}
