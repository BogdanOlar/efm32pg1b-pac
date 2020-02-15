#[doc = "Reader of register PWRCTRL"]
pub type R = crate::R<u32, super::PWRCTRL>;
#[doc = "Writer for register PWRCTRL"]
pub type W = crate::W<u32, super::PWRCTRL>;
#[doc = "Register PWRCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ANASW`"]
pub type ANASW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANASW`"]
pub struct ANASW_W<'a> {
    w: &'a mut W,
}
impl<'a> ANASW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Analog Switch Selection"]
    #[inline(always)]
    pub fn anasw(&self) -> ANASW_R {
        ANASW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Analog Switch Selection"]
    #[inline(always)]
    pub fn anasw(&mut self) -> ANASW_W {
        ANASW_W { w: self }
    }
}
