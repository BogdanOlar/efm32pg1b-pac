#[doc = "Register `FREEZE` reader"]
pub type R = crate::R<FREEZErs>;
#[doc = "Register `FREEZE` writer"]
pub type W = crate::W<FREEZErs>;
#[doc = "Field `REGFREEZE` reader - Register Update Freeze"]
pub type REGFREEZE_R = crate::BitReader;
#[doc = "Field `REGFREEZE` writer - Register Update Freeze"]
pub type REGFREEZE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Register Update Freeze"]
    #[inline(always)]
    pub fn regfreeze(&self) -> REGFREEZE_R {
        REGFREEZE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Update Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn regfreeze(&mut self) -> REGFREEZE_W<FREEZErs> {
        REGFREEZE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Freeze Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freeze::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freeze::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FREEZErs;
impl crate::RegisterSpec for FREEZErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freeze::R`](R) reader structure"]
impl crate::Readable for FREEZErs {}
#[doc = "`write(|w| ..)` method takes [`freeze::W`](W) writer structure"]
impl crate::Writable for FREEZErs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FREEZE to value 0"]
impl crate::Resettable for FREEZErs {
    const RESET_VALUE: u32 = 0;
}
