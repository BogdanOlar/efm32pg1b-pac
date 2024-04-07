#[doc = "Register `PF_DIN` reader"]
pub type R = crate::R<PF_DINrs>;
#[doc = "Field `DIN` reader - Data in"]
pub type DinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data in"]
    #[inline(always)]
    pub fn din(&self) -> DinR {
        DinR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_din::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PF_DINrs;
impl crate::RegisterSpec for PF_DINrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_din::R`](R) reader structure"]
impl crate::Readable for PF_DINrs {}
#[doc = "`reset()` method sets PF_DIN to value 0"]
impl crate::Resettable for PF_DINrs {
    const RESET_VALUE: u32 = 0;
}
