///Register `CNT` reader
pub type R = crate::R<CNTrs>;
///Field `CNT` reader - Counter Value
pub type CntR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Counter Value
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT").field("cnt", &self.cnt()).finish()
    }
}
///Counter Value
///
///You can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CNTrs;
impl crate::RegisterSpec for CNTrs {
    type Ux = u32;
}
///`read()` method returns [`cnt::R`](R) reader structure
impl crate::Readable for CNTrs {}
///`reset()` method sets CNT to value 0
impl crate::Resettable for CNTrs {
    const RESET_VALUE: u32 = 0;
}
