#[doc = "Reader of register ROUTEPEN"]
pub type R = crate::R<u32, super::ROUTEPEN>;
#[doc = "Writer for register ROUTEPEN"]
pub type W = crate::W<u32, super::ROUTEPEN>;
#[doc = "Register ROUTEPEN `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::ROUTEPEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `SWCLKTCKPEN`"]
pub type SWCLKTCKPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWCLKTCKPEN`"]
pub struct SWCLKTCKPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCLKTCKPEN_W<'a> {
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
#[doc = "Reader of field `SWDIOTMSPEN`"]
pub type SWDIOTMSPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWDIOTMSPEN`"]
pub struct SWDIOTMSPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDIOTMSPEN_W<'a> {
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
#[doc = "Reader of field `TDOPEN`"]
pub type TDOPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TDOPEN`"]
pub struct TDOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDOPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TDIPEN`"]
pub type TDIPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TDIPEN`"]
pub struct TDIPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIPEN_W<'a> {
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
#[doc = "Reader of field `SWVPEN`"]
pub type SWVPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWVPEN`"]
pub struct SWVPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWVPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Serial Wire Clock and JTAG Test Clock Pin Enable"]
    #[inline(always)]
    pub fn swclktckpen(&self) -> SWCLKTCKPEN_R {
        SWCLKTCKPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
    #[inline(always)]
    pub fn swdiotmspen(&self) -> SWDIOTMSPEN_R {
        SWDIOTMSPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    pub fn tdopen(&self) -> TDOPEN_R {
        TDOPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    pub fn tdipen(&self) -> TDIPEN_R {
        TDIPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swvpen(&self) -> SWVPEN_R {
        SWVPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Wire Clock and JTAG Test Clock Pin Enable"]
    #[inline(always)]
    pub fn swclktckpen(&mut self) -> SWCLKTCKPEN_W {
        SWCLKTCKPEN_W { w: self }
    }
    #[doc = "Bit 1 - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
    #[inline(always)]
    pub fn swdiotmspen(&mut self) -> SWDIOTMSPEN_W {
        SWDIOTMSPEN_W { w: self }
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    pub fn tdopen(&mut self) -> TDOPEN_W {
        TDOPEN_W { w: self }
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    pub fn tdipen(&mut self) -> TDIPEN_W {
        TDIPEN_W { w: self }
    }
    #[doc = "Bit 4 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swvpen(&mut self) -> SWVPEN_W {
        SWVPEN_W { w: self }
    }
}
