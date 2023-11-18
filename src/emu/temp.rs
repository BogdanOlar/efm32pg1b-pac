#[doc = "Register `TEMP` reader"]
pub type R = crate::R<TEMP_SPEC>;
#[doc = "Field `TEMP` reader - Temperature Measurement"]
pub type TEMP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Temperature Measurement"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMP")
            .field("temp", &format_args!("{}", self.temp().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TEMP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Value of Last Temperature Measurement\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`temp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEMP_SPEC;
impl crate::RegisterSpec for TEMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`temp::R`](R) reader structure"]
impl crate::Readable for TEMP_SPEC {}
#[doc = "`reset()` method sets TEMP to value 0"]
impl crate::Resettable for TEMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
