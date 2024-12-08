///Register `CMD` writer
pub type W = crate::W<CMDrs>;
///Field `SINGLESTART` writer - Single Channel Conversion Start
pub type SinglestartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SINGLESTOP` writer - Single Channel Conversion Stop
pub type SinglestopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCANSTART` writer - Scan Sequence Start
pub type ScanstartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCANSTOP` writer - Scan Sequence Stop
pub type ScanstopW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CMDrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Single Channel Conversion Start
    #[inline(always)]
    #[must_use]
    pub fn singlestart(&mut self) -> SinglestartW<CMDrs> {
        SinglestartW::new(self, 0)
    }
    ///Bit 1 - Single Channel Conversion Stop
    #[inline(always)]
    #[must_use]
    pub fn singlestop(&mut self) -> SinglestopW<CMDrs> {
        SinglestopW::new(self, 1)
    }
    ///Bit 2 - Scan Sequence Start
    #[inline(always)]
    #[must_use]
    pub fn scanstart(&mut self) -> ScanstartW<CMDrs> {
        ScanstartW::new(self, 2)
    }
    ///Bit 3 - Scan Sequence Stop
    #[inline(always)]
    #[must_use]
    pub fn scanstop(&mut self) -> ScanstopW<CMDrs> {
        ScanstopW::new(self, 3)
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
