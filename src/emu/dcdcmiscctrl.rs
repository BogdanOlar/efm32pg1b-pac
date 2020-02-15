#[doc = "Reader of register DCDCMISCCTRL"]
pub type R = crate::R<u32, super::DCDCMISCCTRL>;
#[doc = "Writer for register DCDCMISCCTRL"]
pub type W = crate::W<u32, super::DCDCMISCCTRL>;
#[doc = "Register DCDCMISCCTRL `reset()`'s with value 0x3330_7700"]
impl crate::ResetValue for super::DCDCMISCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3330_7700
    }
}
#[doc = "Reader of field `LNFORCECCM`"]
pub type LNFORCECCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LNFORCECCM`"]
pub struct LNFORCECCM_W<'a> {
    w: &'a mut W,
}
impl<'a> LNFORCECCM_W<'a> {
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
#[doc = "Reader of field `PFETCNT`"]
pub type PFETCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PFETCNT`"]
pub struct PFETCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PFETCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `NFETCNT`"]
pub type NFETCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NFETCNT`"]
pub struct NFETCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> NFETCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `BYPLIMSEL`"]
pub type BYPLIMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYPLIMSEL`"]
pub struct BYPLIMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPLIMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `LPCLIMILIMSEL`"]
pub type LPCLIMILIMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPCLIMILIMSEL`"]
pub struct LPCLIMILIMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCLIMILIMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `LNCLIMILIMSEL`"]
pub type LNCLIMILIMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LNCLIMILIMSEL`"]
pub struct LNCLIMILIMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LNCLIMILIMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "LP Mode Comparator Bias Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPCMPBIAS_A {
    #[doc = "0: Maximum load current less than 75uA."]
    BIAS0 = 0,
    #[doc = "1: Maximum load current less than 500uA."]
    BIAS1 = 1,
    #[doc = "2: Maximum load current less than 2.5mA."]
    BIAS2 = 2,
    #[doc = "3: Maximum load current less than 10mA."]
    BIAS3 = 3,
}
impl From<LPCMPBIAS_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCMPBIAS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPCMPBIAS`"]
pub type LPCMPBIAS_R = crate::R<u8, LPCMPBIAS_A>;
impl LPCMPBIAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCMPBIAS_A {
        match self.bits {
            0 => LPCMPBIAS_A::BIAS0,
            1 => LPCMPBIAS_A::BIAS1,
            2 => LPCMPBIAS_A::BIAS2,
            3 => LPCMPBIAS_A::BIAS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS0`"]
    #[inline(always)]
    pub fn is_bias0(&self) -> bool {
        *self == LPCMPBIAS_A::BIAS0
    }
    #[doc = "Checks if the value of the field is `BIAS1`"]
    #[inline(always)]
    pub fn is_bias1(&self) -> bool {
        *self == LPCMPBIAS_A::BIAS1
    }
    #[doc = "Checks if the value of the field is `BIAS2`"]
    #[inline(always)]
    pub fn is_bias2(&self) -> bool {
        *self == LPCMPBIAS_A::BIAS2
    }
    #[doc = "Checks if the value of the field is `BIAS3`"]
    #[inline(always)]
    pub fn is_bias3(&self) -> bool {
        *self == LPCMPBIAS_A::BIAS3
    }
}
#[doc = "Write proxy for field `LPCMPBIAS`"]
pub struct LPCMPBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCMPBIAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPCMPBIAS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn bias0(self) -> &'a mut W {
        self.variant(LPCMPBIAS_A::BIAS0)
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn bias1(self) -> &'a mut W {
        self.variant(LPCMPBIAS_A::BIAS1)
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn bias2(self) -> &'a mut W {
        self.variant(LPCMPBIAS_A::BIAS2)
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn bias3(self) -> &'a mut W {
        self.variant(LPCMPBIAS_A::BIAS3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline(always)]
    pub fn lnforceccm(&self) -> LNFORCECCM_R {
        LNFORCECCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    pub fn pfetcnt(&self) -> PFETCNT_R {
        PFETCNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    pub fn nfetcnt(&self) -> NFETCNT_R {
        NFETCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    pub fn byplimsel(&self) -> BYPLIMSEL_R {
        BYPLIMSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    pub fn lpclimilimsel(&self) -> LPCLIMILIMSEL_R {
        LPCLIMILIMSEL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    pub fn lnclimilimsel(&self) -> LNCLIMILIMSEL_R {
        LNCLIMILIMSEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection"]
    #[inline(always)]
    pub fn lpcmpbias(&self) -> LPCMPBIAS_R {
        LPCMPBIAS_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline(always)]
    pub fn lnforceccm(&mut self) -> LNFORCECCM_W {
        LNFORCECCM_W { w: self }
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    pub fn pfetcnt(&mut self) -> PFETCNT_W {
        PFETCNT_W { w: self }
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    pub fn nfetcnt(&mut self) -> NFETCNT_W {
        NFETCNT_W { w: self }
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    pub fn byplimsel(&mut self) -> BYPLIMSEL_W {
        BYPLIMSEL_W { w: self }
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    pub fn lpclimilimsel(&mut self) -> LPCLIMILIMSEL_W {
        LPCLIMILIMSEL_W { w: self }
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    pub fn lnclimilimsel(&mut self) -> LNCLIMILIMSEL_W {
        LNCLIMILIMSEL_W { w: self }
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection"]
    #[inline(always)]
    pub fn lpcmpbias(&mut self) -> LPCMPBIAS_W {
        LPCMPBIAS_W { w: self }
    }
}
