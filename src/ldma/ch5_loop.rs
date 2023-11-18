#[doc = "Register `CH5_LOOP` reader"]
pub type R = crate::R<CH5_LOOP_SPEC>;
#[doc = "Register `CH5_LOOP` writer"]
pub type W = crate::W<CH5_LOOP_SPEC>;
#[doc = "Field `LOOPCNT` reader - Linked Structure Sequence Loop Counter"]
pub type LOOPCNT_R = crate::FieldReader;
#[doc = "Field `LOOPCNT` writer - Linked Structure Sequence Loop Counter"]
pub type LOOPCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Linked Structure Sequence Loop Counter"]
    #[inline(always)]
    pub fn loopcnt(&self) -> LOOPCNT_R {
        LOOPCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH5_LOOP")
            .field("loopcnt", &format_args!("{}", self.loopcnt().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CH5_LOOP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Linked Structure Sequence Loop Counter"]
    #[inline(always)]
    #[must_use]
    pub fn loopcnt(&mut self) -> LOOPCNT_W<CH5_LOOP_SPEC, 0> {
        LOOPCNT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_loop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_loop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH5_LOOP_SPEC;
impl crate::RegisterSpec for CH5_LOOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5_loop::R`](R) reader structure"]
impl crate::Readable for CH5_LOOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch5_loop::W`](W) writer structure"]
impl crate::Writable for CH5_LOOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH5_LOOP to value 0"]
impl crate::Resettable for CH5_LOOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
