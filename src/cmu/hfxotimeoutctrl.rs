///Register `HFXOTIMEOUTCTRL` reader
pub type R = crate::R<HFXOTIMEOUTCTRLrs>;
///Register `HFXOTIMEOUTCTRL` writer
pub type W = crate::W<HFXOTIMEOUTCTRLrs>;
///Wait Duration in HFXO Startup Enable Wait State
///
///Value on reset: 7
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STARTUPTIMEOUT {
    ///0: Timeout period of 2 cycles
    _2cycles = 0,
    ///1: Timeout period of 4 cycles
    _4cycles = 1,
    ///2: Timeout period of 16 cycles
    _16cycles = 2,
    ///3: Timeout period of 32 cycles
    _32cycles = 3,
    ///4: Timeout period of 256 cycles
    _256cycles = 4,
    ///5: Timeout period of 1024 cycles
    _1kcycles = 5,
    ///6: Timeout period of 2048 cycles
    _2kcycles = 6,
    ///7: Timeout period of 4096 cycles
    _4kcycles = 7,
    ///8: Timeout period of 8192 cycles
    _8kcycles = 8,
    ///9: Timeout period of 16384 cycles
    _16kcycles = 9,
    ///10: Timeout period of 32768 cycles
    _32kcycles = 10,
}
impl From<STARTUPTIMEOUT> for u8 {
    #[inline(always)]
    fn from(variant: STARTUPTIMEOUT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STARTUPTIMEOUT {
    type Ux = u8;
}
impl crate::IsEnum for STARTUPTIMEOUT {}
///Field `STARTUPTIMEOUT` reader - Wait Duration in HFXO Startup Enable Wait State
pub type StartuptimeoutR = crate::FieldReader<STARTUPTIMEOUT>;
impl StartuptimeoutR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<STARTUPTIMEOUT> {
        match self.bits {
            0 => Some(STARTUPTIMEOUT::_2cycles),
            1 => Some(STARTUPTIMEOUT::_4cycles),
            2 => Some(STARTUPTIMEOUT::_16cycles),
            3 => Some(STARTUPTIMEOUT::_32cycles),
            4 => Some(STARTUPTIMEOUT::_256cycles),
            5 => Some(STARTUPTIMEOUT::_1kcycles),
            6 => Some(STARTUPTIMEOUT::_2kcycles),
            7 => Some(STARTUPTIMEOUT::_4kcycles),
            8 => Some(STARTUPTIMEOUT::_8kcycles),
            9 => Some(STARTUPTIMEOUT::_16kcycles),
            10 => Some(STARTUPTIMEOUT::_32kcycles),
            _ => None,
        }
    }
    ///Timeout period of 2 cycles
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == STARTUPTIMEOUT::_2cycles
    }
    ///Timeout period of 4 cycles
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == STARTUPTIMEOUT::_4cycles
    }
    ///Timeout period of 16 cycles
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == STARTUPTIMEOUT::_16cycles
    }
    ///Timeout period of 32 cycles
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == STARTUPTIMEOUT::_32cycles
    }
    ///Timeout period of 256 cycles
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == STARTUPTIMEOUT::_256cycles
    }
    ///Timeout period of 1024 cycles
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT::_1kcycles
    }
    ///Timeout period of 2048 cycles
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT::_2kcycles
    }
    ///Timeout period of 4096 cycles
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT::_4kcycles
    }
    ///Timeout period of 8192 cycles
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT::_8kcycles
    }
    ///Timeout period of 16384 cycles
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT::_16kcycles
    }
    ///Timeout period of 32768 cycles
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT::_32kcycles
    }
}
///Field `STARTUPTIMEOUT` writer - Wait Duration in HFXO Startup Enable Wait State
pub type StartuptimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 4, STARTUPTIMEOUT>;
impl<'a, REG> StartuptimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timeout period of 2 cycles
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPTIMEOUT::_2cycles)
    }
    ///Timeout period of 4 cycles
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPTIMEOUT::_4cycles)
    }
    ///Timeout period of 16 cycles
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPTIMEOUT::_16cycles)
    }
    ///Timeout period of 32 cycles
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPTIMEOUT::_32cycles)
    }
    ///Timeout period of 256 cycles
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPTIMEOUT::_256cycles)
    }
    ///Timeout period of 1024 cycles
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPTIMEOUT::_1kcycles)
    }
    ///Timeout period of 2048 cycles
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPTIMEOUT::_2kcycles)
    }
    ///Timeout period of 4096 cycles
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPTIMEOUT::_4kcycles)
    }
    ///Timeout period of 8192 cycles
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPTIMEOUT::_8kcycles)
    }
    ///Timeout period of 16384 cycles
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPTIMEOUT::_16kcycles)
    }
    ///Timeout period of 32768 cycles
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPTIMEOUT::_32kcycles)
    }
}
///Wait Duration in HFXO Startup Steady Wait State
///
///Value on reset: 6
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STEADYTIMEOUT {
    ///0: Timeout period of 2 cycles
    _2cycles = 0,
    ///1: Timeout period of 4 cycles
    _4cycles = 1,
    ///2: Timeout period of 16 cycles
    _16cycles = 2,
    ///3: Timeout period of 32 cycles
    _32cycles = 3,
    ///4: Timeout period of 256 cycles
    _256cycles = 4,
    ///5: Timeout period of 1024 cycles
    _1kcycles = 5,
    ///6: Timeout period of 2048 cycles
    _2kcycles = 6,
    ///7: Timeout period of 4096 cycles
    _4kcycles = 7,
    ///8: Timeout period of 8192 cycles
    _8kcycles = 8,
    ///9: Timeout period of 16384 cycles
    _16kcycles = 9,
    ///10: Timeout period of 32768 cycles
    _32kcycles = 10,
}
impl From<STEADYTIMEOUT> for u8 {
    #[inline(always)]
    fn from(variant: STEADYTIMEOUT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STEADYTIMEOUT {
    type Ux = u8;
}
impl crate::IsEnum for STEADYTIMEOUT {}
///Field `STEADYTIMEOUT` reader - Wait Duration in HFXO Startup Steady Wait State
pub type SteadytimeoutR = crate::FieldReader<STEADYTIMEOUT>;
impl SteadytimeoutR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<STEADYTIMEOUT> {
        match self.bits {
            0 => Some(STEADYTIMEOUT::_2cycles),
            1 => Some(STEADYTIMEOUT::_4cycles),
            2 => Some(STEADYTIMEOUT::_16cycles),
            3 => Some(STEADYTIMEOUT::_32cycles),
            4 => Some(STEADYTIMEOUT::_256cycles),
            5 => Some(STEADYTIMEOUT::_1kcycles),
            6 => Some(STEADYTIMEOUT::_2kcycles),
            7 => Some(STEADYTIMEOUT::_4kcycles),
            8 => Some(STEADYTIMEOUT::_8kcycles),
            9 => Some(STEADYTIMEOUT::_16kcycles),
            10 => Some(STEADYTIMEOUT::_32kcycles),
            _ => None,
        }
    }
    ///Timeout period of 2 cycles
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == STEADYTIMEOUT::_2cycles
    }
    ///Timeout period of 4 cycles
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == STEADYTIMEOUT::_4cycles
    }
    ///Timeout period of 16 cycles
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == STEADYTIMEOUT::_16cycles
    }
    ///Timeout period of 32 cycles
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == STEADYTIMEOUT::_32cycles
    }
    ///Timeout period of 256 cycles
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == STEADYTIMEOUT::_256cycles
    }
    ///Timeout period of 1024 cycles
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == STEADYTIMEOUT::_1kcycles
    }
    ///Timeout period of 2048 cycles
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == STEADYTIMEOUT::_2kcycles
    }
    ///Timeout period of 4096 cycles
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == STEADYTIMEOUT::_4kcycles
    }
    ///Timeout period of 8192 cycles
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == STEADYTIMEOUT::_8kcycles
    }
    ///Timeout period of 16384 cycles
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == STEADYTIMEOUT::_16kcycles
    }
    ///Timeout period of 32768 cycles
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == STEADYTIMEOUT::_32kcycles
    }
}
///Field `STEADYTIMEOUT` writer - Wait Duration in HFXO Startup Steady Wait State
pub type SteadytimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 4, STEADYTIMEOUT>;
impl<'a, REG> SteadytimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timeout period of 2 cycles
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(STEADYTIMEOUT::_2cycles)
    }
    ///Timeout period of 4 cycles
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(STEADYTIMEOUT::_4cycles)
    }
    ///Timeout period of 16 cycles
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(STEADYTIMEOUT::_16cycles)
    }
    ///Timeout period of 32 cycles
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(STEADYTIMEOUT::_32cycles)
    }
    ///Timeout period of 256 cycles
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(STEADYTIMEOUT::_256cycles)
    }
    ///Timeout period of 1024 cycles
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(STEADYTIMEOUT::_1kcycles)
    }
    ///Timeout period of 2048 cycles
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(STEADYTIMEOUT::_2kcycles)
    }
    ///Timeout period of 4096 cycles
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(STEADYTIMEOUT::_4kcycles)
    }
    ///Timeout period of 8192 cycles
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(STEADYTIMEOUT::_8kcycles)
    }
    ///Timeout period of 16384 cycles
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(STEADYTIMEOUT::_16kcycles)
    }
    ///Timeout period of 32768 cycles
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(STEADYTIMEOUT::_32kcycles)
    }
}
///Field `RESERVED2` reader - Wait Duration in HFXO Warm Startup Steady Wait State
pub type Reserved2R = crate::FieldReader;
///Field `RESERVED2` writer - Wait Duration in HFXO Warm Startup Steady Wait State
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Wait Duration in HFXO Peak Detection Wait State
///
///Value on reset: 6
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PEAKDETTIMEOUT {
    ///0: Timeout period of 2 cycles
    _2cycles = 0,
    ///1: Timeout period of 4 cycles
    _4cycles = 1,
    ///2: Timeout period of 16 cycles
    _16cycles = 2,
    ///3: Timeout period of 32 cycles
    _32cycles = 3,
    ///4: Timeout period of 256 cycles
    _256cycles = 4,
    ///5: Timeout period of 1024 cycles
    _1kcycles = 5,
    ///6: Timeout period of 2048 cycles
    _2kcycles = 6,
    ///7: Timeout period of 4096 cycles
    _4kcycles = 7,
    ///8: Timeout period of 8192 cycles
    _8kcycles = 8,
    ///9: Timeout period of 16384 cycles
    _16kcycles = 9,
    ///10: Timeout period of 32768 cycles
    _32kcycles = 10,
}
impl From<PEAKDETTIMEOUT> for u8 {
    #[inline(always)]
    fn from(variant: PEAKDETTIMEOUT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PEAKDETTIMEOUT {
    type Ux = u8;
}
impl crate::IsEnum for PEAKDETTIMEOUT {}
///Field `PEAKDETTIMEOUT` reader - Wait Duration in HFXO Peak Detection Wait State
pub type PeakdettimeoutR = crate::FieldReader<PEAKDETTIMEOUT>;
impl PeakdettimeoutR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PEAKDETTIMEOUT> {
        match self.bits {
            0 => Some(PEAKDETTIMEOUT::_2cycles),
            1 => Some(PEAKDETTIMEOUT::_4cycles),
            2 => Some(PEAKDETTIMEOUT::_16cycles),
            3 => Some(PEAKDETTIMEOUT::_32cycles),
            4 => Some(PEAKDETTIMEOUT::_256cycles),
            5 => Some(PEAKDETTIMEOUT::_1kcycles),
            6 => Some(PEAKDETTIMEOUT::_2kcycles),
            7 => Some(PEAKDETTIMEOUT::_4kcycles),
            8 => Some(PEAKDETTIMEOUT::_8kcycles),
            9 => Some(PEAKDETTIMEOUT::_16kcycles),
            10 => Some(PEAKDETTIMEOUT::_32kcycles),
            _ => None,
        }
    }
    ///Timeout period of 2 cycles
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT::_2cycles
    }
    ///Timeout period of 4 cycles
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT::_4cycles
    }
    ///Timeout period of 16 cycles
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT::_16cycles
    }
    ///Timeout period of 32 cycles
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT::_32cycles
    }
    ///Timeout period of 256 cycles
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT::_256cycles
    }
    ///Timeout period of 1024 cycles
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT::_1kcycles
    }
    ///Timeout period of 2048 cycles
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT::_2kcycles
    }
    ///Timeout period of 4096 cycles
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT::_4kcycles
    }
    ///Timeout period of 8192 cycles
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT::_8kcycles
    }
    ///Timeout period of 16384 cycles
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT::_16kcycles
    }
    ///Timeout period of 32768 cycles
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT::_32kcycles
    }
}
///Field `PEAKDETTIMEOUT` writer - Wait Duration in HFXO Peak Detection Wait State
pub type PeakdettimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 4, PEAKDETTIMEOUT>;
impl<'a, REG> PeakdettimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timeout period of 2 cycles
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETTIMEOUT::_2cycles)
    }
    ///Timeout period of 4 cycles
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETTIMEOUT::_4cycles)
    }
    ///Timeout period of 16 cycles
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETTIMEOUT::_16cycles)
    }
    ///Timeout period of 32 cycles
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETTIMEOUT::_32cycles)
    }
    ///Timeout period of 256 cycles
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETTIMEOUT::_256cycles)
    }
    ///Timeout period of 1024 cycles
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETTIMEOUT::_1kcycles)
    }
    ///Timeout period of 2048 cycles
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETTIMEOUT::_2kcycles)
    }
    ///Timeout period of 4096 cycles
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETTIMEOUT::_4kcycles)
    }
    ///Timeout period of 8192 cycles
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETTIMEOUT::_8kcycles)
    }
    ///Timeout period of 16384 cycles
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETTIMEOUT::_16kcycles)
    }
    ///Timeout period of 32768 cycles
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETTIMEOUT::_32kcycles)
    }
}
///Wait Duration in HFXO Shunt Current Optimization Wait State
///
///Value on reset: 2
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHUNTOPTTIMEOUT {
    ///0: Timeout period of 2 cycles
    _2cycles = 0,
    ///1: Timeout period of 4 cycles
    _4cycles = 1,
    ///2: Timeout period of 16 cycles
    _16cycles = 2,
    ///3: Timeout period of 32 cycles
    _32cycles = 3,
    ///4: Timeout period of 256 cycles
    _256cycles = 4,
    ///5: Timeout period of 1024 cycles
    _1kcycles = 5,
    ///6: Timeout period of 2048 cycles
    _2kcycles = 6,
    ///7: Timeout period of 4096 cycles
    _4kcycles = 7,
    ///8: Timeout period of 8192 cycles
    _8kcycles = 8,
    ///9: Timeout period of 16384 cycles
    _16kcycles = 9,
    ///10: Timeout period of 32768 cycles
    _32kcycles = 10,
}
impl From<SHUNTOPTTIMEOUT> for u8 {
    #[inline(always)]
    fn from(variant: SHUNTOPTTIMEOUT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SHUNTOPTTIMEOUT {
    type Ux = u8;
}
impl crate::IsEnum for SHUNTOPTTIMEOUT {}
///Field `SHUNTOPTTIMEOUT` reader - Wait Duration in HFXO Shunt Current Optimization Wait State
pub type ShuntopttimeoutR = crate::FieldReader<SHUNTOPTTIMEOUT>;
impl ShuntopttimeoutR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SHUNTOPTTIMEOUT> {
        match self.bits {
            0 => Some(SHUNTOPTTIMEOUT::_2cycles),
            1 => Some(SHUNTOPTTIMEOUT::_4cycles),
            2 => Some(SHUNTOPTTIMEOUT::_16cycles),
            3 => Some(SHUNTOPTTIMEOUT::_32cycles),
            4 => Some(SHUNTOPTTIMEOUT::_256cycles),
            5 => Some(SHUNTOPTTIMEOUT::_1kcycles),
            6 => Some(SHUNTOPTTIMEOUT::_2kcycles),
            7 => Some(SHUNTOPTTIMEOUT::_4kcycles),
            8 => Some(SHUNTOPTTIMEOUT::_8kcycles),
            9 => Some(SHUNTOPTTIMEOUT::_16kcycles),
            10 => Some(SHUNTOPTTIMEOUT::_32kcycles),
            _ => None,
        }
    }
    ///Timeout period of 2 cycles
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT::_2cycles
    }
    ///Timeout period of 4 cycles
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT::_4cycles
    }
    ///Timeout period of 16 cycles
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT::_16cycles
    }
    ///Timeout period of 32 cycles
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT::_32cycles
    }
    ///Timeout period of 256 cycles
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT::_256cycles
    }
    ///Timeout period of 1024 cycles
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT::_1kcycles
    }
    ///Timeout period of 2048 cycles
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT::_2kcycles
    }
    ///Timeout period of 4096 cycles
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT::_4kcycles
    }
    ///Timeout period of 8192 cycles
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT::_8kcycles
    }
    ///Timeout period of 16384 cycles
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT::_16kcycles
    }
    ///Timeout period of 32768 cycles
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT::_32kcycles
    }
}
///Field `SHUNTOPTTIMEOUT` writer - Wait Duration in HFXO Shunt Current Optimization Wait State
pub type ShuntopttimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 4, SHUNTOPTTIMEOUT>;
impl<'a, REG> ShuntopttimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timeout period of 2 cycles
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(SHUNTOPTTIMEOUT::_2cycles)
    }
    ///Timeout period of 4 cycles
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(SHUNTOPTTIMEOUT::_4cycles)
    }
    ///Timeout period of 16 cycles
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(SHUNTOPTTIMEOUT::_16cycles)
    }
    ///Timeout period of 32 cycles
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(SHUNTOPTTIMEOUT::_32cycles)
    }
    ///Timeout period of 256 cycles
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(SHUNTOPTTIMEOUT::_256cycles)
    }
    ///Timeout period of 1024 cycles
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(SHUNTOPTTIMEOUT::_1kcycles)
    }
    ///Timeout period of 2048 cycles
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(SHUNTOPTTIMEOUT::_2kcycles)
    }
    ///Timeout period of 4096 cycles
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(SHUNTOPTTIMEOUT::_4kcycles)
    }
    ///Timeout period of 8192 cycles
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(SHUNTOPTTIMEOUT::_8kcycles)
    }
    ///Timeout period of 16384 cycles
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(SHUNTOPTTIMEOUT::_16kcycles)
    }
    ///Timeout period of 32768 cycles
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(SHUNTOPTTIMEOUT::_32kcycles)
    }
}
impl R {
    ///Bits 0:3 - Wait Duration in HFXO Startup Enable Wait State
    #[inline(always)]
    pub fn startuptimeout(&self) -> StartuptimeoutR {
        StartuptimeoutR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Wait Duration in HFXO Startup Steady Wait State
    #[inline(always)]
    pub fn steadytimeout(&self) -> SteadytimeoutR {
        SteadytimeoutR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Wait Duration in HFXO Warm Startup Steady Wait State
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Wait Duration in HFXO Peak Detection Wait State
    #[inline(always)]
    pub fn peakdettimeout(&self) -> PeakdettimeoutR {
        PeakdettimeoutR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Wait Duration in HFXO Shunt Current Optimization Wait State
    #[inline(always)]
    pub fn shuntopttimeout(&self) -> ShuntopttimeoutR {
        ShuntopttimeoutR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFXOTIMEOUTCTRL")
            .field("startuptimeout", &self.startuptimeout())
            .field("steadytimeout", &self.steadytimeout())
            .field("reserved2", &self.reserved2())
            .field("peakdettimeout", &self.peakdettimeout())
            .field("shuntopttimeout", &self.shuntopttimeout())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Wait Duration in HFXO Startup Enable Wait State
    #[inline(always)]
    #[must_use]
    pub fn startuptimeout(&mut self) -> StartuptimeoutW<HFXOTIMEOUTCTRLrs> {
        StartuptimeoutW::new(self, 0)
    }
    ///Bits 4:7 - Wait Duration in HFXO Startup Steady Wait State
    #[inline(always)]
    #[must_use]
    pub fn steadytimeout(&mut self) -> SteadytimeoutW<HFXOTIMEOUTCTRLrs> {
        SteadytimeoutW::new(self, 4)
    }
    ///Bits 8:11 - Wait Duration in HFXO Warm Startup Steady Wait State
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<HFXOTIMEOUTCTRLrs> {
        Reserved2W::new(self, 8)
    }
    ///Bits 12:15 - Wait Duration in HFXO Peak Detection Wait State
    #[inline(always)]
    #[must_use]
    pub fn peakdettimeout(&mut self) -> PeakdettimeoutW<HFXOTIMEOUTCTRLrs> {
        PeakdettimeoutW::new(self, 12)
    }
    ///Bits 16:19 - Wait Duration in HFXO Shunt Current Optimization Wait State
    #[inline(always)]
    #[must_use]
    pub fn shuntopttimeout(&mut self) -> ShuntopttimeoutW<HFXOTIMEOUTCTRLrs> {
        ShuntopttimeoutW::new(self, 16)
    }
}
///HFXO Timeout Control
///
///You can [`read`](crate::Reg::read) this register and get [`hfxotimeoutctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxotimeoutctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HFXOTIMEOUTCTRLrs;
impl crate::RegisterSpec for HFXOTIMEOUTCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`hfxotimeoutctrl::R`](R) reader structure
impl crate::Readable for HFXOTIMEOUTCTRLrs {}
///`write(|w| ..)` method takes [`hfxotimeoutctrl::W`](W) writer structure
impl crate::Writable for HFXOTIMEOUTCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HFXOTIMEOUTCTRL to value 0x0002_6667
impl crate::Resettable for HFXOTIMEOUTCTRLrs {
    const RESET_VALUE: u32 = 0x0002_6667;
}
