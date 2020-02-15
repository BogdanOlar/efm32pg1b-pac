#[doc = "Reader of register BIASTESTCTRL"]
pub type R = crate::R<u32, super::BIASTESTCTRL>;
#[doc = "Writer for register BIASTESTCTRL"]
pub type W = crate::W<u32, super::BIASTESTCTRL>;
#[doc = "Register BIASTESTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BIASTESTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BIAS_RIP_RESET`"]
pub type BIAS_RIP_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_RIP_RESET`"]
pub struct BIAS_RIP_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_RIP_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Reset Bias Ripple Counter"]
    #[inline(always)]
    pub fn bias_rip_reset(&self) -> BIAS_RIP_RESET_R {
        BIAS_RIP_RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Reset Bias Ripple Counter"]
    #[inline(always)]
    pub fn bias_rip_reset(&mut self) -> BIAS_RIP_RESET_W {
        BIAS_RIP_RESET_W { w: self }
    }
}
