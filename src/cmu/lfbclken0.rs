#[doc = "Reader of register LFBCLKEN0"]
pub type R = crate::R<u32, super::LFBCLKEN0>;
#[doc = "Writer for register LFBCLKEN0"]
pub type W = crate::W<u32, super::LFBCLKEN0>;
#[doc = "Register LFBCLKEN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFBCLKEN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEUART0`"]
pub type LEUART0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEUART0`"]
pub struct LEUART0_W<'a> {
    w: &'a mut W,
}
impl<'a> LEUART0_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Low Energy UART 0 Clock Enable"]
    #[inline(always)]
    pub fn leuart0(&self) -> LEUART0_R {
        LEUART0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Energy UART 0 Clock Enable"]
    #[inline(always)]
    pub fn leuart0(&mut self) -> LEUART0_W {
        LEUART0_W { w: self }
    }
}
