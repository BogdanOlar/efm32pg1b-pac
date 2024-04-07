#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFSrs>;
#[doc = "Field `COMP0` writer - Set COMP0 Interrupt Flag"]
pub type Comp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1` writer - Set COMP1 Interrupt Flag"]
pub type Comp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` writer - Set UF Interrupt Flag"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REP0` writer - Set REP0 Interrupt Flag"]
pub type Rep0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REP1` writer - Set REP1 Interrupt Flag"]
pub type Rep1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set COMP0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> Comp0W<IFSrs> {
        Comp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Set COMP1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> Comp1W<IFSrs> {
        Comp1W::new(self, 1)
    }
    #[doc = "Bit 2 - Set UF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UfW<IFSrs> {
        UfW::new(self, 2)
    }
    #[doc = "Bit 3 - Set REP0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rep0(&mut self) -> Rep0W<IFSrs> {
        Rep0W::new(self, 3)
    }
    #[doc = "Bit 4 - Set REP1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rep1(&mut self) -> Rep1W<IFSrs> {
        Rep1W::new(self, 4)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
