#[doc = "Reader of register PWRCFG"]
pub type R = crate::R<u32, super::PWRCFG>;
#[doc = "Writer for register PWRCFG"]
pub type W = crate::W<u32, super::PWRCFG>;
#[doc = "Register PWRCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Power Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRCFG_A {
    #[doc = "0: Power up configuration. Works with any external configuration."]
    STARTUP = 0,
    #[doc = "2: DCDC is enabled and routed to DVDD."]
    DCDCTODVDD = 2,
}
impl From<PWRCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWRCFG`"]
pub type PWRCFG_R = crate::R<u8, PWRCFG_A>;
impl PWRCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWRCFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWRCFG_A::STARTUP),
            2 => Val(PWRCFG_A::DCDCTODVDD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STARTUP`"]
    #[inline(always)]
    pub fn is_startup(&self) -> bool {
        *self == PWRCFG_A::STARTUP
    }
    #[doc = "Checks if the value of the field is `DCDCTODVDD`"]
    #[inline(always)]
    pub fn is_dcdctodvdd(&self) -> bool {
        *self == PWRCFG_A::DCDCTODVDD
    }
}
#[doc = "Write proxy for field `PWRCFG`"]
pub struct PWRCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Power up configuration. Works with any external configuration."]
    #[inline(always)]
    pub fn startup(self) -> &'a mut W {
        self.variant(PWRCFG_A::STARTUP)
    }
    #[doc = "DCDC is enabled and routed to DVDD."]
    #[inline(always)]
    pub fn dcdctodvdd(self) -> &'a mut W {
        self.variant(PWRCFG_A::DCDCTODVDD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Power Configuration"]
    #[inline(always)]
    pub fn pwrcfg(&self) -> PWRCFG_R {
        PWRCFG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Power Configuration"]
    #[inline(always)]
    pub fn pwrcfg(&mut self) -> PWRCFG_W {
        PWRCFG_W { w: self }
    }
}
