#[doc = "Reader of register DCDCTIMING"]
pub type R = crate::R<u32, super::DCDCTIMING>;
#[doc = "Writer for register DCDCTIMING"]
pub type W = crate::W<u32, super::DCDCTIMING>;
#[doc = "Register DCDCTIMING `reset()`'s with value 0x0ff1_f8ff"]
impl crate::ResetValue for super::DCDCTIMING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0ff1_f8ff
    }
}
#[doc = "Reader of field `LPINITWAIT`"]
pub type LPINITWAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPINITWAIT`"]
pub struct LPINITWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPINITWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `COMPENPRCHGEN`"]
pub type COMPENPRCHGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPENPRCHGEN`"]
pub struct COMPENPRCHGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPENPRCHGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `LNWAIT`"]
pub type LNWAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LNWAIT`"]
pub struct LNWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> LNWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | (((value as u32) & 0x1f) << 12);
        self.w
    }
}
#[doc = "Reader of field `BYPWAIT`"]
pub type BYPWAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYPWAIT`"]
pub struct BYPWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Reader of field `DUTYSCALE`"]
pub type DUTYSCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DUTYSCALE`"]
pub struct DUTYSCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTYSCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low Power Initialization Wait Time"]
    #[inline(always)]
    pub fn lpinitwait(&self) -> LPINITWAIT_R {
        LPINITWAIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 11 - LN Mode Precharge Enable"]
    #[inline(always)]
    pub fn compenprchgen(&self) -> COMPENPRCHGEN_R {
        COMPENPRCHGEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:16 - Low Noise Controller Initialization Wait Time"]
    #[inline(always)]
    pub fn lnwait(&self) -> LNWAIT_R {
        LNWAIT_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 20:27 - Bypass Mode Transition From Low Power or Low Noise Modes Wait Wait"]
    #[inline(always)]
    pub fn bypwait(&self) -> BYPWAIT_R {
        BYPWAIT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 29:30 - Select Bias Duty Cycle Clock"]
    #[inline(always)]
    pub fn dutyscale(&self) -> DUTYSCALE_R {
        DUTYSCALE_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low Power Initialization Wait Time"]
    #[inline(always)]
    pub fn lpinitwait(&mut self) -> LPINITWAIT_W {
        LPINITWAIT_W { w: self }
    }
    #[doc = "Bit 11 - LN Mode Precharge Enable"]
    #[inline(always)]
    pub fn compenprchgen(&mut self) -> COMPENPRCHGEN_W {
        COMPENPRCHGEN_W { w: self }
    }
    #[doc = "Bits 12:16 - Low Noise Controller Initialization Wait Time"]
    #[inline(always)]
    pub fn lnwait(&mut self) -> LNWAIT_W {
        LNWAIT_W { w: self }
    }
    #[doc = "Bits 20:27 - Bypass Mode Transition From Low Power or Low Noise Modes Wait Wait"]
    #[inline(always)]
    pub fn bypwait(&mut self) -> BYPWAIT_W {
        BYPWAIT_W { w: self }
    }
    #[doc = "Bits 29:30 - Select Bias Duty Cycle Clock"]
    #[inline(always)]
    pub fn dutyscale(&mut self) -> DUTYSCALE_W {
        DUTYSCALE_W { w: self }
    }
}
