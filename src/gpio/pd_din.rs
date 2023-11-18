#[doc = "Register `PD_DIN` reader"]
pub type R = crate::R<PD_DIN_SPEC>;
#[doc = "Field `DIN` reader - Data in"]
pub type DIN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data in"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PD_DIN")
            .field("din", &format_args!("{}", self.din().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PD_DIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_din::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD_DIN_SPEC;
impl crate::RegisterSpec for PD_DIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_din::R`](R) reader structure"]
impl crate::Readable for PD_DIN_SPEC {}
#[doc = "`reset()` method sets PD_DIN to value 0"]
impl crate::Resettable for PD_DIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
