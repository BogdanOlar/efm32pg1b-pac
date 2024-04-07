#[doc = "Register `SCANFIFOCLEAR` writer"]
pub type W = crate::W<SCANFIFOCLEARrs>;
#[doc = "Field `SCANFIFOCLEAR` writer - Clear Scan FIFO Content"]
pub type ScanfifoclearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear Scan FIFO Content"]
    #[inline(always)]
    #[must_use]
    pub fn scanfifoclear(&mut self) -> ScanfifoclearW<SCANFIFOCLEARrs> {
        ScanfifoclearW::new(self, 0)
    }
}
#[doc = "Scan FIFO Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanfifoclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANFIFOCLEARrs;
impl crate::RegisterSpec for SCANFIFOCLEARrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scanfifoclear::W`](W) writer structure"]
impl crate::Writable for SCANFIFOCLEARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCANFIFOCLEAR to value 0"]
impl crate::Resettable for SCANFIFOCLEARrs {
    const RESET_VALUE: u32 = 0;
}
