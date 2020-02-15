#[doc = "Reader of register HFXOSTEADYSTATECTRL"]
pub type R = crate::R<u32, super::HFXOSTEADYSTATECTRL>;
#[doc = "Writer for register HFXOSTEADYSTATECTRL"]
pub type W = crate::W<u32, super::HFXOSTEADYSTATECTRL>;
#[doc = "Register HFXOSTEADYSTATECTRL `reset()`'s with value 0xa30a_ad09"]
impl crate::ResetValue for super::HFXOSTEADYSTATECTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xa30a_ad09
    }
}
#[doc = "Reader of field `IBTRIMXOCORE`"]
pub type IBTRIMXOCORE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBTRIMXOCORE`"]
pub struct IBTRIMXOCORE_W<'a> {
    w: &'a mut W,
}
impl<'a> IBTRIMXOCORE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `REGISH`"]
pub type REGISH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGISH`"]
pub struct REGISH_W<'a> {
    w: &'a mut W,
}
impl<'a> REGISH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Reader of field `CTUNE`"]
pub type CTUNE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTUNE`"]
pub struct CTUNE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTUNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 11)) | (((value as u32) & 0x01ff) << 11);
        self.w
    }
}
#[doc = "Reader of field `REGSELILOW`"]
pub type REGSELILOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGSELILOW`"]
pub struct REGSELILOW_W<'a> {
    w: &'a mut W,
}
impl<'a> REGSELILOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `PEAKDETEN`"]
pub type PEAKDETEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEAKDETEN`"]
pub struct PEAKDETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAKDETEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `REGISHUPPER`"]
pub type REGISHUPPER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGISHUPPER`"]
pub struct REGISHUPPER_W<'a> {
    w: &'a mut W,
}
impl<'a> REGISHUPPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:10 - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
    #[inline(always)]
    pub fn regish(&self) -> REGISH_R {
        REGISH_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&self) -> CTUNE_R {
        CTUNE_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:25 - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
    #[inline(always)]
    pub fn regselilow(&self) -> REGSELILOW_R {
        REGSELILOW_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    pub fn peakdeten(&self) -> PEAKDETEN_R {
        PEAKDETEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
    #[inline(always)]
    pub fn regishupper(&self) -> REGISHUPPER_R {
        REGISHUPPER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    pub fn ibtrimxocore(&mut self) -> IBTRIMXOCORE_W {
        IBTRIMXOCORE_W { w: self }
    }
    #[doc = "Bits 7:10 - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
    #[inline(always)]
    pub fn regish(&mut self) -> REGISH_W {
        REGISH_W { w: self }
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&mut self) -> CTUNE_W {
        CTUNE_W { w: self }
    }
    #[doc = "Bits 24:25 - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
    #[inline(always)]
    pub fn regselilow(&mut self) -> REGSELILOW_W {
        REGSELILOW_W { w: self }
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    pub fn peakdeten(&mut self) -> PEAKDETEN_W {
        PEAKDETEN_W { w: self }
    }
    #[doc = "Bits 28:31 - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
    #[inline(always)]
    pub fn regishupper(&mut self) -> REGISHUPPER_W {
        REGISHUPPER_W { w: self }
    }
}
