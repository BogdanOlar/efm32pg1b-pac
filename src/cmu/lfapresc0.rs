#[doc = "Reader of register LFAPRESC0"]
pub type R = crate::R<u32, super::LFAPRESC0>;
#[doc = "Writer for register LFAPRESC0"]
pub type W = crate::W<u32, super::LFAPRESC0>;
#[doc = "Register LFAPRESC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFAPRESC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Low Energy Timer 0 Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LETIMER0_A {
    #[doc = "0: LFACLKLETIMER0 = LFACLK"]
    DIV1 = 0,
    #[doc = "1: LFACLKLETIMER0 = LFACLK/2"]
    DIV2 = 1,
    #[doc = "2: LFACLKLETIMER0 = LFACLK/4"]
    DIV4 = 2,
    #[doc = "3: LFACLKLETIMER0 = LFACLK/8"]
    DIV8 = 3,
    #[doc = "4: LFACLKLETIMER0 = LFACLK/16"]
    DIV16 = 4,
    #[doc = "5: LFACLKLETIMER0 = LFACLK/32"]
    DIV32 = 5,
    #[doc = "6: LFACLKLETIMER0 = LFACLK/64"]
    DIV64 = 6,
    #[doc = "7: LFACLKLETIMER0 = LFACLK/128"]
    DIV128 = 7,
    #[doc = "8: LFACLKLETIMER0 = LFACLK/256"]
    DIV256 = 8,
    #[doc = "9: LFACLKLETIMER0 = LFACLK/512"]
    DIV512 = 9,
    #[doc = "10: LFACLKLETIMER0 = LFACLK/1024"]
    DIV1024 = 10,
    #[doc = "11: LFACLKLETIMER0 = LFACLK/2048"]
    DIV2048 = 11,
    #[doc = "12: LFACLKLETIMER0 = LFACLK/4096"]
    DIV4096 = 12,
    #[doc = "13: LFACLKLETIMER0 = LFACLK/8192"]
    DIV8192 = 13,
    #[doc = "14: LFACLKLETIMER0 = LFACLK/16384"]
    DIV16384 = 14,
    #[doc = "15: LFACLKLETIMER0 = LFACLK/32768"]
    DIV32768 = 15,
}
impl From<LETIMER0_A> for u8 {
    #[inline(always)]
    fn from(variant: LETIMER0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LETIMER0`"]
pub type LETIMER0_R = crate::R<u8, LETIMER0_A>;
impl LETIMER0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LETIMER0_A {
        match self.bits {
            0 => LETIMER0_A::DIV1,
            1 => LETIMER0_A::DIV2,
            2 => LETIMER0_A::DIV4,
            3 => LETIMER0_A::DIV8,
            4 => LETIMER0_A::DIV16,
            5 => LETIMER0_A::DIV32,
            6 => LETIMER0_A::DIV64,
            7 => LETIMER0_A::DIV128,
            8 => LETIMER0_A::DIV256,
            9 => LETIMER0_A::DIV512,
            10 => LETIMER0_A::DIV1024,
            11 => LETIMER0_A::DIV2048,
            12 => LETIMER0_A::DIV4096,
            13 => LETIMER0_A::DIV8192,
            14 => LETIMER0_A::DIV16384,
            15 => LETIMER0_A::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LETIMER0_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LETIMER0_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LETIMER0_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LETIMER0_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == LETIMER0_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == LETIMER0_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == LETIMER0_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LETIMER0_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == LETIMER0_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == LETIMER0_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == LETIMER0_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == LETIMER0_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == LETIMER0_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == LETIMER0_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == LETIMER0_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == LETIMER0_A::DIV32768
    }
}
#[doc = "Write proxy for field `LETIMER0`"]
pub struct LETIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> LETIMER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LETIMER0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LFACLKLETIMER0 = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV1)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV2)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV4)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV8)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV16)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV32)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV64)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV128)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV256)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV512)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV1024)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV2048)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV4096)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV8192)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV16384)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV32768)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Low Energy Timer 0 Prescaler"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Low Energy Timer 0 Prescaler"]
    #[inline(always)]
    pub fn letimer0(&mut self) -> LETIMER0_W {
        LETIMER0_W { w: self }
    }
}
