#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFCrs>;
#[doc = "Field `UF` writer - Clear UF Interrupt Flag"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OF` writer - Clear OF Interrupt Flag"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCNG` writer - Clear DIRCNG Interrupt Flag"]
pub type DircngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXOF` writer - Clear AUXOF Interrupt Flag"]
pub type AuxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` writer - Clear TCC Interrupt Flag"]
pub type TccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OQSTERR` writer - Clear OQSTERR Interrupt Flag"]
pub type OqsterrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear UF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UfW<IFCrs> {
        UfW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear OF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OfW<IFCrs> {
        OfW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear DIRCNG Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dircng(&mut self) -> DircngW<IFCrs> {
        DircngW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear AUXOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn auxof(&mut self) -> AuxofW<IFCrs> {
        AuxofW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear TCC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TccW<IFCrs> {
        TccW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear OQSTERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn oqsterr(&mut self) -> OqsterrW<IFCrs> {
        OqsterrW::new(self, 5)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCrs;
impl crate::RegisterSpec for IFCrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFCrs {
    const RESET_VALUE: u32 = 0;
}
