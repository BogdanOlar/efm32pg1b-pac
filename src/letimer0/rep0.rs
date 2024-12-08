///Register `REP0` reader
pub type R = crate::R<REP0rs>;
///Register `REP0` writer
pub type W = crate::W<REP0rs>;
///Field `REP0` reader - Repeat Counter 0
pub type Rep0R = crate::FieldReader;
///Field `REP0` writer - Repeat Counter 0
pub type Rep0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Repeat Counter 0
    #[inline(always)]
    pub fn rep0(&self) -> Rep0R {
        Rep0R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REP0").field("rep0", &self.rep0()).finish()
    }
}
impl W {
    ///Bits 0:7 - Repeat Counter 0
    #[inline(always)]
    #[must_use]
    pub fn rep0(&mut self) -> Rep0W<REP0rs> {
        Rep0W::new(self, 0)
    }
}
///Repeat Counter Register 0
///
///You can [`read`](crate::Reg::read) this register and get [`rep0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rep0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct REP0rs;
impl crate::RegisterSpec for REP0rs {
    type Ux = u32;
}
///`read()` method returns [`rep0::R`](R) reader structure
impl crate::Readable for REP0rs {}
///`write(|w| ..)` method takes [`rep0::W`](W) writer structure
impl crate::Writable for REP0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REP0 to value 0
impl crate::Resettable for REP0rs {
    const RESET_VALUE: u32 = 0;
}
