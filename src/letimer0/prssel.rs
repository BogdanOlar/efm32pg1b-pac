///Register `PRSSEL` reader
pub type R = crate::R<PRSSELrs>;
///Register `PRSSEL` writer
pub type W = crate::W<PRSSELrs>;
///PRS Start Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTARTSEL {
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
impl From<PRSSTARTSEL> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTARTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSTARTSEL {
    type Ux = u8;
}
impl crate::IsEnum for PRSSTARTSEL {}
///Field `PRSSTARTSEL` reader - PRS Start Select
pub type PrsstartselR = crate::FieldReader<PRSSTARTSEL>;
impl PrsstartselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSTARTSEL> {
        match self.bits {
            0 => Some(PRSSTARTSEL::Prsch0),
            1 => Some(PRSSTARTSEL::Prsch1),
            2 => Some(PRSSTARTSEL::Prsch2),
            3 => Some(PRSSTARTSEL::Prsch3),
            4 => Some(PRSSTARTSEL::Prsch4),
            5 => Some(PRSSTARTSEL::Prsch5),
            6 => Some(PRSSTARTSEL::Prsch6),
            7 => Some(PRSSTARTSEL::Prsch7),
            8 => Some(PRSSTARTSEL::Prsch8),
            9 => Some(PRSSTARTSEL::Prsch9),
            10 => Some(PRSSTARTSEL::Prsch10),
            11 => Some(PRSSTARTSEL::Prsch11),
            _ => None,
        }
    }
    ///PRS Channel 0 selected as input
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSTARTSEL::Prsch0
    }
    ///PRS Channel 1 selected as input
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSTARTSEL::Prsch1
    }
    ///PRS Channel 2 selected as input
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSTARTSEL::Prsch2
    }
    ///PRS Channel 3 selected as input
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSTARTSEL::Prsch3
    }
    ///PRS Channel 4 selected as input
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSTARTSEL::Prsch4
    }
    ///PRS Channel 5 selected as input
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSTARTSEL::Prsch5
    }
    ///PRS Channel 6 selected as input
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSTARTSEL::Prsch6
    }
    ///PRS Channel 7 selected as input
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSTARTSEL::Prsch7
    }
    ///PRS Channel 8 selected as input
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSTARTSEL::Prsch8
    }
    ///PRS Channel 9 selected as input
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSTARTSEL::Prsch9
    }
    ///PRS Channel 10 selected as input
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSTARTSEL::Prsch10
    }
    ///PRS Channel 11 selected as input
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSTARTSEL::Prsch11
    }
}
///Field `PRSSTARTSEL` writer - PRS Start Select
pub type PrsstartselW<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSSTARTSEL>;
impl<'a, REG> PrsstartselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected as input
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch0)
    }
    ///PRS Channel 1 selected as input
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch1)
    }
    ///PRS Channel 2 selected as input
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch2)
    }
    ///PRS Channel 3 selected as input
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch3)
    }
    ///PRS Channel 4 selected as input
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch4)
    }
    ///PRS Channel 5 selected as input
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch5)
    }
    ///PRS Channel 6 selected as input
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch6)
    }
    ///PRS Channel 7 selected as input
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch7)
    }
    ///PRS Channel 8 selected as input
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch8)
    }
    ///PRS Channel 9 selected as input
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch9)
    }
    ///PRS Channel 10 selected as input
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch10)
    }
    ///PRS Channel 11 selected as input
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch11)
    }
}
///PRS Stop Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTOPSEL {
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
impl From<PRSSTOPSEL> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTOPSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSTOPSEL {
    type Ux = u8;
}
impl crate::IsEnum for PRSSTOPSEL {}
///Field `PRSSTOPSEL` reader - PRS Stop Select
pub type PrsstopselR = crate::FieldReader<PRSSTOPSEL>;
impl PrsstopselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSTOPSEL> {
        match self.bits {
            0 => Some(PRSSTOPSEL::Prsch0),
            1 => Some(PRSSTOPSEL::Prsch1),
            2 => Some(PRSSTOPSEL::Prsch2),
            3 => Some(PRSSTOPSEL::Prsch3),
            4 => Some(PRSSTOPSEL::Prsch4),
            5 => Some(PRSSTOPSEL::Prsch5),
            6 => Some(PRSSTOPSEL::Prsch6),
            7 => Some(PRSSTOPSEL::Prsch7),
            8 => Some(PRSSTOPSEL::Prsch8),
            9 => Some(PRSSTOPSEL::Prsch9),
            10 => Some(PRSSTOPSEL::Prsch10),
            11 => Some(PRSSTOPSEL::Prsch11),
            _ => None,
        }
    }
    ///PRS Channel 0 selected as input
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSTOPSEL::Prsch0
    }
    ///PRS Channel 1 selected as input
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSTOPSEL::Prsch1
    }
    ///PRS Channel 2 selected as input
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSTOPSEL::Prsch2
    }
    ///PRS Channel 3 selected as input
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSTOPSEL::Prsch3
    }
    ///PRS Channel 4 selected as input
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSTOPSEL::Prsch4
    }
    ///PRS Channel 5 selected as input
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSTOPSEL::Prsch5
    }
    ///PRS Channel 6 selected as input
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSTOPSEL::Prsch6
    }
    ///PRS Channel 7 selected as input
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSTOPSEL::Prsch7
    }
    ///PRS Channel 8 selected as input
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSTOPSEL::Prsch8
    }
    ///PRS Channel 9 selected as input
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSTOPSEL::Prsch9
    }
    ///PRS Channel 10 selected as input
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSTOPSEL::Prsch10
    }
    ///PRS Channel 11 selected as input
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSTOPSEL::Prsch11
    }
}
///Field `PRSSTOPSEL` writer - PRS Stop Select
pub type PrsstopselW<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSSTOPSEL>;
impl<'a, REG> PrsstopselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected as input
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch0)
    }
    ///PRS Channel 1 selected as input
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch1)
    }
    ///PRS Channel 2 selected as input
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch2)
    }
    ///PRS Channel 3 selected as input
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch3)
    }
    ///PRS Channel 4 selected as input
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch4)
    }
    ///PRS Channel 5 selected as input
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch5)
    }
    ///PRS Channel 6 selected as input
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch6)
    }
    ///PRS Channel 7 selected as input
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch7)
    }
    ///PRS Channel 8 selected as input
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch8)
    }
    ///PRS Channel 9 selected as input
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch9)
    }
    ///PRS Channel 10 selected as input
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch10)
    }
    ///PRS Channel 11 selected as input
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch11)
    }
}
///PRS Clear Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSCLEARSEL {
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
impl From<PRSCLEARSEL> for u8 {
    #[inline(always)]
    fn from(variant: PRSCLEARSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSCLEARSEL {
    type Ux = u8;
}
impl crate::IsEnum for PRSCLEARSEL {}
///Field `PRSCLEARSEL` reader - PRS Clear Select
pub type PrsclearselR = crate::FieldReader<PRSCLEARSEL>;
impl PrsclearselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSCLEARSEL> {
        match self.bits {
            0 => Some(PRSCLEARSEL::Prsch0),
            1 => Some(PRSCLEARSEL::Prsch1),
            2 => Some(PRSCLEARSEL::Prsch2),
            3 => Some(PRSCLEARSEL::Prsch3),
            4 => Some(PRSCLEARSEL::Prsch4),
            5 => Some(PRSCLEARSEL::Prsch5),
            6 => Some(PRSCLEARSEL::Prsch6),
            7 => Some(PRSCLEARSEL::Prsch7),
            8 => Some(PRSCLEARSEL::Prsch8),
            9 => Some(PRSCLEARSEL::Prsch9),
            10 => Some(PRSCLEARSEL::Prsch10),
            11 => Some(PRSCLEARSEL::Prsch11),
            _ => None,
        }
    }
    ///PRS Channel 0 selected as input
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSCLEARSEL::Prsch0
    }
    ///PRS Channel 1 selected as input
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSCLEARSEL::Prsch1
    }
    ///PRS Channel 2 selected as input
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSCLEARSEL::Prsch2
    }
    ///PRS Channel 3 selected as input
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSCLEARSEL::Prsch3
    }
    ///PRS Channel 4 selected as input
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSCLEARSEL::Prsch4
    }
    ///PRS Channel 5 selected as input
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSCLEARSEL::Prsch5
    }
    ///PRS Channel 6 selected as input
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSCLEARSEL::Prsch6
    }
    ///PRS Channel 7 selected as input
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSCLEARSEL::Prsch7
    }
    ///PRS Channel 8 selected as input
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSCLEARSEL::Prsch8
    }
    ///PRS Channel 9 selected as input
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSCLEARSEL::Prsch9
    }
    ///PRS Channel 10 selected as input
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSCLEARSEL::Prsch10
    }
    ///PRS Channel 11 selected as input
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSCLEARSEL::Prsch11
    }
}
///Field `PRSCLEARSEL` writer - PRS Clear Select
pub type PrsclearselW<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSCLEARSEL>;
impl<'a, REG> PrsclearselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected as input
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch0)
    }
    ///PRS Channel 1 selected as input
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch1)
    }
    ///PRS Channel 2 selected as input
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch2)
    }
    ///PRS Channel 3 selected as input
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch3)
    }
    ///PRS Channel 4 selected as input
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch4)
    }
    ///PRS Channel 5 selected as input
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch5)
    }
    ///PRS Channel 6 selected as input
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch6)
    }
    ///PRS Channel 7 selected as input
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch7)
    }
    ///PRS Channel 8 selected as input
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch8)
    }
    ///PRS Channel 9 selected as input
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch9)
    }
    ///PRS Channel 10 selected as input
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch10)
    }
    ///PRS Channel 11 selected as input
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch11)
    }
}
///PRS Start Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTARTMODE {
    ///0: PRS cannot start the LETIMER
    None = 0,
    ///1: Rising edge of selected PRS input can start the LETIMER
    Rising = 1,
    ///2: Falling edge of selected PRS input can start the LETIMER
    Falling = 2,
    ///3: Both the rising or falling edge of the selected PRS input can start the LETIMER
    Both = 3,
}
impl From<PRSSTARTMODE> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTARTMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSTARTMODE {
    type Ux = u8;
}
impl crate::IsEnum for PRSSTARTMODE {}
///Field `PRSSTARTMODE` reader - PRS Start Mode
pub type PrsstartmodeR = crate::FieldReader<PRSSTARTMODE>;
impl PrsstartmodeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRSSTARTMODE {
        match self.bits {
            0 => PRSSTARTMODE::None,
            1 => PRSSTARTMODE::Rising,
            2 => PRSSTARTMODE::Falling,
            3 => PRSSTARTMODE::Both,
            _ => unreachable!(),
        }
    }
    ///PRS cannot start the LETIMER
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSSTARTMODE::None
    }
    ///Rising edge of selected PRS input can start the LETIMER
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSSTARTMODE::Rising
    }
    ///Falling edge of selected PRS input can start the LETIMER
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSSTARTMODE::Falling
    }
    ///Both the rising or falling edge of the selected PRS input can start the LETIMER
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSSTARTMODE::Both
    }
}
///Field `PRSSTARTMODE` writer - PRS Start Mode
pub type PrsstartmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, PRSSTARTMODE, crate::Safe>;
impl<'a, REG> PrsstartmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS cannot start the LETIMER
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTMODE::None)
    }
    ///Rising edge of selected PRS input can start the LETIMER
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTMODE::Rising)
    }
    ///Falling edge of selected PRS input can start the LETIMER
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTMODE::Falling)
    }
    ///Both the rising or falling edge of the selected PRS input can start the LETIMER
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTMODE::Both)
    }
}
///PRS Stop Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTOPMODE {
    ///0: PRS cannot stop the LETIMER
    None = 0,
    ///1: Rising edge of selected PRS input can stop the LETIMER
    Rising = 1,
    ///2: Falling edge of selected PRS input can stop the LETIMER
    Falling = 2,
    ///3: Both the rising or falling edge of the selected PRS input can stop the LETIMER
    Both = 3,
}
impl From<PRSSTOPMODE> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTOPMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSTOPMODE {
    type Ux = u8;
}
impl crate::IsEnum for PRSSTOPMODE {}
///Field `PRSSTOPMODE` reader - PRS Stop Mode
pub type PrsstopmodeR = crate::FieldReader<PRSSTOPMODE>;
impl PrsstopmodeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRSSTOPMODE {
        match self.bits {
            0 => PRSSTOPMODE::None,
            1 => PRSSTOPMODE::Rising,
            2 => PRSSTOPMODE::Falling,
            3 => PRSSTOPMODE::Both,
            _ => unreachable!(),
        }
    }
    ///PRS cannot stop the LETIMER
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSSTOPMODE::None
    }
    ///Rising edge of selected PRS input can stop the LETIMER
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSSTOPMODE::Rising
    }
    ///Falling edge of selected PRS input can stop the LETIMER
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSSTOPMODE::Falling
    }
    ///Both the rising or falling edge of the selected PRS input can stop the LETIMER
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSSTOPMODE::Both
    }
}
///Field `PRSSTOPMODE` writer - PRS Stop Mode
pub type PrsstopmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, PRSSTOPMODE, crate::Safe>;
impl<'a, REG> PrsstopmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS cannot stop the LETIMER
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPMODE::None)
    }
    ///Rising edge of selected PRS input can stop the LETIMER
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPMODE::Rising)
    }
    ///Falling edge of selected PRS input can stop the LETIMER
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPMODE::Falling)
    }
    ///Both the rising or falling edge of the selected PRS input can stop the LETIMER
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPMODE::Both)
    }
}
///PRS Clear Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSCLEARMODE {
    ///0: PRS cannot clear the LETIMER
    None = 0,
    ///1: Rising edge of selected PRS input can clear the LETIMER
    Rising = 1,
    ///2: Falling edge of selected PRS input can clear the LETIMER
    Falling = 2,
    ///3: Both the rising or falling edge of the selected PRS input can clear the LETIMER
    Both = 3,
}
impl From<PRSCLEARMODE> for u8 {
    #[inline(always)]
    fn from(variant: PRSCLEARMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSCLEARMODE {
    type Ux = u8;
}
impl crate::IsEnum for PRSCLEARMODE {}
///Field `PRSCLEARMODE` reader - PRS Clear Mode
pub type PrsclearmodeR = crate::FieldReader<PRSCLEARMODE>;
impl PrsclearmodeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRSCLEARMODE {
        match self.bits {
            0 => PRSCLEARMODE::None,
            1 => PRSCLEARMODE::Rising,
            2 => PRSCLEARMODE::Falling,
            3 => PRSCLEARMODE::Both,
            _ => unreachable!(),
        }
    }
    ///PRS cannot clear the LETIMER
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSCLEARMODE::None
    }
    ///Rising edge of selected PRS input can clear the LETIMER
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSCLEARMODE::Rising
    }
    ///Falling edge of selected PRS input can clear the LETIMER
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSCLEARMODE::Falling
    }
    ///Both the rising or falling edge of the selected PRS input can clear the LETIMER
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSCLEARMODE::Both
    }
}
///Field `PRSCLEARMODE` writer - PRS Clear Mode
pub type PrsclearmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, PRSCLEARMODE, crate::Safe>;
impl<'a, REG> PrsclearmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS cannot clear the LETIMER
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARMODE::None)
    }
    ///Rising edge of selected PRS input can clear the LETIMER
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARMODE::Rising)
    }
    ///Falling edge of selected PRS input can clear the LETIMER
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARMODE::Falling)
    }
    ///Both the rising or falling edge of the selected PRS input can clear the LETIMER
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARMODE::Both)
    }
}
impl R {
    ///Bits 0:3 - PRS Start Select
    #[inline(always)]
    pub fn prsstartsel(&self) -> PrsstartselR {
        PrsstartselR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 6:9 - PRS Stop Select
    #[inline(always)]
    pub fn prsstopsel(&self) -> PrsstopselR {
        PrsstopselR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    ///Bits 12:15 - PRS Clear Select
    #[inline(always)]
    pub fn prsclearsel(&self) -> PrsclearselR {
        PrsclearselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 18:19 - PRS Start Mode
    #[inline(always)]
    pub fn prsstartmode(&self) -> PrsstartmodeR {
        PrsstartmodeR::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 22:23 - PRS Stop Mode
    #[inline(always)]
    pub fn prsstopmode(&self) -> PrsstopmodeR {
        PrsstopmodeR::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 26:27 - PRS Clear Mode
    #[inline(always)]
    pub fn prsclearmode(&self) -> PrsclearmodeR {
        PrsclearmodeR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRSSEL")
            .field("prsstartsel", &self.prsstartsel())
            .field("prsstopsel", &self.prsstopsel())
            .field("prsclearsel", &self.prsclearsel())
            .field("prsstartmode", &self.prsstartmode())
            .field("prsstopmode", &self.prsstopmode())
            .field("prsclearmode", &self.prsclearmode())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - PRS Start Select
    #[inline(always)]
    #[must_use]
    pub fn prsstartsel(&mut self) -> PrsstartselW<PRSSELrs> {
        PrsstartselW::new(self, 0)
    }
    ///Bits 6:9 - PRS Stop Select
    #[inline(always)]
    #[must_use]
    pub fn prsstopsel(&mut self) -> PrsstopselW<PRSSELrs> {
        PrsstopselW::new(self, 6)
    }
    ///Bits 12:15 - PRS Clear Select
    #[inline(always)]
    #[must_use]
    pub fn prsclearsel(&mut self) -> PrsclearselW<PRSSELrs> {
        PrsclearselW::new(self, 12)
    }
    ///Bits 18:19 - PRS Start Mode
    #[inline(always)]
    #[must_use]
    pub fn prsstartmode(&mut self) -> PrsstartmodeW<PRSSELrs> {
        PrsstartmodeW::new(self, 18)
    }
    ///Bits 22:23 - PRS Stop Mode
    #[inline(always)]
    #[must_use]
    pub fn prsstopmode(&mut self) -> PrsstopmodeW<PRSSELrs> {
        PrsstopmodeW::new(self, 22)
    }
    ///Bits 26:27 - PRS Clear Mode
    #[inline(always)]
    #[must_use]
    pub fn prsclearmode(&mut self) -> PrsclearmodeW<PRSSELrs> {
        PrsclearmodeW::new(self, 26)
    }
}
///PRS Input Select Register
///
///You can [`read`](crate::Reg::read) this register and get [`prssel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prssel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PRSSELrs;
impl crate::RegisterSpec for PRSSELrs {
    type Ux = u32;
}
///`read()` method returns [`prssel::R`](R) reader structure
impl crate::Readable for PRSSELrs {}
///`write(|w| ..)` method takes [`prssel::W`](W) writer structure
impl crate::Writable for PRSSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRSSEL to value 0
impl crate::Resettable for PRSSELrs {
    const RESET_VALUE: u32 = 0;
}
