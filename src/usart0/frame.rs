///Register `FRAME` reader
pub type R = crate::R<FRAMErs>;
///Register `FRAME` writer
pub type W = crate::W<FRAMErs>;
///Data-Bit Mode
///
///Value on reset: 5
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATABITS {
    ///1: Each frame contains 4 data bits
    Four = 1,
    ///2: Each frame contains 5 data bits
    Five = 2,
    ///3: Each frame contains 6 data bits
    Six = 3,
    ///4: Each frame contains 7 data bits
    Seven = 4,
    ///5: Each frame contains 8 data bits
    Eight = 5,
    ///6: Each frame contains 9 data bits
    Nine = 6,
    ///7: Each frame contains 10 data bits
    Ten = 7,
    ///8: Each frame contains 11 data bits
    Eleven = 8,
    ///9: Each frame contains 12 data bits
    Twelve = 9,
    ///10: Each frame contains 13 data bits
    Thirteen = 10,
    ///11: Each frame contains 14 data bits
    Fourteen = 11,
    ///12: Each frame contains 15 data bits
    Fifteen = 12,
    ///13: Each frame contains 16 data bits
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
impl crate::IsEnum for DATABITS {}
///Field `DATABITS` reader - Data-Bit Mode
pub type DatabitsR = crate::FieldReader<DATABITS>;
impl DatabitsR {
    ///Get enumerated values variant
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
    ///Each frame contains 4 data bits
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == DATABITS::Four
    }
    ///Each frame contains 5 data bits
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == DATABITS::Five
    }
    ///Each frame contains 6 data bits
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == DATABITS::Six
    }
    ///Each frame contains 7 data bits
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == DATABITS::Seven
    }
    ///Each frame contains 8 data bits
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == DATABITS::Eight
    }
    ///Each frame contains 9 data bits
    #[inline(always)]
    pub fn is_nine(&self) -> bool {
        *self == DATABITS::Nine
    }
    ///Each frame contains 10 data bits
    #[inline(always)]
    pub fn is_ten(&self) -> bool {
        *self == DATABITS::Ten
    }
    ///Each frame contains 11 data bits
    #[inline(always)]
    pub fn is_eleven(&self) -> bool {
        *self == DATABITS::Eleven
    }
    ///Each frame contains 12 data bits
    #[inline(always)]
    pub fn is_twelve(&self) -> bool {
        *self == DATABITS::Twelve
    }
    ///Each frame contains 13 data bits
    #[inline(always)]
    pub fn is_thirteen(&self) -> bool {
        *self == DATABITS::Thirteen
    }
    ///Each frame contains 14 data bits
    #[inline(always)]
    pub fn is_fourteen(&self) -> bool {
        *self == DATABITS::Fourteen
    }
    ///Each frame contains 15 data bits
    #[inline(always)]
    pub fn is_fifteen(&self) -> bool {
        *self == DATABITS::Fifteen
    }
    ///Each frame contains 16 data bits
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == DATABITS::Sixteen
    }
}
///Field `DATABITS` writer - Data-Bit Mode
pub type DatabitsW<'a, REG> = crate::FieldWriter<'a, REG, 4, DATABITS>;
impl<'a, REG> DatabitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Each frame contains 4 data bits
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Four)
    }
    ///Each frame contains 5 data bits
    #[inline(always)]
    pub fn five(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Five)
    }
    ///Each frame contains 6 data bits
    #[inline(always)]
    pub fn six(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Six)
    }
    ///Each frame contains 7 data bits
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Seven)
    }
    ///Each frame contains 8 data bits
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Eight)
    }
    ///Each frame contains 9 data bits
    #[inline(always)]
    pub fn nine(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Nine)
    }
    ///Each frame contains 10 data bits
    #[inline(always)]
    pub fn ten(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Ten)
    }
    ///Each frame contains 11 data bits
    #[inline(always)]
    pub fn eleven(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Eleven)
    }
    ///Each frame contains 12 data bits
    #[inline(always)]
    pub fn twelve(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Twelve)
    }
    ///Each frame contains 13 data bits
    #[inline(always)]
    pub fn thirteen(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Thirteen)
    }
    ///Each frame contains 14 data bits
    #[inline(always)]
    pub fn fourteen(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Fourteen)
    }
    ///Each frame contains 15 data bits
    #[inline(always)]
    pub fn fifteen(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Fifteen)
    }
    ///Each frame contains 16 data bits
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS::Sixteen)
    }
}
///Parity-Bit Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PARITY {
    ///0: Parity bits are not used
    None = 0,
    ///2: Even parity are used. Parity bits are automatically generated and checked by hardware.
    Even = 2,
    ///3: Odd parity is used. Parity bits are automatically generated and checked by hardware.
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
impl crate::IsEnum for PARITY {}
///Field `PARITY` reader - Parity-Bit Mode
pub type ParityR = crate::FieldReader<PARITY>;
impl ParityR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PARITY> {
        match self.bits {
            0 => Some(PARITY::None),
            2 => Some(PARITY::Even),
            3 => Some(PARITY::Odd),
            _ => None,
        }
    }
    ///Parity bits are not used
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PARITY::None
    }
    ///Even parity are used. Parity bits are automatically generated and checked by hardware.
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARITY::Even
    }
    ///Odd parity is used. Parity bits are automatically generated and checked by hardware.
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARITY::Odd
    }
}
///Field `PARITY` writer - Parity-Bit Mode
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 2, PARITY>;
impl<'a, REG> ParityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Parity bits are not used
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY::None)
    }
    ///Even parity are used. Parity bits are automatically generated and checked by hardware.
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY::Even)
    }
    ///Odd parity is used. Parity bits are automatically generated and checked by hardware.
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY::Odd)
    }
}
///Stop-Bit Mode
///
///Value on reset: 1
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOPBITS {
    ///0: The transmitter generates a half stop bit. Stop-bits are not verified by receiver
    Half = 0,
    ///1: One stop bit is generated and verified
    One = 1,
    ///2: The transmitter generates one and a half stop bit. The receiver verifies the first stop bit
    Oneandahalf = 2,
    ///3: The transmitter generates two stop bits. The receiver checks the first stop-bit only
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
impl crate::IsEnum for STOPBITS {}
///Field `STOPBITS` reader - Stop-Bit Mode
pub type StopbitsR = crate::FieldReader<STOPBITS>;
impl StopbitsR {
    ///Get enumerated values variant
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
    ///The transmitter generates a half stop bit. Stop-bits are not verified by receiver
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == STOPBITS::Half
    }
    ///One stop bit is generated and verified
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == STOPBITS::One
    }
    ///The transmitter generates one and a half stop bit. The receiver verifies the first stop bit
    #[inline(always)]
    pub fn is_oneandahalf(&self) -> bool {
        *self == STOPBITS::Oneandahalf
    }
    ///The transmitter generates two stop bits. The receiver checks the first stop-bit only
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == STOPBITS::Two
    }
}
///Field `STOPBITS` writer - Stop-Bit Mode
pub type StopbitsW<'a, REG> = crate::FieldWriter<'a, REG, 2, STOPBITS, crate::Safe>;
impl<'a, REG> StopbitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The transmitter generates a half stop bit. Stop-bits are not verified by receiver
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBITS::Half)
    }
    ///One stop bit is generated and verified
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBITS::One)
    }
    ///The transmitter generates one and a half stop bit. The receiver verifies the first stop bit
    #[inline(always)]
    pub fn oneandahalf(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBITS::Oneandahalf)
    }
    ///The transmitter generates two stop bits. The receiver checks the first stop-bit only
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBITS::Two)
    }
}
impl R {
    ///Bits 0:3 - Data-Bit Mode
    #[inline(always)]
    pub fn databits(&self) -> DatabitsR {
        DatabitsR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:9 - Parity-Bit Mode
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Stop-Bit Mode
    #[inline(always)]
    pub fn stopbits(&self) -> StopbitsR {
        StopbitsR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAME")
            .field("databits", &self.databits())
            .field("parity", &self.parity())
            .field("stopbits", &self.stopbits())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Data-Bit Mode
    #[inline(always)]
    #[must_use]
    pub fn databits(&mut self) -> DatabitsW<FRAMErs> {
        DatabitsW::new(self, 0)
    }
    ///Bits 8:9 - Parity-Bit Mode
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<FRAMErs> {
        ParityW::new(self, 8)
    }
    ///Bits 12:13 - Stop-Bit Mode
    #[inline(always)]
    #[must_use]
    pub fn stopbits(&mut self) -> StopbitsW<FRAMErs> {
        StopbitsW::new(self, 12)
    }
}
///USART Frame Format Register
///
///You can [`read`](crate::Reg::read) this register and get [`frame::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FRAMErs;
impl crate::RegisterSpec for FRAMErs {
    type Ux = u32;
}
///`read()` method returns [`frame::R`](R) reader structure
impl crate::Readable for FRAMErs {}
///`write(|w| ..)` method takes [`frame::W`](W) writer structure
impl crate::Writable for FRAMErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FRAME to value 0x1005
impl crate::Resettable for FRAMErs {
    const RESET_VALUE: u32 = 0x1005;
}
