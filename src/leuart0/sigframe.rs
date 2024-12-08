///Register `SIGFRAME` reader
pub type R = crate::R<SIGFRAMErs>;
///Register `SIGFRAME` writer
pub type W = crate::W<SIGFRAMErs>;
///Field `SIGFRAME` reader - Signal Frame
pub type SigframeR = crate::FieldReader<u16>;
///Field `SIGFRAME` writer - Signal Frame
pub type SigframeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Signal Frame
    #[inline(always)]
    pub fn sigframe(&self) -> SigframeR {
        SigframeR::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGFRAME")
            .field("sigframe", &self.sigframe())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Signal Frame
    #[inline(always)]
    #[must_use]
    pub fn sigframe(&mut self) -> SigframeW<SIGFRAMErs> {
        SigframeW::new(self, 0)
    }
}
///Signal Frame Register
///
///You can [`read`](crate::Reg::read) this register and get [`sigframe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigframe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SIGFRAMErs;
impl crate::RegisterSpec for SIGFRAMErs {
    type Ux = u32;
}
///`read()` method returns [`sigframe::R`](R) reader structure
impl crate::Readable for SIGFRAMErs {}
///`write(|w| ..)` method takes [`sigframe::W`](W) writer structure
impl crate::Writable for SIGFRAMErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SIGFRAME to value 0
impl crate::Resettable for SIGFRAMErs {
    const RESET_VALUE: u32 = 0;
}
