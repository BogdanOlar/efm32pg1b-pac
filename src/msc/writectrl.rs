#[doc = "Register `WRITECTRL` reader"]
pub type R = crate::R<WRITECTRLrs>;
#[doc = "Register `WRITECTRL` writer"]
pub type W = crate::W<WRITECTRLrs>;
#[doc = "Field `WREN` reader - Enable Write/Erase Controller"]
pub type WREN_R = crate::BitReader;
#[doc = "Field `WREN` writer - Enable Write/Erase Controller"]
pub type WREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQERASEABORT` reader - Abort Page Erase on Interrupt"]
pub type IRQERASEABORT_R = crate::BitReader;
#[doc = "Field `IRQERASEABORT` writer - Abort Page Erase on Interrupt"]
pub type IRQERASEABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    pub fn irqeraseabort(&self) -> IRQERASEABORT_R {
        IRQERASEABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WREN_W<WRITECTRLrs> {
        WREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn irqeraseabort(&mut self) -> IRQERASEABORT_W<WRITECTRLrs> {
        IRQERASEABORT_W::new(self, 1)
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
#[doc = "Write Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`writectrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`writectrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRITECTRLrs;
impl crate::RegisterSpec for WRITECTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`writectrl::R`](R) reader structure"]
impl crate::Readable for WRITECTRLrs {}
#[doc = "`write(|w| ..)` method takes [`writectrl::W`](W) writer structure"]
impl crate::Writable for WRITECTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRITECTRL to value 0"]
impl crate::Resettable for WRITECTRLrs {
    const RESET_VALUE: u32 = 0;
}
