#[doc = "Register `CH1_LOOP` reader"]
pub type R = crate::R<CH1_LOOPrs>;
#[doc = "Register `CH1_LOOP` writer"]
pub type W = crate::W<CH1_LOOPrs>;
#[doc = "Field `LOOPCNT` reader - Linked Structure Sequence Loop Counter"]
pub type LoopcntR = crate::FieldReader;
#[doc = "Field `LOOPCNT` writer - Linked Structure Sequence Loop Counter"]
pub type LoopcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Linked Structure Sequence Loop Counter"]
    #[inline(always)]
    pub fn loopcnt(&self) -> LoopcntR {
        LoopcntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Linked Structure Sequence Loop Counter"]
    #[inline(always)]
    #[must_use]
    pub fn loopcnt(&mut self) -> LoopcntW<CH1_LOOPrs> {
        LoopcntW::new(self, 0)
    }
}
#[doc = "Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_loop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_loop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH1_LOOPrs;
impl crate::RegisterSpec for CH1_LOOPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_loop::R`](R) reader structure"]
impl crate::Readable for CH1_LOOPrs {}
#[doc = "`write(|w| ..)` method takes [`ch1_loop::W`](W) writer structure"]
impl crate::Writable for CH1_LOOPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1_LOOP to value 0"]
impl crate::Resettable for CH1_LOOPrs {
    const RESET_VALUE: u32 = 0;
}
