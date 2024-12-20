///Register `REP1` reader
pub type R = crate::R<REP1rs>;
///Register `REP1` writer
pub type W = crate::W<REP1rs>;
///Field `REP1` reader - Repeat Counter 1
pub type Rep1R = crate::FieldReader;
///Field `REP1` writer - Repeat Counter 1
pub type Rep1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Repeat Counter 1
    #[inline(always)]
    pub fn rep1(&self) -> Rep1R {
        Rep1R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REP1").field("rep1", &self.rep1()).finish()
    }
}
impl W {
    ///Bits 0:7 - Repeat Counter 1
    #[inline(always)]
    #[must_use]
    pub fn rep1(&mut self) -> Rep1W<REP1rs> {
        Rep1W::new(self, 0)
    }
}
///Repeat Counter Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`rep1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rep1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct REP1rs;
impl crate::RegisterSpec for REP1rs {
    type Ux = u32;
}
///`read()` method returns [`rep1::R`](R) reader structure
impl crate::Readable for REP1rs {}
///`write(|w| ..)` method takes [`rep1::W`](W) writer structure
impl crate::Writable for REP1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REP1 to value 0
impl crate::Resettable for REP1rs {
    const RESET_VALUE: u32 = 0;
}
