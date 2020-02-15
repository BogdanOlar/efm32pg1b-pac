#[doc = "Reader of register INPUT"]
pub type R = crate::R<u32, super::INPUT>;
#[doc = "Writer for register INPUT"]
pub type W = crate::W<u32, super::INPUT>;
#[doc = "Register INPUT `reset()`'s with value 0"]
impl crate::ResetValue for super::INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RX PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected"]
    PRSCH11 = 11,
}
impl From<RXPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXPRSSEL`"]
pub type RXPRSSEL_R = crate::R<u8, RXPRSSEL_A>;
impl RXPRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXPRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXPRSSEL_A::PRSCH0),
            1 => Val(RXPRSSEL_A::PRSCH1),
            2 => Val(RXPRSSEL_A::PRSCH2),
            3 => Val(RXPRSSEL_A::PRSCH3),
            4 => Val(RXPRSSEL_A::PRSCH4),
            5 => Val(RXPRSSEL_A::PRSCH5),
            6 => Val(RXPRSSEL_A::PRSCH6),
            7 => Val(RXPRSSEL_A::PRSCH7),
            8 => Val(RXPRSSEL_A::PRSCH8),
            9 => Val(RXPRSSEL_A::PRSCH9),
            10 => Val(RXPRSSEL_A::PRSCH10),
            11 => Val(RXPRSSEL_A::PRSCH11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH11
    }
}
#[doc = "Write proxy for field `RXPRSSEL`"]
pub struct RXPRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPRSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RXPRS`"]
pub type RXPRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXPRS`"]
pub struct RXPRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPRS_W<'a> {
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
#[doc = "CLK PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected"]
    PRSCH11 = 11,
}
impl From<CLKPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKPRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKPRSSEL`"]
pub type CLKPRSSEL_R = crate::R<u8, CLKPRSSEL_A>;
impl CLKPRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKPRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKPRSSEL_A::PRSCH0),
            1 => Val(CLKPRSSEL_A::PRSCH1),
            2 => Val(CLKPRSSEL_A::PRSCH2),
            3 => Val(CLKPRSSEL_A::PRSCH3),
            4 => Val(CLKPRSSEL_A::PRSCH4),
            5 => Val(CLKPRSSEL_A::PRSCH5),
            6 => Val(CLKPRSSEL_A::PRSCH6),
            7 => Val(CLKPRSSEL_A::PRSCH7),
            8 => Val(CLKPRSSEL_A::PRSCH8),
            9 => Val(CLKPRSSEL_A::PRSCH9),
            10 => Val(CLKPRSSEL_A::PRSCH10),
            11 => Val(CLKPRSSEL_A::PRSCH11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH11
    }
}
#[doc = "Write proxy for field `CLKPRSSEL`"]
pub struct CLKPRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKPRSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLKPRS`"]
pub type CLKPRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKPRS`"]
pub struct CLKPRS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPRS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - RX PRS Channel Select"]
    #[inline(always)]
    pub fn rxprssel(&self) -> RXPRSSEL_R {
        RXPRSSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprs(&self) -> RXPRS_R {
        RXPRS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - CLK PRS Channel Select"]
    #[inline(always)]
    pub fn clkprssel(&self) -> CLKPRSSEL_R {
        CLKPRSSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PRS CLK Enable"]
    #[inline(always)]
    pub fn clkprs(&self) -> CLKPRS_R {
        CLKPRS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - RX PRS Channel Select"]
    #[inline(always)]
    pub fn rxprssel(&mut self) -> RXPRSSEL_W {
        RXPRSSEL_W { w: self }
    }
    #[doc = "Bit 7 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprs(&mut self) -> RXPRS_W {
        RXPRS_W { w: self }
    }
    #[doc = "Bits 8:11 - CLK PRS Channel Select"]
    #[inline(always)]
    pub fn clkprssel(&mut self) -> CLKPRSSEL_W {
        CLKPRSSEL_W { w: self }
    }
    #[doc = "Bit 15 - PRS CLK Enable"]
    #[inline(always)]
    pub fn clkprs(&mut self) -> CLKPRS_W {
        CLKPRS_W { w: self }
    }
}
