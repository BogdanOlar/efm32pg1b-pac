#[doc = "Reader of register HFXOCTRL"]
pub type R = crate::R<u32, super::HFXOCTRL>;
#[doc = "Writer for register HFXOCTRL"]
pub type W = crate::W<u32, super::HFXOCTRL>;
#[doc = "Register HFXOCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::HFXOCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
#[doc = "HFXO Automatic Peak Detection and Shunt Current Optimization Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PEAKDETSHUNTOPTMODE_A {
    #[doc = "0: Automatic control of HFXO peak detection and shunt optimization sequences. CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can also be used."]
    AUTOCMD = 0,
    #[doc = "1: CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can be used to trigger peak detection and shunt optimization sequences."]
    CMD = 1,
    #[doc = "2: CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE, REGISH, REGSELILOW, and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    MANUAL = 2,
}
impl From<PEAKDETSHUNTOPTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PEAKDETSHUNTOPTMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PEAKDETSHUNTOPTMODE`"]
pub type PEAKDETSHUNTOPTMODE_R = crate::R<u8, PEAKDETSHUNTOPTMODE_A>;
impl PEAKDETSHUNTOPTMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PEAKDETSHUNTOPTMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PEAKDETSHUNTOPTMODE_A::AUTOCMD),
            1 => Val(PEAKDETSHUNTOPTMODE_A::CMD),
            2 => Val(PEAKDETSHUNTOPTMODE_A::MANUAL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUTOCMD`"]
    #[inline(always)]
    pub fn is_autocmd(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODE_A::AUTOCMD
    }
    #[doc = "Checks if the value of the field is `CMD`"]
    #[inline(always)]
    pub fn is_cmd(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODE_A::CMD
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODE_A::MANUAL
    }
}
#[doc = "Write proxy for field `PEAKDETSHUNTOPTMODE`"]
pub struct PEAKDETSHUNTOPTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAKDETSHUNTOPTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEAKDETSHUNTOPTMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Automatic control of HFXO peak detection and shunt optimization sequences. CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can also be used."]
    #[inline(always)]
    pub fn autocmd(self) -> &'a mut W {
        self.variant(PEAKDETSHUNTOPTMODE_A::AUTOCMD)
    }
    #[doc = "CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can be used to trigger peak detection and shunt optimization sequences."]
    #[inline(always)]
    pub fn cmd(self) -> &'a mut W {
        self.variant(PEAKDETSHUNTOPTMODE_A::CMD)
    }
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE, REGISH, REGSELILOW, and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(PEAKDETSHUNTOPTMODE_A::MANUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `LOWPOWER`"]
pub type LOWPOWER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOWPOWER`"]
pub struct LOWPOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWPOWER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `XTI2GND`"]
pub type XTI2GND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTI2GND`"]
pub struct XTI2GND_W<'a> {
    w: &'a mut W,
}
impl<'a> XTI2GND_W<'a> {
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
#[doc = "Reader of field `XTO2GND`"]
pub type XTO2GND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTO2GND`"]
pub struct XTO2GND_W<'a> {
    w: &'a mut W,
}
impl<'a> XTO2GND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "HFXO Low Frequency Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFTIMEOUT_A {
    #[doc = "0: Timeout period of 0 cycles (disabled)"]
    _0CYCLES = 0,
    #[doc = "1: Timeout period of 2 cycles"]
    _2CYCLES = 1,
    #[doc = "2: Timeout period of 4 cycles"]
    _4CYCLES = 2,
    #[doc = "3: Timeout period of 16 cycles"]
    _16CYCLES = 3,
    #[doc = "4: Timeout period of 32 cycles"]
    _32CYCLES = 4,
    #[doc = "5: Timeout period of 64 cycles"]
    _64CYCLES = 5,
    #[doc = "6: Timeout period of 1024 cycles"]
    _1KCYCLES = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4KCYCLES = 7,
}
impl From<LFTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: LFTIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LFTIMEOUT`"]
pub type LFTIMEOUT_R = crate::R<u8, LFTIMEOUT_A>;
impl LFTIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFTIMEOUT_A {
        match self.bits {
            0 => LFTIMEOUT_A::_0CYCLES,
            1 => LFTIMEOUT_A::_2CYCLES,
            2 => LFTIMEOUT_A::_4CYCLES,
            3 => LFTIMEOUT_A::_16CYCLES,
            4 => LFTIMEOUT_A::_32CYCLES,
            5 => LFTIMEOUT_A::_64CYCLES,
            6 => LFTIMEOUT_A::_1KCYCLES,
            7 => LFTIMEOUT_A::_4KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0CYCLES`"]
    #[inline(always)]
    pub fn is_0cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_0CYCLES
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == LFTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == LFTIMEOUT_A::_4KCYCLES
    }
}
#[doc = "Write proxy for field `LFTIMEOUT`"]
pub struct LFTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> LFTIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFTIMEOUT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timeout period of 0 cycles (disabled)"]
    #[inline(always)]
    pub fn _0cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_0CYCLES)
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_32CYCLES)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_64CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_4KCYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `AUTOSTARTEM0EM1`"]
pub type AUTOSTARTEM0EM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOSTARTEM0EM1`"]
pub struct AUTOSTARTEM0EM1_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSTARTEM0EM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `AUTOSTARTSELEM0EM1`"]
pub type AUTOSTARTSELEM0EM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOSTARTSELEM0EM1`"]
pub struct AUTOSTARTSELEM0EM1_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSTARTSELEM0EM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HFXO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
    #[inline(always)]
    pub fn peakdetshuntoptmode(&self) -> PEAKDETSHUNTOPTMODE_R {
        PEAKDETSHUNTOPTMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Low Power Mode Control"]
    #[inline(always)]
    pub fn lowpower(&self) -> LOWPOWER_R {
        LOWPOWER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    pub fn xti2gnd(&self) -> XTI2GND_R {
        XTI2GND_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    pub fn xto2gnd(&self) -> XTO2GND_R {
        XTO2GND_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    pub fn lftimeout(&self) -> LFTIMEOUT_R {
        LFTIMEOUT_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartem0em1(&self) -> AUTOSTARTEM0EM1_R {
        AUTOSTARTEM0EM1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartselem0em1(&self) -> AUTOSTARTSELEM0EM1_R {
        AUTOSTARTSELEM0EM1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFXO Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
    #[inline(always)]
    pub fn peakdetshuntoptmode(&mut self) -> PEAKDETSHUNTOPTMODE_W {
        PEAKDETSHUNTOPTMODE_W { w: self }
    }
    #[doc = "Bit 8 - Low Power Mode Control"]
    #[inline(always)]
    pub fn lowpower(&mut self) -> LOWPOWER_W {
        LOWPOWER_W { w: self }
    }
    #[doc = "Bit 9 - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    pub fn xti2gnd(&mut self) -> XTI2GND_W {
        XTI2GND_W { w: self }
    }
    #[doc = "Bit 10 - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    pub fn xto2gnd(&mut self) -> XTO2GND_W {
        XTO2GND_W { w: self }
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    pub fn lftimeout(&mut self) -> LFTIMEOUT_W {
        LFTIMEOUT_W { w: self }
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartem0em1(&mut self) -> AUTOSTARTEM0EM1_W {
        AUTOSTARTEM0EM1_W { w: self }
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartselem0em1(&mut self) -> AUTOSTARTSELEM0EM1_W {
        AUTOSTARTSELEM0EM1_W { w: self }
    }
}
