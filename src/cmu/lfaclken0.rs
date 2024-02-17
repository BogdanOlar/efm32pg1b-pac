#[doc = "Register `LFACLKEN0` reader"]
pub type R = crate::R<LFACLKEN0rs>;
#[doc = "Register `LFACLKEN0` writer"]
pub type W = crate::W<LFACLKEN0rs>;
#[doc = "Field `LETIMER0` reader - Low Energy Timer 0 Clock Enable"]
pub type LETIMER0_R = crate::BitReader;
#[doc = "Field `LETIMER0` writer - Low Energy Timer 0 Clock Enable"]
pub type LETIMER0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low Energy Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Energy Timer 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn letimer0(&mut self) -> LETIMER0_W<LFACLKEN0rs> {
        LETIMER0_W::new(self, 0)
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
#[doc = "Low Frequency a Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfaclken0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfaclken0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFACLKEN0rs;
impl crate::RegisterSpec for LFACLKEN0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfaclken0::R`](R) reader structure"]
impl crate::Readable for LFACLKEN0rs {}
#[doc = "`write(|w| ..)` method takes [`lfaclken0::W`](W) writer structure"]
impl crate::Writable for LFACLKEN0rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFACLKEN0 to value 0"]
impl crate::Resettable for LFACLKEN0rs {
    const RESET_VALUE: u32 = 0;
}
