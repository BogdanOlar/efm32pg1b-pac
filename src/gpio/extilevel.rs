///Register `EXTILEVEL` reader
pub type R = crate::R<EXTILEVELrs>;
///Register `EXTILEVEL` writer
pub type W = crate::W<EXTILEVELrs>;
///Field `EM4WU0` reader - EM4 Wake Up Level for EM4WU0 Pin
pub type Em4wu0R = crate::BitReader;
///Field `EM4WU0` writer - EM4 Wake Up Level for EM4WU0 Pin
pub type Em4wu0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM4WU1` reader - EM4 Wake Up Level for EM4WU1 Pin
pub type Em4wu1R = crate::BitReader;
///Field `EM4WU1` writer - EM4 Wake Up Level for EM4WU1 Pin
pub type Em4wu1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM4WU4` reader - EM4 Wake Up Level for EM4WU4 Pin
pub type Em4wu4R = crate::BitReader;
///Field `EM4WU4` writer - EM4 Wake Up Level for EM4WU4 Pin
pub type Em4wu4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM4WU8` reader - EM4 Wake Up Level for EM4WU8 Pin
pub type Em4wu8R = crate::BitReader;
///Field `EM4WU8` writer - EM4 Wake Up Level for EM4WU8 Pin
pub type Em4wu8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM4WU9` reader - EM4 Wake Up Level for EM4WU9 Pin
pub type Em4wu9R = crate::BitReader;
///Field `EM4WU9` writer - EM4 Wake Up Level for EM4WU9 Pin
pub type Em4wu9W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM4WU12` reader - EM4 Wake Up Level for EM4WU12 Pin
pub type Em4wu12R = crate::BitReader;
///Field `EM4WU12` writer - EM4 Wake Up Level for EM4WU12 Pin
pub type Em4wu12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 16 - EM4 Wake Up Level for EM4WU0 Pin
    #[inline(always)]
    pub fn em4wu0(&self) -> Em4wu0R {
        Em4wu0R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - EM4 Wake Up Level for EM4WU1 Pin
    #[inline(always)]
    pub fn em4wu1(&self) -> Em4wu1R {
        Em4wu1R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - EM4 Wake Up Level for EM4WU4 Pin
    #[inline(always)]
    pub fn em4wu4(&self) -> Em4wu4R {
        Em4wu4R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - EM4 Wake Up Level for EM4WU8 Pin
    #[inline(always)]
    pub fn em4wu8(&self) -> Em4wu8R {
        Em4wu8R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - EM4 Wake Up Level for EM4WU9 Pin
    #[inline(always)]
    pub fn em4wu9(&self) -> Em4wu9R {
        Em4wu9R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - EM4 Wake Up Level for EM4WU12 Pin
    #[inline(always)]
    pub fn em4wu12(&self) -> Em4wu12R {
        Em4wu12R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTILEVEL")
            .field("em4wu0", &self.em4wu0())
            .field("em4wu1", &self.em4wu1())
            .field("em4wu4", &self.em4wu4())
            .field("em4wu8", &self.em4wu8())
            .field("em4wu9", &self.em4wu9())
            .field("em4wu12", &self.em4wu12())
            .finish()
    }
}
impl W {
    ///Bit 16 - EM4 Wake Up Level for EM4WU0 Pin
    #[inline(always)]
    #[must_use]
    pub fn em4wu0(&mut self) -> Em4wu0W<EXTILEVELrs> {
        Em4wu0W::new(self, 16)
    }
    ///Bit 17 - EM4 Wake Up Level for EM4WU1 Pin
    #[inline(always)]
    #[must_use]
    pub fn em4wu1(&mut self) -> Em4wu1W<EXTILEVELrs> {
        Em4wu1W::new(self, 17)
    }
    ///Bit 20 - EM4 Wake Up Level for EM4WU4 Pin
    #[inline(always)]
    #[must_use]
    pub fn em4wu4(&mut self) -> Em4wu4W<EXTILEVELrs> {
        Em4wu4W::new(self, 20)
    }
    ///Bit 24 - EM4 Wake Up Level for EM4WU8 Pin
    #[inline(always)]
    #[must_use]
    pub fn em4wu8(&mut self) -> Em4wu8W<EXTILEVELrs> {
        Em4wu8W::new(self, 24)
    }
    ///Bit 25 - EM4 Wake Up Level for EM4WU9 Pin
    #[inline(always)]
    #[must_use]
    pub fn em4wu9(&mut self) -> Em4wu9W<EXTILEVELrs> {
        Em4wu9W::new(self, 25)
    }
    ///Bit 28 - EM4 Wake Up Level for EM4WU12 Pin
    #[inline(always)]
    #[must_use]
    pub fn em4wu12(&mut self) -> Em4wu12W<EXTILEVELrs> {
        Em4wu12W::new(self, 28)
    }
}
///External Interrupt Level Register
///
///You can [`read`](crate::Reg::read) this register and get [`extilevel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extilevel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct EXTILEVELrs;
impl crate::RegisterSpec for EXTILEVELrs {
    type Ux = u32;
}
///`read()` method returns [`extilevel::R`](R) reader structure
impl crate::Readable for EXTILEVELrs {}
///`write(|w| ..)` method takes [`extilevel::W`](W) writer structure
impl crate::Writable for EXTILEVELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTILEVEL to value 0
impl crate::Resettable for EXTILEVELrs {
    const RESET_VALUE: u32 = 0;
}
