#[doc = "Reader of register LFEPRESC0"]
pub type R = crate::R<u32, super::LFEPRESC0>;
#[doc = "Writer for register LFEPRESC0"]
pub type W = crate::W<u32, super::LFEPRESC0>;
#[doc = "Register LFEPRESC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFEPRESC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Real-Time Counter and Calendar Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `RTCC`"]
pub type RTCC_R = crate::R<u8, RTCC_A>;
impl RTCC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RTCC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RTCC_A::DIV1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
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
impl W {}
