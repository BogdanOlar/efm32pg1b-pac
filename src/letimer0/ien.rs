#[doc = "Register `IEN` reader"]
pub type R = crate::R<IENrs>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IENrs>;
#[doc = "Field `COMP0` reader - COMP0 Interrupt Enable"]
pub type Comp0R = crate::BitReader;
#[doc = "Field `COMP0` writer - COMP0 Interrupt Enable"]
pub type Comp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1` reader - COMP1 Interrupt Enable"]
pub type Comp1R = crate::BitReader;
#[doc = "Field `COMP1` writer - COMP1 Interrupt Enable"]
pub type Comp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` reader - UF Interrupt Enable"]
pub type UfR = crate::BitReader;
#[doc = "Field `UF` writer - UF Interrupt Enable"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REP0` reader - REP0 Interrupt Enable"]
pub type Rep0R = crate::BitReader;
#[doc = "Field `REP0` writer - REP0 Interrupt Enable"]
pub type Rep0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REP1` reader - REP1 Interrupt Enable"]
pub type Rep1R = crate::BitReader;
#[doc = "Field `REP1` writer - REP1 Interrupt Enable"]
pub type Rep1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - COMP0 Interrupt Enable"]
    #[inline(always)]
    pub fn comp0(&self) -> Comp0R {
        Comp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COMP1 Interrupt Enable"]
    #[inline(always)]
    pub fn comp1(&self) -> Comp1R {
        Comp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UF Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - REP0 Interrupt Enable"]
    #[inline(always)]
    pub fn rep0(&self) -> Rep0R {
        Rep0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REP1 Interrupt Enable"]
    #[inline(always)]
    pub fn rep1(&self) -> Rep1R {
        Rep1R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("comp0", &self.comp0())
            .field("comp1", &self.comp1())
            .field("uf", &self.uf())
            .field("rep0", &self.rep0())
            .field("rep1", &self.rep1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - COMP0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> Comp0W<IENrs> {
        Comp0W::new(self, 0)
    }
    #[doc = "Bit 1 - COMP1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> Comp1W<IENrs> {
        Comp1W::new(self, 1)
    }
    #[doc = "Bit 2 - UF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UfW<IENrs> {
        UfW::new(self, 2)
    }
    #[doc = "Bit 3 - REP0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rep0(&mut self) -> Rep0W<IENrs> {
        Rep0W::new(self, 3)
    }
    #[doc = "Bit 4 - REP1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rep1(&mut self) -> Rep1W<IENrs> {
        Rep1W::new(self, 4)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IENrs {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
