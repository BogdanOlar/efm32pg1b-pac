#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPENrs>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPENrs>;
#[doc = "Field `CLKOUT0PEN` reader - CLKOUT0 Pin Enable"]
pub type CLKOUT0PEN_R = crate::BitReader;
#[doc = "Field `CLKOUT0PEN` writer - CLKOUT0 Pin Enable"]
pub type CLKOUT0PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUT1PEN` reader - CLKOUT1 Pin Enable"]
pub type CLKOUT1PEN_R = crate::BitReader;
#[doc = "Field `CLKOUT1PEN` writer - CLKOUT1 Pin Enable"]
pub type CLKOUT1PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CLKOUT0 Pin Enable"]
    #[inline(always)]
    pub fn clkout0pen(&self) -> CLKOUT0PEN_R {
        CLKOUT0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CLKOUT1 Pin Enable"]
    #[inline(always)]
    pub fn clkout1pen(&self) -> CLKOUT1PEN_R {
        CLKOUT1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLKOUT0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkout0pen(&mut self) -> CLKOUT0PEN_W<ROUTEPENrs> {
        CLKOUT0PEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CLKOUT1 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkout1pen(&mut self) -> CLKOUT1PEN_W<ROUTEPENrs> {
        CLKOUT1PEN_W::new(self, 1)
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
#[doc = "I/O Routing Pin Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTEPENrs;
impl crate::RegisterSpec for ROUTEPENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for ROUTEPENrs {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for ROUTEPENrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPENrs {
    const RESET_VALUE: u32 = 0;
}
