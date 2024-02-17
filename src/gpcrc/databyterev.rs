#[doc = "Register `DATABYTEREV` reader"]
pub type R = crate::R<DATABYTEREVrs>;
#[doc = "Field `DATABYTEREV` reader - Data Byte Reverse Value"]
pub type DATABYTEREV_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data Byte Reverse Value"]
    #[inline(always)]
    pub fn databyterev(&self) -> DATABYTEREV_R {
        DATABYTEREV_R::new(self.bits)
    }
}
#[doc = "CRC Data Byte Reverse Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`databyterev::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATABYTEREVrs;
impl crate::RegisterSpec for DATABYTEREVrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`databyterev::R`](R) reader structure"]
impl crate::Readable for DATABYTEREVrs {}
#[doc = "`reset()` method sets DATABYTEREV to value 0"]
impl crate::Resettable for DATABYTEREVrs {
    const RESET_VALUE: u32 = 0;
}
