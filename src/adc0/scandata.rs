#[doc = "Register `SCANDATA` reader"]
pub type R = crate::R<SCANDATArs>;
#[doc = "Field `DATA` reader - Scan Conversion Result Data"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Scan Conversion Result Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for crate::generic::Reg<SCANDATArs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
#[doc = "Scan Conversion Result Data\n\nYou can [`read`](crate::Reg::read) this register and get [`scandata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct SCANDATArs;
impl crate::RegisterSpec for SCANDATArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scandata::R`](R) reader structure"]
impl crate::Readable for SCANDATArs {}
#[doc = "`reset()` method sets SCANDATA to value 0"]
impl crate::Resettable for SCANDATArs {
    const RESET_VALUE: u32 = 0;
}
