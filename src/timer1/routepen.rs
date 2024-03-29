#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPENrs>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPENrs>;
#[doc = "Field `CC0PEN` reader - CC Channel 0 Pin Enable"]
pub type CC0PEN_R = crate::BitReader;
#[doc = "Field `CC0PEN` writer - CC Channel 0 Pin Enable"]
pub type CC0PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1PEN` reader - CC Channel 1 Pin Enable"]
pub type CC1PEN_R = crate::BitReader;
#[doc = "Field `CC1PEN` writer - CC Channel 1 Pin Enable"]
pub type CC1PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2PEN` reader - CC Channel 2 Pin Enable"]
pub type CC2PEN_R = crate::BitReader;
#[doc = "Field `CC2PEN` writer - CC Channel 2 Pin Enable"]
pub type CC2PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3PEN` reader - CC Channel 3 Pin Enable"]
pub type CC3PEN_R = crate::BitReader;
#[doc = "Field `CC3PEN` writer - CC Channel 3 Pin Enable"]
pub type CC3PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTI0PEN` reader - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI0PEN_R = crate::BitReader;
#[doc = "Field `CDTI0PEN` writer - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI0PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTI1PEN` reader - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI1PEN_R = crate::BitReader;
#[doc = "Field `CDTI1PEN` writer - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI1PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTI2PEN` reader - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI2PEN_R = crate::BitReader;
#[doc = "Field `CDTI2PEN` writer - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI2PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CC Channel 0 Pin Enable"]
    #[inline(always)]
    pub fn cc0pen(&self) -> CC0PEN_R {
        CC0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC Channel 1 Pin Enable"]
    #[inline(always)]
    pub fn cc1pen(&self) -> CC1PEN_R {
        CC1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC Channel 2 Pin Enable"]
    #[inline(always)]
    pub fn cc2pen(&self) -> CC2PEN_R {
        CC2PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC Channel 3 Pin Enable"]
    #[inline(always)]
    pub fn cc3pen(&self) -> CC3PEN_R {
        CC3PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti0pen(&self) -> CDTI0PEN_R {
        CDTI0PEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti1pen(&self) -> CDTI1PEN_R {
        CDTI1PEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti2pen(&self) -> CDTI2PEN_R {
        CDTI2PEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC Channel 0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc0pen(&mut self) -> CC0PEN_W<ROUTEPENrs> {
        CC0PEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC Channel 1 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1pen(&mut self) -> CC1PEN_W<ROUTEPENrs> {
        CC1PEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CC Channel 2 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2pen(&mut self) -> CC2PEN_W<ROUTEPENrs> {
        CC2PEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - CC Channel 3 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc3pen(&mut self) -> CC3PEN_W<ROUTEPENrs> {
        CC3PEN_W::new(self, 3)
    }
    #[doc = "Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdti0pen(&mut self) -> CDTI0PEN_W<ROUTEPENrs> {
        CDTI0PEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdti1pen(&mut self) -> CDTI1PEN_W<ROUTEPENrs> {
        CDTI1PEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdti2pen(&mut self) -> CDTI2PEN_W<ROUTEPENrs> {
        CDTI2PEN_W::new(self, 10)
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
