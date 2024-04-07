#[doc = "Register `INPUTDATABYTE` reader"]
pub type R = crate::R<INPUTDATABYTErs>;
#[doc = "Register `INPUTDATABYTE` writer"]
pub type W = crate::W<INPUTDATABYTErs>;
#[doc = "Field `INPUTDATABYTE` reader - Input Data for 8-bit"]
pub type InputdatabyteR = crate::FieldReader;
#[doc = "Field `INPUTDATABYTE` writer - Input Data for 8-bit"]
pub type InputdatabyteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Input Data for 8-bit"]
    #[inline(always)]
    pub fn inputdatabyte(&self) -> InputdatabyteR {
        InputdatabyteR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input Data for 8-bit"]
    #[inline(always)]
    #[must_use]
    pub fn inputdatabyte(&mut self) -> InputdatabyteW<INPUTDATABYTErs> {
        InputdatabyteW::new(self, 0)
    }
}
#[doc = "Input 8-bit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputdatabyte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputdatabyte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUTDATABYTErs;
impl crate::RegisterSpec for INPUTDATABYTErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputdatabyte::R`](R) reader structure"]
impl crate::Readable for INPUTDATABYTErs {}
#[doc = "`write(|w| ..)` method takes [`inputdatabyte::W`](W) writer structure"]
impl crate::Writable for INPUTDATABYTErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INPUTDATABYTE to value 0"]
impl crate::Resettable for INPUTDATABYTErs {
    const RESET_VALUE: u32 = 0;
}
