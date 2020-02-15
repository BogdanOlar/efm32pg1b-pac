#[doc = "Reader of register DMAREQ1"]
pub type R = crate::R<u32, super::DMAREQ1>;
#[doc = "Writer for register DMAREQ1"]
pub type W = crate::W<u32, super::DMAREQ1>;
#[doc = "Register DMAREQ1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAREQ1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA Request 1 PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRSSEL_A {
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
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRSSEL`"]
pub type PRSSEL_R = crate::R<u8, PRSSEL_A>;
impl PRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRSSEL_A::PRSCH0),
            1 => Val(PRSSEL_A::PRSCH1),
            2 => Val(PRSSEL_A::PRSCH2),
            3 => Val(PRSSEL_A::PRSCH3),
            4 => Val(PRSSEL_A::PRSCH4),
            5 => Val(PRSSEL_A::PRSCH5),
            6 => Val(PRSSEL_A::PRSCH6),
            7 => Val(PRSSEL_A::PRSCH7),
            8 => Val(PRSSEL_A::PRSCH8),
            9 => Val(PRSSEL_A::PRSCH9),
            10 => Val(PRSSEL_A::PRSCH10),
            11 => Val(PRSSEL_A::PRSCH11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL_A::PRSCH11
    }
}
#[doc = "Write proxy for field `PRSSEL`"]
pub struct PRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:9 - DMA Request 1 PRS Channel Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:9 - DMA Request 1 PRS Channel Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PRSSEL_W {
        PRSSEL_W { w: self }
    }
}
