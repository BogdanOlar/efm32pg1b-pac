///Register `SCANFIFOCLEAR` writer
pub type W = crate::W<SCANFIFOCLEARrs>;
///Field `SCANFIFOCLEAR` writer - Clear Scan FIFO Content
pub type ScanfifoclearW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SCANFIFOCLEARrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear Scan FIFO Content
    #[inline(always)]
    #[must_use]
    pub fn scanfifoclear(&mut self) -> ScanfifoclearW<SCANFIFOCLEARrs> {
        ScanfifoclearW::new(self, 0)
    }
}
///Scan FIFO Clear Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanfifoclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SCANFIFOCLEARrs;
impl crate::RegisterSpec for SCANFIFOCLEARrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`scanfifoclear::W`](W) writer structure
impl crate::Writable for SCANFIFOCLEARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCANFIFOCLEAR to value 0
impl crate::Resettable for SCANFIFOCLEARrs {
    const RESET_VALUE: u32 = 0;
}
