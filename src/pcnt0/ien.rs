///Register `IEN` reader
pub type R = crate::R<IENrs>;
///Register `IEN` writer
pub type W = crate::W<IENrs>;
///Field `UF` reader - UF Interrupt Enable
pub type UfR = crate::BitReader;
///Field `UF` writer - UF Interrupt Enable
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OF` reader - OF Interrupt Enable
pub type OfR = crate::BitReader;
///Field `OF` writer - OF Interrupt Enable
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIRCNG` reader - DIRCNG Interrupt Enable
pub type DircngR = crate::BitReader;
///Field `DIRCNG` writer - DIRCNG Interrupt Enable
pub type DircngW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUXOF` reader - AUXOF Interrupt Enable
pub type AuxofR = crate::BitReader;
///Field `AUXOF` writer - AUXOF Interrupt Enable
pub type AuxofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCC` reader - TCC Interrupt Enable
pub type TccR = crate::BitReader;
///Field `TCC` writer - TCC Interrupt Enable
pub type TccW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OQSTERR` reader - OQSTERR Interrupt Enable
pub type OqsterrR = crate::BitReader;
///Field `OQSTERR` writer - OQSTERR Interrupt Enable
pub type OqsterrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - UF Interrupt Enable
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - OF Interrupt Enable
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DIRCNG Interrupt Enable
    #[inline(always)]
    pub fn dircng(&self) -> DircngR {
        DircngR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AUXOF Interrupt Enable
    #[inline(always)]
    pub fn auxof(&self) -> AuxofR {
        AuxofR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TCC Interrupt Enable
    #[inline(always)]
    pub fn tcc(&self) -> TccR {
        TccR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - OQSTERR Interrupt Enable
    #[inline(always)]
    pub fn oqsterr(&self) -> OqsterrR {
        OqsterrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("uf", &self.uf())
            .field("of", &self.of())
            .field("dircng", &self.dircng())
            .field("auxof", &self.auxof())
            .field("tcc", &self.tcc())
            .field("oqsterr", &self.oqsterr())
            .finish()
    }
}
impl W {
    ///Bit 0 - UF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UfW<IENrs> {
        UfW::new(self, 0)
    }
    ///Bit 1 - OF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OfW<IENrs> {
        OfW::new(self, 1)
    }
    ///Bit 2 - DIRCNG Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn dircng(&mut self) -> DircngW<IENrs> {
        DircngW::new(self, 2)
    }
    ///Bit 3 - AUXOF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn auxof(&mut self) -> AuxofW<IENrs> {
        AuxofW::new(self, 3)
    }
    ///Bit 4 - TCC Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TccW<IENrs> {
        TccW::new(self, 4)
    }
    ///Bit 5 - OQSTERR Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn oqsterr(&mut self) -> OqsterrW<IENrs> {
        OqsterrW::new(self, 5)
    }
}
///Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
///`read()` method returns [`ien::R`](R) reader structure
impl crate::Readable for IENrs {}
///`write(|w| ..)` method takes [`ien::W`](W) writer structure
impl crate::Writable for IENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IEN to value 0
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
