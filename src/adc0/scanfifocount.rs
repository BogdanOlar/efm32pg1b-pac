#[doc = "Register `SCANFIFOCOUNT` reader"]
pub type R = crate::R<SCANFIFOCOUNTrs>;
#[doc = "Field `SCANDC` reader - Scan Data Count"]
pub type ScandcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Scan Data Count"]
    #[inline(always)]
    pub fn scandc(&self) -> ScandcR {
        ScandcR::new((self.bits & 7) as u8)
    }
}
#[doc = "Scan FIFO Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanfifocount::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANFIFOCOUNTrs;
impl crate::RegisterSpec for SCANFIFOCOUNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanfifocount::R`](R) reader structure"]
impl crate::Readable for SCANFIFOCOUNTrs {}
#[doc = "`reset()` method sets SCANFIFOCOUNT to value 0"]
impl crate::Resettable for SCANFIFOCOUNTrs {
    const RESET_VALUE: u32 = 0;
}
