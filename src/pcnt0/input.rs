///Register `INPUT` reader
pub type R = crate::R<INPUTrs>;
///Register `INPUT` writer
pub type W = crate::W<INPUTrs>;
///S0IN PRS Channel Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S0PRSSEL {
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
impl From<S0PRSSEL> for u8 {
    #[inline(always)]
    fn from(variant: S0PRSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S0PRSSEL {
    type Ux = u8;
}
impl crate::IsEnum for S0PRSSEL {}
///Field `S0PRSSEL` reader - S0IN PRS Channel Select
pub type S0prsselR = crate::FieldReader<S0PRSSEL>;
impl S0prsselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<S0PRSSEL> {
        match self.bits {
            0 => Some(S0PRSSEL::Prsch0),
            1 => Some(S0PRSSEL::Prsch1),
            2 => Some(S0PRSSEL::Prsch2),
            3 => Some(S0PRSSEL::Prsch3),
            4 => Some(S0PRSSEL::Prsch4),
            5 => Some(S0PRSSEL::Prsch5),
            6 => Some(S0PRSSEL::Prsch6),
            7 => Some(S0PRSSEL::Prsch7),
            8 => Some(S0PRSSEL::Prsch8),
            9 => Some(S0PRSSEL::Prsch9),
            10 => Some(S0PRSSEL::Prsch10),
            11 => Some(S0PRSSEL::Prsch11),
            _ => None,
        }
    }
    ///PRS Channel 0 selected.
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == S0PRSSEL::Prsch0
    }
    ///PRS Channel 1 selected.
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == S0PRSSEL::Prsch1
    }
    ///PRS Channel 2 selected.
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == S0PRSSEL::Prsch2
    }
    ///PRS Channel 3 selected.
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == S0PRSSEL::Prsch3
    }
    ///PRS Channel 4 selected.
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == S0PRSSEL::Prsch4
    }
    ///PRS Channel 5 selected.
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == S0PRSSEL::Prsch5
    }
    ///PRS Channel 6 selected.
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == S0PRSSEL::Prsch6
    }
    ///PRS Channel 7 selected.
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == S0PRSSEL::Prsch7
    }
    ///PRS Channel 8 selected.
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == S0PRSSEL::Prsch8
    }
    ///PRS Channel 9 selected.
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == S0PRSSEL::Prsch9
    }
    ///PRS Channel 10 selected.
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == S0PRSSEL::Prsch10
    }
    ///PRS Channel 11 selected.
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == S0PRSSEL::Prsch11
    }
}
///Field `S0PRSSEL` writer - S0IN PRS Channel Select
pub type S0prsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, S0PRSSEL>;
impl<'a, REG> S0prsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected.
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch0)
    }
    ///PRS Channel 1 selected.
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch1)
    }
    ///PRS Channel 2 selected.
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch2)
    }
    ///PRS Channel 3 selected.
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch3)
    }
    ///PRS Channel 4 selected.
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch4)
    }
    ///PRS Channel 5 selected.
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch5)
    }
    ///PRS Channel 6 selected.
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch6)
    }
    ///PRS Channel 7 selected.
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch7)
    }
    ///PRS Channel 8 selected.
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch8)
    }
    ///PRS Channel 9 selected.
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch9)
    }
    ///PRS Channel 10 selected.
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch10)
    }
    ///PRS Channel 11 selected.
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch11)
    }
}
///Field `S0PRSEN` reader - S0IN PRS Enable
pub type S0prsenR = crate::BitReader;
///Field `S0PRSEN` writer - S0IN PRS Enable
pub type S0prsenW<'a, REG> = crate::BitWriter<'a, REG>;
///S1IN PRS Channel Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S1PRSSEL {
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
impl From<S1PRSSEL> for u8 {
    #[inline(always)]
    fn from(variant: S1PRSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S1PRSSEL {
    type Ux = u8;
}
impl crate::IsEnum for S1PRSSEL {}
///Field `S1PRSSEL` reader - S1IN PRS Channel Select
pub type S1prsselR = crate::FieldReader<S1PRSSEL>;
impl S1prsselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<S1PRSSEL> {
        match self.bits {
            0 => Some(S1PRSSEL::Prsch0),
            1 => Some(S1PRSSEL::Prsch1),
            2 => Some(S1PRSSEL::Prsch2),
            3 => Some(S1PRSSEL::Prsch3),
            4 => Some(S1PRSSEL::Prsch4),
            5 => Some(S1PRSSEL::Prsch5),
            6 => Some(S1PRSSEL::Prsch6),
            7 => Some(S1PRSSEL::Prsch7),
            8 => Some(S1PRSSEL::Prsch8),
            9 => Some(S1PRSSEL::Prsch9),
            10 => Some(S1PRSSEL::Prsch10),
            11 => Some(S1PRSSEL::Prsch11),
            _ => None,
        }
    }
    ///PRS Channel 0 selected.
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == S1PRSSEL::Prsch0
    }
    ///PRS Channel 1 selected.
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == S1PRSSEL::Prsch1
    }
    ///PRS Channel 2 selected.
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == S1PRSSEL::Prsch2
    }
    ///PRS Channel 3 selected.
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == S1PRSSEL::Prsch3
    }
    ///PRS Channel 4 selected.
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == S1PRSSEL::Prsch4
    }
    ///PRS Channel 5 selected.
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == S1PRSSEL::Prsch5
    }
    ///PRS Channel 6 selected.
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == S1PRSSEL::Prsch6
    }
    ///PRS Channel 7 selected.
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == S1PRSSEL::Prsch7
    }
    ///PRS Channel 8 selected.
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == S1PRSSEL::Prsch8
    }
    ///PRS Channel 9 selected.
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == S1PRSSEL::Prsch9
    }
    ///PRS Channel 10 selected.
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == S1PRSSEL::Prsch10
    }
    ///PRS Channel 11 selected.
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == S1PRSSEL::Prsch11
    }
}
///Field `S1PRSSEL` writer - S1IN PRS Channel Select
pub type S1prsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, S1PRSSEL>;
impl<'a, REG> S1prsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected.
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch0)
    }
    ///PRS Channel 1 selected.
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch1)
    }
    ///PRS Channel 2 selected.
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch2)
    }
    ///PRS Channel 3 selected.
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch3)
    }
    ///PRS Channel 4 selected.
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch4)
    }
    ///PRS Channel 5 selected.
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch5)
    }
    ///PRS Channel 6 selected.
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch6)
    }
    ///PRS Channel 7 selected.
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch7)
    }
    ///PRS Channel 8 selected.
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch8)
    }
    ///PRS Channel 9 selected.
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch9)
    }
    ///PRS Channel 10 selected.
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch10)
    }
    ///PRS Channel 11 selected.
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch11)
    }
}
///Field `S1PRSEN` reader - S1IN PRS Enable
pub type S1prsenR = crate::BitReader;
///Field `S1PRSEN` writer - S1IN PRS Enable
pub type S1prsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - S0IN PRS Channel Select
    #[inline(always)]
    pub fn s0prssel(&self) -> S0prsselR {
        S0prsselR::new((self.bits & 0x0f) as u8)
    }
    ///Bit 5 - S0IN PRS Enable
    #[inline(always)]
    pub fn s0prsen(&self) -> S0prsenR {
        S0prsenR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:9 - S1IN PRS Channel Select
    #[inline(always)]
    pub fn s1prssel(&self) -> S1prsselR {
        S1prsselR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    ///Bit 11 - S1IN PRS Enable
    #[inline(always)]
    pub fn s1prsen(&self) -> S1prsenR {
        S1prsenR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INPUT")
            .field("s0prssel", &self.s0prssel())
            .field("s0prsen", &self.s0prsen())
            .field("s1prssel", &self.s1prssel())
            .field("s1prsen", &self.s1prsen())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - S0IN PRS Channel Select
    #[inline(always)]
    #[must_use]
    pub fn s0prssel(&mut self) -> S0prsselW<INPUTrs> {
        S0prsselW::new(self, 0)
    }
    ///Bit 5 - S0IN PRS Enable
    #[inline(always)]
    #[must_use]
    pub fn s0prsen(&mut self) -> S0prsenW<INPUTrs> {
        S0prsenW::new(self, 5)
    }
    ///Bits 6:9 - S1IN PRS Channel Select
    #[inline(always)]
    #[must_use]
    pub fn s1prssel(&mut self) -> S1prsselW<INPUTrs> {
        S1prsselW::new(self, 6)
    }
    ///Bit 11 - S1IN PRS Enable
    #[inline(always)]
    #[must_use]
    pub fn s1prsen(&mut self) -> S1prsenW<INPUTrs> {
        S1prsenW::new(self, 11)
    }
}
///PCNT Input Register
///
///You can [`read`](crate::Reg::read) this register and get [`input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INPUTrs;
impl crate::RegisterSpec for INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`input::R`](R) reader structure
impl crate::Readable for INPUTrs {}
///`write(|w| ..)` method takes [`input::W`](W) writer structure
impl crate::Writable for INPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INPUT to value 0
impl crate::Resettable for INPUTrs {
    const RESET_VALUE: u32 = 0;
}
