#[doc = "Register `BIASCONF` reader"]
pub type R = crate::R<BIASCONFrs>;
#[doc = "Register `BIASCONF` writer"]
pub type W = crate::W<BIASCONFrs>;
#[doc = "Field `NADUTYEM01` reader - NA DUTY in EM01"]
pub type NADUTYEM01_R = crate::BitReader;
#[doc = "Field `NADUTYEM01` writer - NA DUTY in EM01"]
pub type NADUTYEM01_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPEM01` reader - LP in EM01"]
pub type LPEM01_R = crate::BitReader;
#[doc = "Field `LPEM01` writer - LP in EM01"]
pub type LPEM01_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMCEM23` reader - GMC in EM234"]
pub type GMCEM23_R = crate::BitReader;
#[doc = "Field `GMCEM23` writer - GMC in EM234"]
pub type GMCEM23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UADUTYEM23` reader - UADUTY in EM234"]
pub type UADUTYEM23_R = crate::BitReader;
#[doc = "Field `UADUTYEM23` writer - UADUTY in EM234"]
pub type UADUTYEM23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NADUTYEM23` reader - NA DUTY in EM234"]
pub type NADUTYEM23_R = crate::BitReader;
#[doc = "Field `NADUTYEM23` writer - NA DUTY in EM234"]
pub type NADUTYEM23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPEM23` reader - LP in EM234"]
pub type LPEM23_R = crate::BitReader;
#[doc = "Field `LPEM23` writer - LP in EM234"]
pub type LPEM23_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - NA DUTY in EM01"]
    #[inline(always)]
    pub fn nadutyem01(&self) -> NADUTYEM01_R {
        NADUTYEM01_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LP in EM01"]
    #[inline(always)]
    pub fn lpem01(&self) -> LPEM01_R {
        LPEM01_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GMC in EM234"]
    #[inline(always)]
    pub fn gmcem23(&self) -> GMCEM23_R {
        GMCEM23_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UADUTY in EM234"]
    #[inline(always)]
    pub fn uadutyem23(&self) -> UADUTYEM23_R {
        UADUTYEM23_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA DUTY in EM234"]
    #[inline(always)]
    pub fn nadutyem23(&self) -> NADUTYEM23_R {
        NADUTYEM23_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LP in EM234"]
    #[inline(always)]
    pub fn lpem23(&self) -> LPEM23_R {
        LPEM23_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - NA DUTY in EM01"]
    #[inline(always)]
    #[must_use]
    pub fn nadutyem01(&mut self) -> NADUTYEM01_W<BIASCONFrs> {
        NADUTYEM01_W::new(self, 2)
    }
    #[doc = "Bit 3 - LP in EM01"]
    #[inline(always)]
    #[must_use]
    pub fn lpem01(&mut self) -> LPEM01_W<BIASCONFrs> {
        LPEM01_W::new(self, 3)
    }
    #[doc = "Bit 4 - GMC in EM234"]
    #[inline(always)]
    #[must_use]
    pub fn gmcem23(&mut self) -> GMCEM23_W<BIASCONFrs> {
        GMCEM23_W::new(self, 4)
    }
    #[doc = "Bit 5 - UADUTY in EM234"]
    #[inline(always)]
    #[must_use]
    pub fn uadutyem23(&mut self) -> UADUTYEM23_W<BIASCONFrs> {
        UADUTYEM23_W::new(self, 5)
    }
    #[doc = "Bit 6 - NA DUTY in EM234"]
    #[inline(always)]
    #[must_use]
    pub fn nadutyem23(&mut self) -> NADUTYEM23_W<BIASCONFrs> {
        NADUTYEM23_W::new(self, 6)
    }
    #[doc = "Bit 7 - LP in EM234"]
    #[inline(always)]
    #[must_use]
    pub fn lpem23(&mut self) -> LPEM23_W<BIASCONFrs> {
        LPEM23_W::new(self, 7)
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
#[doc = "Configurations Related to the Bias\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`biasconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`biasconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIASCONFrs;
impl crate::RegisterSpec for BIASCONFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`biasconf::R`](R) reader structure"]
impl crate::Readable for BIASCONFrs {}
#[doc = "`write(|w| ..)` method takes [`biasconf::W`](W) writer structure"]
impl crate::Writable for BIASCONFrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BIASCONF to value 0xf8"]
impl crate::Resettable for BIASCONFrs {
    const RESET_VALUE: u32 = 0xf8;
}
