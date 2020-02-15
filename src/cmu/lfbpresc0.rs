#[doc = "Reader of register LFBPRESC0"]
pub type R = crate::R<u32, super::LFBPRESC0>;
#[doc = "Writer for register LFBPRESC0"]
pub type W = crate::W<u32, super::LFBPRESC0>;
#[doc = "Register LFBPRESC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFBPRESC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Low Energy UART 0 Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEUART0_A {
    #[doc = "0: LFBCLKLEUART0 = LFBCLK"]
    DIV1 = 0,
    #[doc = "1: LFBCLKLEUART0 = LFBCLK/2"]
    DIV2 = 1,
    #[doc = "2: LFBCLKLEUART0 = LFBCLK/4"]
    DIV4 = 2,
    #[doc = "3: LFBCLKLEUART0 = LFBCLK/8"]
    DIV8 = 3,
}
impl From<LEUART0_A> for u8 {
    #[inline(always)]
    fn from(variant: LEUART0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEUART0`"]
pub type LEUART0_R = crate::R<u8, LEUART0_A>;
impl LEUART0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEUART0_A {
        match self.bits {
            0 => LEUART0_A::DIV1,
            1 => LEUART0_A::DIV2,
            2 => LEUART0_A::DIV4,
            3 => LEUART0_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LEUART0_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LEUART0_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LEUART0_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LEUART0_A::DIV8
    }
}
#[doc = "Write proxy for field `LEUART0`"]
pub struct LEUART0_W<'a> {
    w: &'a mut W,
}
impl<'a> LEUART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEUART0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LEUART0_A::DIV1)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LEUART0_A::DIV2)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LEUART0_A::DIV4)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LEUART0_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Low Energy UART 0 Prescaler"]
    #[inline(always)]
    pub fn leuart0(&self) -> LEUART0_R {
        LEUART0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low Energy UART 0 Prescaler"]
    #[inline(always)]
    pub fn leuart0(&mut self) -> LEUART0_W {
        LEUART0_W { w: self }
    }
}
