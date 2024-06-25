#[doc = "Register `SINGLEDATA` reader"]
pub type R = crate::R<SINGLEDATArs>;
#[doc = "Field `DATA` reader - Single Conversion Result Data"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Single Conversion Result Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for crate::generic::Reg<SINGLEDATArs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
#[doc = "Single Conversion Result Data\n\nYou can [`read`](crate::Reg::read) this register and get [`singledata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct SINGLEDATArs;
impl crate::RegisterSpec for SINGLEDATArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singledata::R`](R) reader structure"]
impl crate::Readable for SINGLEDATArs {}
#[doc = "`reset()` method sets SINGLEDATA to value 0"]
impl crate::Resettable for SINGLEDATArs {
    const RESET_VALUE: u32 = 0;
}
