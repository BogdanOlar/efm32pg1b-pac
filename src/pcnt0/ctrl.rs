///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Mode Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    ///0: The module is disabled.
    Disable = 0,
    ///1: Single input LFACLK oversampling mode (available in EM0-EM3).
    Ovssingle = 1,
    ///2: Externally clocked single input counter mode (available in EM0-EM3).
    Extclksingle = 2,
    ///3: Externally clocked quadrature decoder mode (available in EM0-EM3).
    Extclkquad = 3,
    ///4: LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3).
    Ovsquad1x = 4,
    ///5: LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3).
    Ovsquad2x = 5,
    ///6: LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3).
    Ovsquad4x = 6,
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(variant: MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE {
    type Ux = u8;
}
impl crate::IsEnum for MODE {}
///Field `MODE` reader - Mode Select
pub type ModeR = crate::FieldReader<MODE>;
impl ModeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE> {
        match self.bits {
            0 => Some(MODE::Disable),
            1 => Some(MODE::Ovssingle),
            2 => Some(MODE::Extclksingle),
            3 => Some(MODE::Extclkquad),
            4 => Some(MODE::Ovsquad1x),
            5 => Some(MODE::Ovsquad2x),
            6 => Some(MODE::Ovsquad4x),
            _ => None,
        }
    }
    ///The module is disabled.
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MODE::Disable
    }
    ///Single input LFACLK oversampling mode (available in EM0-EM3).
    #[inline(always)]
    pub fn is_ovssingle(&self) -> bool {
        *self == MODE::Ovssingle
    }
    ///Externally clocked single input counter mode (available in EM0-EM3).
    #[inline(always)]
    pub fn is_extclksingle(&self) -> bool {
        *self == MODE::Extclksingle
    }
    ///Externally clocked quadrature decoder mode (available in EM0-EM3).
    #[inline(always)]
    pub fn is_extclkquad(&self) -> bool {
        *self == MODE::Extclkquad
    }
    ///LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3).
    #[inline(always)]
    pub fn is_ovsquad1x(&self) -> bool {
        *self == MODE::Ovsquad1x
    }
    ///LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3).
    #[inline(always)]
    pub fn is_ovsquad2x(&self) -> bool {
        *self == MODE::Ovsquad2x
    }
    ///LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3).
    #[inline(always)]
    pub fn is_ovsquad4x(&self) -> bool {
        *self == MODE::Ovsquad4x
    }
}
///Field `MODE` writer - Mode Select
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, MODE>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The module is disabled.
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Disable)
    }
    ///Single input LFACLK oversampling mode (available in EM0-EM3).
    #[inline(always)]
    pub fn ovssingle(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Ovssingle)
    }
    ///Externally clocked single input counter mode (available in EM0-EM3).
    #[inline(always)]
    pub fn extclksingle(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Extclksingle)
    }
    ///Externally clocked quadrature decoder mode (available in EM0-EM3).
    #[inline(always)]
    pub fn extclkquad(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Extclkquad)
    }
    ///LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3).
    #[inline(always)]
    pub fn ovsquad1x(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Ovsquad1x)
    }
    ///LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3).
    #[inline(always)]
    pub fn ovsquad2x(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Ovsquad2x)
    }
    ///LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3).
    #[inline(always)]
    pub fn ovsquad4x(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Ovsquad4x)
    }
}
///Field `FILT` reader - Enable Digital Pulse Width Filter
pub type FiltR = crate::BitReader;
///Field `FILT` writer - Enable Digital Pulse Width Filter
pub type FiltW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTEN` reader - Enable PCNT Clock Domain Reset
pub type RstenR = crate::BitReader;
///Field `RSTEN` writer - Enable PCNT Clock Domain Reset
pub type RstenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTRSTEN` reader - Enable CNT Reset
pub type CntrstenR = crate::BitReader;
///Field `CNTRSTEN` writer - Enable CNT Reset
pub type CntrstenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUXCNTRSTEN` reader - Enable AUXCNT Reset
pub type AuxcntrstenR = crate::BitReader;
///Field `AUXCNTRSTEN` writer - Enable AUXCNT Reset
pub type AuxcntrstenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEBUGHALT` reader - Debug Mode Halt Enable
pub type DebughaltR = crate::BitReader;
///Field `DEBUGHALT` writer - Debug Mode Halt Enable
pub type DebughaltW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HYST` reader - Enable Hysteresis
pub type HystR = crate::BitReader;
///Field `HYST` writer - Enable Hysteresis
pub type HystW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S1CDIR` reader - Count Direction Determined By S1
pub type S1cdirR = crate::BitReader;
///Field `S1CDIR` writer - Count Direction Determined By S1
pub type S1cdirW<'a, REG> = crate::BitWriter<'a, REG>;
///Controls When the Counter Counts
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTEV {
    ///0: Counts up on up-count and down on down-count events.
    Both = 0,
    ///1: Only counts up on up-count events.
    Up = 1,
    ///2: Only counts down on down-count events.
    Down = 2,
    ///3: Never counts.
    None = 3,
}
impl From<CNTEV> for u8 {
    #[inline(always)]
    fn from(variant: CNTEV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CNTEV {
    type Ux = u8;
}
impl crate::IsEnum for CNTEV {}
///Field `CNTEV` reader - Controls When the Counter Counts
pub type CntevR = crate::FieldReader<CNTEV>;
impl CntevR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CNTEV {
        match self.bits {
            0 => CNTEV::Both,
            1 => CNTEV::Up,
            2 => CNTEV::Down,
            3 => CNTEV::None,
            _ => unreachable!(),
        }
    }
    ///Counts up on up-count and down on down-count events.
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CNTEV::Both
    }
    ///Only counts up on up-count events.
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == CNTEV::Up
    }
    ///Only counts down on down-count events.
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == CNTEV::Down
    }
    ///Never counts.
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CNTEV::None
    }
}
///Field `CNTEV` writer - Controls When the Counter Counts
pub type CntevW<'a, REG> = crate::FieldWriter<'a, REG, 2, CNTEV, crate::Safe>;
impl<'a, REG> CntevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Counts up on up-count and down on down-count events.
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV::Both)
    }
    ///Only counts up on up-count events.
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV::Up)
    }
    ///Only counts down on down-count events.
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV::Down)
    }
    ///Never counts.
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV::None)
    }
}
///Controls When the Auxiliary Counter Counts
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AUXCNTEV {
    ///0: Never counts.
    None = 0,
    ///1: Counts up on up-count events.
    Up = 1,
    ///2: Counts up on down-count events.
    Down = 2,
    ///3: Counts up on both up-count and down-count events.
    Both = 3,
}
impl From<AUXCNTEV> for u8 {
    #[inline(always)]
    fn from(variant: AUXCNTEV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AUXCNTEV {
    type Ux = u8;
}
impl crate::IsEnum for AUXCNTEV {}
///Field `AUXCNTEV` reader - Controls When the Auxiliary Counter Counts
pub type AuxcntevR = crate::FieldReader<AUXCNTEV>;
impl AuxcntevR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AUXCNTEV {
        match self.bits {
            0 => AUXCNTEV::None,
            1 => AUXCNTEV::Up,
            2 => AUXCNTEV::Down,
            3 => AUXCNTEV::Both,
            _ => unreachable!(),
        }
    }
    ///Never counts.
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AUXCNTEV::None
    }
    ///Counts up on up-count events.
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == AUXCNTEV::Up
    }
    ///Counts up on down-count events.
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == AUXCNTEV::Down
    }
    ///Counts up on both up-count and down-count events.
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == AUXCNTEV::Both
    }
}
///Field `AUXCNTEV` writer - Controls When the Auxiliary Counter Counts
pub type AuxcntevW<'a, REG> = crate::FieldWriter<'a, REG, 2, AUXCNTEV, crate::Safe>;
impl<'a, REG> AuxcntevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Never counts.
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV::None)
    }
    ///Counts up on up-count events.
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV::Up)
    }
    ///Counts up on down-count events.
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV::Down)
    }
    ///Counts up on both up-count and down-count events.
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV::Both)
    }
}
///Field `CNTDIR` reader - Non-Quadrature Mode Counter Direction Control
pub type CntdirR = crate::BitReader;
///Field `CNTDIR` writer - Non-Quadrature Mode Counter Direction Control
pub type CntdirW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EDGE` reader - Edge Select
pub type EdgeR = crate::BitReader;
///Field `EDGE` writer - Edge Select
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG>;
///Sets the Mode for Triggered Compare and Clear
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCMODE {
    ///0: Triggered compare and clear not enabled.
    Disabled = 0,
    ///1: Compare and clear performed on each (optionally prescaled) LFA clock cycle.
    Lfa = 1,
    ///2: Compare and clear performed on positive PRS edges.
    Prs = 2,
}
impl From<TCCMODE> for u8 {
    #[inline(always)]
    fn from(variant: TCCMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCMODE {
    type Ux = u8;
}
impl crate::IsEnum for TCCMODE {}
///Field `TCCMODE` reader - Sets the Mode for Triggered Compare and Clear
pub type TccmodeR = crate::FieldReader<TCCMODE>;
impl TccmodeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCCMODE> {
        match self.bits {
            0 => Some(TCCMODE::Disabled),
            1 => Some(TCCMODE::Lfa),
            2 => Some(TCCMODE::Prs),
            _ => None,
        }
    }
    ///Triggered compare and clear not enabled.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCCMODE::Disabled
    }
    ///Compare and clear performed on each (optionally prescaled) LFA clock cycle.
    #[inline(always)]
    pub fn is_lfa(&self) -> bool {
        *self == TCCMODE::Lfa
    }
    ///Compare and clear performed on positive PRS edges.
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == TCCMODE::Prs
    }
}
///Field `TCCMODE` writer - Sets the Mode for Triggered Compare and Clear
pub type TccmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, TCCMODE>;
impl<'a, REG> TccmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Triggered compare and clear not enabled.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCCMODE::Disabled)
    }
    ///Compare and clear performed on each (optionally prescaled) LFA clock cycle.
    #[inline(always)]
    pub fn lfa(self) -> &'a mut crate::W<REG> {
        self.variant(TCCMODE::Lfa)
    }
    ///Compare and clear performed on positive PRS edges.
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(TCCMODE::Prs)
    }
}
///Set the LFA Prescaler for Triggered Compare and Clear
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCPRESC {
    ///0: Compare and clear event each LFA cycle.
    Div1 = 0,
    ///1: Compare and clear performed on every other LFA cycle.
    Div2 = 1,
    ///2: Compare and clear performed on every 4th LFA cycle.
    Div4 = 2,
    ///3: Compare and clear performed on every 8th LFA cycle.
    Div8 = 3,
}
impl From<TCCPRESC> for u8 {
    #[inline(always)]
    fn from(variant: TCCPRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCPRESC {
    type Ux = u8;
}
impl crate::IsEnum for TCCPRESC {}
///Field `TCCPRESC` reader - Set the LFA Prescaler for Triggered Compare and Clear
pub type TccprescR = crate::FieldReader<TCCPRESC>;
impl TccprescR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCCPRESC {
        match self.bits {
            0 => TCCPRESC::Div1,
            1 => TCCPRESC::Div2,
            2 => TCCPRESC::Div4,
            3 => TCCPRESC::Div8,
            _ => unreachable!(),
        }
    }
    ///Compare and clear event each LFA cycle.
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == TCCPRESC::Div1
    }
    ///Compare and clear performed on every other LFA cycle.
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == TCCPRESC::Div2
    }
    ///Compare and clear performed on every 4th LFA cycle.
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == TCCPRESC::Div4
    }
    ///Compare and clear performed on every 8th LFA cycle.
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == TCCPRESC::Div8
    }
}
///Field `TCCPRESC` writer - Set the LFA Prescaler for Triggered Compare and Clear
pub type TccprescW<'a, REG> = crate::FieldWriter<'a, REG, 2, TCCPRESC, crate::Safe>;
impl<'a, REG> TccprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Compare and clear event each LFA cycle.
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC::Div1)
    }
    ///Compare and clear performed on every other LFA cycle.
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC::Div2)
    }
    ///Compare and clear performed on every 4th LFA cycle.
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC::Div4)
    }
    ///Compare and clear performed on every 8th LFA cycle.
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC::Div8)
    }
}
///Triggered Compare and Clear Compare Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCCOMP {
    ///0: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP.
    Ltoe = 0,
    ///1: Compare match if PCNT_CNT is greater than or equal to PCNT_TOP.
    Gtoe = 1,
    ///2: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\[15:8\]\], and greater than, or equal to PCNT_TOP\[7:0\].
    Range = 2,
}
impl From<TCCCOMP> for u8 {
    #[inline(always)]
    fn from(variant: TCCCOMP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCCOMP {
    type Ux = u8;
}
impl crate::IsEnum for TCCCOMP {}
///Field `TCCCOMP` reader - Triggered Compare and Clear Compare Mode
pub type TcccompR = crate::FieldReader<TCCCOMP>;
impl TcccompR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCCCOMP> {
        match self.bits {
            0 => Some(TCCCOMP::Ltoe),
            1 => Some(TCCCOMP::Gtoe),
            2 => Some(TCCCOMP::Range),
            _ => None,
        }
    }
    ///Compare match if PCNT_CNT is less than, or equal to PCNT_TOP.
    #[inline(always)]
    pub fn is_ltoe(&self) -> bool {
        *self == TCCCOMP::Ltoe
    }
    ///Compare match if PCNT_CNT is greater than or equal to PCNT_TOP.
    #[inline(always)]
    pub fn is_gtoe(&self) -> bool {
        *self == TCCCOMP::Gtoe
    }
    ///Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\[15:8\]\], and greater than, or equal to PCNT_TOP\[7:0\].
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == TCCCOMP::Range
    }
}
///Field `TCCCOMP` writer - Triggered Compare and Clear Compare Mode
pub type TcccompW<'a, REG> = crate::FieldWriter<'a, REG, 2, TCCCOMP>;
impl<'a, REG> TcccompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Compare match if PCNT_CNT is less than, or equal to PCNT_TOP.
    #[inline(always)]
    pub fn ltoe(self) -> &'a mut crate::W<REG> {
        self.variant(TCCCOMP::Ltoe)
    }
    ///Compare match if PCNT_CNT is greater than or equal to PCNT_TOP.
    #[inline(always)]
    pub fn gtoe(self) -> &'a mut crate::W<REG> {
        self.variant(TCCCOMP::Gtoe)
    }
    ///Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\[15:8\]\], and greater than, or equal to PCNT_TOP\[7:0\].
    #[inline(always)]
    pub fn range(self) -> &'a mut crate::W<REG> {
        self.variant(TCCCOMP::Range)
    }
}
///Field `PRSGATEEN` reader - PRS Gate Enable
pub type PrsgateenR = crate::BitReader;
///Field `PRSGATEEN` writer - PRS Gate Enable
pub type PrsgateenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCCPRSPOL` reader - TCC PRS Polarity Select
pub type TccprspolR = crate::BitReader;
///Field `TCCPRSPOL` writer - TCC PRS Polarity Select
pub type TccprspolW<'a, REG> = crate::BitWriter<'a, REG>;
///TCC PRS Channel Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCPRSSEL {
    ///0: PRS Channel 0 selected.
    Prsch0 = 0,
    ///1: PRS Channel 1 selected.
    Prsch1 = 1,
    ///2: PRS Channel 2 selected.
    Prsch2 = 2,
    ///3: PRS Channel 3 selected.
    Prsch3 = 3,
    ///4: PRS Channel 4 selected.
    Prsch4 = 4,
    ///5: PRS Channel 5 selected.
    Prsch5 = 5,
    ///6: PRS Channel 6 selected.
    Prsch6 = 6,
    ///7: PRS Channel 7 selected.
    Prsch7 = 7,
    ///8: PRS Channel 8 selected.
    Prsch8 = 8,
    ///9: PRS Channel 9 selected.
    Prsch9 = 9,
    ///10: PRS Channel 10 selected.
    Prsch10 = 10,
    ///11: PRS Channel 11 selected.
    Prsch11 = 11,
}
impl From<TCCPRSSEL> for u8 {
    #[inline(always)]
    fn from(variant: TCCPRSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCPRSSEL {
    type Ux = u8;
}
impl crate::IsEnum for TCCPRSSEL {}
///Field `TCCPRSSEL` reader - TCC PRS Channel Select
pub type TccprsselR = crate::FieldReader<TCCPRSSEL>;
impl TccprsselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCCPRSSEL> {
        match self.bits {
            0 => Some(TCCPRSSEL::Prsch0),
            1 => Some(TCCPRSSEL::Prsch1),
            2 => Some(TCCPRSSEL::Prsch2),
            3 => Some(TCCPRSSEL::Prsch3),
            4 => Some(TCCPRSSEL::Prsch4),
            5 => Some(TCCPRSSEL::Prsch5),
            6 => Some(TCCPRSSEL::Prsch6),
            7 => Some(TCCPRSSEL::Prsch7),
            8 => Some(TCCPRSSEL::Prsch8),
            9 => Some(TCCPRSSEL::Prsch9),
            10 => Some(TCCPRSSEL::Prsch10),
            11 => Some(TCCPRSSEL::Prsch11),
            _ => None,
        }
    }
    ///PRS Channel 0 selected.
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TCCPRSSEL::Prsch0
    }
    ///PRS Channel 1 selected.
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TCCPRSSEL::Prsch1
    }
    ///PRS Channel 2 selected.
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TCCPRSSEL::Prsch2
    }
    ///PRS Channel 3 selected.
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TCCPRSSEL::Prsch3
    }
    ///PRS Channel 4 selected.
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TCCPRSSEL::Prsch4
    }
    ///PRS Channel 5 selected.
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TCCPRSSEL::Prsch5
    }
    ///PRS Channel 6 selected.
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == TCCPRSSEL::Prsch6
    }
    ///PRS Channel 7 selected.
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == TCCPRSSEL::Prsch7
    }
    ///PRS Channel 8 selected.
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == TCCPRSSEL::Prsch8
    }
    ///PRS Channel 9 selected.
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == TCCPRSSEL::Prsch9
    }
    ///PRS Channel 10 selected.
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == TCCPRSSEL::Prsch10
    }
    ///PRS Channel 11 selected.
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == TCCPRSSEL::Prsch11
    }
}
///Field `TCCPRSSEL` writer - TCC PRS Channel Select
pub type TccprsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, TCCPRSSEL>;
impl<'a, REG> TccprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected.
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch0)
    }
    ///PRS Channel 1 selected.
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch1)
    }
    ///PRS Channel 2 selected.
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch2)
    }
    ///PRS Channel 3 selected.
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch3)
    }
    ///PRS Channel 4 selected.
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch4)
    }
    ///PRS Channel 5 selected.
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch5)
    }
    ///PRS Channel 6 selected.
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch6)
    }
    ///PRS Channel 7 selected.
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch7)
    }
    ///PRS Channel 8 selected.
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch8)
    }
    ///PRS Channel 9 selected.
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch9)
    }
    ///PRS Channel 10 selected.
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch10)
    }
    ///PRS Channel 11 selected.
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch11)
    }
}
///Field `TOPBHFSEL` reader - TOPB High Frequency Value Select
pub type TopbhfselR = crate::BitReader;
///Field `TOPBHFSEL` writer - TOPB High Frequency Value Select
pub type TopbhfselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Mode Select
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Enable Digital Pulse Width Filter
    #[inline(always)]
    pub fn filt(&self) -> FiltR {
        FiltR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable PCNT Clock Domain Reset
    #[inline(always)]
    pub fn rsten(&self) -> RstenR {
        RstenR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Enable CNT Reset
    #[inline(always)]
    pub fn cntrsten(&self) -> CntrstenR {
        CntrstenR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Enable AUXCNT Reset
    #[inline(always)]
    pub fn auxcntrsten(&self) -> AuxcntrstenR {
        AuxcntrstenR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Debug Mode Halt Enable
    #[inline(always)]
    pub fn debughalt(&self) -> DebughaltR {
        DebughaltR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Enable Hysteresis
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Count Direction Determined By S1
    #[inline(always)]
    pub fn s1cdir(&self) -> S1cdirR {
        S1cdirR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - Controls When the Counter Counts
    #[inline(always)]
    pub fn cntev(&self) -> CntevR {
        CntevR::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Controls When the Auxiliary Counter Counts
    #[inline(always)]
    pub fn auxcntev(&self) -> AuxcntevR {
        AuxcntevR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - Non-Quadrature Mode Counter Direction Control
    #[inline(always)]
    pub fn cntdir(&self) -> CntdirR {
        CntdirR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Edge Select
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Sets the Mode for Triggered Compare and Clear
    #[inline(always)]
    pub fn tccmode(&self) -> TccmodeR {
        TccmodeR::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 19:20 - Set the LFA Prescaler for Triggered Compare and Clear
    #[inline(always)]
    pub fn tccpresc(&self) -> TccprescR {
        TccprescR::new(((self.bits >> 19) & 3) as u8)
    }
    ///Bits 22:23 - Triggered Compare and Clear Compare Mode
    #[inline(always)]
    pub fn tcccomp(&self) -> TcccompR {
        TcccompR::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - PRS Gate Enable
    #[inline(always)]
    pub fn prsgateen(&self) -> PrsgateenR {
        PrsgateenR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TCC PRS Polarity Select
    #[inline(always)]
    pub fn tccprspol(&self) -> TccprspolR {
        TccprspolR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:29 - TCC PRS Channel Select
    #[inline(always)]
    pub fn tccprssel(&self) -> TccprsselR {
        TccprsselR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    ///Bit 31 - TOPB High Frequency Value Select
    #[inline(always)]
    pub fn topbhfsel(&self) -> TopbhfselR {
        TopbhfselR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("mode", &self.mode())
            .field("filt", &self.filt())
            .field("rsten", &self.rsten())
            .field("cntrsten", &self.cntrsten())
            .field("auxcntrsten", &self.auxcntrsten())
            .field("debughalt", &self.debughalt())
            .field("hyst", &self.hyst())
            .field("s1cdir", &self.s1cdir())
            .field("cntev", &self.cntev())
            .field("auxcntev", &self.auxcntev())
            .field("cntdir", &self.cntdir())
            .field("edge", &self.edge())
            .field("tccmode", &self.tccmode())
            .field("tccpresc", &self.tccpresc())
            .field("tcccomp", &self.tcccomp())
            .field("prsgateen", &self.prsgateen())
            .field("tccprspol", &self.tccprspol())
            .field("tccprssel", &self.tccprssel())
            .field("topbhfsel", &self.topbhfsel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Mode Select
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CTRLrs> {
        ModeW::new(self, 0)
    }
    ///Bit 3 - Enable Digital Pulse Width Filter
    #[inline(always)]
    #[must_use]
    pub fn filt(&mut self) -> FiltW<CTRLrs> {
        FiltW::new(self, 3)
    }
    ///Bit 4 - Enable PCNT Clock Domain Reset
    #[inline(always)]
    #[must_use]
    pub fn rsten(&mut self) -> RstenW<CTRLrs> {
        RstenW::new(self, 4)
    }
    ///Bit 5 - Enable CNT Reset
    #[inline(always)]
    #[must_use]
    pub fn cntrsten(&mut self) -> CntrstenW<CTRLrs> {
        CntrstenW::new(self, 5)
    }
    ///Bit 6 - Enable AUXCNT Reset
    #[inline(always)]
    #[must_use]
    pub fn auxcntrsten(&mut self) -> AuxcntrstenW<CTRLrs> {
        AuxcntrstenW::new(self, 6)
    }
    ///Bit 7 - Debug Mode Halt Enable
    #[inline(always)]
    #[must_use]
    pub fn debughalt(&mut self) -> DebughaltW<CTRLrs> {
        DebughaltW::new(self, 7)
    }
    ///Bit 8 - Enable Hysteresis
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HystW<CTRLrs> {
        HystW::new(self, 8)
    }
    ///Bit 9 - Count Direction Determined By S1
    #[inline(always)]
    #[must_use]
    pub fn s1cdir(&mut self) -> S1cdirW<CTRLrs> {
        S1cdirW::new(self, 9)
    }
    ///Bits 10:11 - Controls When the Counter Counts
    #[inline(always)]
    #[must_use]
    pub fn cntev(&mut self) -> CntevW<CTRLrs> {
        CntevW::new(self, 10)
    }
    ///Bits 12:13 - Controls When the Auxiliary Counter Counts
    #[inline(always)]
    #[must_use]
    pub fn auxcntev(&mut self) -> AuxcntevW<CTRLrs> {
        AuxcntevW::new(self, 12)
    }
    ///Bit 14 - Non-Quadrature Mode Counter Direction Control
    #[inline(always)]
    #[must_use]
    pub fn cntdir(&mut self) -> CntdirW<CTRLrs> {
        CntdirW::new(self, 14)
    }
    ///Bit 15 - Edge Select
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EdgeW<CTRLrs> {
        EdgeW::new(self, 15)
    }
    ///Bits 16:17 - Sets the Mode for Triggered Compare and Clear
    #[inline(always)]
    #[must_use]
    pub fn tccmode(&mut self) -> TccmodeW<CTRLrs> {
        TccmodeW::new(self, 16)
    }
    ///Bits 19:20 - Set the LFA Prescaler for Triggered Compare and Clear
    #[inline(always)]
    #[must_use]
    pub fn tccpresc(&mut self) -> TccprescW<CTRLrs> {
        TccprescW::new(self, 19)
    }
    ///Bits 22:23 - Triggered Compare and Clear Compare Mode
    #[inline(always)]
    #[must_use]
    pub fn tcccomp(&mut self) -> TcccompW<CTRLrs> {
        TcccompW::new(self, 22)
    }
    ///Bit 24 - PRS Gate Enable
    #[inline(always)]
    #[must_use]
    pub fn prsgateen(&mut self) -> PrsgateenW<CTRLrs> {
        PrsgateenW::new(self, 24)
    }
    ///Bit 25 - TCC PRS Polarity Select
    #[inline(always)]
    #[must_use]
    pub fn tccprspol(&mut self) -> TccprspolW<CTRLrs> {
        TccprspolW::new(self, 25)
    }
    ///Bits 26:29 - TCC PRS Channel Select
    #[inline(always)]
    #[must_use]
    pub fn tccprssel(&mut self) -> TccprsselW<CTRLrs> {
        TccprsselW::new(self, 26)
    }
    ///Bit 31 - TOPB High Frequency Value Select
    #[inline(always)]
    #[must_use]
    pub fn topbhfsel(&mut self) -> TopbhfselW<CTRLrs> {
        TopbhfselW::new(self, 31)
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
///`reset()` method sets CTRL to value 0
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
