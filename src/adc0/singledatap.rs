///Register `SINGLEDATAP` reader
pub type R = crate::R<SINGLEDATAPrs>;
///Field `DATAP` reader - Single Conversion Result Data Peek
pub type DatapR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Single Conversion Result Data Peek
    #[inline(always)]
    pub fn datap(&self) -> DatapR {
        DatapR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SINGLEDATAP")
            .field("datap", &self.datap())
            .finish()
    }
}
///Single Conversion Result Data Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`singledatap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SINGLEDATAPrs;
impl crate::RegisterSpec for SINGLEDATAPrs {
    type Ux = u32;
}
///`read()` method returns [`singledatap::R`](R) reader structure
impl crate::Readable for SINGLEDATAPrs {}
///`reset()` method sets SINGLEDATAP to value 0
impl crate::Resettable for SINGLEDATAPrs {
    const RESET_VALUE: u32 = 0;
}
