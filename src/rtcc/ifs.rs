#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFSrs>;
#[doc = "Field `OF` writer - Set OF Interrupt Flag"]
pub type OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0` writer - Set CC0 Interrupt Flag"]
pub type CC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1` writer - Set CC1 Interrupt Flag"]
pub type CC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2` writer - Set CC2 Interrupt Flag"]
pub type CC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCFAIL` writer - Set OSCFAIL Interrupt Flag"]
pub type OSCFAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTTICK` writer - Set CNTTICK Interrupt Flag"]
pub type CNTTICK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINTICK` writer - Set MINTICK Interrupt Flag"]
pub type MINTICK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOURTICK` writer - Set HOURTICK Interrupt Flag"]
pub type HOURTICK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAYTICK` writer - Set DAYTICK Interrupt Flag"]
pub type DAYTICK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAYOWOF` writer - Set DAYOWOF Interrupt Flag"]
pub type DAYOWOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONTHTICK` writer - Set MONTHTICK Interrupt Flag"]
pub type MONTHTICK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set OF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<IFSrs> {
        OF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set CC0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc0(&mut self) -> CC0_W<IFSrs> {
        CC0_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set CC1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> CC1_W<IFSrs> {
        CC1_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set CC2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> CC2_W<IFSrs> {
        CC2_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set OSCFAIL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn oscfail(&mut self) -> OSCFAIL_W<IFSrs> {
        OSCFAIL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set CNTTICK Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cnttick(&mut self) -> CNTTICK_W<IFSrs> {
        CNTTICK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set MINTICK Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mintick(&mut self) -> MINTICK_W<IFSrs> {
        MINTICK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set HOURTICK Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hourtick(&mut self) -> HOURTICK_W<IFSrs> {
        HOURTICK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set DAYTICK Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn daytick(&mut self) -> DAYTICK_W<IFSrs> {
        DAYTICK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set DAYOWOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dayowof(&mut self) -> DAYOWOF_W<IFSrs> {
        DAYOWOF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set MONTHTICK Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn monthtick(&mut self) -> MONTHTICK_W<IFSrs> {
        MONTHTICK_W::new(self, 10)
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
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFSrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
