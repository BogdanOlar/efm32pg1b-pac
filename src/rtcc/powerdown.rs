#[doc = "Register `POWERDOWN` reader"]
pub type R = crate::R<POWERDOWNrs>;
#[doc = "Register `POWERDOWN` writer"]
pub type W = crate::W<POWERDOWNrs>;
#[doc = "Field `RAM` reader - Retention RAM Power-down"]
pub type RamR = crate::BitReader;
#[doc = "Field `RAM` writer - Retention RAM Power-down"]
pub type RamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Retention RAM Power-down"]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Retention RAM Power-down"]
    #[inline(always)]
    #[must_use]
    pub fn ram(&mut self) -> RamW<POWERDOWNrs> {
        RamW::new(self, 0)
    }
}
#[doc = "Retention RAM Power-down Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`powerdown::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`powerdown::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWERDOWNrs;
impl crate::RegisterSpec for POWERDOWNrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`powerdown::R`](R) reader structure"]
impl crate::Readable for POWERDOWNrs {}
#[doc = "`write(|w| ..)` method takes [`powerdown::W`](W) writer structure"]
impl crate::Writable for POWERDOWNrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWERDOWN to value 0"]
impl crate::Resettable for POWERDOWNrs {
    const RESET_VALUE: u32 = 0;
}
