#[doc = "Reader of register ULFRCOCTRL"]
pub type R = crate::R<u32, super::ULFRCOCTRL>;
#[doc = "Writer for register ULFRCOCTRL"]
pub type W = crate::W<u32, super::ULFRCOCTRL>;
#[doc = "Register ULFRCOCTRL `reset()`'s with value 0x0002_0020"]
impl crate::ResetValue for super::ULFRCOCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_0020
    }
}
#[doc = "Reader of field `TUNING`"]
pub type TUNING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TUNING`"]
pub struct TUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "ULFRCO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: ULFRCO = 1 kHz"]
    _1KHZ = 0,
    #[doc = "1: ULFRCO = 2 kHz"]
    _2KHZ = 1,
    #[doc = "2: ULFRCO = 4 kHz"]
    _4KHZ = 2,
    #[doc = "3: ULFRCO = 32 kHz"]
    _32KHZ = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::_1KHZ,
            1 => MODE_A::_2KHZ,
            2 => MODE_A::_4KHZ,
            3 => MODE_A::_32KHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1KHZ`"]
    #[inline(always)]
    pub fn is_1khz(&self) -> bool {
        *self == MODE_A::_1KHZ
    }
    #[doc = "Checks if the value of the field is `_2KHZ`"]
    #[inline(always)]
    pub fn is_2khz(&self) -> bool {
        *self == MODE_A::_2KHZ
    }
    #[doc = "Checks if the value of the field is `_4KHZ`"]
    #[inline(always)]
    pub fn is_4khz(&self) -> bool {
        *self == MODE_A::_4KHZ
    }
    #[doc = "Checks if the value of the field is `_32KHZ`"]
    #[inline(always)]
    pub fn is_32khz(&self) -> bool {
        *self == MODE_A::_32KHZ
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ULFRCO = 1 kHz"]
    #[inline(always)]
    pub fn _1khz(self) -> &'a mut W {
        self.variant(MODE_A::_1KHZ)
    }
    #[doc = "ULFRCO = 2 kHz"]
    #[inline(always)]
    pub fn _2khz(self) -> &'a mut W {
        self.variant(MODE_A::_2KHZ)
    }
    #[doc = "ULFRCO = 4 kHz"]
    #[inline(always)]
    pub fn _4khz(self) -> &'a mut W {
        self.variant(MODE_A::_4KHZ)
    }
    #[doc = "ULFRCO = 32 kHz"]
    #[inline(always)]
    pub fn _32khz(self) -> &'a mut W {
        self.variant(MODE_A::_32KHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `RESTRIM`"]
pub type RESTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESTRIM`"]
pub struct RESTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - ULFRCO TUNING Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 10:11 - ULFRCO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
    #[inline(always)]
    pub fn restrim(&self) -> RESTRIM_R {
        RESTRIM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ULFRCO TUNING Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W { w: self }
    }
    #[doc = "Bits 10:11 - ULFRCO Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 16:17 - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
    #[inline(always)]
    pub fn restrim(&mut self) -> RESTRIM_W {
        RESTRIM_W { w: self }
    }
}
