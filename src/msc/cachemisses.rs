///Register `CACHEMISSES` reader
pub type R = crate::R<CACHEMISSESrs>;
///Field `CACHEMISSES` reader - Cache Misses Since Last Performance Counter Start Command
pub type CachemissesR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:19 - Cache Misses Since Last Performance Counter Start Command
    #[inline(always)]
    pub fn cachemisses(&self) -> CachemissesR {
        CachemissesR::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHEMISSES")
            .field("cachemisses", &self.cachemisses())
            .finish()
    }
}
///Cache Misses Performance Counter
///
///You can [`read`](crate::Reg::read) this register and get [`cachemisses::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CACHEMISSESrs;
impl crate::RegisterSpec for CACHEMISSESrs {
    type Ux = u32;
}
///`read()` method returns [`cachemisses::R`](R) reader structure
impl crate::Readable for CACHEMISSESrs {}
///`reset()` method sets CACHEMISSES to value 0
impl crate::Resettable for CACHEMISSESrs {
    const RESET_VALUE: u32 = 0;
}
