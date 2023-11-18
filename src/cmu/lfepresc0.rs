#[doc = "Register `LFEPRESC0` reader"]
pub type R = crate::R<LFEPRESC0_SPEC>;
#[doc = "Register `LFEPRESC0` writer"]
pub type W = crate::W<LFEPRESC0_SPEC>;
#[doc = "Field `RTCC` reader - Real-Time Counter and Calendar Prescaler"]
pub type RTCC_R = crate::FieldReader<RTCC_A>;
#[doc = "Real-Time Counter and Calendar Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCC_A {
    #[doc = "0: LFECLKRTCC = LFECLK"]
    DIV1 = 0,
}
impl From<RTCC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCC_A {
    type Ux = u8;
}
impl RTCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RTCC_A> {
        match self.bits {
            0 => Some(RTCC_A::DIV1),
            _ => None,
        }
    }
    #[doc = "LFECLKRTCC = LFECLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == RTCC_A::DIV1
    }
}
impl R {
    #[doc = "Bits 0:3 - Real-Time Counter and Calendar Prescaler"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFEPRESC0")
            .field("rtcc", &format_args!("{}", self.rtcc().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<LFEPRESC0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
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
#[doc = "Low Frequency E Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfepresc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfepresc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFEPRESC0_SPEC;
impl crate::RegisterSpec for LFEPRESC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfepresc0::R`](R) reader structure"]
impl crate::Readable for LFEPRESC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfepresc0::W`](W) writer structure"]
impl crate::Writable for LFEPRESC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFEPRESC0 to value 0"]
impl crate::Resettable for LFEPRESC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
