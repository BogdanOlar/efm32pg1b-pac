#[doc = "Register `SCANDATAXP` reader"]
pub type R = crate::R<SCANDATAXPrs>;
#[doc = "Field `DATAP` reader - Scan Conversion Result Data Peek"]
pub type DatapR = crate::FieldReader<u16>;
#[doc = "Field `SCANINPUTIDPEEK` reader - Scan Conversion Data Source Peek"]
pub type ScaninputidpeekR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Scan Conversion Result Data Peek"]
    #[inline(always)]
    pub fn datap(&self) -> DatapR {
        DatapR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Scan Conversion Data Source Peek"]
    #[inline(always)]
    pub fn scaninputidpeek(&self) -> ScaninputidpeekR {
        ScaninputidpeekR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCANDATAXP")
            .field("datap", &self.datap())
            .field("scaninputidpeek", &self.scaninputidpeek())
            .finish()
    }
}
#[doc = "Scan Sequence Result Data + Data Source Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scandataxp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANDATAXPrs;
impl crate::RegisterSpec for SCANDATAXPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scandataxp::R`](R) reader structure"]
impl crate::Readable for SCANDATAXPrs {}
#[doc = "`reset()` method sets SCANDATAXP to value 0"]
impl crate::Resettable for SCANDATAXPrs {
    const RESET_VALUE: u32 = 0;
}
