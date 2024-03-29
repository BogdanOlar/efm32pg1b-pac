#[doc = "Register `LFBCLKEN0` reader"]
pub type R = crate::R<LFBCLKEN0rs>;
#[doc = "Register `LFBCLKEN0` writer"]
pub type W = crate::W<LFBCLKEN0rs>;
#[doc = "Field `LEUART0` reader - Low Energy UART 0 Clock Enable"]
pub type LEUART0_R = crate::BitReader;
#[doc = "Field `LEUART0` writer - Low Energy UART 0 Clock Enable"]
pub type LEUART0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low Energy UART 0 Clock Enable"]
    #[inline(always)]
    pub fn leuart0(&self) -> LEUART0_R {
        LEUART0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Energy UART 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn leuart0(&mut self) -> LEUART0_W<LFBCLKEN0rs> {
        LEUART0_W::new(self, 0)
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
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfbclken0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfbclken0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFBCLKEN0rs;
impl crate::RegisterSpec for LFBCLKEN0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfbclken0::R`](R) reader structure"]
impl crate::Readable for LFBCLKEN0rs {}
#[doc = "`write(|w| ..)` method takes [`lfbclken0::W`](W) writer structure"]
impl crate::Writable for LFBCLKEN0rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFBCLKEN0 to value 0"]
impl crate::Resettable for LFBCLKEN0rs {
    const RESET_VALUE: u32 = 0;
}
