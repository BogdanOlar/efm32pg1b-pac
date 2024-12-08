///Register `IEN` reader
pub type R = crate::R<IENrs>;
///Register `IEN` writer
pub type W = crate::W<IENrs>;
///Field `OF` reader - OF Interrupt Enable
pub type OfR = crate::BitReader;
///Field `OF` writer - OF Interrupt Enable
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UF` reader - UF Interrupt Enable
pub type UfR = crate::BitReader;
///Field `UF` writer - UF Interrupt Enable
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIRCHG` reader - DIRCHG Interrupt Enable
pub type DirchgR = crate::BitReader;
///Field `DIRCHG` writer - DIRCHG Interrupt Enable
pub type DirchgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC0` reader - CC0 Interrupt Enable
pub type Cc0R = crate::BitReader;
///Field `CC0` writer - CC0 Interrupt Enable
pub type Cc0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1` reader - CC1 Interrupt Enable
pub type Cc1R = crate::BitReader;
///Field `CC1` writer - CC1 Interrupt Enable
pub type Cc1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2` reader - CC2 Interrupt Enable
pub type Cc2R = crate::BitReader;
///Field `CC2` writer - CC2 Interrupt Enable
pub type Cc2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3` reader - CC3 Interrupt Enable
pub type Cc3R = crate::BitReader;
///Field `CC3` writer - CC3 Interrupt Enable
pub type Cc3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICBOF0` reader - ICBOF0 Interrupt Enable
pub type Icbof0R = crate::BitReader;
///Field `ICBOF0` writer - ICBOF0 Interrupt Enable
pub type Icbof0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICBOF1` reader - ICBOF1 Interrupt Enable
pub type Icbof1R = crate::BitReader;
///Field `ICBOF1` writer - ICBOF1 Interrupt Enable
pub type Icbof1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICBOF2` reader - ICBOF2 Interrupt Enable
pub type Icbof2R = crate::BitReader;
///Field `ICBOF2` writer - ICBOF2 Interrupt Enable
pub type Icbof2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICBOF3` reader - ICBOF3 Interrupt Enable
pub type Icbof3R = crate::BitReader;
///Field `ICBOF3` writer - ICBOF3 Interrupt Enable
pub type Icbof3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - OF Interrupt Enable
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UF Interrupt Enable
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DIRCHG Interrupt Enable
    #[inline(always)]
    pub fn dirchg(&self) -> DirchgR {
        DirchgR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - CC0 Interrupt Enable
    #[inline(always)]
    pub fn cc0(&self) -> Cc0R {
        Cc0R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CC1 Interrupt Enable
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CC2 Interrupt Enable
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CC3 Interrupt Enable
    #[inline(always)]
    pub fn cc3(&self) -> Cc3R {
        Cc3R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ICBOF0 Interrupt Enable
    #[inline(always)]
    pub fn icbof0(&self) -> Icbof0R {
        Icbof0R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ICBOF1 Interrupt Enable
    #[inline(always)]
    pub fn icbof1(&self) -> Icbof1R {
        Icbof1R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ICBOF2 Interrupt Enable
    #[inline(always)]
    pub fn icbof2(&self) -> Icbof2R {
        Icbof2R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ICBOF3 Interrupt Enable
    #[inline(always)]
    pub fn icbof3(&self) -> Icbof3R {
        Icbof3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("of", &self.of())
            .field("uf", &self.uf())
            .field("dirchg", &self.dirchg())
            .field("cc0", &self.cc0())
            .field("cc1", &self.cc1())
            .field("cc2", &self.cc2())
            .field("cc3", &self.cc3())
            .field("icbof0", &self.icbof0())
            .field("icbof1", &self.icbof1())
            .field("icbof2", &self.icbof2())
            .field("icbof3", &self.icbof3())
            .finish()
    }
}
impl W {
    ///Bit 0 - OF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OfW<IENrs> {
        OfW::new(self, 0)
    }
    ///Bit 1 - UF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UfW<IENrs> {
        UfW::new(self, 1)
    }
    ///Bit 2 - DIRCHG Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn dirchg(&mut self) -> DirchgW<IENrs> {
        DirchgW::new(self, 2)
    }
    ///Bit 4 - CC0 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn cc0(&mut self) -> Cc0W<IENrs> {
        Cc0W::new(self, 4)
    }
    ///Bit 5 - CC1 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> Cc1W<IENrs> {
        Cc1W::new(self, 5)
    }
    ///Bit 6 - CC2 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> Cc2W<IENrs> {
        Cc2W::new(self, 6)
    }
    ///Bit 7 - CC3 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn cc3(&mut self) -> Cc3W<IENrs> {
        Cc3W::new(self, 7)
    }
    ///Bit 8 - ICBOF0 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn icbof0(&mut self) -> Icbof0W<IENrs> {
        Icbof0W::new(self, 8)
    }
    ///Bit 9 - ICBOF1 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn icbof1(&mut self) -> Icbof1W<IENrs> {
        Icbof1W::new(self, 9)
    }
    ///Bit 10 - ICBOF2 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn icbof2(&mut self) -> Icbof2W<IENrs> {
        Icbof2W::new(self, 10)
    }
    ///Bit 11 - ICBOF3 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn icbof3(&mut self) -> Icbof3W<IENrs> {
        Icbof3W::new(self, 11)
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
