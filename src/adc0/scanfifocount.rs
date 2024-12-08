///Register `SCANFIFOCOUNT` reader
pub type R = crate::R<SCANFIFOCOUNTrs>;
///Field `SCANDC` reader - Scan Data Count
pub type ScandcR = crate::FieldReader;
impl R {
    ///Bits 0:2 - Scan Data Count
    #[inline(always)]
    pub fn scandc(&self) -> ScandcR {
        ScandcR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCANFIFOCOUNT")
            .field("scandc", &self.scandc())
            .finish()
    }
}
///Scan FIFO Count Register
///
///You can [`read`](crate::Reg::read) this register and get [`scanfifocount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SCANFIFOCOUNTrs;
impl crate::RegisterSpec for SCANFIFOCOUNTrs {
    type Ux = u32;
}
///`read()` method returns [`scanfifocount::R`](R) reader structure
impl crate::Readable for SCANFIFOCOUNTrs {}
///`reset()` method sets SCANFIFOCOUNT to value 0
impl crate::Resettable for SCANFIFOCOUNTrs {
    const RESET_VALUE: u32 = 0;
}
