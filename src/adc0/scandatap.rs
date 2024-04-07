#[doc = "Register `SCANDATAP` reader"]
pub type R = crate::R<SCANDATAPrs>;
#[doc = "Field `DATAP` reader - Scan Conversion Result Data Peek"]
pub type DatapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Scan Conversion Result Data Peek"]
    #[inline(always)]
    pub fn datap(&self) -> DatapR {
        DatapR::new(self.bits)
    }
}
#[doc = "Scan Sequence Result Data Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scandatap::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANDATAPrs;
impl crate::RegisterSpec for SCANDATAPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scandatap::R`](R) reader structure"]
impl crate::Readable for SCANDATAPrs {}
#[doc = "`reset()` method sets SCANDATAP to value 0"]
impl crate::Resettable for SCANDATAPrs {
    const RESET_VALUE: u32 = 0;
}
