#[doc = "Register `IEN` reader"]
pub type R = crate::R<IENrs>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IENrs>;
#[doc = "Field `UF` reader - UF Interrupt Enable"]
pub type UfR = crate::BitReader;
#[doc = "Field `UF` writer - UF Interrupt Enable"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OF` reader - OF Interrupt Enable"]
pub type OfR = crate::BitReader;
#[doc = "Field `OF` writer - OF Interrupt Enable"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCNG` reader - DIRCNG Interrupt Enable"]
pub type DircngR = crate::BitReader;
#[doc = "Field `DIRCNG` writer - DIRCNG Interrupt Enable"]
pub type DircngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXOF` reader - AUXOF Interrupt Enable"]
pub type AuxofR = crate::BitReader;
#[doc = "Field `AUXOF` writer - AUXOF Interrupt Enable"]
pub type AuxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` reader - TCC Interrupt Enable"]
pub type TccR = crate::BitReader;
#[doc = "Field `TCC` writer - TCC Interrupt Enable"]
pub type TccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OQSTERR` reader - OQSTERR Interrupt Enable"]
pub type OqsterrR = crate::BitReader;
#[doc = "Field `OQSTERR` writer - OQSTERR Interrupt Enable"]
pub type OqsterrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UF Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIRCNG Interrupt Enable"]
    #[inline(always)]
    pub fn dircng(&self) -> DircngR {
        DircngR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AUXOF Interrupt Enable"]
    #[inline(always)]
    pub fn auxof(&self) -> AuxofR {
        AuxofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TCC Interrupt Enable"]
    #[inline(always)]
    pub fn tcc(&self) -> TccR {
        TccR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OQSTERR Interrupt Enable"]
    #[inline(always)]
    pub fn oqsterr(&self) -> OqsterrR {
        OqsterrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UfW<IENrs> {
        UfW::new(self, 0)
    }
    #[doc = "Bit 1 - OF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OfW<IENrs> {
        OfW::new(self, 1)
    }
    #[doc = "Bit 2 - DIRCNG Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dircng(&mut self) -> DircngW<IENrs> {
        DircngW::new(self, 2)
    }
    #[doc = "Bit 3 - AUXOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auxof(&mut self) -> AuxofW<IENrs> {
        AuxofW::new(self, 3)
    }
    #[doc = "Bit 4 - TCC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TccW<IENrs> {
        TccW::new(self, 4)
    }
    #[doc = "Bit 5 - OQSTERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oqsterr(&mut self) -> OqsterrW<IENrs> {
        OqsterrW::new(self, 5)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
