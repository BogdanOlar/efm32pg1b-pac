///Register `BIASCONF` reader
pub type R = crate::R<BIASCONFrs>;
///Register `BIASCONF` writer
pub type W = crate::W<BIASCONFrs>;
///Field `NADUTYEM01` reader - NA DUTY in EM01
pub type Nadutyem01R = crate::BitReader;
///Field `NADUTYEM01` writer - NA DUTY in EM01
pub type Nadutyem01W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPEM01` reader - LP in EM01
pub type Lpem01R = crate::BitReader;
///Field `LPEM01` writer - LP in EM01
pub type Lpem01W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GMCEM23` reader - GMC in EM234
pub type Gmcem23R = crate::BitReader;
///Field `GMCEM23` writer - GMC in EM234
pub type Gmcem23W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UADUTYEM23` reader - UADUTY in EM234
pub type Uadutyem23R = crate::BitReader;
///Field `UADUTYEM23` writer - UADUTY in EM234
pub type Uadutyem23W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NADUTYEM23` reader - NA DUTY in EM234
pub type Nadutyem23R = crate::BitReader;
///Field `NADUTYEM23` writer - NA DUTY in EM234
pub type Nadutyem23W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPEM23` reader - LP in EM234
pub type Lpem23R = crate::BitReader;
///Field `LPEM23` writer - LP in EM234
pub type Lpem23W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - NA DUTY in EM01
    #[inline(always)]
    pub fn nadutyem01(&self) -> Nadutyem01R {
        Nadutyem01R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LP in EM01
    #[inline(always)]
    pub fn lpem01(&self) -> Lpem01R {
        Lpem01R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GMC in EM234
    #[inline(always)]
    pub fn gmcem23(&self) -> Gmcem23R {
        Gmcem23R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - UADUTY in EM234
    #[inline(always)]
    pub fn uadutyem23(&self) -> Uadutyem23R {
        Uadutyem23R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - NA DUTY in EM234
    #[inline(always)]
    pub fn nadutyem23(&self) -> Nadutyem23R {
        Nadutyem23R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LP in EM234
    #[inline(always)]
    pub fn lpem23(&self) -> Lpem23R {
        Lpem23R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BIASCONF")
            .field("nadutyem01", &self.nadutyem01())
            .field("lpem01", &self.lpem01())
            .field("gmcem23", &self.gmcem23())
            .field("uadutyem23", &self.uadutyem23())
            .field("nadutyem23", &self.nadutyem23())
            .field("lpem23", &self.lpem23())
            .finish()
    }
}
impl W {
    ///Bit 2 - NA DUTY in EM01
    #[inline(always)]
    #[must_use]
    pub fn nadutyem01(&mut self) -> Nadutyem01W<BIASCONFrs> {
        Nadutyem01W::new(self, 2)
    }
    ///Bit 3 - LP in EM01
    #[inline(always)]
    #[must_use]
    pub fn lpem01(&mut self) -> Lpem01W<BIASCONFrs> {
        Lpem01W::new(self, 3)
    }
    ///Bit 4 - GMC in EM234
    #[inline(always)]
    #[must_use]
    pub fn gmcem23(&mut self) -> Gmcem23W<BIASCONFrs> {
        Gmcem23W::new(self, 4)
    }
    ///Bit 5 - UADUTY in EM234
    #[inline(always)]
    #[must_use]
    pub fn uadutyem23(&mut self) -> Uadutyem23W<BIASCONFrs> {
        Uadutyem23W::new(self, 5)
    }
    ///Bit 6 - NA DUTY in EM234
    #[inline(always)]
    #[must_use]
    pub fn nadutyem23(&mut self) -> Nadutyem23W<BIASCONFrs> {
        Nadutyem23W::new(self, 6)
    }
    ///Bit 7 - LP in EM234
    #[inline(always)]
    #[must_use]
    pub fn lpem23(&mut self) -> Lpem23W<BIASCONFrs> {
        Lpem23W::new(self, 7)
    }
}
///Configurations Related to the Bias
///
///You can [`read`](crate::Reg::read) this register and get [`biasconf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biasconf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BIASCONFrs;
impl crate::RegisterSpec for BIASCONFrs {
    type Ux = u32;
}
///`read()` method returns [`biasconf::R`](R) reader structure
impl crate::Readable for BIASCONFrs {}
///`write(|w| ..)` method takes [`biasconf::W`](W) writer structure
impl crate::Writable for BIASCONFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BIASCONF to value 0xf8
impl crate::Resettable for BIASCONFrs {
    const RESET_VALUE: u32 = 0xf8;
}
