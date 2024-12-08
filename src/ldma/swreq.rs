///Register `SWREQ` writer
pub type W = crate::W<SWREQrs>;
///Field `SWREQ` writer - Software Transfer Requests
pub type SwreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<SWREQrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Software Transfer Requests
    #[inline(always)]
    #[must_use]
    pub fn swreq(&mut self) -> SwreqW<SWREQrs> {
        SwreqW::new(self, 0)
    }
}
///DMA Channel Software Transfer Request Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SWREQrs;
impl crate::RegisterSpec for SWREQrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`swreq::W`](W) writer structure
impl crate::Writable for SWREQrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SWREQ to value 0
impl crate::Resettable for SWREQrs {
    const RESET_VALUE: u32 = 0;
}
