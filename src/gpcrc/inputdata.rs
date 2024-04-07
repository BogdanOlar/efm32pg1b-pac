#[doc = "Register `INPUTDATA` reader"]
pub type R = crate::R<INPUTDATArs>;
#[doc = "Register `INPUTDATA` writer"]
pub type W = crate::W<INPUTDATArs>;
#[doc = "Field `INPUTDATA` reader - Input Data for 32-bit"]
pub type InputdataR = crate::FieldReader<u32>;
#[doc = "Field `INPUTDATA` writer - Input Data for 32-bit"]
pub type InputdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Input Data for 32-bit"]
    #[inline(always)]
    pub fn inputdata(&self) -> InputdataR {
        InputdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Data for 32-bit"]
    #[inline(always)]
    #[must_use]
    pub fn inputdata(&mut self) -> InputdataW<INPUTDATArs> {
        InputdataW::new(self, 0)
    }
}
#[doc = "Input 32-bit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUTDATArs;
impl crate::RegisterSpec for INPUTDATArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputdata::R`](R) reader structure"]
impl crate::Readable for INPUTDATArs {}
#[doc = "`write(|w| ..)` method takes [`inputdata::W`](W) writer structure"]
impl crate::Writable for INPUTDATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INPUTDATA to value 0"]
impl crate::Resettable for INPUTDATArs {
    const RESET_VALUE: u32 = 0;
}
