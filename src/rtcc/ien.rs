#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `OF` reader - OF Interrupt Enable"]
pub type OF_R = crate::BitReader;
#[doc = "Field `OF` writer - OF Interrupt Enable"]
pub type OF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CC0` reader - CC0 Interrupt Enable"]
pub type CC0_R = crate::BitReader;
#[doc = "Field `CC0` writer - CC0 Interrupt Enable"]
pub type CC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CC1` reader - CC1 Interrupt Enable"]
pub type CC1_R = crate::BitReader;
#[doc = "Field `CC1` writer - CC1 Interrupt Enable"]
pub type CC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CC2` reader - CC2 Interrupt Enable"]
pub type CC2_R = crate::BitReader;
#[doc = "Field `CC2` writer - CC2 Interrupt Enable"]
pub type CC2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCFAIL` reader - OSCFAIL Interrupt Enable"]
pub type OSCFAIL_R = crate::BitReader;
#[doc = "Field `OSCFAIL` writer - OSCFAIL Interrupt Enable"]
pub type OSCFAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CNTTICK` reader - CNTTICK Interrupt Enable"]
pub type CNTTICK_R = crate::BitReader;
#[doc = "Field `CNTTICK` writer - CNTTICK Interrupt Enable"]
pub type CNTTICK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MINTICK` reader - MINTICK Interrupt Enable"]
pub type MINTICK_R = crate::BitReader;
#[doc = "Field `MINTICK` writer - MINTICK Interrupt Enable"]
pub type MINTICK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HOURTICK` reader - HOURTICK Interrupt Enable"]
pub type HOURTICK_R = crate::BitReader;
#[doc = "Field `HOURTICK` writer - HOURTICK Interrupt Enable"]
pub type HOURTICK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAYTICK` reader - DAYTICK Interrupt Enable"]
pub type DAYTICK_R = crate::BitReader;
#[doc = "Field `DAYTICK` writer - DAYTICK Interrupt Enable"]
pub type DAYTICK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAYOWOF` reader - DAYOWOF Interrupt Enable"]
pub type DAYOWOF_R = crate::BitReader;
#[doc = "Field `DAYOWOF` writer - DAYOWOF Interrupt Enable"]
pub type DAYOWOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MONTHTICK` reader - MONTHTICK Interrupt Enable"]
pub type MONTHTICK_R = crate::BitReader;
#[doc = "Field `MONTHTICK` writer - MONTHTICK Interrupt Enable"]
pub type MONTHTICK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OSCFAIL Interrupt Enable"]
    #[inline(always)]
    pub fn oscfail(&self) -> OSCFAIL_R {
        OSCFAIL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CNTTICK Interrupt Enable"]
    #[inline(always)]
    pub fn cnttick(&self) -> CNTTICK_R {
        CNTTICK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MINTICK Interrupt Enable"]
    #[inline(always)]
    pub fn mintick(&self) -> MINTICK_R {
        MINTICK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HOURTICK Interrupt Enable"]
    #[inline(always)]
    pub fn hourtick(&self) -> HOURTICK_R {
        HOURTICK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DAYTICK Interrupt Enable"]
    #[inline(always)]
    pub fn daytick(&self) -> DAYTICK_R {
        DAYTICK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAYOWOF Interrupt Enable"]
    #[inline(always)]
    pub fn dayowof(&self) -> DAYOWOF_R {
        DAYOWOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MONTHTICK Interrupt Enable"]
    #[inline(always)]
    pub fn monthtick(&self) -> MONTHTICK_R {
        MONTHTICK_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("of", &format_args!("{}", self.of().bit()))
            .field("cc0", &format_args!("{}", self.cc0().bit()))
            .field("cc1", &format_args!("{}", self.cc1().bit()))
            .field("cc2", &format_args!("{}", self.cc2().bit()))
            .field("oscfail", &format_args!("{}", self.oscfail().bit()))
            .field("cnttick", &format_args!("{}", self.cnttick().bit()))
            .field("mintick", &format_args!("{}", self.mintick().bit()))
            .field("hourtick", &format_args!("{}", self.hourtick().bit()))
            .field("daytick", &format_args!("{}", self.daytick().bit()))
            .field("dayowof", &format_args!("{}", self.dayowof().bit()))
            .field("monthtick", &format_args!("{}", self.monthtick().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<IEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<IEN_SPEC, 0> {
        OF_W::new(self)
    }
    #[doc = "Bit 1 - CC0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc0(&mut self) -> CC0_W<IEN_SPEC, 1> {
        CC0_W::new(self)
    }
    #[doc = "Bit 2 - CC1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> CC1_W<IEN_SPEC, 2> {
        CC1_W::new(self)
    }
    #[doc = "Bit 3 - CC2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> CC2_W<IEN_SPEC, 3> {
        CC2_W::new(self)
    }
    #[doc = "Bit 4 - OSCFAIL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oscfail(&mut self) -> OSCFAIL_W<IEN_SPEC, 4> {
        OSCFAIL_W::new(self)
    }
    #[doc = "Bit 5 - CNTTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnttick(&mut self) -> CNTTICK_W<IEN_SPEC, 5> {
        CNTTICK_W::new(self)
    }
    #[doc = "Bit 6 - MINTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mintick(&mut self) -> MINTICK_W<IEN_SPEC, 6> {
        MINTICK_W::new(self)
    }
    #[doc = "Bit 7 - HOURTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hourtick(&mut self) -> HOURTICK_W<IEN_SPEC, 7> {
        HOURTICK_W::new(self)
    }
    #[doc = "Bit 8 - DAYTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn daytick(&mut self) -> DAYTICK_W<IEN_SPEC, 8> {
        DAYTICK_W::new(self)
    }
    #[doc = "Bit 9 - DAYOWOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dayowof(&mut self) -> DAYOWOF_W<IEN_SPEC, 9> {
        DAYOWOF_W::new(self)
    }
    #[doc = "Bit 10 - MONTHTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn monthtick(&mut self) -> MONTHTICK_W<IEN_SPEC, 10> {
        MONTHTICK_W::new(self)
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
