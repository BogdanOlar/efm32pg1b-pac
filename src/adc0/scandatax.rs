///Register `SCANDATAX` reader
pub type R = crate::R<SCANDATAXrs>;
///Field `DATA` reader - Scan Conversion Result Data
pub type DataR = crate::FieldReader<u16>;
///Field `SCANINPUTID` reader - Scan Conversion Input ID
pub type ScaninputidR = crate::FieldReader;
impl R {
    ///Bits 0:15 - Scan Conversion Result Data
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:20 - Scan Conversion Input ID
    #[inline(always)]
    pub fn scaninputid(&self) -> ScaninputidR {
        ScaninputidR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for crate::generic::Reg<SCANDATAXrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
///Scan Sequence Result Data + Data Source Register
///
///You can [`read`](crate::Reg::read) this register and get [`scandatax::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct SCANDATAXrs;
impl crate::RegisterSpec for SCANDATAXrs {
    type Ux = u32;
}
///`read()` method returns [`scandatax::R`](R) reader structure
impl crate::Readable for SCANDATAXrs {}
///`reset()` method sets SCANDATAX to value 0
impl crate::Resettable for SCANDATAXrs {
    const RESET_VALUE: u32 = 0;
}
