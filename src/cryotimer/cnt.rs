#[doc = "Register `CNT` reader"]
pub type R = crate::R<CNTrs>;
#[doc = "Field `CNT` reader - Counter Value"]
pub type CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Counter Value"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
#[doc = "Counter Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTrs;
impl crate::RegisterSpec for CNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CNTrs {}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNTrs {
    const RESET_VALUE: u32 = 0;
}
