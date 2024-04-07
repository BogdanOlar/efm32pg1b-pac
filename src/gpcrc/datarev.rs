#[doc = "Register `DATAREV` reader"]
pub type R = crate::R<DATAREVrs>;
#[doc = "Field `DATAREV` reader - Data Reverse Value"]
pub type DatarevR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data Reverse Value"]
    #[inline(always)]
    pub fn datarev(&self) -> DatarevR {
        DatarevR::new(self.bits)
    }
}
#[doc = "CRC Data Reverse Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datarev::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATAREVrs;
impl crate::RegisterSpec for DATAREVrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datarev::R`](R) reader structure"]
impl crate::Readable for DATAREVrs {}
#[doc = "`reset()` method sets DATAREV to value 0"]
impl crate::Resettable for DATAREVrs {
    const RESET_VALUE: u32 = 0;
}
