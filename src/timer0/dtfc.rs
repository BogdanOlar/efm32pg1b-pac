///Register `DTFC` reader
pub type R = crate::R<DTFCrs>;
///Register `DTFC` writer
pub type W = crate::W<DTFCrs>;
///DTI PRS Fault Source 0 Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTPRS0FSEL {
    ///0: PRS Channel 0 selected as fault source 0
    Prsch0 = 0,
    ///1: PRS Channel 1 selected as fault source 1
    Prsch1 = 1,
    ///2: PRS Channel 2 selected as fault source 2
    Prsch2 = 2,
    ///3: PRS Channel 3 selected as fault source 3
    Prsch3 = 3,
    ///4: PRS Channel 4 selected as fault source 4
    Prsch4 = 4,
    ///5: PRS Channel 5 selected as fault source 5
    Prsch5 = 5,
    ///6: PRS Channel 6 selected as fault source 6
    Prsch6 = 6,
    ///7: PRS Channel 7 selected as fault source 7
    Prsch7 = 7,
    ///8: PRS Channel 8 selected as fault source 8
    Prsch8 = 8,
    ///9: PRS Channel 9 selected as fault source 9
    Prsch9 = 9,
    ///10: PRS Channel 10 selected as fault source 10
    Prsch10 = 10,
    ///11: PRS Channel 11 selected as fault source 11
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
impl crate::IsEnum for DTPRS0FSEL {}
///Field `DTPRS0FSEL` reader - DTI PRS Fault Source 0 Select
pub type Dtprs0fselR = crate::FieldReader<DTPRS0FSEL>;
impl Dtprs0fselR {
    ///Get enumerated values variant
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
    ///PRS Channel 0 selected as fault source 0
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRS0FSEL::Prsch0
    }
    ///PRS Channel 1 selected as fault source 1
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRS0FSEL::Prsch1
    }
    ///PRS Channel 2 selected as fault source 2
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRS0FSEL::Prsch2
    }
    ///PRS Channel 3 selected as fault source 3
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRS0FSEL::Prsch3
    }
    ///PRS Channel 4 selected as fault source 4
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRS0FSEL::Prsch4
    }
    ///PRS Channel 5 selected as fault source 5
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRS0FSEL::Prsch5
    }
    ///PRS Channel 6 selected as fault source 6
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRS0FSEL::Prsch6
    }
    ///PRS Channel 7 selected as fault source 7
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRS0FSEL::Prsch7
    }
    ///PRS Channel 8 selected as fault source 8
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == DTPRS0FSEL::Prsch8
    }
    ///PRS Channel 9 selected as fault source 9
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == DTPRS0FSEL::Prsch9
    }
    ///PRS Channel 10 selected as fault source 10
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == DTPRS0FSEL::Prsch10
    }
    ///PRS Channel 11 selected as fault source 11
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == DTPRS0FSEL::Prsch11
    }
}
///Field `DTPRS0FSEL` writer - DTI PRS Fault Source 0 Select
pub type Dtprs0fselW<'a, REG> = crate::FieldWriter<'a, REG, 4, DTPRS0FSEL>;
impl<'a, REG> Dtprs0fselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected as fault source 0
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch0)
    }
    ///PRS Channel 1 selected as fault source 1
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch1)
    }
    ///PRS Channel 2 selected as fault source 2
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch2)
    }
    ///PRS Channel 3 selected as fault source 3
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch3)
    }
    ///PRS Channel 4 selected as fault source 4
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch4)
    }
    ///PRS Channel 5 selected as fault source 5
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch5)
    }
    ///PRS Channel 6 selected as fault source 6
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch6)
    }
    ///PRS Channel 7 selected as fault source 7
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch7)
    }
    ///PRS Channel 8 selected as fault source 8
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch8)
    }
    ///PRS Channel 9 selected as fault source 9
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch9)
    }
    ///PRS Channel 10 selected as fault source 10
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch10)
    }
    ///PRS Channel 11 selected as fault source 11
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS0FSEL::Prsch11)
    }
}
///DTI PRS Fault Source 1 Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTPRS1FSEL {
    ///0: PRS Channel 0 selected as fault source 1
    Prsch0 = 0,
    ///1: PRS Channel 1 selected as fault source 1
    Prsch1 = 1,
    ///2: PRS Channel 2 selected as fault source 1
    Prsch2 = 2,
    ///3: PRS Channel 3 selected as fault source 1
    Prsch3 = 3,
    ///4: PRS Channel 4 selected as fault source 1
    Prsch4 = 4,
    ///5: PRS Channel 5 selected as fault source 1
    Prsch5 = 5,
    ///6: PRS Channel 6 selected as fault source 1
    Prsch6 = 6,
    ///7: PRS Channel 7 selected as fault source 1
    Prsch7 = 7,
    ///8: PRS Channel 8 selected as fault source 1
    Prsch8 = 8,
    ///9: PRS Channel 9 selected as fault source 1
    Prsch9 = 9,
    ///10: PRS Channel 10 selected as fault source 1
    Prsch10 = 10,
    ///11: PRS Channel 11 selected as fault source 1
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
impl crate::IsEnum for DTPRS1FSEL {}
///Field `DTPRS1FSEL` reader - DTI PRS Fault Source 1 Select
pub type Dtprs1fselR = crate::FieldReader<DTPRS1FSEL>;
impl Dtprs1fselR {
    ///Get enumerated values variant
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
    ///PRS Channel 0 selected as fault source 1
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRS1FSEL::Prsch0
    }
    ///PRS Channel 1 selected as fault source 1
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRS1FSEL::Prsch1
    }
    ///PRS Channel 2 selected as fault source 1
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRS1FSEL::Prsch2
    }
    ///PRS Channel 3 selected as fault source 1
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRS1FSEL::Prsch3
    }
    ///PRS Channel 4 selected as fault source 1
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRS1FSEL::Prsch4
    }
    ///PRS Channel 5 selected as fault source 1
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRS1FSEL::Prsch5
    }
    ///PRS Channel 6 selected as fault source 1
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRS1FSEL::Prsch6
    }
    ///PRS Channel 7 selected as fault source 1
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRS1FSEL::Prsch7
    }
    ///PRS Channel 8 selected as fault source 1
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == DTPRS1FSEL::Prsch8
    }
    ///PRS Channel 9 selected as fault source 1
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == DTPRS1FSEL::Prsch9
    }
    ///PRS Channel 10 selected as fault source 1
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == DTPRS1FSEL::Prsch10
    }
    ///PRS Channel 11 selected as fault source 1
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == DTPRS1FSEL::Prsch11
    }
}
///Field `DTPRS1FSEL` writer - DTI PRS Fault Source 1 Select
pub type Dtprs1fselW<'a, REG> = crate::FieldWriter<'a, REG, 4, DTPRS1FSEL>;
impl<'a, REG> Dtprs1fselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected as fault source 1
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch0)
    }
    ///PRS Channel 1 selected as fault source 1
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch1)
    }
    ///PRS Channel 2 selected as fault source 1
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch2)
    }
    ///PRS Channel 3 selected as fault source 1
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch3)
    }
    ///PRS Channel 4 selected as fault source 1
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch4)
    }
    ///PRS Channel 5 selected as fault source 1
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch5)
    }
    ///PRS Channel 6 selected as fault source 1
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch6)
    }
    ///PRS Channel 7 selected as fault source 1
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch7)
    }
    ///PRS Channel 8 selected as fault source 1
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch8)
    }
    ///PRS Channel 9 selected as fault source 1
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch9)
    }
    ///PRS Channel 10 selected as fault source 1
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch10)
    }
    ///PRS Channel 11 selected as fault source 1
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRS1FSEL::Prsch11)
    }
}
///DTI Fault Action
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTFA {
    ///0: No action on fault
    None = 0,
    ///1: Set outputs inactive
    Inactive = 1,
    ///2: Clear outputs
    Clear = 2,
    ///3: Tristate outputs
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
impl crate::IsEnum for DTFA {}
///Field `DTFA` reader - DTI Fault Action
pub type DtfaR = crate::FieldReader<DTFA>;
impl DtfaR {
    ///Get enumerated values variant
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
    ///No action on fault
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DTFA::None
    }
    ///Set outputs inactive
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DTFA::Inactive
    }
    ///Clear outputs
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == DTFA::Clear
    }
    ///Tristate outputs
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == DTFA::Tristate
    }
}
///Field `DTFA` writer - DTI Fault Action
pub type DtfaW<'a, REG> = crate::FieldWriter<'a, REG, 2, DTFA, crate::Safe>;
impl<'a, REG> DtfaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No action on fault
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DTFA::None)
    }
    ///Set outputs inactive
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(DTFA::Inactive)
    }
    ///Clear outputs
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DTFA::Clear)
    }
    ///Tristate outputs
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(DTFA::Tristate)
    }
}
///Field `DTPRS0FEN` reader - DTI PRS 0 Fault Enable
pub type Dtprs0fenR = crate::BitReader;
///Field `DTPRS0FEN` writer - DTI PRS 0 Fault Enable
pub type Dtprs0fenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTPRS1FEN` reader - DTI PRS 1 Fault Enable
pub type Dtprs1fenR = crate::BitReader;
///Field `DTPRS1FEN` writer - DTI PRS 1 Fault Enable
pub type Dtprs1fenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTDBGFEN` reader - DTI Debugger Fault Enable
pub type DtdbgfenR = crate::BitReader;
///Field `DTDBGFEN` writer - DTI Debugger Fault Enable
pub type DtdbgfenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTLOCKUPFEN` reader - DTI Lockup Fault Enable
pub type DtlockupfenR = crate::BitReader;
///Field `DTLOCKUPFEN` writer - DTI Lockup Fault Enable
pub type DtlockupfenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - DTI PRS Fault Source 0 Select
    #[inline(always)]
    pub fn dtprs0fsel(&self) -> Dtprs0fselR {
        Dtprs0fselR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - DTI PRS Fault Source 1 Select
    #[inline(always)]
    pub fn dtprs1fsel(&self) -> Dtprs1fselR {
        Dtprs1fselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:17 - DTI Fault Action
    #[inline(always)]
    pub fn dtfa(&self) -> DtfaR {
        DtfaR::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 24 - DTI PRS 0 Fault Enable
    #[inline(always)]
    pub fn dtprs0fen(&self) -> Dtprs0fenR {
        Dtprs0fenR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - DTI PRS 1 Fault Enable
    #[inline(always)]
    pub fn dtprs1fen(&self) -> Dtprs1fenR {
        Dtprs1fenR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - DTI Debugger Fault Enable
    #[inline(always)]
    pub fn dtdbgfen(&self) -> DtdbgfenR {
        DtdbgfenR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - DTI Lockup Fault Enable
    #[inline(always)]
    pub fn dtlockupfen(&self) -> DtlockupfenR {
        DtlockupfenR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTFC")
            .field("dtprs0fsel", &self.dtprs0fsel())
            .field("dtprs1fsel", &self.dtprs1fsel())
            .field("dtfa", &self.dtfa())
            .field("dtprs0fen", &self.dtprs0fen())
            .field("dtprs1fen", &self.dtprs1fen())
            .field("dtdbgfen", &self.dtdbgfen())
            .field("dtlockupfen", &self.dtlockupfen())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - DTI PRS Fault Source 0 Select
    #[inline(always)]
    #[must_use]
    pub fn dtprs0fsel(&mut self) -> Dtprs0fselW<DTFCrs> {
        Dtprs0fselW::new(self, 0)
    }
    ///Bits 8:11 - DTI PRS Fault Source 1 Select
    #[inline(always)]
    #[must_use]
    pub fn dtprs1fsel(&mut self) -> Dtprs1fselW<DTFCrs> {
        Dtprs1fselW::new(self, 8)
    }
    ///Bits 16:17 - DTI Fault Action
    #[inline(always)]
    #[must_use]
    pub fn dtfa(&mut self) -> DtfaW<DTFCrs> {
        DtfaW::new(self, 16)
    }
    ///Bit 24 - DTI PRS 0 Fault Enable
    #[inline(always)]
    #[must_use]
    pub fn dtprs0fen(&mut self) -> Dtprs0fenW<DTFCrs> {
        Dtprs0fenW::new(self, 24)
    }
    ///Bit 25 - DTI PRS 1 Fault Enable
    #[inline(always)]
    #[must_use]
    pub fn dtprs1fen(&mut self) -> Dtprs1fenW<DTFCrs> {
        Dtprs1fenW::new(self, 25)
    }
    ///Bit 26 - DTI Debugger Fault Enable
    #[inline(always)]
    #[must_use]
    pub fn dtdbgfen(&mut self) -> DtdbgfenW<DTFCrs> {
        DtdbgfenW::new(self, 26)
    }
    ///Bit 27 - DTI Lockup Fault Enable
    #[inline(always)]
    #[must_use]
    pub fn dtlockupfen(&mut self) -> DtlockupfenW<DTFCrs> {
        DtlockupfenW::new(self, 27)
    }
}
///DTI Fault Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`dtfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DTFCrs;
impl crate::RegisterSpec for DTFCrs {
    type Ux = u32;
}
///`read()` method returns [`dtfc::R`](R) reader structure
impl crate::Readable for DTFCrs {}
///`write(|w| ..)` method takes [`dtfc::W`](W) writer structure
impl crate::Writable for DTFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DTFC to value 0
impl crate::Resettable for DTFCrs {
    const RESET_VALUE: u32 = 0;
}
