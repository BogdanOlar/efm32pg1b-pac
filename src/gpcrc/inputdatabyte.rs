///Register `INPUTDATABYTE` reader
pub type R = crate::R<INPUTDATABYTErs>;
///Register `INPUTDATABYTE` writer
pub type W = crate::W<INPUTDATABYTErs>;
///Field `INPUTDATABYTE` reader - Input Data for 8-bit
pub type InputdatabyteR = crate::FieldReader;
///Field `INPUTDATABYTE` writer - Input Data for 8-bit
pub type InputdatabyteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Input Data for 8-bit
    #[inline(always)]
    pub fn inputdatabyte(&self) -> InputdatabyteR {
        InputdatabyteR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INPUTDATABYTE")
            .field("inputdatabyte", &self.inputdatabyte())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Input Data for 8-bit
    #[inline(always)]
    #[must_use]
    pub fn inputdatabyte(&mut self) -> InputdatabyteW<INPUTDATABYTErs> {
        InputdatabyteW::new(self, 0)
    }
}
///Input 8-bit Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`inputdatabyte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdatabyte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INPUTDATABYTErs;
impl crate::RegisterSpec for INPUTDATABYTErs {
    type Ux = u32;
}
///`read()` method returns [`inputdatabyte::R`](R) reader structure
impl crate::Readable for INPUTDATABYTErs {}
///`write(|w| ..)` method takes [`inputdatabyte::W`](W) writer structure
impl crate::Writable for INPUTDATABYTErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INPUTDATABYTE to value 0
impl crate::Resettable for INPUTDATABYTErs {
    const RESET_VALUE: u32 = 0;
}
