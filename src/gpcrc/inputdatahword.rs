///Register `INPUTDATAHWORD` reader
pub type R = crate::R<INPUTDATAHWORDrs>;
///Register `INPUTDATAHWORD` writer
pub type W = crate::W<INPUTDATAHWORDrs>;
///Field `INPUTDATAHWORD` reader - Input Data for 16-bit
pub type InputdatahwordR = crate::FieldReader<u16>;
///Field `INPUTDATAHWORD` writer - Input Data for 16-bit
pub type InputdatahwordW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Input Data for 16-bit
    #[inline(always)]
    pub fn inputdatahword(&self) -> InputdatahwordR {
        InputdatahwordR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INPUTDATAHWORD")
            .field("inputdatahword", &self.inputdatahword())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Input Data for 16-bit
    #[inline(always)]
    #[must_use]
    pub fn inputdatahword(&mut self) -> InputdatahwordW<INPUTDATAHWORDrs> {
        InputdatahwordW::new(self, 0)
    }
}
///Input 16-bit Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`inputdatahword::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdatahword::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INPUTDATAHWORDrs;
impl crate::RegisterSpec for INPUTDATAHWORDrs {
    type Ux = u32;
}
///`read()` method returns [`inputdatahword::R`](R) reader structure
impl crate::Readable for INPUTDATAHWORDrs {}
///`write(|w| ..)` method takes [`inputdatahword::W`](W) writer structure
impl crate::Writable for INPUTDATAHWORDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INPUTDATAHWORD to value 0
impl crate::Resettable for INPUTDATAHWORDrs {
    const RESET_VALUE: u32 = 0;
}
