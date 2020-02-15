#[doc = "Reader of register HFPRESC"]
pub type R = crate::R<u32, super::HFPRESC>;
#[doc = "Writer for register HFPRESC"]
pub type W = crate::W<u32, super::HFPRESC>;
#[doc = "Register HFPRESC `reset()`'s with value 0"]
impl crate::ResetValue for super::HFPRESC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "HFCLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESC`"]
pub type PRESC_R = crate::R<u8, PRESC_A>;
impl PRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRESC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESC_A::NODIVISION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC_A::NODIVISION
    }
}
#[doc = "Write proxy for field `PRESC`"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESC_A::NODIVISION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "HFCLKLE Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKLEPRESC_A {
    #[doc = "0: HFCLKLE is HFBUSCLKLE divided by 2."]
    DIV2 = 0,
    #[doc = "1: HFCLKLE is HFBUSCLKLE divided by 4."]
    DIV4 = 1,
}
impl From<HFCLKLEPRESC_A> for bool {
    #[inline(always)]
    fn from(variant: HFCLKLEPRESC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFCLKLEPRESC`"]
pub type HFCLKLEPRESC_R = crate::R<bool, HFCLKLEPRESC_A>;
impl HFCLKLEPRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFCLKLEPRESC_A {
        match self.bits {
            false => HFCLKLEPRESC_A::DIV2,
            true => HFCLKLEPRESC_A::DIV4,
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HFCLKLEPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HFCLKLEPRESC_A::DIV4
    }
}
#[doc = "Write proxy for field `HFCLKLEPRESC`"]
pub struct HFCLKLEPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> HFCLKLEPRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFCLKLEPRESC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HFCLKLEPRESC_A::DIV2)
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HFCLKLEPRESC_A::DIV4)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12 - HFCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - HFCLKLE Prescaler"]
    #[inline(always)]
    pub fn hfclklepresc(&self) -> HFCLKLEPRESC_R {
        HFCLKLEPRESC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:12 - HFCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bit 24 - HFCLKLE Prescaler"]
    #[inline(always)]
    pub fn hfclklepresc(&mut self) -> HFCLKLEPRESC_W {
        HFCLKLEPRESC_W { w: self }
    }
}
