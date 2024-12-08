///Register `STARTFRAME` reader
pub type R = crate::R<STARTFRAMErs>;
///Register `STARTFRAME` writer
pub type W = crate::W<STARTFRAMErs>;
///Field `STARTFRAME` reader - Start Frame
pub type StartframeR = crate::FieldReader<u16>;
///Field `STARTFRAME` writer - Start Frame
pub type StartframeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Start Frame
    #[inline(always)]
    pub fn startframe(&self) -> StartframeR {
        StartframeR::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STARTFRAME")
            .field("startframe", &self.startframe())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Start Frame
    #[inline(always)]
    #[must_use]
    pub fn startframe(&mut self) -> StartframeW<STARTFRAMErs> {
        StartframeW::new(self, 0)
    }
}
///Start Frame Register
///
///You can [`read`](crate::Reg::read) this register and get [`startframe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`startframe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct STARTFRAMErs;
impl crate::RegisterSpec for STARTFRAMErs {
    type Ux = u32;
}
///`read()` method returns [`startframe::R`](R) reader structure
impl crate::Readable for STARTFRAMErs {}
///`write(|w| ..)` method takes [`startframe::W`](W) writer structure
impl crate::Writable for STARTFRAMErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STARTFRAME to value 0
impl crate::Resettable for STARTFRAMErs {
    const RESET_VALUE: u32 = 0;
}
