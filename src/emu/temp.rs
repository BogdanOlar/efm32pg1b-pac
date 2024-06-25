#[doc = "Register `TEMP` reader"]
pub type R = crate::R<TEMPrs>;
#[doc = "Field `TEMP` reader - Temperature Measurement"]
pub type TempR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Temperature Measurement"]
    #[inline(always)]
    pub fn temp(&self) -> TempR {
        TempR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMP").field("temp", &self.temp()).finish()
    }
}
#[doc = "Value of Last Temperature Measurement\n\nYou can [`read`](crate::Reg::read) this register and get [`temp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEMPrs;
impl crate::RegisterSpec for TEMPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`temp::R`](R) reader structure"]
impl crate::Readable for TEMPrs {}
#[doc = "`reset()` method sets TEMP to value 0"]
impl crate::Resettable for TEMPrs {
    const RESET_VALUE: u32 = 0;
}
