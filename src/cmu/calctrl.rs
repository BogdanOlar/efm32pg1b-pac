///Register `CALCTRL` reader
pub type R = crate::R<CALCTRLrs>;
///Register `CALCTRL` writer
pub type W = crate::W<CALCTRLrs>;
///Calibration Up-counter Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPSEL {
    ///0: Select HFXO as up-counter
    Hfxo = 0,
    ///1: Select LFXO as up-counter
    Lfxo = 1,
    ///2: Select HFRCO as up-counter
    Hfrco = 2,
    ///3: Select LFRCO as up-counter
    Lfrco = 3,
    ///4: Select AUXHFRCO as up-counter
    Auxhfrco = 4,
    ///5: Select PRS input selected by PRSUPSEL as up-counter
    Prs = 5,
}
impl From<UPSEL> for u8 {
    #[inline(always)]
    fn from(variant: UPSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UPSEL {
    type Ux = u8;
}
impl crate::IsEnum for UPSEL {}
///Field `UPSEL` reader - Calibration Up-counter Select
pub type UpselR = crate::FieldReader<UPSEL>;
impl UpselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<UPSEL> {
        match self.bits {
            0 => Some(UPSEL::Hfxo),
            1 => Some(UPSEL::Lfxo),
            2 => Some(UPSEL::Hfrco),
            3 => Some(UPSEL::Lfrco),
            4 => Some(UPSEL::Auxhfrco),
            5 => Some(UPSEL::Prs),
            _ => None,
        }
    }
    ///Select HFXO as up-counter
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == UPSEL::Hfxo
    }
    ///Select LFXO as up-counter
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == UPSEL::Lfxo
    }
    ///Select HFRCO as up-counter
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == UPSEL::Hfrco
    }
    ///Select LFRCO as up-counter
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == UPSEL::Lfrco
    }
    ///Select AUXHFRCO as up-counter
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == UPSEL::Auxhfrco
    }
    ///Select PRS input selected by PRSUPSEL as up-counter
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == UPSEL::Prs
    }
}
///Field `UPSEL` writer - Calibration Up-counter Select
pub type UpselW<'a, REG> = crate::FieldWriter<'a, REG, 3, UPSEL>;
impl<'a, REG> UpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select HFXO as up-counter
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(UPSEL::Hfxo)
    }
    ///Select LFXO as up-counter
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(UPSEL::Lfxo)
    }
    ///Select HFRCO as up-counter
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(UPSEL::Hfrco)
    }
    ///Select LFRCO as up-counter
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(UPSEL::Lfrco)
    }
    ///Select AUXHFRCO as up-counter
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(UPSEL::Auxhfrco)
    }
    ///Select PRS input selected by PRSUPSEL as up-counter
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(UPSEL::Prs)
    }
}
///Calibration Down-counter Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DOWNSEL {
    ///0: Select HFCLK for down-counter
    Hfclk = 0,
    ///1: Select HFXO for down-counter
    Hfxo = 1,
    ///2: Select LFXO for down-counter
    Lfxo = 2,
    ///3: Select HFRCO for down-counter
    Hfrco = 3,
    ///4: Select LFRCO for down-counter
    Lfrco = 4,
    ///5: Select AUXHFRCO for down-counter
    Auxhfrco = 5,
    ///6: Select PRS input selected by PRSDOWNSEL as down-counter
    Prs = 6,
}
impl From<DOWNSEL> for u8 {
    #[inline(always)]
    fn from(variant: DOWNSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DOWNSEL {
    type Ux = u8;
}
impl crate::IsEnum for DOWNSEL {}
///Field `DOWNSEL` reader - Calibration Down-counter Select
pub type DownselR = crate::FieldReader<DOWNSEL>;
impl DownselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DOWNSEL> {
        match self.bits {
            0 => Some(DOWNSEL::Hfclk),
            1 => Some(DOWNSEL::Hfxo),
            2 => Some(DOWNSEL::Lfxo),
            3 => Some(DOWNSEL::Hfrco),
            4 => Some(DOWNSEL::Lfrco),
            5 => Some(DOWNSEL::Auxhfrco),
            6 => Some(DOWNSEL::Prs),
            _ => None,
        }
    }
    ///Select HFCLK for down-counter
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == DOWNSEL::Hfclk
    }
    ///Select HFXO for down-counter
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == DOWNSEL::Hfxo
    }
    ///Select LFXO for down-counter
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == DOWNSEL::Lfxo
    }
    ///Select HFRCO for down-counter
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == DOWNSEL::Hfrco
    }
    ///Select LFRCO for down-counter
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == DOWNSEL::Lfrco
    }
    ///Select AUXHFRCO for down-counter
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DOWNSEL::Auxhfrco
    }
    ///Select PRS input selected by PRSDOWNSEL as down-counter
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == DOWNSEL::Prs
    }
}
///Field `DOWNSEL` writer - Calibration Down-counter Select
pub type DownselW<'a, REG> = crate::FieldWriter<'a, REG, 3, DOWNSEL>;
impl<'a, REG> DownselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select HFCLK for down-counter
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(DOWNSEL::Hfclk)
    }
    ///Select HFXO for down-counter
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(DOWNSEL::Hfxo)
    }
    ///Select LFXO for down-counter
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(DOWNSEL::Lfxo)
    }
    ///Select HFRCO for down-counter
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(DOWNSEL::Hfrco)
    }
    ///Select LFRCO for down-counter
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(DOWNSEL::Lfrco)
    }
    ///Select AUXHFRCO for down-counter
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(DOWNSEL::Auxhfrco)
    }
    ///Select PRS input selected by PRSDOWNSEL as down-counter
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(DOWNSEL::Prs)
    }
}
///Field `CONT` reader - Continuous Calibration
pub type ContR = crate::BitReader;
///Field `CONT` writer - Continuous Calibration
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
///PRS Select for PRS Input When Selected in UPSEL
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSUPSEL {
    ///0: PRS Channel 0 selected as input
    Prsch0 = 0,
    ///1: PRS Channel 1 selected as input
    Prsch1 = 1,
    ///2: PRS Channel 2 selected as input
    Prsch2 = 2,
    ///3: PRS Channel 3 selected as input
    Prsch3 = 3,
    ///4: PRS Channel 4 selected as input
    Prsch4 = 4,
    ///5: PRS Channel 5 selected as input
    Prsch5 = 5,
    ///6: PRS Channel 6 selected as input
    Prsch6 = 6,
    ///7: PRS Channel 7 selected as input
    Prsch7 = 7,
    ///8: PRS Channel 8 selected as input
    Prsch8 = 8,
    ///9: PRS Channel 9 selected as input
    Prsch9 = 9,
    ///10: PRS Channel 10 selected as input
    Prsch10 = 10,
    ///11: PRS Channel 11 selected as input
    Prsch11 = 11,
}
impl From<PRSUPSEL> for u8 {
    #[inline(always)]
    fn from(variant: PRSUPSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSUPSEL {
    type Ux = u8;
}
impl crate::IsEnum for PRSUPSEL {}
///Field `PRSUPSEL` reader - PRS Select for PRS Input When Selected in UPSEL
pub type PrsupselR = crate::FieldReader<PRSUPSEL>;
impl PrsupselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSUPSEL> {
        match self.bits {
            0 => Some(PRSUPSEL::Prsch0),
            1 => Some(PRSUPSEL::Prsch1),
            2 => Some(PRSUPSEL::Prsch2),
            3 => Some(PRSUPSEL::Prsch3),
            4 => Some(PRSUPSEL::Prsch4),
            5 => Some(PRSUPSEL::Prsch5),
            6 => Some(PRSUPSEL::Prsch6),
            7 => Some(PRSUPSEL::Prsch7),
            8 => Some(PRSUPSEL::Prsch8),
            9 => Some(PRSUPSEL::Prsch9),
            10 => Some(PRSUPSEL::Prsch10),
            11 => Some(PRSUPSEL::Prsch11),
            _ => None,
        }
    }
    ///PRS Channel 0 selected as input
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSUPSEL::Prsch0
    }
    ///PRS Channel 1 selected as input
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSUPSEL::Prsch1
    }
    ///PRS Channel 2 selected as input
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSUPSEL::Prsch2
    }
    ///PRS Channel 3 selected as input
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSUPSEL::Prsch3
    }
    ///PRS Channel 4 selected as input
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSUPSEL::Prsch4
    }
    ///PRS Channel 5 selected as input
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSUPSEL::Prsch5
    }
    ///PRS Channel 6 selected as input
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSUPSEL::Prsch6
    }
    ///PRS Channel 7 selected as input
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSUPSEL::Prsch7
    }
    ///PRS Channel 8 selected as input
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSUPSEL::Prsch8
    }
    ///PRS Channel 9 selected as input
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSUPSEL::Prsch9
    }
    ///PRS Channel 10 selected as input
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSUPSEL::Prsch10
    }
    ///PRS Channel 11 selected as input
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSUPSEL::Prsch11
    }
}
///Field `PRSUPSEL` writer - PRS Select for PRS Input When Selected in UPSEL
pub type PrsupselW<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSUPSEL>;
impl<'a, REG> PrsupselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected as input
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSUPSEL::Prsch0)
    }
    ///PRS Channel 1 selected as input
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSUPSEL::Prsch1)
    }
    ///PRS Channel 2 selected as input
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSUPSEL::Prsch2)
    }
    ///PRS Channel 3 selected as input
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSUPSEL::Prsch3)
    }
    ///PRS Channel 4 selected as input
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSUPSEL::Prsch4)
    }
    ///PRS Channel 5 selected as input
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSUPSEL::Prsch5)
    }
    ///PRS Channel 6 selected as input
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSUPSEL::Prsch6)
    }
    ///PRS Channel 7 selected as input
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSUPSEL::Prsch7)
    }
    ///PRS Channel 8 selected as input
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSUPSEL::Prsch8)
    }
    ///PRS Channel 9 selected as input
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSUPSEL::Prsch9)
    }
    ///PRS Channel 10 selected as input
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSUPSEL::Prsch10)
    }
    ///PRS Channel 11 selected as input
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSUPSEL::Prsch11)
    }
}
///PRS Select for PRS Input When Selected in DOWNSEL
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSDOWNSEL {
    ///0: PRS Channel 0 selected as input
    Prsch0 = 0,
    ///1: PRS Channel 1 selected as input
    Prsch1 = 1,
    ///2: PRS Channel 2 selected as input
    Prsch2 = 2,
    ///3: PRS Channel 3 selected as input
    Prsch3 = 3,
    ///4: PRS Channel 4 selected as input
    Prsch4 = 4,
    ///5: PRS Channel 5 selected as input
    Prsch5 = 5,
    ///6: PRS Channel 6 selected as input
    Prsch6 = 6,
    ///7: PRS Channel 7 selected as input
    Prsch7 = 7,
    ///8: PRS Channel 8 selected as input
    Prsch8 = 8,
    ///9: PRS Channel 9 selected as input
    Prsch9 = 9,
    ///10: PRS Channel 10 selected as input
    Prsch10 = 10,
    ///11: PRS Channel 11 selected as input
    Prsch11 = 11,
}
impl From<PRSDOWNSEL> for u8 {
    #[inline(always)]
    fn from(variant: PRSDOWNSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSDOWNSEL {
    type Ux = u8;
}
impl crate::IsEnum for PRSDOWNSEL {}
///Field `PRSDOWNSEL` reader - PRS Select for PRS Input When Selected in DOWNSEL
pub type PrsdownselR = crate::FieldReader<PRSDOWNSEL>;
impl PrsdownselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSDOWNSEL> {
        match self.bits {
            0 => Some(PRSDOWNSEL::Prsch0),
            1 => Some(PRSDOWNSEL::Prsch1),
            2 => Some(PRSDOWNSEL::Prsch2),
            3 => Some(PRSDOWNSEL::Prsch3),
            4 => Some(PRSDOWNSEL::Prsch4),
            5 => Some(PRSDOWNSEL::Prsch5),
            6 => Some(PRSDOWNSEL::Prsch6),
            7 => Some(PRSDOWNSEL::Prsch7),
            8 => Some(PRSDOWNSEL::Prsch8),
            9 => Some(PRSDOWNSEL::Prsch9),
            10 => Some(PRSDOWNSEL::Prsch10),
            11 => Some(PRSDOWNSEL::Prsch11),
            _ => None,
        }
    }
    ///PRS Channel 0 selected as input
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSDOWNSEL::Prsch0
    }
    ///PRS Channel 1 selected as input
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSDOWNSEL::Prsch1
    }
    ///PRS Channel 2 selected as input
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSDOWNSEL::Prsch2
    }
    ///PRS Channel 3 selected as input
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSDOWNSEL::Prsch3
    }
    ///PRS Channel 4 selected as input
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSDOWNSEL::Prsch4
    }
    ///PRS Channel 5 selected as input
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSDOWNSEL::Prsch5
    }
    ///PRS Channel 6 selected as input
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSDOWNSEL::Prsch6
    }
    ///PRS Channel 7 selected as input
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSDOWNSEL::Prsch7
    }
    ///PRS Channel 8 selected as input
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSDOWNSEL::Prsch8
    }
    ///PRS Channel 9 selected as input
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSDOWNSEL::Prsch9
    }
    ///PRS Channel 10 selected as input
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSDOWNSEL::Prsch10
    }
    ///PRS Channel 11 selected as input
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSDOWNSEL::Prsch11
    }
}
///Field `PRSDOWNSEL` writer - PRS Select for PRS Input When Selected in DOWNSEL
pub type PrsdownselW<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSDOWNSEL>;
impl<'a, REG> PrsdownselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected as input
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSDOWNSEL::Prsch0)
    }
    ///PRS Channel 1 selected as input
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSDOWNSEL::Prsch1)
    }
    ///PRS Channel 2 selected as input
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSDOWNSEL::Prsch2)
    }
    ///PRS Channel 3 selected as input
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSDOWNSEL::Prsch3)
    }
    ///PRS Channel 4 selected as input
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSDOWNSEL::Prsch4)
    }
    ///PRS Channel 5 selected as input
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSDOWNSEL::Prsch5)
    }
    ///PRS Channel 6 selected as input
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSDOWNSEL::Prsch6)
    }
    ///PRS Channel 7 selected as input
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSDOWNSEL::Prsch7)
    }
    ///PRS Channel 8 selected as input
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSDOWNSEL::Prsch8)
    }
    ///PRS Channel 9 selected as input
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSDOWNSEL::Prsch9)
    }
    ///PRS Channel 10 selected as input
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSDOWNSEL::Prsch10)
    }
    ///PRS Channel 11 selected as input
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSDOWNSEL::Prsch11)
    }
}
impl R {
    ///Bits 0:2 - Calibration Up-counter Select
    #[inline(always)]
    pub fn upsel(&self) -> UpselR {
        UpselR::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Calibration Down-counter Select
    #[inline(always)]
    pub fn downsel(&self) -> DownselR {
        DownselR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - Continuous Calibration
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:19 - PRS Select for PRS Input When Selected in UPSEL
    #[inline(always)]
    pub fn prsupsel(&self) -> PrsupselR {
        PrsupselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:27 - PRS Select for PRS Input When Selected in DOWNSEL
    #[inline(always)]
    pub fn prsdownsel(&self) -> PrsdownselR {
        PrsdownselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALCTRL")
            .field("upsel", &self.upsel())
            .field("downsel", &self.downsel())
            .field("cont", &self.cont())
            .field("prsupsel", &self.prsupsel())
            .field("prsdownsel", &self.prsdownsel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Calibration Up-counter Select
    #[inline(always)]
    #[must_use]
    pub fn upsel(&mut self) -> UpselW<CALCTRLrs> {
        UpselW::new(self, 0)
    }
    ///Bits 4:6 - Calibration Down-counter Select
    #[inline(always)]
    #[must_use]
    pub fn downsel(&mut self) -> DownselW<CALCTRLrs> {
        DownselW::new(self, 4)
    }
    ///Bit 8 - Continuous Calibration
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> ContW<CALCTRLrs> {
        ContW::new(self, 8)
    }
    ///Bits 16:19 - PRS Select for PRS Input When Selected in UPSEL
    #[inline(always)]
    #[must_use]
    pub fn prsupsel(&mut self) -> PrsupselW<CALCTRLrs> {
        PrsupselW::new(self, 16)
    }
    ///Bits 24:27 - PRS Select for PRS Input When Selected in DOWNSEL
    #[inline(always)]
    #[must_use]
    pub fn prsdownsel(&mut self) -> PrsdownselW<CALCTRLrs> {
        PrsdownselW::new(self, 24)
    }
}
///Calibration Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`calctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CALCTRLrs;
impl crate::RegisterSpec for CALCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`calctrl::R`](R) reader structure
impl crate::Readable for CALCTRLrs {}
///`write(|w| ..)` method takes [`calctrl::W`](W) writer structure
impl crate::Writable for CALCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CALCTRL to value 0
impl crate::Resettable for CALCTRLrs {
    const RESET_VALUE: u32 = 0;
}
