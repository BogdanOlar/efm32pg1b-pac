///Register `REQCLEAR` writer
pub type W = crate::W<REQCLEARrs>;
///Field `REQCLEAR` writer - DMA Request Clear
pub type ReqclearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<REQCLEARrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - DMA Request Clear
    #[inline(always)]
    #[must_use]
    pub fn reqclear(&mut self) -> ReqclearW<REQCLEARrs> {
        ReqclearW::new(self, 0)
    }
}
///DMA Channel Request Clear Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct REQCLEARrs;
impl crate::RegisterSpec for REQCLEARrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`reqclear::W`](W) writer structure
impl crate::Writable for REQCLEARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REQCLEAR to value 0
impl crate::Resettable for REQCLEARrs {
    const RESET_VALUE: u32 = 0;
}
