#[doc = "Reader of register LFACLKEN0"]
pub type R = crate::R<u32, super::LFACLKEN0>;
#[doc = "Writer for register LFACLKEN0"]
pub type W = crate::W<u32, super::LFACLKEN0>;
#[doc = "Register LFACLKEN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFACLKEN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LETIMER0`"]
pub type LETIMER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LETIMER0`"]
pub struct LETIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> LETIMER0_W<'a> {
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
    #[doc = "Bit 0 - Low Energy Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Energy Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn letimer0(&mut self) -> LETIMER0_W {
        LETIMER0_W { w: self }
    }
}
