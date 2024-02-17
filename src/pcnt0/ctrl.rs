#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Field `MODE` reader - Mode Select"]
pub type MODE_R = crate::FieldReader<MODE>;
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    #[doc = "0: The module is disabled."]
    Disable = 0,
    #[doc = "1: Single input LFACLK oversampling mode (available in EM0-EM3)."]
    Ovssingle = 1,
    #[doc = "2: Externally clocked single input counter mode (available in EM0-EM3)."]
    Extclksingle = 2,
    #[doc = "3: Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    Extclkquad = 3,
    #[doc = "4: LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    Ovsquad1x = 4,
    #[doc = "5: LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    Ovsquad2x = 5,
    #[doc = "6: LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
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
impl MODE_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MODE::Disable
    }
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovssingle(&self) -> bool {
        *self == MODE::Ovssingle
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_extclksingle(&self) -> bool {
        *self == MODE::Extclksingle
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_extclkquad(&self) -> bool {
        *self == MODE::Extclkquad
    }
    #[doc = "LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovsquad1x(&self) -> bool {
        *self == MODE::Ovsquad1x
    }
    #[doc = "LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovsquad2x(&self) -> bool {
        *self == MODE::Ovsquad2x
    }
    #[doc = "LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovsquad4x(&self) -> bool {
        *self == MODE::Ovsquad4x
    }
}
#[doc = "Field `MODE` writer - Mode Select"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MODE>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Disable)
    }
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovssingle(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Ovssingle)
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclksingle(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Extclksingle)
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclkquad(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Extclkquad)
    }
    #[doc = "LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad1x(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Ovsquad1x)
    }
    #[doc = "LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad2x(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Ovsquad2x)
    }
    #[doc = "LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad4x(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Ovsquad4x)
    }
}
#[doc = "Field `FILT` reader - Enable Digital Pulse Width Filter"]
pub type FILT_R = crate::BitReader;
#[doc = "Field `FILT` writer - Enable Digital Pulse Width Filter"]
pub type FILT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTEN` reader - Enable PCNT Clock Domain Reset"]
pub type RSTEN_R = crate::BitReader;
#[doc = "Field `RSTEN` writer - Enable PCNT Clock Domain Reset"]
pub type RSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTRSTEN` reader - Enable CNT Reset"]
pub type CNTRSTEN_R = crate::BitReader;
#[doc = "Field `CNTRSTEN` writer - Enable CNT Reset"]
pub type CNTRSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXCNTRSTEN` reader - Enable AUXCNT Reset"]
pub type AUXCNTRSTEN_R = crate::BitReader;
#[doc = "Field `AUXCNTRSTEN` writer - Enable AUXCNT Reset"]
pub type AUXCNTRSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGHALT` reader - Debug Mode Halt Enable"]
pub type DEBUGHALT_R = crate::BitReader;
#[doc = "Field `DEBUGHALT` writer - Debug Mode Halt Enable"]
pub type DEBUGHALT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Enable Hysteresis"]
pub type HYST_R = crate::BitReader;
#[doc = "Field `HYST` writer - Enable Hysteresis"]
pub type HYST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1CDIR` reader - Count Direction Determined By S1"]
pub type S1CDIR_R = crate::BitReader;
#[doc = "Field `S1CDIR` writer - Count Direction Determined By S1"]
pub type S1CDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTEV` reader - Controls When the Counter Counts"]
pub type CNTEV_R = crate::FieldReader<CNTEV>;
#[doc = "Controls When the Counter Counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTEV {
    #[doc = "0: Counts up on up-count and down on down-count events."]
    Both = 0,
    #[doc = "1: Only counts up on up-count events."]
    Up = 1,
    #[doc = "2: Only counts down on down-count events."]
    Down = 2,
    #[doc = "3: Never counts."]
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
impl CNTEV_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CNTEV::Both
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == CNTEV::Up
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == CNTEV::Down
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CNTEV::None
    }
}
#[doc = "Field `CNTEV` writer - Controls When the Counter Counts"]
pub type CNTEV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CNTEV>;
impl<'a, REG> CNTEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV::Both)
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV::Up)
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV::Down)
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV::None)
    }
}
#[doc = "Field `AUXCNTEV` reader - Controls When the Auxiliary Counter Counts"]
pub type AUXCNTEV_R = crate::FieldReader<AUXCNTEV>;
#[doc = "Controls When the Auxiliary Counter Counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AUXCNTEV {
    #[doc = "0: Never counts."]
    None = 0,
    #[doc = "1: Counts up on up-count events."]
    Up = 1,
    #[doc = "2: Counts up on down-count events."]
    Down = 2,
    #[doc = "3: Counts up on both up-count and down-count events."]
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
impl AUXCNTEV_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AUXCNTEV::None
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == AUXCNTEV::Up
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == AUXCNTEV::Down
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == AUXCNTEV::Both
    }
}
#[doc = "Field `AUXCNTEV` writer - Controls When the Auxiliary Counter Counts"]
pub type AUXCNTEV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AUXCNTEV>;
impl<'a, REG> AUXCNTEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV::None)
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV::Up)
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV::Down)
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV::Both)
    }
}
#[doc = "Field `CNTDIR` reader - Non-Quadrature Mode Counter Direction Control"]
pub type CNTDIR_R = crate::BitReader;
#[doc = "Field `CNTDIR` writer - Non-Quadrature Mode Counter Direction Control"]
pub type CNTDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE` reader - Edge Select"]
pub type EDGE_R = crate::BitReader;
#[doc = "Field `EDGE` writer - Edge Select"]
pub type EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCMODE` reader - Sets the Mode for Triggered Compare and Clear"]
pub type TCCMODE_R = crate::FieldReader<TCCMODE>;
#[doc = "Sets the Mode for Triggered Compare and Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCMODE {
    #[doc = "0: Triggered compare and clear not enabled."]
    Disabled = 0,
    #[doc = "1: Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    Lfa = 1,
    #[doc = "2: Compare and clear performed on positive PRS edges."]
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
impl TCCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCCMODE> {
        match self.bits {
            0 => Some(TCCMODE::Disabled),
            1 => Some(TCCMODE::Lfa),
            2 => Some(TCCMODE::Prs),
            _ => None,
        }
    }
    #[doc = "Triggered compare and clear not enabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCCMODE::Disabled
    }
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    #[inline(always)]
    pub fn is_lfa(&self) -> bool {
        *self == TCCMODE::Lfa
    }
    #[doc = "Compare and clear performed on positive PRS edges."]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == TCCMODE::Prs
    }
}
#[doc = "Field `TCCMODE` writer - Sets the Mode for Triggered Compare and Clear"]
pub type TCCMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TCCMODE>;
impl<'a, REG> TCCMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Triggered compare and clear not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCCMODE::Disabled)
    }
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    #[inline(always)]
    pub fn lfa(self) -> &'a mut crate::W<REG> {
        self.variant(TCCMODE::Lfa)
    }
    #[doc = "Compare and clear performed on positive PRS edges."]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(TCCMODE::Prs)
    }
}
#[doc = "Field `TCCPRESC` reader - Set the LFA Prescaler for Triggered Compare and Clear"]
pub type TCCPRESC_R = crate::FieldReader<TCCPRESC>;
#[doc = "Set the LFA Prescaler for Triggered Compare and Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCPRESC {
    #[doc = "0: Compare and clear event each LFA cycle."]
    Div1 = 0,
    #[doc = "1: Compare and clear performed on every other LFA cycle."]
    Div2 = 1,
    #[doc = "2: Compare and clear performed on every 4th LFA cycle."]
    Div4 = 2,
    #[doc = "3: Compare and clear performed on every 8th LFA cycle."]
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
impl TCCPRESC_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Compare and clear event each LFA cycle."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == TCCPRESC::Div1
    }
    #[doc = "Compare and clear performed on every other LFA cycle."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == TCCPRESC::Div2
    }
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == TCCPRESC::Div4
    }
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == TCCPRESC::Div8
    }
}
#[doc = "Field `TCCPRESC` writer - Set the LFA Prescaler for Triggered Compare and Clear"]
pub type TCCPRESC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TCCPRESC>;
impl<'a, REG> TCCPRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare and clear event each LFA cycle."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC::Div1)
    }
    #[doc = "Compare and clear performed on every other LFA cycle."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC::Div2)
    }
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC::Div4)
    }
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC::Div8)
    }
}
#[doc = "Field `TCCCOMP` reader - Triggered Compare and Clear Compare Mode"]
pub type TCCCOMP_R = crate::FieldReader<TCCCOMP>;
#[doc = "Triggered Compare and Clear Compare Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCCOMP {
    #[doc = "0: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    Ltoe = 0,
    #[doc = "1: Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    Gtoe = 1,
    #[doc = "2: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
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
impl TCCCOMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCCCOMP> {
        match self.bits {
            0 => Some(TCCCOMP::Ltoe),
            1 => Some(TCCCOMP::Gtoe),
            2 => Some(TCCCOMP::Range),
            _ => None,
        }
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn is_ltoe(&self) -> bool {
        *self == TCCCOMP::Ltoe
    }
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn is_gtoe(&self) -> bool {
        *self == TCCCOMP::Gtoe
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == TCCCOMP::Range
    }
}
#[doc = "Field `TCCCOMP` writer - Triggered Compare and Clear Compare Mode"]
pub type TCCCOMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TCCCOMP>;
impl<'a, REG> TCCCOMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn ltoe(self) -> &'a mut crate::W<REG> {
        self.variant(TCCCOMP::Ltoe)
    }
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn gtoe(self) -> &'a mut crate::W<REG> {
        self.variant(TCCCOMP::Gtoe)
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    #[inline(always)]
    pub fn range(self) -> &'a mut crate::W<REG> {
        self.variant(TCCCOMP::Range)
    }
}
#[doc = "Field `PRSGATEEN` reader - PRS Gate Enable"]
pub type PRSGATEEN_R = crate::BitReader;
#[doc = "Field `PRSGATEEN` writer - PRS Gate Enable"]
pub type PRSGATEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCPRSPOL` reader - TCC PRS Polarity Select"]
pub type TCCPRSPOL_R = crate::BitReader;
#[doc = "Field `TCCPRSPOL` writer - TCC PRS Polarity Select"]
pub type TCCPRSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCPRSSEL` reader - TCC PRS Channel Select"]
pub type TCCPRSSEL_R = crate::FieldReader<TCCPRSSEL>;
#[doc = "TCC PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCPRSSEL {
    #[doc = "0: PRS Channel 0 selected."]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected."]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected."]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected."]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected."]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected."]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected."]
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
impl TCCPRSSEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TCCPRSSEL::Prsch0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TCCPRSSEL::Prsch1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TCCPRSSEL::Prsch2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TCCPRSSEL::Prsch3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TCCPRSSEL::Prsch4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TCCPRSSEL::Prsch5
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == TCCPRSSEL::Prsch6
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == TCCPRSSEL::Prsch7
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == TCCPRSSEL::Prsch8
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == TCCPRSSEL::Prsch9
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == TCCPRSSEL::Prsch10
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == TCCPRSSEL::Prsch11
    }
}
#[doc = "Field `TCCPRSSEL` writer - TCC PRS Channel Select"]
pub type TCCPRSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TCCPRSSEL>;
impl<'a, REG> TCCPRSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL::Prsch11)
    }
}
#[doc = "Field `TOPBHFSEL` reader - TOPB High Frequency Value Select"]
pub type TOPBHFSEL_R = crate::BitReader;
#[doc = "Field `TOPBHFSEL` writer - TOPB High Frequency Value Select"]
pub type TOPBHFSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable PCNT Clock Domain Reset"]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable CNT Reset"]
    #[inline(always)]
    pub fn cntrsten(&self) -> CNTRSTEN_R {
        CNTRSTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline(always)]
    pub fn auxcntrsten(&self) -> AUXCNTRSTEN_R {
        AUXCNTRSTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debug Mode Halt Enable"]
    #[inline(always)]
    pub fn debughalt(&self) -> DEBUGHALT_R {
        DEBUGHALT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Count Direction Determined By S1"]
    #[inline(always)]
    pub fn s1cdir(&self) -> S1CDIR_R {
        S1CDIR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Controls When the Counter Counts"]
    #[inline(always)]
    pub fn cntev(&self) -> CNTEV_R {
        CNTEV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Controls When the Auxiliary Counter Counts"]
    #[inline(always)]
    pub fn auxcntev(&self) -> AUXCNTEV_R {
        AUXCNTEV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Non-Quadrature Mode Counter Direction Control"]
    #[inline(always)]
    pub fn cntdir(&self) -> CNTDIR_R {
        CNTDIR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Edge Select"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Sets the Mode for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccmode(&self) -> TCCMODE_R {
        TCCMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 19:20 - Set the LFA Prescaler for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccpresc(&self) -> TCCPRESC_R {
        TCCPRESC_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Triggered Compare and Clear Compare Mode"]
    #[inline(always)]
    pub fn tcccomp(&self) -> TCCCOMP_R {
        TCCCOMP_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - PRS Gate Enable"]
    #[inline(always)]
    pub fn prsgateen(&self) -> PRSGATEEN_R {
        PRSGATEEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TCC PRS Polarity Select"]
    #[inline(always)]
    pub fn tccprspol(&self) -> TCCPRSPOL_R {
        TCCPRSPOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29 - TCC PRS Channel Select"]
    #[inline(always)]
    pub fn tccprssel(&self) -> TCCPRSSEL_R {
        TCCPRSSEL_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - TOPB High Frequency Value Select"]
    #[inline(always)]
    pub fn topbhfsel(&self) -> TOPBHFSEL_R {
        TOPBHFSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRLrs> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bit 3 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filt(&mut self) -> FILT_W<CTRLrs> {
        FILT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable PCNT Clock Domain Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rsten(&mut self) -> RSTEN_W<CTRLrs> {
        RSTEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable CNT Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cntrsten(&mut self) -> CNTRSTEN_W<CTRLrs> {
        CNTRSTEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline(always)]
    #[must_use]
    pub fn auxcntrsten(&mut self) -> AUXCNTRSTEN_W<CTRLrs> {
        AUXCNTRSTEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Debug Mode Halt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debughalt(&mut self) -> DEBUGHALT_W<CTRLrs> {
        DEBUGHALT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<CTRLrs> {
        HYST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Count Direction Determined By S1"]
    #[inline(always)]
    #[must_use]
    pub fn s1cdir(&mut self) -> S1CDIR_W<CTRLrs> {
        S1CDIR_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - Controls When the Counter Counts"]
    #[inline(always)]
    #[must_use]
    pub fn cntev(&mut self) -> CNTEV_W<CTRLrs> {
        CNTEV_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Controls When the Auxiliary Counter Counts"]
    #[inline(always)]
    #[must_use]
    pub fn auxcntev(&mut self) -> AUXCNTEV_W<CTRLrs> {
        AUXCNTEV_W::new(self, 12)
    }
    #[doc = "Bit 14 - Non-Quadrature Mode Counter Direction Control"]
    #[inline(always)]
    #[must_use]
    pub fn cntdir(&mut self) -> CNTDIR_W<CTRLrs> {
        CNTDIR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EDGE_W<CTRLrs> {
        EDGE_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Sets the Mode for Triggered Compare and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tccmode(&mut self) -> TCCMODE_W<CTRLrs> {
        TCCMODE_W::new(self, 16)
    }
    #[doc = "Bits 19:20 - Set the LFA Prescaler for Triggered Compare and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tccpresc(&mut self) -> TCCPRESC_W<CTRLrs> {
        TCCPRESC_W::new(self, 19)
    }
    #[doc = "Bits 22:23 - Triggered Compare and Clear Compare Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tcccomp(&mut self) -> TCCCOMP_W<CTRLrs> {
        TCCCOMP_W::new(self, 22)
    }
    #[doc = "Bit 24 - PRS Gate Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prsgateen(&mut self) -> PRSGATEEN_W<CTRLrs> {
        PRSGATEEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - TCC PRS Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn tccprspol(&mut self) -> TCCPRSPOL_W<CTRLrs> {
        TCCPRSPOL_W::new(self, 25)
    }
    #[doc = "Bits 26:29 - TCC PRS Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn tccprssel(&mut self) -> TCCPRSSEL_W<CTRLrs> {
        TCCPRSSEL_W::new(self, 26)
    }
    #[doc = "Bit 31 - TOPB High Frequency Value Select"]
    #[inline(always)]
    #[must_use]
    pub fn topbhfsel(&mut self) -> TOPBHFSEL_W<CTRLrs> {
        TOPBHFSEL_W::new(self, 31)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
