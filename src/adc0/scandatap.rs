///Register `SCANDATAP` reader
pub type R = crate::R<SCANDATAPrs>;
///Field `DATAP` reader - Scan Conversion Result Data Peek
pub type DatapR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Scan Conversion Result Data Peek
    #[inline(always)]
    pub fn datap(&self) -> DatapR {
        DatapR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCANDATAP")
            .field("datap", &self.datap())
            .finish()
    }
}
///Scan Sequence Result Data Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`scandatap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SCANDATAPrs;
impl crate::RegisterSpec for SCANDATAPrs {
    type Ux = u32;
}
///`read()` method returns [`scandatap::R`](R) reader structure
impl crate::Readable for SCANDATAPrs {}
///`reset()` method sets SCANDATAP to value 0
impl crate::Resettable for SCANDATAPrs {
    const RESET_VALUE: u32 = 0;
}
