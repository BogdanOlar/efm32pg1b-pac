#[doc = "Register `LFEPRESC0` reader"]
pub type R = crate::R<LFEPRESC0rs>;
#[doc = "Register `LFEPRESC0` writer"]
pub type W = crate::W<LFEPRESC0rs>;
#[doc = "Field `RTCC` reader - Real-Time Counter and Calendar Prescaler"]
pub type RTCC_R = crate::FieldReader<RTCC>;
#[doc = "Real-Time Counter and Calendar Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCC {
    #[doc = "0: LFECLKRTCC = LFECLK"]
    Div1 = 0,
}
impl From<RTCC> for u8 {
    #[inline(always)]
    fn from(variant: RTCC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCC {
    type Ux = u8;
}
impl RTCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RTCC> {
        match self.bits {
            0 => Some(RTCC::Div1),
            _ => None,
        }
    }
    #[doc = "LFECLKRTCC = LFECLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == RTCC::Div1
    }
}
impl R {
    #[doc = "Bits 0:3 - Real-Time Counter and Calendar Prescaler"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new((self.bits & 0x0f) as u8)
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
pub struct LFEPRESC0rs;
impl crate::RegisterSpec for LFEPRESC0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfepresc0::R`](R) reader structure"]
impl crate::Readable for LFEPRESC0rs {}
#[doc = "`write(|w| ..)` method takes [`lfepresc0::W`](W) writer structure"]
impl crate::Writable for LFEPRESC0rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFEPRESC0 to value 0"]
impl crate::Resettable for LFEPRESC0rs {
    const RESET_VALUE: u32 = 0;
}
