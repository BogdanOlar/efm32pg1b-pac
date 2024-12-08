///Register `PERIODSEL` reader
pub type R = crate::R<PERIODSELrs>;
///Register `PERIODSEL` writer
pub type W = crate::W<PERIODSELrs>;
///Field `PERIODSEL` reader - Interrupts/Wakeup Events Period Setting
pub type PeriodselR = crate::FieldReader;
///Field `PERIODSEL` writer - Interrupts/Wakeup Events Period Setting
pub type PeriodselW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - Interrupts/Wakeup Events Period Setting
    #[inline(always)]
    pub fn periodsel(&self) -> PeriodselR {
        PeriodselR::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIODSEL")
            .field("periodsel", &self.periodsel())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Interrupts/Wakeup Events Period Setting
    #[inline(always)]
    #[must_use]
    pub fn periodsel(&mut self) -> PeriodselW<PERIODSELrs> {
        PeriodselW::new(self, 0)
    }
}
///Interrupt Duration
///
///You can [`read`](crate::Reg::read) this register and get [`periodsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periodsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PERIODSELrs;
impl crate::RegisterSpec for PERIODSELrs {
    type Ux = u32;
}
///`read()` method returns [`periodsel::R`](R) reader structure
impl crate::Readable for PERIODSELrs {}
///`write(|w| ..)` method takes [`periodsel::W`](W) writer structure
impl crate::Writable for PERIODSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERIODSEL to value 0x20
impl crate::Resettable for PERIODSELrs {
    const RESET_VALUE: u32 = 0x20;
}
