#[doc = "Register `PRSSEL` reader"]
pub type R = crate::R<PRSSELrs>;
#[doc = "Register `PRSSEL` writer"]
pub type W = crate::W<PRSSELrs>;
#[doc = "Field `PRSSTARTSEL` reader - PRS Start Select"]
pub type PRSSTARTSEL_R = crate::FieldReader<PRSSTARTSEL>;
#[doc = "PRS Start Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTARTSEL {
    #[doc = "0: PRS Channel 0 selected as input"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
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
impl PRSSTARTSEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSTARTSEL::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSTARTSEL::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSTARTSEL::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSTARTSEL::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSTARTSEL::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSTARTSEL::Prsch5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSTARTSEL::Prsch6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSTARTSEL::Prsch7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSTARTSEL::Prsch8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSTARTSEL::Prsch9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSTARTSEL::Prsch10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSTARTSEL::Prsch11
    }
}
#[doc = "Field `PRSSTARTSEL` writer - PRS Start Select"]
pub type PRSSTARTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSSTARTSEL>;
impl<'a, REG> PRSSTARTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL::Prsch11)
    }
}
#[doc = "Field `PRSSTOPSEL` reader - PRS Stop Select"]
pub type PRSSTOPSEL_R = crate::FieldReader<PRSSTOPSEL>;
#[doc = "PRS Stop Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTOPSEL {
    #[doc = "0: PRS Channel 0 selected as input"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
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
impl PRSSTOPSEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSTOPSEL::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSTOPSEL::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSTOPSEL::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSTOPSEL::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSTOPSEL::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSTOPSEL::Prsch5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSTOPSEL::Prsch6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSTOPSEL::Prsch7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSTOPSEL::Prsch8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSTOPSEL::Prsch9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSTOPSEL::Prsch10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSTOPSEL::Prsch11
    }
}
#[doc = "Field `PRSSTOPSEL` writer - PRS Stop Select"]
pub type PRSSTOPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSSTOPSEL>;
impl<'a, REG> PRSSTOPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL::Prsch11)
    }
}
#[doc = "Field `PRSCLEARSEL` reader - PRS Clear Select"]
pub type PRSCLEARSEL_R = crate::FieldReader<PRSCLEARSEL>;
#[doc = "PRS Clear Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSCLEARSEL {
    #[doc = "0: PRS Channel 0 selected as input"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
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
impl PRSCLEARSEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSCLEARSEL::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSCLEARSEL::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSCLEARSEL::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSCLEARSEL::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSCLEARSEL::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSCLEARSEL::Prsch5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSCLEARSEL::Prsch6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSCLEARSEL::Prsch7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSCLEARSEL::Prsch8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSCLEARSEL::Prsch9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSCLEARSEL::Prsch10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSCLEARSEL::Prsch11
    }
}
#[doc = "Field `PRSCLEARSEL` writer - PRS Clear Select"]
pub type PRSCLEARSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSCLEARSEL>;
impl<'a, REG> PRSCLEARSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL::Prsch11)
    }
}
#[doc = "Field `PRSSTARTMODE` reader - PRS Start Mode"]
pub type PRSSTARTMODE_R = crate::FieldReader<PRSSTARTMODE>;
#[doc = "PRS Start Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTARTMODE {
    #[doc = "0: PRS cannot start the LETIMER"]
    None = 0,
    #[doc = "1: Rising edge of selected PRS input can start the LETIMER"]
    Rising = 1,
    #[doc = "2: Falling edge of selected PRS input can start the LETIMER"]
    Falling = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can start the LETIMER"]
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
impl PRSSTARTMODE_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "PRS cannot start the LETIMER"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSSTARTMODE::None
    }
    #[doc = "Rising edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSSTARTMODE::Rising
    }
    #[doc = "Falling edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSSTARTMODE::Falling
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSSTARTMODE::Both
    }
}
#[doc = "Field `PRSSTARTMODE` writer - PRS Start Mode"]
pub type PRSSTARTMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PRSSTARTMODE>;
impl<'a, REG> PRSSTARTMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS cannot start the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTMODE::None)
    }
    #[doc = "Rising edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTMODE::Rising)
    }
    #[doc = "Falling edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTMODE::Falling)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTMODE::Both)
    }
}
#[doc = "Field `PRSSTOPMODE` reader - PRS Stop Mode"]
pub type PRSSTOPMODE_R = crate::FieldReader<PRSSTOPMODE>;
#[doc = "PRS Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTOPMODE {
    #[doc = "0: PRS cannot stop the LETIMER"]
    None = 0,
    #[doc = "1: Rising edge of selected PRS input can stop the LETIMER"]
    Rising = 1,
    #[doc = "2: Falling edge of selected PRS input can stop the LETIMER"]
    Falling = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
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
impl PRSSTOPMODE_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "PRS cannot stop the LETIMER"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSSTOPMODE::None
    }
    #[doc = "Rising edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSSTOPMODE::Rising
    }
    #[doc = "Falling edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSSTOPMODE::Falling
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSSTOPMODE::Both
    }
}
#[doc = "Field `PRSSTOPMODE` writer - PRS Stop Mode"]
pub type PRSSTOPMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PRSSTOPMODE>;
impl<'a, REG> PRSSTOPMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS cannot stop the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPMODE::None)
    }
    #[doc = "Rising edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPMODE::Rising)
    }
    #[doc = "Falling edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPMODE::Falling)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPMODE::Both)
    }
}
#[doc = "Field `PRSCLEARMODE` reader - PRS Clear Mode"]
pub type PRSCLEARMODE_R = crate::FieldReader<PRSCLEARMODE>;
#[doc = "PRS Clear Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSCLEARMODE {
    #[doc = "0: PRS cannot clear the LETIMER"]
    None = 0,
    #[doc = "1: Rising edge of selected PRS input can clear the LETIMER"]
    Rising = 1,
    #[doc = "2: Falling edge of selected PRS input can clear the LETIMER"]
    Falling = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
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
impl PRSCLEARMODE_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "PRS cannot clear the LETIMER"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSCLEARMODE::None
    }
    #[doc = "Rising edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSCLEARMODE::Rising
    }
    #[doc = "Falling edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSCLEARMODE::Falling
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSCLEARMODE::Both
    }
}
#[doc = "Field `PRSCLEARMODE` writer - PRS Clear Mode"]
pub type PRSCLEARMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PRSCLEARMODE>;
impl<'a, REG> PRSCLEARMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS cannot clear the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARMODE::None)
    }
    #[doc = "Rising edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARMODE::Rising)
    }
    #[doc = "Falling edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARMODE::Falling)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARMODE::Both)
    }
}
impl R {
    #[doc = "Bits 0:3 - PRS Start Select"]
    #[inline(always)]
    pub fn prsstartsel(&self) -> PRSSTARTSEL_R {
        PRSSTARTSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - PRS Stop Select"]
    #[inline(always)]
    pub fn prsstopsel(&self) -> PRSSTOPSEL_R {
        PRSSTOPSEL_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PRS Clear Select"]
    #[inline(always)]
    pub fn prsclearsel(&self) -> PRSCLEARSEL_R {
        PRSCLEARSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19 - PRS Start Mode"]
    #[inline(always)]
    pub fn prsstartmode(&self) -> PRSSTARTMODE_R {
        PRSSTARTMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PRS Stop Mode"]
    #[inline(always)]
    pub fn prsstopmode(&self) -> PRSSTOPMODE_R {
        PRSSTOPMODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PRS Clear Mode"]
    #[inline(always)]
    pub fn prsclearmode(&self) -> PRSCLEARMODE_R {
        PRSCLEARMODE_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PRS Start Select"]
    #[inline(always)]
    #[must_use]
    pub fn prsstartsel(&mut self) -> PRSSTARTSEL_W<PRSSELrs> {
        PRSSTARTSEL_W::new(self, 0)
    }
    #[doc = "Bits 6:9 - PRS Stop Select"]
    #[inline(always)]
    #[must_use]
    pub fn prsstopsel(&mut self) -> PRSSTOPSEL_W<PRSSELrs> {
        PRSSTOPSEL_W::new(self, 6)
    }
    #[doc = "Bits 12:15 - PRS Clear Select"]
    #[inline(always)]
    #[must_use]
    pub fn prsclearsel(&mut self) -> PRSCLEARSEL_W<PRSSELrs> {
        PRSCLEARSEL_W::new(self, 12)
    }
    #[doc = "Bits 18:19 - PRS Start Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsstartmode(&mut self) -> PRSSTARTMODE_W<PRSSELrs> {
        PRSSTARTMODE_W::new(self, 18)
    }
    #[doc = "Bits 22:23 - PRS Stop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsstopmode(&mut self) -> PRSSTOPMODE_W<PRSSELrs> {
        PRSSTOPMODE_W::new(self, 22)
    }
    #[doc = "Bits 26:27 - PRS Clear Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsclearmode(&mut self) -> PRSCLEARMODE_W<PRSSELrs> {
        PRSCLEARMODE_W::new(self, 26)
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
#[doc = "PRS Input Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSSELrs;
impl crate::RegisterSpec for PRSSELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prssel::R`](R) reader structure"]
impl crate::Readable for PRSSELrs {}
#[doc = "`write(|w| ..)` method takes [`prssel::W`](W) writer structure"]
impl crate::Writable for PRSSELrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSSEL to value 0"]
impl crate::Resettable for PRSSELrs {
    const RESET_VALUE: u32 = 0;
}
