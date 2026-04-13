///Register `LOOP` reader
pub type R = crate::R<LOOPrs>;
///Register `LOOP` writer
pub type W = crate::W<LOOPrs>;
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
        f.debug_struct("LOOP")
            .field("loopcnt", &self.loopcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Linked Structure Sequence Loop Counter
    #[inline(always)]
    pub fn loopcnt(&mut self) -> LoopcntW<'_, LOOPrs> {
        LoopcntW::new(self, 0)
    }
}
///Channel Loop Counter Register
///
///You can [`read`](crate::Reg::read) this register and get [`loop_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loop_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LOOPrs;
impl crate::RegisterSpec for LOOPrs {
    type Ux = u32;
}
///`read()` method returns [`loop_::R`](R) reader structure
impl crate::Readable for LOOPrs {}
///`write(|w| ..)` method takes [`loop_::W`](W) writer structure
impl crate::Writable for LOOPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOOP to value 0
impl crate::Resettable for LOOPrs {}
