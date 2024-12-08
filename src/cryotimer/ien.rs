///Register `IEN` reader
pub type R = crate::R<IENrs>;
///Register `IEN` writer
pub type W = crate::W<IENrs>;
///Field `PERIOD` reader - PERIOD Interrupt Enable
pub type PeriodR = crate::BitReader;
///Field `PERIOD` writer - PERIOD Interrupt Enable
pub type PeriodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PERIOD Interrupt Enable
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("period", &self.period())
            .finish()
    }
}
impl W {
    ///Bit 0 - PERIOD Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PeriodW<IENrs> {
        PeriodW::new(self, 0)
    }
}
///Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
///`read()` method returns [`ien::R`](R) reader structure
impl crate::Readable for IENrs {}
///`write(|w| ..)` method takes [`ien::W`](W) writer structure
impl crate::Writable for IENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IEN to value 0
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
