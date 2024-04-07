#[doc = "Register `DATA` reader"]
pub type R = crate::R<DATArs>;
#[doc = "Field `DATA` reader - CRC Data Register"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Data Register"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "CRC Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATArs;
impl crate::RegisterSpec for DATArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DATArs {}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DATArs {
    const RESET_VALUE: u32 = 0;
}
