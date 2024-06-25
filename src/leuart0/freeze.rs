#[doc = "Register `FREEZE` reader"]
pub type R = crate::R<FREEZErs>;
#[doc = "Register `FREEZE` writer"]
pub type W = crate::W<FREEZErs>;
#[doc = "Field `REGFREEZE` reader - Register Update Freeze"]
pub type RegfreezeR = crate::BitReader;
#[doc = "Field `REGFREEZE` writer - Register Update Freeze"]
pub type RegfreezeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Register Update Freeze"]
    #[inline(always)]
    pub fn regfreeze(&self) -> RegfreezeR {
        RegfreezeR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FREEZE")
            .field("regfreeze", &self.regfreeze())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Register Update Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn regfreeze(&mut self) -> RegfreezeW<FREEZErs> {
        RegfreezeW::new(self, 0)
    }
}
#[doc = "Freeze Register\n\nYou can [`read`](crate::Reg::read) this register and get [`freeze::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freeze::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FREEZErs;
impl crate::RegisterSpec for FREEZErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freeze::R`](R) reader structure"]
impl crate::Readable for FREEZErs {}
#[doc = "`write(|w| ..)` method takes [`freeze::W`](W) writer structure"]
impl crate::Writable for FREEZErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FREEZE to value 0"]
impl crate::Resettable for FREEZErs {
    const RESET_VALUE: u32 = 0;
}
