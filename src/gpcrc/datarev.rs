///Register `DATAREV` reader
pub type R = crate::R<DATAREVrs>;
///Field `DATAREV` reader - Data Reverse Value
pub type DatarevR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Data Reverse Value
    #[inline(always)]
    pub fn datarev(&self) -> DatarevR {
        DatarevR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATAREV")
            .field("datarev", &self.datarev())
            .finish()
    }
}
///CRC Data Reverse Register
///
///You can [`read`](crate::Reg::read) this register and get [`datarev::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DATAREVrs;
impl crate::RegisterSpec for DATAREVrs {
    type Ux = u32;
}
///`read()` method returns [`datarev::R`](R) reader structure
impl crate::Readable for DATAREVrs {}
///`reset()` method sets DATAREV to value 0
impl crate::Resettable for DATAREVrs {
    const RESET_VALUE: u32 = 0;
}
