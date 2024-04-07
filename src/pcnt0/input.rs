#[doc = "Register `INPUT` reader"]
pub type R = crate::R<INPUTrs>;
#[doc = "Register `INPUT` writer"]
pub type W = crate::W<INPUTrs>;
#[doc = "S0IN PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S0PRSSEL {
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
#[doc = "Field `S0PRSSEL` reader - S0IN PRS Channel Select"]
pub type S0prsselR = crate::FieldReader<S0PRSSEL>;
impl S0prsselR {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == S0PRSSEL::Prsch0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == S0PRSSEL::Prsch1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == S0PRSSEL::Prsch2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == S0PRSSEL::Prsch3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == S0PRSSEL::Prsch4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == S0PRSSEL::Prsch5
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == S0PRSSEL::Prsch6
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == S0PRSSEL::Prsch7
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == S0PRSSEL::Prsch8
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == S0PRSSEL::Prsch9
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == S0PRSSEL::Prsch10
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == S0PRSSEL::Prsch11
    }
}
#[doc = "Field `S0PRSSEL` writer - S0IN PRS Channel Select"]
pub type S0prsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, S0PRSSEL>;
impl<'a, REG> S0prsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL::Prsch11)
    }
}
#[doc = "Field `S0PRSEN` reader - S0IN PRS Enable"]
pub type S0prsenR = crate::BitReader;
#[doc = "Field `S0PRSEN` writer - S0IN PRS Enable"]
pub type S0prsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "S1IN PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S1PRSSEL {
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
#[doc = "Field `S1PRSSEL` reader - S1IN PRS Channel Select"]
pub type S1prsselR = crate::FieldReader<S1PRSSEL>;
impl S1prsselR {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == S1PRSSEL::Prsch0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == S1PRSSEL::Prsch1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == S1PRSSEL::Prsch2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == S1PRSSEL::Prsch3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == S1PRSSEL::Prsch4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == S1PRSSEL::Prsch5
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == S1PRSSEL::Prsch6
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == S1PRSSEL::Prsch7
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == S1PRSSEL::Prsch8
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == S1PRSSEL::Prsch9
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == S1PRSSEL::Prsch10
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == S1PRSSEL::Prsch11
    }
}
#[doc = "Field `S1PRSSEL` writer - S1IN PRS Channel Select"]
pub type S1prsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, S1PRSSEL>;
impl<'a, REG> S1prsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL::Prsch11)
    }
}
#[doc = "Field `S1PRSEN` reader - S1IN PRS Enable"]
pub type S1prsenR = crate::BitReader;
#[doc = "Field `S1PRSEN` writer - S1IN PRS Enable"]
pub type S1prsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - S0IN PRS Channel Select"]
    #[inline(always)]
    pub fn s0prssel(&self) -> S0prsselR {
        S0prsselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - S0IN PRS Enable"]
    #[inline(always)]
    pub fn s0prsen(&self) -> S0prsenR {
        S0prsenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - S1IN PRS Channel Select"]
    #[inline(always)]
    pub fn s1prssel(&self) -> S1prsselR {
        S1prsselR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - S1IN PRS Enable"]
    #[inline(always)]
    pub fn s1prsen(&self) -> S1prsenR {
        S1prsenR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - S0IN PRS Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn s0prssel(&mut self) -> S0prsselW<INPUTrs> {
        S0prsselW::new(self, 0)
    }
    #[doc = "Bit 5 - S0IN PRS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn s0prsen(&mut self) -> S0prsenW<INPUTrs> {
        S0prsenW::new(self, 5)
    }
    #[doc = "Bits 6:9 - S1IN PRS Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn s1prssel(&mut self) -> S1prsselW<INPUTrs> {
        S1prsselW::new(self, 6)
    }
    #[doc = "Bit 11 - S1IN PRS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1prsen(&mut self) -> S1prsenW<INPUTrs> {
        S1prsenW::new(self, 11)
    }
}
#[doc = "PCNT Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUTrs;
impl crate::RegisterSpec for INPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input::R`](R) reader structure"]
impl crate::Readable for INPUTrs {}
#[doc = "`write(|w| ..)` method takes [`input::W`](W) writer structure"]
impl crate::Writable for INPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INPUT to value 0"]
impl crate::Resettable for INPUTrs {
    const RESET_VALUE: u32 = 0;
}
