///Register `POWERDOWN` reader
pub type R = crate::R<POWERDOWNrs>;
///Register `POWERDOWN` writer
pub type W = crate::W<POWERDOWNrs>;
///Field `RAM` reader - Retention RAM Power-down
pub type RamR = crate::BitReader;
///Field `RAM` writer - Retention RAM Power-down
pub type RamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Retention RAM Power-down
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWERDOWN")
            .field("ram", &self.ram())
            .finish()
    }
}
impl W {
    ///Bit 0 - Retention RAM Power-down
    #[inline(always)]
    pub fn ram(&mut self) -> RamW<'_, POWERDOWNrs> {
        RamW::new(self, 0)
    }
}
///Retention RAM Power-down Register
///
///You can [`read`](crate::Reg::read) this register and get [`powerdown::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`powerdown::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct POWERDOWNrs;
impl crate::RegisterSpec for POWERDOWNrs {
    type Ux = u32;
}
///`read()` method returns [`powerdown::R`](R) reader structure
impl crate::Readable for POWERDOWNrs {}
///`write(|w| ..)` method takes [`powerdown::W`](W) writer structure
impl crate::Writable for POWERDOWNrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets POWERDOWN to value 0
impl crate::Resettable for POWERDOWNrs {}
