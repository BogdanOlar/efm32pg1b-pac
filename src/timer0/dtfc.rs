#[doc = "Register `DTFC` reader"]
pub type R = crate::R<DTFCrs>;
#[doc = "Register `DTFC` writer"]
pub type W = crate::W<DTFCrs>;
#[doc = "Field `DTPRS0FSEL` reader - DTI PRS Fault Source 0 Select"]
pub type DTPRS0FSEL_R = crate::FieldReader<DTPRS0FSEL>;
#[doc = "DTI PRS Fault Source 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTPRS0FSEL {
    #[doc = "0: PRS Channel 0 selected as fault source 0"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as fault source 1"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as fault source 2"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as fault source 3"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as fault source 4"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as fault source 5"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as fault source 6"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as fault source 7"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected as fault source 8"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected as fault source 9"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected as fault source 10"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected as fault source 11"]
    Prsch11 = 11,
}
impl From<DTPRS0FSEL> for u8 {
    #[inline(always)]
    fn from(variant: DTPRS0FSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTPRS0FSEL {
    type Ux = u8;
}
impl DTPRS0FSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DTPRS0FSEL> {
        match self.bits {
            0 => Some(DTPRS0FSEL::Prsch0),
            1 => Some(DTPRS0FSEL::Prsch1),
            2 => Some(DTPRS0FSEL::Prsch2),
            3 => Some(DTPRS0FSEL::Prsch3),
            4 => Some(DTPRS0FSEL::Prsch4),
            5 => Some(DTPRS0FSEL::Prsch5),
            6 => Some(DTPRS0FSEL::Prsch6),
            7 => Some(DTPRS0FSEL::Prsch7),
            8 => Some(DTPRS0FSEL::Prsch8),
            9 => Some(DTPRS0FSEL::Prsch9),
            10 => Some(DTPRS0FSEL::Prsch10),
            11 => Some(DTPRS0FSEL::Prsch11),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as fault source 0"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRS0FSEL::Prsch0
    }
    #[doc = "PRS Channel 1 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRS0FSEL::Prsch1
    }
    #[doc = "PRS Channel 2 selected as fault source 2"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRS0FSEL::Prsch2
    }
    #[doc = "PRS Channel 3 selected as fault source 3"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRS0FSEL::Prsch3
    }
    #[doc = "PRS Channel 4 selected as fault source 4"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRS0FSEL::Prsch4
    }
    #[doc = "PRS Channel 5 selected as fault source 5"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRS0FSEL::Prsch5
    }
    #[doc = "PRS Channel 6 selected as fault source 6"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRS0FSEL::Prsch6
    }
    #[doc = "PRS Channel 7 selected as fault source 7"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRS0FSEL::Prsch7
    }
    #[doc = "PRS Channel 8 selected as fault source 8"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == DTPRS0FSEL::Prsch8
    }
    #[doc = "PRS Channel 9 selected as fault source 9"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == DTPRS0FSEL::Prsch9
    }
    #[doc = "PRS Channel 10 selected as fault source 10"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == DTPRS0FSEL::Prsch10
    }
    #[doc = "PRS Channel 11 selected as fault source 11"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == DTPRS0FSEL::Prsch11
    }
}
#[doc = "Field `DTPRS0FSEL` writer - DTI PRS Fault Source 0 Select"]
pub type DTPRS0FSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DTPRS0FSEL>;
impl<'a, REG> DTPRS0FSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as fault source 2"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as fault source 3"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as fault source 4"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as fault source 5"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as fault source 6"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as fault source 7"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as fault source 8"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as fault source 9"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as fault source 10"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as fault source 11"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch11)
    }
}
#[doc = "Field `DTPRS1FSEL` reader - DTI PRS Fault Source 1 Select"]
pub type DTPRS1FSEL_R = crate::FieldReader<DTPRS1FSEL>;
#[doc = "DTI PRS Fault Source 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTPRS1FSEL {
    #[doc = "0: PRS Channel 0 selected as fault source 1"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as fault source 1"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as fault source 1"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as fault source 1"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as fault source 1"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as fault source 1"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as fault source 1"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as fault source 1"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected as fault source 1"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected as fault source 1"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected as fault source 1"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected as fault source 1"]
    Prsch11 = 11,
}
impl From<DTPRS1FSEL> for u8 {
    #[inline(always)]
    fn from(variant: DTPRS1FSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTPRS1FSEL {
    type Ux = u8;
}
impl DTPRS1FSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DTPRS1FSEL> {
        match self.bits {
            0 => Some(DTPRS1FSEL::Prsch0),
            1 => Some(DTPRS1FSEL::Prsch1),
            2 => Some(DTPRS1FSEL::Prsch2),
            3 => Some(DTPRS1FSEL::Prsch3),
            4 => Some(DTPRS1FSEL::Prsch4),
            5 => Some(DTPRS1FSEL::Prsch5),
            6 => Some(DTPRS1FSEL::Prsch6),
            7 => Some(DTPRS1FSEL::Prsch7),
            8 => Some(DTPRS1FSEL::Prsch8),
            9 => Some(DTPRS1FSEL::Prsch9),
            10 => Some(DTPRS1FSEL::Prsch10),
            11 => Some(DTPRS1FSEL::Prsch11),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRS1FSEL::Prsch0
    }
    #[doc = "PRS Channel 1 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRS1FSEL::Prsch1
    }
    #[doc = "PRS Channel 2 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRS1FSEL::Prsch2
    }
    #[doc = "PRS Channel 3 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRS1FSEL::Prsch3
    }
    #[doc = "PRS Channel 4 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRS1FSEL::Prsch4
    }
    #[doc = "PRS Channel 5 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRS1FSEL::Prsch5
    }
    #[doc = "PRS Channel 6 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRS1FSEL::Prsch6
    }
    #[doc = "PRS Channel 7 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRS1FSEL::Prsch7
    }
    #[doc = "PRS Channel 8 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == DTPRS1FSEL::Prsch8
    }
    #[doc = "PRS Channel 9 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == DTPRS1FSEL::Prsch9
    }
    #[doc = "PRS Channel 10 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == DTPRS1FSEL::Prsch10
    }
    #[doc = "PRS Channel 11 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == DTPRS1FSEL::Prsch11
    }
}
#[doc = "Field `DTPRS1FSEL` writer - DTI PRS Fault Source 1 Select"]
pub type DTPRS1FSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DTPRS1FSEL>;
impl<'a, REG> DTPRS1FSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch11)
    }
}
#[doc = "Field `DTFA` reader - DTI Fault Action"]
pub type DTFA_R = crate::FieldReader<DTFA>;
#[doc = "DTI Fault Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTFA {
    #[doc = "0: No action on fault"]
    None = 0,
    #[doc = "1: Set outputs inactive"]
    Inactive = 1,
    #[doc = "2: Clear outputs"]
    Clear = 2,
    #[doc = "3: Tristate outputs"]
    Tristate = 3,
}
impl From<DTFA> for u8 {
    #[inline(always)]
    fn from(variant: DTFA) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTFA {
    type Ux = u8;
}
impl DTFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTFA {
        match self.bits {
            0 => DTFA::None,
            1 => DTFA::Inactive,
            2 => DTFA::Clear,
            3 => DTFA::Tristate,
            _ => unreachable!(),
        }
    }
    #[doc = "No action on fault"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DTFA::None
    }
    #[doc = "Set outputs inactive"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DTFA::Inactive
    }
    #[doc = "Clear outputs"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == DTFA::Clear
    }
    #[doc = "Tristate outputs"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == DTFA::Tristate
    }
}
#[doc = "Field `DTFA` writer - DTI Fault Action"]
pub type DTFA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DTFA>;
impl<'a, REG> DTFA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action on fault"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DTFA::None)
    }
    #[doc = "Set outputs inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(DTFA::Inactive)
    }
    #[doc = "Clear outputs"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DTFA::Clear)
    }
    #[doc = "Tristate outputs"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(DTFA::Tristate)
    }
}
#[doc = "Field `DTPRS0FEN` reader - DTI PRS 0 Fault Enable"]
pub type DTPRS0FEN_R = crate::BitReader;
#[doc = "Field `DTPRS0FEN` writer - DTI PRS 0 Fault Enable"]
pub type DTPRS0FEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPRS1FEN` reader - DTI PRS 1 Fault Enable"]
pub type DTPRS1FEN_R = crate::BitReader;
#[doc = "Field `DTPRS1FEN` writer - DTI PRS 1 Fault Enable"]
pub type DTPRS1FEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTDBGFEN` reader - DTI Debugger Fault Enable"]
pub type DTDBGFEN_R = crate::BitReader;
#[doc = "Field `DTDBGFEN` writer - DTI Debugger Fault Enable"]
pub type DTDBGFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTLOCKUPFEN` reader - DTI Lockup Fault Enable"]
pub type DTLOCKUPFEN_R = crate::BitReader;
#[doc = "Field `DTLOCKUPFEN` writer - DTI Lockup Fault Enable"]
pub type DTLOCKUPFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - DTI PRS Fault Source 0 Select"]
    #[inline(always)]
    pub fn dtprs0fsel(&self) -> DTPRS0FSEL_R {
        DTPRS0FSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DTI PRS Fault Source 1 Select"]
    #[inline(always)]
    pub fn dtprs1fsel(&self) -> DTPRS1FSEL_R {
        DTPRS1FSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline(always)]
    pub fn dtfa(&self) -> DTFA_R {
        DTFA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline(always)]
    pub fn dtprs0fen(&self) -> DTPRS0FEN_R {
        DTPRS0FEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline(always)]
    pub fn dtprs1fen(&self) -> DTPRS1FEN_R {
        DTPRS1FEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline(always)]
    pub fn dtdbgfen(&self) -> DTDBGFEN_R {
        DTDBGFEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline(always)]
    pub fn dtlockupfen(&self) -> DTLOCKUPFEN_R {
        DTLOCKUPFEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - DTI PRS Fault Source 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn dtprs0fsel(&mut self) -> DTPRS0FSEL_W<DTFCrs> {
        DTPRS0FSEL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DTI PRS Fault Source 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn dtprs1fsel(&mut self) -> DTPRS1FSEL_W<DTFCrs> {
        DTPRS1FSEL_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline(always)]
    #[must_use]
    pub fn dtfa(&mut self) -> DTFA_W<DTFCrs> {
        DTFA_W::new(self, 16)
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtprs0fen(&mut self) -> DTPRS0FEN_W<DTFCrs> {
        DTPRS0FEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtprs1fen(&mut self) -> DTPRS1FEN_W<DTFCrs> {
        DTPRS1FEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtdbgfen(&mut self) -> DTDBGFEN_W<DTFCrs> {
        DTDBGFEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtlockupfen(&mut self) -> DTLOCKUPFEN_W<DTFCrs> {
        DTLOCKUPFEN_W::new(self, 27)
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
#[doc = "DTI Fault Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTFCrs;
impl crate::RegisterSpec for DTFCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtfc::R`](R) reader structure"]
impl crate::Readable for DTFCrs {}
#[doc = "`write(|w| ..)` method takes [`dtfc::W`](W) writer structure"]
impl crate::Writable for DTFCrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTFC to value 0"]
impl crate::Resettable for DTFCrs {
    const RESET_VALUE: u32 = 0;
}
