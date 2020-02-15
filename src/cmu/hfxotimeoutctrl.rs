#[doc = "Reader of register HFXOTIMEOUTCTRL"]
pub type R = crate::R<u32, super::HFXOTIMEOUTCTRL>;
#[doc = "Writer for register HFXOTIMEOUTCTRL"]
pub type W = crate::W<u32, super::HFXOTIMEOUTCTRL>;
#[doc = "Register HFXOTIMEOUTCTRL `reset()`'s with value 0x0002_6667"]
impl crate::ResetValue for super::HFXOTIMEOUTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_6667
    }
}
#[doc = "Wait Duration in HFXO Startup Enable Wait State\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTUPTIMEOUT_A {
    #[doc = "0: Timeout period of 2 cycles"]
    _2CYCLES = 0,
    #[doc = "1: Timeout period of 4 cycles"]
    _4CYCLES = 1,
    #[doc = "2: Timeout period of 16 cycles"]
    _16CYCLES = 2,
    #[doc = "3: Timeout period of 32 cycles"]
    _32CYCLES = 3,
    #[doc = "4: Timeout period of 256 cycles"]
    _256CYCLES = 4,
    #[doc = "5: Timeout period of 1024 cycles"]
    _1KCYCLES = 5,
    #[doc = "6: Timeout period of 2048 cycles"]
    _2KCYCLES = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4KCYCLES = 7,
    #[doc = "8: Timeout period of 8192 cycles"]
    _8KCYCLES = 8,
    #[doc = "9: Timeout period of 16384 cycles"]
    _16KCYCLES = 9,
    #[doc = "10: Timeout period of 32768 cycles"]
    _32KCYCLES = 10,
}
impl From<STARTUPTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUPTIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STARTUPTIMEOUT`"]
pub type STARTUPTIMEOUT_R = crate::R<u8, STARTUPTIMEOUT_A>;
impl STARTUPTIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STARTUPTIMEOUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STARTUPTIMEOUT_A::_2CYCLES),
            1 => Val(STARTUPTIMEOUT_A::_4CYCLES),
            2 => Val(STARTUPTIMEOUT_A::_16CYCLES),
            3 => Val(STARTUPTIMEOUT_A::_32CYCLES),
            4 => Val(STARTUPTIMEOUT_A::_256CYCLES),
            5 => Val(STARTUPTIMEOUT_A::_1KCYCLES),
            6 => Val(STARTUPTIMEOUT_A::_2KCYCLES),
            7 => Val(STARTUPTIMEOUT_A::_4KCYCLES),
            8 => Val(STARTUPTIMEOUT_A::_8KCYCLES),
            9 => Val(STARTUPTIMEOUT_A::_16KCYCLES),
            10 => Val(STARTUPTIMEOUT_A::_32KCYCLES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_32KCYCLES
    }
}
#[doc = "Write proxy for field `STARTUPTIMEOUT`"]
pub struct STARTUPTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUPTIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTUPTIMEOUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_32CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_32KCYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Wait Duration in HFXO Startup Steady Wait State\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STEADYTIMEOUT_A {
    #[doc = "0: Timeout period of 2 cycles"]
    _2CYCLES = 0,
    #[doc = "1: Timeout period of 4 cycles"]
    _4CYCLES = 1,
    #[doc = "2: Timeout period of 16 cycles"]
    _16CYCLES = 2,
    #[doc = "3: Timeout period of 32 cycles"]
    _32CYCLES = 3,
    #[doc = "4: Timeout period of 256 cycles"]
    _256CYCLES = 4,
    #[doc = "5: Timeout period of 1024 cycles"]
    _1KCYCLES = 5,
    #[doc = "6: Timeout period of 2048 cycles"]
    _2KCYCLES = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4KCYCLES = 7,
    #[doc = "8: Timeout period of 8192 cycles"]
    _8KCYCLES = 8,
    #[doc = "9: Timeout period of 16384 cycles"]
    _16KCYCLES = 9,
    #[doc = "10: Timeout period of 32768 cycles"]
    _32KCYCLES = 10,
}
impl From<STEADYTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: STEADYTIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STEADYTIMEOUT`"]
pub type STEADYTIMEOUT_R = crate::R<u8, STEADYTIMEOUT_A>;
impl STEADYTIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STEADYTIMEOUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STEADYTIMEOUT_A::_2CYCLES),
            1 => Val(STEADYTIMEOUT_A::_4CYCLES),
            2 => Val(STEADYTIMEOUT_A::_16CYCLES),
            3 => Val(STEADYTIMEOUT_A::_32CYCLES),
            4 => Val(STEADYTIMEOUT_A::_256CYCLES),
            5 => Val(STEADYTIMEOUT_A::_1KCYCLES),
            6 => Val(STEADYTIMEOUT_A::_2KCYCLES),
            7 => Val(STEADYTIMEOUT_A::_4KCYCLES),
            8 => Val(STEADYTIMEOUT_A::_8KCYCLES),
            9 => Val(STEADYTIMEOUT_A::_16KCYCLES),
            10 => Val(STEADYTIMEOUT_A::_32KCYCLES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_32KCYCLES
    }
}
#[doc = "Write proxy for field `STEADYTIMEOUT`"]
pub struct STEADYTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> STEADYTIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STEADYTIMEOUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_32CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_32KCYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Wait Duration in HFXO Peak Detection Wait State\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PEAKDETTIMEOUT_A {
    #[doc = "0: Timeout period of 2 cycles"]
    _2CYCLES = 0,
    #[doc = "1: Timeout period of 4 cycles"]
    _4CYCLES = 1,
    #[doc = "2: Timeout period of 16 cycles"]
    _16CYCLES = 2,
    #[doc = "3: Timeout period of 32 cycles"]
    _32CYCLES = 3,
    #[doc = "4: Timeout period of 256 cycles"]
    _256CYCLES = 4,
    #[doc = "5: Timeout period of 1024 cycles"]
    _1KCYCLES = 5,
    #[doc = "6: Timeout period of 2048 cycles"]
    _2KCYCLES = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4KCYCLES = 7,
    #[doc = "8: Timeout period of 8192 cycles"]
    _8KCYCLES = 8,
    #[doc = "9: Timeout period of 16384 cycles"]
    _16KCYCLES = 9,
    #[doc = "10: Timeout period of 32768 cycles"]
    _32KCYCLES = 10,
}
impl From<PEAKDETTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: PEAKDETTIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PEAKDETTIMEOUT`"]
pub type PEAKDETTIMEOUT_R = crate::R<u8, PEAKDETTIMEOUT_A>;
impl PEAKDETTIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PEAKDETTIMEOUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PEAKDETTIMEOUT_A::_2CYCLES),
            1 => Val(PEAKDETTIMEOUT_A::_4CYCLES),
            2 => Val(PEAKDETTIMEOUT_A::_16CYCLES),
            3 => Val(PEAKDETTIMEOUT_A::_32CYCLES),
            4 => Val(PEAKDETTIMEOUT_A::_256CYCLES),
            5 => Val(PEAKDETTIMEOUT_A::_1KCYCLES),
            6 => Val(PEAKDETTIMEOUT_A::_2KCYCLES),
            7 => Val(PEAKDETTIMEOUT_A::_4KCYCLES),
            8 => Val(PEAKDETTIMEOUT_A::_8KCYCLES),
            9 => Val(PEAKDETTIMEOUT_A::_16KCYCLES),
            10 => Val(PEAKDETTIMEOUT_A::_32KCYCLES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_32KCYCLES
    }
}
#[doc = "Write proxy for field `PEAKDETTIMEOUT`"]
pub struct PEAKDETTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAKDETTIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEAKDETTIMEOUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_32CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_32KCYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Wait Duration in HFXO Shunt Current Optimization Wait State\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SHUNTOPTTIMEOUT_A {
    #[doc = "0: Timeout period of 2 cycles"]
    _2CYCLES = 0,
    #[doc = "1: Timeout period of 4 cycles"]
    _4CYCLES = 1,
    #[doc = "2: Timeout period of 16 cycles"]
    _16CYCLES = 2,
    #[doc = "3: Timeout period of 32 cycles"]
    _32CYCLES = 3,
    #[doc = "4: Timeout period of 256 cycles"]
    _256CYCLES = 4,
    #[doc = "5: Timeout period of 1024 cycles"]
    _1KCYCLES = 5,
    #[doc = "6: Timeout period of 2048 cycles"]
    _2KCYCLES = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4KCYCLES = 7,
    #[doc = "8: Timeout period of 8192 cycles"]
    _8KCYCLES = 8,
    #[doc = "9: Timeout period of 16384 cycles"]
    _16KCYCLES = 9,
    #[doc = "10: Timeout period of 32768 cycles"]
    _32KCYCLES = 10,
}
impl From<SHUNTOPTTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: SHUNTOPTTIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SHUNTOPTTIMEOUT`"]
pub type SHUNTOPTTIMEOUT_R = crate::R<u8, SHUNTOPTTIMEOUT_A>;
impl SHUNTOPTTIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SHUNTOPTTIMEOUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SHUNTOPTTIMEOUT_A::_2CYCLES),
            1 => Val(SHUNTOPTTIMEOUT_A::_4CYCLES),
            2 => Val(SHUNTOPTTIMEOUT_A::_16CYCLES),
            3 => Val(SHUNTOPTTIMEOUT_A::_32CYCLES),
            4 => Val(SHUNTOPTTIMEOUT_A::_256CYCLES),
            5 => Val(SHUNTOPTTIMEOUT_A::_1KCYCLES),
            6 => Val(SHUNTOPTTIMEOUT_A::_2KCYCLES),
            7 => Val(SHUNTOPTTIMEOUT_A::_4KCYCLES),
            8 => Val(SHUNTOPTTIMEOUT_A::_8KCYCLES),
            9 => Val(SHUNTOPTTIMEOUT_A::_16KCYCLES),
            10 => Val(SHUNTOPTTIMEOUT_A::_32KCYCLES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_32KCYCLES
    }
}
#[doc = "Write proxy for field `SHUNTOPTTIMEOUT`"]
pub struct SHUNTOPTTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHUNTOPTTIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHUNTOPTTIMEOUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_32CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_32KCYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Wait Duration in HFXO Startup Enable Wait State"]
    #[inline(always)]
    pub fn startuptimeout(&self) -> STARTUPTIMEOUT_R {
        STARTUPTIMEOUT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Wait Duration in HFXO Startup Steady Wait State"]
    #[inline(always)]
    pub fn steadytimeout(&self) -> STEADYTIMEOUT_R {
        STEADYTIMEOUT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Wait Duration in HFXO Warm Startup Steady Wait State"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Wait Duration in HFXO Peak Detection Wait State"]
    #[inline(always)]
    pub fn peakdettimeout(&self) -> PEAKDETTIMEOUT_R {
        PEAKDETTIMEOUT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Wait Duration in HFXO Shunt Current Optimization Wait State"]
    #[inline(always)]
    pub fn shuntopttimeout(&self) -> SHUNTOPTTIMEOUT_R {
        SHUNTOPTTIMEOUT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait Duration in HFXO Startup Enable Wait State"]
    #[inline(always)]
    pub fn startuptimeout(&mut self) -> STARTUPTIMEOUT_W {
        STARTUPTIMEOUT_W { w: self }
    }
    #[doc = "Bits 4:7 - Wait Duration in HFXO Startup Steady Wait State"]
    #[inline(always)]
    pub fn steadytimeout(&mut self) -> STEADYTIMEOUT_W {
        STEADYTIMEOUT_W { w: self }
    }
    #[doc = "Bits 8:11 - Wait Duration in HFXO Warm Startup Steady Wait State"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 12:15 - Wait Duration in HFXO Peak Detection Wait State"]
    #[inline(always)]
    pub fn peakdettimeout(&mut self) -> PEAKDETTIMEOUT_W {
        PEAKDETTIMEOUT_W { w: self }
    }
    #[doc = "Bits 16:19 - Wait Duration in HFXO Shunt Current Optimization Wait State"]
    #[inline(always)]
    pub fn shuntopttimeout(&mut self) -> SHUNTOPTTIMEOUT_W {
        SHUNTOPTTIMEOUT_W { w: self }
    }
}
