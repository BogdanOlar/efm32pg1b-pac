///Register `INPUTDATA` reader
pub type R = crate::R<INPUTDATArs>;
///Register `INPUTDATA` writer
pub type W = crate::W<INPUTDATArs>;
///Field `INPUTDATA` reader - Input Data for 32-bit
pub type InputdataR = crate::FieldReader<u32>;
///Field `INPUTDATA` writer - Input Data for 32-bit
pub type InputdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Input Data for 32-bit
    #[inline(always)]
    pub fn inputdata(&self) -> InputdataR {
        InputdataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INPUTDATA")
            .field("inputdata", &self.inputdata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Input Data for 32-bit
    #[inline(always)]
    #[must_use]
    pub fn inputdata(&mut self) -> InputdataW<INPUTDATArs> {
        InputdataW::new(self, 0)
    }
}
///Input 32-bit Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`inputdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INPUTDATArs;
impl crate::RegisterSpec for INPUTDATArs {
    type Ux = u32;
}
///`read()` method returns [`inputdata::R`](R) reader structure
impl crate::Readable for INPUTDATArs {}
///`write(|w| ..)` method takes [`inputdata::W`](W) writer structure
impl crate::Writable for INPUTDATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INPUTDATA to value 0
impl crate::Resettable for INPUTDATArs {
    const RESET_VALUE: u32 = 0;
}
