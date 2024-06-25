#[doc = "Register `RXDATA` reader"]
pub type R = crate::R<RXDATArs>;
#[doc = "Field `RXDATA` reader - RX Data"]
pub type RxdataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXDATArs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
#[doc = "Receive Buffer Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct RXDATArs;
impl crate::RegisterSpec for RXDATArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata::R`](R) reader structure"]
impl crate::Readable for RXDATArs {}
#[doc = "`reset()` method sets RXDATA to value 0"]
impl crate::Resettable for RXDATArs {
    const RESET_VALUE: u32 = 0;
}
