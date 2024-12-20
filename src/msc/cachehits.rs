///Register `CACHEHITS` reader
pub type R = crate::R<CACHEHITSrs>;
///Field `CACHEHITS` reader - Cache Hits Since Last Performance Counter Start Command
pub type CachehitsR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:19 - Cache Hits Since Last Performance Counter Start Command
    #[inline(always)]
    pub fn cachehits(&self) -> CachehitsR {
        CachehitsR::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHEHITS")
            .field("cachehits", &self.cachehits())
            .finish()
    }
}
///Cache Hits Performance Counter
///
///You can [`read`](crate::Reg::read) this register and get [`cachehits::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CACHEHITSrs;
impl crate::RegisterSpec for CACHEHITSrs {
    type Ux = u32;
}
///`read()` method returns [`cachehits::R`](R) reader structure
impl crate::Readable for CACHEHITSrs {}
///`reset()` method sets CACHEHITS to value 0
impl crate::Resettable for CACHEHITSrs {
    const RESET_VALUE: u32 = 0;
}
