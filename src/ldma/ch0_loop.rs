///Register `CH0_LOOP` reader
pub type R = crate::R<CH0_LOOPrs>;
///Register `CH0_LOOP` writer
pub type W = crate::W<CH0_LOOPrs>;
///Field `LOOPCNT` reader - Linked Structure Sequence Loop Counter
pub type LoopcntR = crate::FieldReader;
///Field `LOOPCNT` writer - Linked Structure Sequence Loop Counter
pub type LoopcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Linked Structure Sequence Loop Counter
    #[inline(always)]
    pub fn loopcnt(&self) -> LoopcntR {
        LoopcntR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH0_LOOP")
            .field("loopcnt", &self.loopcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Linked Structure Sequence Loop Counter
    #[inline(always)]
    #[must_use]
    pub fn loopcnt(&mut self) -> LoopcntW<CH0_LOOPrs> {
        LoopcntW::new(self, 0)
    }
}
///Channel Loop Counter Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch0_loop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_loop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CH0_LOOPrs;
impl crate::RegisterSpec for CH0_LOOPrs {
    type Ux = u32;
}
///`read()` method returns [`ch0_loop::R`](R) reader structure
impl crate::Readable for CH0_LOOPrs {}
///`write(|w| ..)` method takes [`ch0_loop::W`](W) writer structure
impl crate::Writable for CH0_LOOPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH0_LOOP to value 0
impl crate::Resettable for CH0_LOOPrs {
    const RESET_VALUE: u32 = 0;
}
