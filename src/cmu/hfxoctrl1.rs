#[doc = "Reader of register HFXOCTRL1"]
pub type R = crate::R<u32, super::HFXOCTRL1>;
#[doc = "Writer for register HFXOCTRL1"]
pub type W = crate::W<u32, super::HFXOCTRL1>;
#[doc = "Register HFXOCTRL1 `reset()`'s with value 0x0240"]
impl crate::ResetValue for super::HFXOCTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0240
    }
}
#[doc = "Reader of field `PEAKDETTHR`"]
pub type PEAKDETTHR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PEAKDETTHR`"]
pub struct PEAKDETTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAKDETTHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `REGLVL`"]
pub type REGLVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGLVL`"]
pub struct REGLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> REGLVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `XTIBIASEN`"]
pub type XTIBIASEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTIBIASEN`"]
pub struct XTIBIASEN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTIBIASEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sets the Peak Detector amplitude detection threshold levels"]
    #[inline(always)]
    pub fn peakdetthr(&self) -> PEAKDETTHR_R {
        PEAKDETTHR_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn reglvl(&self) -> REGLVL_R {
        REGLVL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn xtibiasen(&self) -> XTIBIASEN_R {
        XTIBIASEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets the Peak Detector amplitude detection threshold levels"]
    #[inline(always)]
    pub fn peakdetthr(&mut self) -> PEAKDETTHR_W {
        PEAKDETTHR_W { w: self }
    }
    #[doc = "Bits 4:6 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn reglvl(&mut self) -> REGLVL_W {
        REGLVL_W { w: self }
    }
    #[doc = "Bit 9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn xtibiasen(&mut self) -> XTIBIASEN_W {
        XTIBIASEN_W { w: self }
    }
}
