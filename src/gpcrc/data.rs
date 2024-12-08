///Register `DATA` reader
pub type R = crate::R<DATArs>;
///Field `DATA` reader - CRC Data Register
pub type DataR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CRC Data Register
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA").field("data", &self.data()).finish()
    }
}
///CRC Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DATArs;
impl crate::RegisterSpec for DATArs {
    type Ux = u32;
}
///`read()` method returns [`data::R`](R) reader structure
impl crate::Readable for DATArs {}
///`reset()` method sets DATA to value 0
impl crate::Resettable for DATArs {
    const RESET_VALUE: u32 = 0;
}
