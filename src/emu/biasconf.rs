#[doc = "Reader of register BIASCONF"]
pub type R = crate::R<u32, super::BIASCONF>;
#[doc = "Writer for register BIASCONF"]
pub type W = crate::W<u32, super::BIASCONF>;
#[doc = "Register BIASCONF `reset()`'s with value 0xf8"]
impl crate::ResetValue for super::BIASCONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf8
    }
}
#[doc = "Reader of field `NADUTYEM01`"]
pub type NADUTYEM01_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NADUTYEM01`"]
pub struct NADUTYEM01_W<'a> {
    w: &'a mut W,
}
impl<'a> NADUTYEM01_W<'a> {
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
#[doc = "Reader of field `LPEM01`"]
pub type LPEM01_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPEM01`"]
pub struct LPEM01_W<'a> {
    w: &'a mut W,
}
impl<'a> LPEM01_W<'a> {
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
#[doc = "Reader of field `GMCEM23`"]
pub type GMCEM23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GMCEM23`"]
pub struct GMCEM23_W<'a> {
    w: &'a mut W,
}
impl<'a> GMCEM23_W<'a> {
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
#[doc = "Reader of field `UADUTYEM23`"]
pub type UADUTYEM23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UADUTYEM23`"]
pub struct UADUTYEM23_W<'a> {
    w: &'a mut W,
}
impl<'a> UADUTYEM23_W<'a> {
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
#[doc = "Reader of field `NADUTYEM23`"]
pub type NADUTYEM23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NADUTYEM23`"]
pub struct NADUTYEM23_W<'a> {
    w: &'a mut W,
}
impl<'a> NADUTYEM23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `LPEM23`"]
pub type LPEM23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPEM23`"]
pub struct LPEM23_W<'a> {
    w: &'a mut W,
}
impl<'a> LPEM23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - NA DUTY in EM01"]
    #[inline(always)]
    pub fn nadutyem01(&self) -> NADUTYEM01_R {
        NADUTYEM01_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LP in EM01"]
    #[inline(always)]
    pub fn lpem01(&self) -> LPEM01_R {
        LPEM01_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GMC in EM234"]
    #[inline(always)]
    pub fn gmcem23(&self) -> GMCEM23_R {
        GMCEM23_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UADUTY in EM234"]
    #[inline(always)]
    pub fn uadutyem23(&self) -> UADUTYEM23_R {
        UADUTYEM23_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - NA DUTY in EM234"]
    #[inline(always)]
    pub fn nadutyem23(&self) -> NADUTYEM23_R {
        NADUTYEM23_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LP in EM234"]
    #[inline(always)]
    pub fn lpem23(&self) -> LPEM23_R {
        LPEM23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - NA DUTY in EM01"]
    #[inline(always)]
    pub fn nadutyem01(&mut self) -> NADUTYEM01_W {
        NADUTYEM01_W { w: self }
    }
    #[doc = "Bit 3 - LP in EM01"]
    #[inline(always)]
    pub fn lpem01(&mut self) -> LPEM01_W {
        LPEM01_W { w: self }
    }
    #[doc = "Bit 4 - GMC in EM234"]
    #[inline(always)]
    pub fn gmcem23(&mut self) -> GMCEM23_W {
        GMCEM23_W { w: self }
    }
    #[doc = "Bit 5 - UADUTY in EM234"]
    #[inline(always)]
    pub fn uadutyem23(&mut self) -> UADUTYEM23_W {
        UADUTYEM23_W { w: self }
    }
    #[doc = "Bit 6 - NA DUTY in EM234"]
    #[inline(always)]
    pub fn nadutyem23(&mut self) -> NADUTYEM23_W {
        NADUTYEM23_W { w: self }
    }
    #[doc = "Bit 7 - LP in EM234"]
    #[inline(always)]
    pub fn lpem23(&mut self) -> LPEM23_W {
        LPEM23_W { w: self }
    }
}
