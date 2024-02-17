#[doc = "Register `FRAME` reader"]
pub type R = crate::R<FRAMErs>;
#[doc = "Register `FRAME` writer"]
pub type W = crate::W<FRAMErs>;
#[doc = "Field `DATABITS` reader - Data-Bit Mode"]
pub type DATABITS_R = crate::FieldReader<DATABITS>;
#[doc = "Data-Bit Mode\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATABITS {
    #[doc = "1: Each frame contains 4 data bits"]
    Four = 1,
    #[doc = "2: Each frame contains 5 data bits"]
    Five = 2,
    #[doc = "3: Each frame contains 6 data bits"]
    Six = 3,
    #[doc = "4: Each frame contains 7 data bits"]
    Seven = 4,
    #[doc = "5: Each frame contains 8 data bits"]
    Eight = 5,
    #[doc = "6: Each frame contains 9 data bits"]
    Nine = 6,
    #[doc = "7: Each frame contains 10 data bits"]
    Ten = 7,
    #[doc = "8: Each frame contains 11 data bits"]
    Eleven = 8,
    #[doc = "9: Each frame contains 12 data bits"]
    Twelve = 9,
    #[doc = "10: Each frame contains 13 data bits"]
    Thirteen = 10,
    #[doc = "11: Each frame contains 14 data bits"]
    Fourteen = 11,
    #[doc = "12: Each frame contains 15 data bits"]
    Fifteen = 12,
    #[doc = "13: Each frame contains 16 data bits"]
    Sixteen = 13,
}
impl From<DATABITS> for u8 {
    #[inline(always)]
    fn from(variant: DATABITS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATABITS {
    type Ux = u8;
}
impl DATABITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATABITS> {
        match self.bits {
            1 => Some(DATABITS::Four),
            2 => Some(DATABITS::Five),
            3 => Some(DATABITS::Six),
            4 => Some(DATABITS::Seven),
            5 => Some(DATABITS::Eight),
            6 => Some(DATABITS::Nine),
            7 => Some(DATABITS::Ten),
            8 => Some(DATABITS::Eleven),
            9 => Some(DATABITS::Twelve),
            10 => Some(DATABITS::Thirteen),
            11 => Some(DATABITS::Fourteen),
            12 => Some(DATABITS::Fifteen),
            13 => Some(DATABITS::Sixteen),
            _ => None,
        }
    }
    #[doc = "Each frame contains 4 data bits"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == DATABITS::Four
    }
    #[doc = "Each frame contains 5 data bits"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == DATABITS::Five
    }
    #[doc = "Each frame contains 6 data bits"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == DATABITS::Six
    }
    #[doc = "Each frame contains 7 data bits"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == DATABITS::Seven
    }
    #[doc = "Each frame contains 8 data bits"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == DATABITS::Eight
    }
    #[doc = "Each frame contains 9 data bits"]
    #[inline(always)]
    pub fn is_nine(&self) -> bool {
        *self == DATABITS::Nine
    }
    #[doc = "Each frame contains 10 data bits"]
    #[inline(always)]
    pub fn is_ten(&self) -> bool {
        *self == DATABITS::Ten
    }
    #[doc = "Each frame contains 11 data bits"]
    #[inline(always)]
    pub fn is_eleven(&self) -> bool {
        *self == DATABITS::Eleven
    }
    #[doc = "Each frame contains 12 data bits"]
    #[inline(always)]
    pub fn is_twelve(&self) -> bool {
        *self == DATABITS::Twelve
    }
    #[doc = "Each frame contains 13 data bits"]
    #[inline(always)]
    pub fn is_thirteen(&self) -> bool {
        *self == DATABITS::Thirteen
    }
    #[doc = "Each frame contains 14 data bits"]
    #[inline(always)]
    pub fn is_fourteen(&self) -> bool {
        *self == DATABITS::Fourteen
    }
    #[doc = "Each frame contains 15 data bits"]
    #[inline(always)]
    pub fn is_fifteen(&self) -> bool {
        *self == DATABITS::Fifteen
    }
    #[doc = "Each frame contains 16 data bits"]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == DATABITS::Sixteen
    }
}
#[doc = "Field `DATABITS` writer - Data-Bit Mode"]
pub type DATABITS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DATABITS>;
impl<'a, REG> DATABITS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Each frame contains 4 data bits"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Four)
    }
    #[doc = "Each frame contains 5 data bits"]
    #[inline(always)]
    pub fn five(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Five)
    }
    #[doc = "Each frame contains 6 data bits"]
    #[inline(always)]
    pub fn six(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Six)
    }
    #[doc = "Each frame contains 7 data bits"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Seven)
    }
    #[doc = "Each frame contains 8 data bits"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Eight)
    }
    #[doc = "Each frame contains 9 data bits"]
    #[inline(always)]
    pub fn nine(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Nine)
    }
    #[doc = "Each frame contains 10 data bits"]
    #[inline(always)]
    pub fn ten(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Ten)
    }
    #[doc = "Each frame contains 11 data bits"]
    #[inline(always)]
    pub fn eleven(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Eleven)
    }
    #[doc = "Each frame contains 12 data bits"]
    #[inline(always)]
    pub fn twelve(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Twelve)
    }
    #[doc = "Each frame contains 13 data bits"]
    #[inline(always)]
    pub fn thirteen(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Thirteen)
    }
    #[doc = "Each frame contains 14 data bits"]
    #[inline(always)]
    pub fn fourteen(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Fourteen)
    }
    #[doc = "Each frame contains 15 data bits"]
    #[inline(always)]
    pub fn fifteen(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Fifteen)
    }
    #[doc = "Each frame contains 16 data bits"]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Sixteen)
    }
}
#[doc = "Field `PARITY` reader - Parity-Bit Mode"]
pub type PARITY_R = crate::FieldReader<PARITY>;
#[doc = "Parity-Bit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PARITY {
    #[doc = "0: Parity bits are not used"]
    None = 0,
    #[doc = "2: Even parity are used. Parity bits are automatically generated and checked by hardware."]
    Even = 2,
    #[doc = "3: Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    Odd = 3,
}
impl From<PARITY> for u8 {
    #[inline(always)]
    fn from(variant: PARITY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PARITY {
    type Ux = u8;
}
impl PARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PARITY> {
        match self.bits {
            0 => Some(PARITY::None),
            2 => Some(PARITY::Even),
            3 => Some(PARITY::Odd),
            _ => None,
        }
    }
    #[doc = "Parity bits are not used"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PARITY::None
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARITY::Even
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARITY::Odd
    }
}
#[doc = "Field `PARITY` writer - Parity-Bit Mode"]
pub type PARITY_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PARITY>;
impl<'a, REG> PARITY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Parity bits are not used"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY::None)
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY::Even)
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY::Odd)
    }
}
#[doc = "Field `STOPBITS` reader - Stop-Bit Mode"]
pub type STOPBITS_R = crate::FieldReader<STOPBITS>;
#[doc = "Stop-Bit Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOPBITS {
    #[doc = "0: The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    Half = 0,
    #[doc = "1: One stop bit is generated and verified"]
    One = 1,
    #[doc = "2: The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    Oneandahalf = 2,
    #[doc = "3: The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    Two = 3,
}
impl From<STOPBITS> for u8 {
    #[inline(always)]
    fn from(variant: STOPBITS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STOPBITS {
    type Ux = u8;
}
impl STOPBITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPBITS {
        match self.bits {
            0 => STOPBITS::Half,
            1 => STOPBITS::One,
            2 => STOPBITS::Oneandahalf,
            3 => STOPBITS::Two,
            _ => unreachable!(),
        }
    }
    #[doc = "The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == STOPBITS::Half
    }
    #[doc = "One stop bit is generated and verified"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == STOPBITS::One
    }
    #[doc = "The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    #[inline(always)]
    pub fn is_oneandahalf(&self) -> bool {
        *self == STOPBITS::Oneandahalf
    }
    #[doc = "The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == STOPBITS::Two
    }
}
#[doc = "Field `STOPBITS` writer - Stop-Bit Mode"]
pub type STOPBITS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, STOPBITS>;
impl<'a, REG> STOPBITS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBITS::Half)
    }
    #[doc = "One stop bit is generated and verified"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBITS::One)
    }
    #[doc = "The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    #[inline(always)]
    pub fn oneandahalf(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBITS::Oneandahalf)
    }
    #[doc = "The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBITS::Two)
    }
}
impl R {
    #[doc = "Bits 0:3 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&self) -> DATABITS_R {
        DATABITS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&self) -> STOPBITS_R {
        STOPBITS_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn databits(&mut self) -> DATABITS_W<FRAMErs> {
        DATABITS_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Parity-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<FRAMErs> {
        PARITY_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Stop-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn stopbits(&mut self) -> STOPBITS_W<FRAMErs> {
        STOPBITS_W::new(self, 12)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USART Frame Format Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frame::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frame::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRAMErs;
impl crate::RegisterSpec for FRAMErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame::R`](R) reader structure"]
impl crate::Readable for FRAMErs {}
#[doc = "`write(|w| ..)` method takes [`frame::W`](W) writer structure"]
impl crate::Writable for FRAMErs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRAME to value 0x1005"]
impl crate::Resettable for FRAMErs {
    const RESET_VALUE: u32 = 0x1005;
}
