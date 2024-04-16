#[doc = "Register `DIN` reader"]
pub type R = crate::R<DINrs>;
#[doc = "Field `PINS_DIN` reader - Data in for pins 0:15"]
pub type PinsDinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data in for pins 0:15"]
    #[inline(always)]
    pub fn pins_din(&self) -> PinsDinR {
        PinsDinR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINrs;
impl crate::RegisterSpec for DINrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din::R`](R) reader structure"]
impl crate::Readable for DINrs {}
#[doc = "`reset()` method sets DIN to value 0"]
impl crate::Resettable for DINrs {
    const RESET_VALUE: u32 = 0;
}
