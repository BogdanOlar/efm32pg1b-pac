#[doc = "Reader of register PCNTCTRL"]
pub type R = crate::R<u32, super::PCNTCTRL>;
#[doc = "Writer for register PCNTCTRL"]
pub type W = crate::W<u32, super::PCNTCTRL>;
#[doc = "Register PCNTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PCNTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCNT0CLKEN`"]
pub type PCNT0CLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT0CLKEN`"]
pub struct PCNT0CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT0CLKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `PCNT0CLKSEL`"]
pub type PCNT0CLKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT0CLKSEL`"]
pub struct PCNT0CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT0CLKSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    pub fn pcnt0clken(&self) -> PCNT0CLKEN_R {
        PCNT0CLKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    pub fn pcnt0clksel(&self) -> PCNT0CLKSEL_R {
        PCNT0CLKSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    pub fn pcnt0clken(&mut self) -> PCNT0CLKEN_W {
        PCNT0CLKEN_W { w: self }
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    pub fn pcnt0clksel(&mut self) -> PCNT0CLKSEL_W {
        PCNT0CLKSEL_W { w: self }
    }
}
