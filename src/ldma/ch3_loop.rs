///Register `CH3_LOOP` reader
pub type R = crate::R<CH3_LOOPrs>;
///Register `CH3_LOOP` writer
pub type W = crate::W<CH3_LOOPrs>;
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
        f.debug_struct("CH3_LOOP")
            .field("loopcnt", &self.loopcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Linked Structure Sequence Loop Counter
    #[inline(always)]
    #[must_use]
    pub fn loopcnt(&mut self) -> LoopcntW<CH3_LOOPrs> {
        LoopcntW::new(self, 0)
    }
}
///Channel Loop Counter Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch3_loop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_loop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CH3_LOOPrs;
impl crate::RegisterSpec for CH3_LOOPrs {
    type Ux = u32;
}
///`read()` method returns [`ch3_loop::R`](R) reader structure
impl crate::Readable for CH3_LOOPrs {}
///`write(|w| ..)` method takes [`ch3_loop::W`](W) writer structure
impl crate::Writable for CH3_LOOPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH3_LOOP to value 0
impl crate::Resettable for CH3_LOOPrs {
    const RESET_VALUE: u32 = 0;
}
