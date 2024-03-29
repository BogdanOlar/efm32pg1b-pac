#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPENrs>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPENrs>;
#[doc = "Field `RXPEN` reader - RX Pin Enable"]
pub type RXPEN_R = crate::BitReader;
#[doc = "Field `RXPEN` writer - RX Pin Enable"]
pub type RXPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPEN` reader - TX Pin Enable"]
pub type TXPEN_R = crate::BitReader;
#[doc = "Field `TXPEN` writer - TX Pin Enable"]
pub type TXPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPEN` reader - CS Pin Enable"]
pub type CSPEN_R = crate::BitReader;
#[doc = "Field `CSPEN` writer - CS Pin Enable"]
pub type CSPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPEN` reader - CLK Pin Enable"]
pub type CLKPEN_R = crate::BitReader;
#[doc = "Field `CLKPEN` writer - CLK Pin Enable"]
pub type CLKPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSPEN` reader - CTS Pin Enable"]
pub type CTSPEN_R = crate::BitReader;
#[doc = "Field `CTSPEN` writer - CTS Pin Enable"]
pub type CTSPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSPEN` reader - RTS Pin Enable"]
pub type RTSPEN_R = crate::BitReader;
#[doc = "Field `RTSPEN` writer - RTS Pin Enable"]
pub type RTSPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX Pin Enable"]
    #[inline(always)]
    pub fn rxpen(&self) -> RXPEN_R {
        RXPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Pin Enable"]
    #[inline(always)]
    pub fn txpen(&self) -> TXPEN_R {
        TXPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CS Pin Enable"]
    #[inline(always)]
    pub fn cspen(&self) -> CSPEN_R {
        CSPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CLK Pin Enable"]
    #[inline(always)]
    pub fn clkpen(&self) -> CLKPEN_R {
        CLKPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CTS Pin Enable"]
    #[inline(always)]
    pub fn ctspen(&self) -> CTSPEN_R {
        CTSPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTS Pin Enable"]
    #[inline(always)]
    pub fn rtspen(&self) -> RTSPEN_R {
        RTSPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxpen(&mut self) -> RXPEN_W<ROUTEPENrs> {
        RXPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TX Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txpen(&mut self) -> TXPEN_W<ROUTEPENrs> {
        TXPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cspen(&mut self) -> CSPEN_W<ROUTEPENrs> {
        CSPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - CLK Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkpen(&mut self) -> CLKPEN_W<ROUTEPENrs> {
        CLKPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - CTS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctspen(&mut self) -> CTSPEN_W<ROUTEPENrs> {
        CTSPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - RTS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtspen(&mut self) -> RTSPEN_W<ROUTEPENrs> {
        RTSPEN_W::new(self, 5)
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
