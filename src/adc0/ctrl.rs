///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Warm-up Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WARMUPMODE {
    ///0: ADC is shut down after each conversion. 5us warmup time is used before each conversion.
    Normal = 0,
    ///1: ADC is kept in standby mode between conversions. 1us warmup time is used before each conversion.
    Keepinstandby = 1,
    ///2: ADC is kept in slow acquisition mode between conversions. 1us warmup time is used before each conversion.
    Keepinslowacc = 2,
    ///3: ADC is kept on after conversions, allowing for continuous conversion.
    Keepadcwarm = 3,
}
impl From<WARMUPMODE> for u8 {
    #[inline(always)]
    fn from(variant: WARMUPMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WARMUPMODE {
    type Ux = u8;
}
impl crate::IsEnum for WARMUPMODE {}
///Field `WARMUPMODE` reader - Warm-up Mode
pub type WarmupmodeR = crate::FieldReader<WARMUPMODE>;
impl WarmupmodeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WARMUPMODE {
        match self.bits {
            0 => WARMUPMODE::Normal,
            1 => WARMUPMODE::Keepinstandby,
            2 => WARMUPMODE::Keepinslowacc,
            3 => WARMUPMODE::Keepadcwarm,
            _ => unreachable!(),
        }
    }
    ///ADC is shut down after each conversion. 5us warmup time is used before each conversion.
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WARMUPMODE::Normal
    }
    ///ADC is kept in standby mode between conversions. 1us warmup time is used before each conversion.
    #[inline(always)]
    pub fn is_keepinstandby(&self) -> bool {
        *self == WARMUPMODE::Keepinstandby
    }
    ///ADC is kept in slow acquisition mode between conversions. 1us warmup time is used before each conversion.
    #[inline(always)]
    pub fn is_keepinslowacc(&self) -> bool {
        *self == WARMUPMODE::Keepinslowacc
    }
    ///ADC is kept on after conversions, allowing for continuous conversion.
    #[inline(always)]
    pub fn is_keepadcwarm(&self) -> bool {
        *self == WARMUPMODE::Keepadcwarm
    }
}
///Field `WARMUPMODE` writer - Warm-up Mode
pub type WarmupmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, WARMUPMODE, crate::Safe>;
impl<'a, REG> WarmupmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ADC is shut down after each conversion. 5us warmup time is used before each conversion.
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE::Normal)
    }
    ///ADC is kept in standby mode between conversions. 1us warmup time is used before each conversion.
    #[inline(always)]
    pub fn keepinstandby(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE::Keepinstandby)
    }
    ///ADC is kept in slow acquisition mode between conversions. 1us warmup time is used before each conversion.
    #[inline(always)]
    pub fn keepinslowacc(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE::Keepinslowacc)
    }
    ///ADC is kept on after conversions, allowing for continuous conversion.
    #[inline(always)]
    pub fn keepadcwarm(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE::Keepadcwarm)
    }
}
///Field `SINGLEDMAWU` reader - SINGLEFIFO DMA Wakeup
pub type SingledmawuR = crate::BitReader;
///Field `SINGLEDMAWU` writer - SINGLEFIFO DMA Wakeup
pub type SingledmawuW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCANDMAWU` reader - SCANFIFO DMA Wakeup
pub type ScandmawuR = crate::BitReader;
///Field `SCANDMAWU` writer - SCANFIFO DMA Wakeup
pub type ScandmawuW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAILGATE` reader - Conversion Tailgating
pub type TailgateR = crate::BitReader;
///Field `TAILGATE` writer - Conversion Tailgating
pub type TailgateW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASYNCCLKEN` reader - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1
pub type AsyncclkenR = crate::BitReader;
///Field `ASYNCCLKEN` writer - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1
pub type AsyncclkenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCCLKMODE` reader - ADC Clock Mode
pub type AdcclkmodeR = crate::BitReader;
///Field `ADCCLKMODE` writer - ADC Clock Mode
pub type AdcclkmodeW<'a, REG> = crate::BitWriter<'a, REG>;
///Prescalar Setting for ADC Sample and Conversion Clock
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC {
    ///0: `0`
    Nodivision = 0,
}
impl From<PRESC> for u8 {
    #[inline(always)]
    fn from(variant: PRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC {
    type Ux = u8;
}
impl crate::IsEnum for PRESC {}
///Field `PRESC` reader - Prescalar Setting for ADC Sample and Conversion Clock
pub type PrescR = crate::FieldReader<PRESC>;
impl PrescR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESC> {
        match self.bits {
            0 => Some(PRESC::Nodivision),
            _ => None,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC::Nodivision
    }
}
///Field `PRESC` writer - Prescalar Setting for ADC Sample and Conversion Clock
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 7, PRESC>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///`0`
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Nodivision)
    }
}
///Field `TIMEBASE` reader - 1us Time Base
pub type TimebaseR = crate::FieldReader;
///Field `TIMEBASE` writer - 1us Time Base
pub type TimebaseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Oversample Rate Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSRSEL {
    ///0: 2 samples for each conversion result
    X2 = 0,
    ///1: 4 samples for each conversion result
    X4 = 1,
    ///2: 8 samples for each conversion result
    X8 = 2,
    ///3: 16 samples for each conversion result
    X16 = 3,
    ///4: 32 samples for each conversion result
    X32 = 4,
    ///5: 64 samples for each conversion result
    X64 = 5,
    ///6: 128 samples for each conversion result
    X128 = 6,
    ///7: 256 samples for each conversion result
    X256 = 7,
    ///8: 512 samples for each conversion result
    X512 = 8,
    ///9: 1024 samples for each conversion result
    X1024 = 9,
    ///10: 2048 samples for each conversion result
    X2048 = 10,
    ///11: 4096 samples for each conversion result
    X4096 = 11,
}
impl From<OVSRSEL> for u8 {
    #[inline(always)]
    fn from(variant: OVSRSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVSRSEL {
    type Ux = u8;
}
impl crate::IsEnum for OVSRSEL {}
///Field `OVSRSEL` reader - Oversample Rate Select
pub type OvsrselR = crate::FieldReader<OVSRSEL>;
impl OvsrselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OVSRSEL> {
        match self.bits {
            0 => Some(OVSRSEL::X2),
            1 => Some(OVSRSEL::X4),
            2 => Some(OVSRSEL::X8),
            3 => Some(OVSRSEL::X16),
            4 => Some(OVSRSEL::X32),
            5 => Some(OVSRSEL::X64),
            6 => Some(OVSRSEL::X128),
            7 => Some(OVSRSEL::X256),
            8 => Some(OVSRSEL::X512),
            9 => Some(OVSRSEL::X1024),
            10 => Some(OVSRSEL::X2048),
            11 => Some(OVSRSEL::X4096),
            _ => None,
        }
    }
    ///2 samples for each conversion result
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == OVSRSEL::X2
    }
    ///4 samples for each conversion result
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == OVSRSEL::X4
    }
    ///8 samples for each conversion result
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == OVSRSEL::X8
    }
    ///16 samples for each conversion result
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == OVSRSEL::X16
    }
    ///32 samples for each conversion result
    #[inline(always)]
    pub fn is_x32(&self) -> bool {
        *self == OVSRSEL::X32
    }
    ///64 samples for each conversion result
    #[inline(always)]
    pub fn is_x64(&self) -> bool {
        *self == OVSRSEL::X64
    }
    ///128 samples for each conversion result
    #[inline(always)]
    pub fn is_x128(&self) -> bool {
        *self == OVSRSEL::X128
    }
    ///256 samples for each conversion result
    #[inline(always)]
    pub fn is_x256(&self) -> bool {
        *self == OVSRSEL::X256
    }
    ///512 samples for each conversion result
    #[inline(always)]
    pub fn is_x512(&self) -> bool {
        *self == OVSRSEL::X512
    }
    ///1024 samples for each conversion result
    #[inline(always)]
    pub fn is_x1024(&self) -> bool {
        *self == OVSRSEL::X1024
    }
    ///2048 samples for each conversion result
    #[inline(always)]
    pub fn is_x2048(&self) -> bool {
        *self == OVSRSEL::X2048
    }
    ///4096 samples for each conversion result
    #[inline(always)]
    pub fn is_x4096(&self) -> bool {
        *self == OVSRSEL::X4096
    }
}
///Field `OVSRSEL` writer - Oversample Rate Select
pub type OvsrselW<'a, REG> = crate::FieldWriter<'a, REG, 4, OVSRSEL>;
impl<'a, REG> OvsrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2 samples for each conversion result
    #[inline(always)]
    pub fn x2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X2)
    }
    ///4 samples for each conversion result
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X4)
    }
    ///8 samples for each conversion result
    #[inline(always)]
    pub fn x8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X8)
    }
    ///16 samples for each conversion result
    #[inline(always)]
    pub fn x16(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X16)
    }
    ///32 samples for each conversion result
    #[inline(always)]
    pub fn x32(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X32)
    }
    ///64 samples for each conversion result
    #[inline(always)]
    pub fn x64(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X64)
    }
    ///128 samples for each conversion result
    #[inline(always)]
    pub fn x128(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X128)
    }
    ///256 samples for each conversion result
    #[inline(always)]
    pub fn x256(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X256)
    }
    ///512 samples for each conversion result
    #[inline(always)]
    pub fn x512(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X512)
    }
    ///1024 samples for each conversion result
    #[inline(always)]
    pub fn x1024(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X1024)
    }
    ///2048 samples for each conversion result
    #[inline(always)]
    pub fn x2048(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X2048)
    }
    ///4096 samples for each conversion result
    #[inline(always)]
    pub fn x4096(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X4096)
    }
}
///Field `CHCONMODE` reader - Channel Connect
pub type ChconmodeR = crate::BitReader;
///Field `CHCONMODE` writer - Channel Connect
pub type ChconmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Warm-up Mode
    #[inline(always)]
    pub fn warmupmode(&self) -> WarmupmodeR {
        WarmupmodeR::new((self.bits & 3) as u8)
    }
    ///Bit 2 - SINGLEFIFO DMA Wakeup
    #[inline(always)]
    pub fn singledmawu(&self) -> SingledmawuR {
        SingledmawuR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SCANFIFO DMA Wakeup
    #[inline(always)]
    pub fn scandmawu(&self) -> ScandmawuR {
        ScandmawuR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Conversion Tailgating
    #[inline(always)]
    pub fn tailgate(&self) -> TailgateR {
        TailgateR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1
    #[inline(always)]
    pub fn asyncclken(&self) -> AsyncclkenR {
        AsyncclkenR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ADC Clock Mode
    #[inline(always)]
    pub fn adcclkmode(&self) -> AdcclkmodeR {
        AdcclkmodeR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:14 - Prescalar Setting for ADC Sample and Conversion Clock
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 16:22 - 1us Time Base
    #[inline(always)]
    pub fn timebase(&self) -> TimebaseR {
        TimebaseR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:27 - Oversample Rate Select
    #[inline(always)]
    pub fn ovsrsel(&self) -> OvsrselR {
        OvsrselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 29 - Channel Connect
    #[inline(always)]
    pub fn chconmode(&self) -> ChconmodeR {
        ChconmodeR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("warmupmode", &self.warmupmode())
            .field("singledmawu", &self.singledmawu())
            .field("scandmawu", &self.scandmawu())
            .field("tailgate", &self.tailgate())
            .field("asyncclken", &self.asyncclken())
            .field("adcclkmode", &self.adcclkmode())
            .field("presc", &self.presc())
            .field("timebase", &self.timebase())
            .field("ovsrsel", &self.ovsrsel())
            .field("chconmode", &self.chconmode())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Warm-up Mode
    #[inline(always)]
    #[must_use]
    pub fn warmupmode(&mut self) -> WarmupmodeW<CTRLrs> {
        WarmupmodeW::new(self, 0)
    }
    ///Bit 2 - SINGLEFIFO DMA Wakeup
    #[inline(always)]
    #[must_use]
    pub fn singledmawu(&mut self) -> SingledmawuW<CTRLrs> {
        SingledmawuW::new(self, 2)
    }
    ///Bit 3 - SCANFIFO DMA Wakeup
    #[inline(always)]
    #[must_use]
    pub fn scandmawu(&mut self) -> ScandmawuW<CTRLrs> {
        ScandmawuW::new(self, 3)
    }
    ///Bit 4 - Conversion Tailgating
    #[inline(always)]
    #[must_use]
    pub fn tailgate(&mut self) -> TailgateW<CTRLrs> {
        TailgateW::new(self, 4)
    }
    ///Bit 6 - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1
    #[inline(always)]
    #[must_use]
    pub fn asyncclken(&mut self) -> AsyncclkenW<CTRLrs> {
        AsyncclkenW::new(self, 6)
    }
    ///Bit 7 - ADC Clock Mode
    #[inline(always)]
    #[must_use]
    pub fn adcclkmode(&mut self) -> AdcclkmodeW<CTRLrs> {
        AdcclkmodeW::new(self, 7)
    }
    ///Bits 8:14 - Prescalar Setting for ADC Sample and Conversion Clock
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PrescW<CTRLrs> {
        PrescW::new(self, 8)
    }
    ///Bits 16:22 - 1us Time Base
    #[inline(always)]
    #[must_use]
    pub fn timebase(&mut self) -> TimebaseW<CTRLrs> {
        TimebaseW::new(self, 16)
    }
    ///Bits 24:27 - Oversample Rate Select
    #[inline(always)]
    #[must_use]
    pub fn ovsrsel(&mut self) -> OvsrselW<CTRLrs> {
        OvsrselW::new(self, 24)
    }
    ///Bit 29 - Channel Connect
    #[inline(always)]
    #[must_use]
    pub fn chconmode(&mut self) -> ChconmodeW<CTRLrs> {
        ChconmodeW::new(self, 29)
    }
}
///Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CTRLrs {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL to value 0x001f_0000
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0x001f_0000;
}
