#[doc = "Register `INPUT` reader"]
pub type R = crate::R<INPUT_SPEC>;
#[doc = "Register `INPUT` writer"]
pub type W = crate::W<INPUT_SPEC>;
#[doc = "Field `S0PRSSEL` reader - S0IN PRS Channel Select"]
pub type S0PRSSEL_R = crate::FieldReader<S0PRSSEL_A>;
#[doc = "S0IN PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S0PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected."]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected."]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected."]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected."]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected."]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected."]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected."]
    PRSCH11 = 11,
}
impl From<S0PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: S0PRSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S0PRSSEL_A {
    type Ux = u8;
}
impl S0PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<S0PRSSEL_A> {
        match self.bits {
            0 => Some(S0PRSSEL_A::PRSCH0),
            1 => Some(S0PRSSEL_A::PRSCH1),
            2 => Some(S0PRSSEL_A::PRSCH2),
            3 => Some(S0PRSSEL_A::PRSCH3),
            4 => Some(S0PRSSEL_A::PRSCH4),
            5 => Some(S0PRSSEL_A::PRSCH5),
            6 => Some(S0PRSSEL_A::PRSCH6),
            7 => Some(S0PRSSEL_A::PRSCH7),
            8 => Some(S0PRSSEL_A::PRSCH8),
            9 => Some(S0PRSSEL_A::PRSCH9),
            10 => Some(S0PRSSEL_A::PRSCH10),
            11 => Some(S0PRSSEL_A::PRSCH11),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH5
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH6
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH7
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH8
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH9
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH10
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH11
    }
}
#[doc = "Field `S0PRSSEL` writer - S0IN PRS Channel Select"]
pub type S0PRSSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, S0PRSSEL_A>;
impl<'a, REG, const O: u8> S0PRSSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(S0PRSSEL_A::PRSCH11)
    }
}
#[doc = "Field `S0PRSEN` reader - S0IN PRS Enable"]
pub type S0PRSEN_R = crate::BitReader;
#[doc = "Field `S0PRSEN` writer - S0IN PRS Enable"]
pub type S0PRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S1PRSSEL` reader - S1IN PRS Channel Select"]
pub type S1PRSSEL_R = crate::FieldReader<S1PRSSEL_A>;
#[doc = "S1IN PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S1PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected."]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected."]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected."]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected."]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected."]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected."]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected."]
    PRSCH11 = 11,
}
impl From<S1PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: S1PRSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S1PRSSEL_A {
    type Ux = u8;
}
impl S1PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<S1PRSSEL_A> {
        match self.bits {
            0 => Some(S1PRSSEL_A::PRSCH0),
            1 => Some(S1PRSSEL_A::PRSCH1),
            2 => Some(S1PRSSEL_A::PRSCH2),
            3 => Some(S1PRSSEL_A::PRSCH3),
            4 => Some(S1PRSSEL_A::PRSCH4),
            5 => Some(S1PRSSEL_A::PRSCH5),
            6 => Some(S1PRSSEL_A::PRSCH6),
            7 => Some(S1PRSSEL_A::PRSCH7),
            8 => Some(S1PRSSEL_A::PRSCH8),
            9 => Some(S1PRSSEL_A::PRSCH9),
            10 => Some(S1PRSSEL_A::PRSCH10),
            11 => Some(S1PRSSEL_A::PRSCH11),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH5
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH6
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH7
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH8
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH9
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH10
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH11
    }
}
#[doc = "Field `S1PRSSEL` writer - S1IN PRS Channel Select"]
pub type S1PRSSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, S1PRSSEL_A>;
impl<'a, REG, const O: u8> S1PRSSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(S1PRSSEL_A::PRSCH11)
    }
}
#[doc = "Field `S1PRSEN` reader - S1IN PRS Enable"]
pub type S1PRSEN_R = crate::BitReader;
#[doc = "Field `S1PRSEN` writer - S1IN PRS Enable"]
pub type S1PRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - S0IN PRS Channel Select"]
    #[inline(always)]
    pub fn s0prssel(&self) -> S0PRSSEL_R {
        S0PRSSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - S0IN PRS Enable"]
    #[inline(always)]
    pub fn s0prsen(&self) -> S0PRSEN_R {
        S0PRSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - S1IN PRS Channel Select"]
    #[inline(always)]
    pub fn s1prssel(&self) -> S1PRSSEL_R {
        S1PRSSEL_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - S1IN PRS Enable"]
    #[inline(always)]
    pub fn s1prsen(&self) -> S1PRSEN_R {
        S1PRSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INPUT")
            .field("s0prssel", &format_args!("{}", self.s0prssel().bits()))
            .field("s0prsen", &format_args!("{}", self.s0prsen().bit()))
            .field("s1prssel", &format_args!("{}", self.s1prssel().bits()))
            .field("s1prsen", &format_args!("{}", self.s1prsen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<INPUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - S0IN PRS Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn s0prssel(&mut self) -> S0PRSSEL_W<INPUT_SPEC, 0> {
        S0PRSSEL_W::new(self)
    }
    #[doc = "Bit 5 - S0IN PRS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn s0prsen(&mut self) -> S0PRSEN_W<INPUT_SPEC, 5> {
        S0PRSEN_W::new(self)
    }
    #[doc = "Bits 6:9 - S1IN PRS Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn s1prssel(&mut self) -> S1PRSSEL_W<INPUT_SPEC, 6> {
        S1PRSSEL_W::new(self)
    }
    #[doc = "Bit 11 - S1IN PRS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1prsen(&mut self) -> S1PRSEN_W<INPUT_SPEC, 11> {
        S1PRSEN_W::new(self)
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
#[doc = "PCNT Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUT_SPEC;
impl crate::RegisterSpec for INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input::R`](R) reader structure"]
impl crate::Readable for INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`input::W`](W) writer structure"]
impl crate::Writable for INPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUT to value 0"]
impl crate::Resettable for INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
