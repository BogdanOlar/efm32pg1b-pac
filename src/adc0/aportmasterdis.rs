///Register `APORTMASTERDIS` reader
pub type R = crate::R<APORTMASTERDISrs>;
///Register `APORTMASTERDIS` writer
pub type W = crate::W<APORTMASTERDISrs>;
///Field `APORT1XMASTERDIS` reader - APORT1X Master Disable
pub type Aport1xmasterdisR = crate::BitReader;
///Field `APORT1XMASTERDIS` writer - APORT1X Master Disable
pub type Aport1xmasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APORT1YMASTERDIS` reader - APORT1Y Master Disable
pub type Aport1ymasterdisR = crate::BitReader;
///Field `APORT1YMASTERDIS` writer - APORT1Y Master Disable
pub type Aport1ymasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APORT2XMASTERDIS` reader - APORT2X Master Disable
pub type Aport2xmasterdisR = crate::BitReader;
///Field `APORT2XMASTERDIS` writer - APORT2X Master Disable
pub type Aport2xmasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APORT2YMASTERDIS` reader - APORT2Y Master Disable
pub type Aport2ymasterdisR = crate::BitReader;
///Field `APORT2YMASTERDIS` writer - APORT2Y Master Disable
pub type Aport2ymasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APORT3XMASTERDIS` reader - APORT3X Master Disable
pub type Aport3xmasterdisR = crate::BitReader;
///Field `APORT3XMASTERDIS` writer - APORT3X Master Disable
pub type Aport3xmasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APORT3YMASTERDIS` reader - APORT3Y Master Disable
pub type Aport3ymasterdisR = crate::BitReader;
///Field `APORT3YMASTERDIS` writer - APORT3Y Master Disable
pub type Aport3ymasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APORT4XMASTERDIS` reader - APORT4X Master Disable
pub type Aport4xmasterdisR = crate::BitReader;
///Field `APORT4XMASTERDIS` writer - APORT4X Master Disable
pub type Aport4xmasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APORT4YMASTERDIS` reader - APORT4Y Master Disable
pub type Aport4ymasterdisR = crate::BitReader;
///Field `APORT4YMASTERDIS` writer - APORT4Y Master Disable
pub type Aport4ymasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - APORT1X Master Disable
    #[inline(always)]
    pub fn aport1xmasterdis(&self) -> Aport1xmasterdisR {
        Aport1xmasterdisR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - APORT1Y Master Disable
    #[inline(always)]
    pub fn aport1ymasterdis(&self) -> Aport1ymasterdisR {
        Aport1ymasterdisR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - APORT2X Master Disable
    #[inline(always)]
    pub fn aport2xmasterdis(&self) -> Aport2xmasterdisR {
        Aport2xmasterdisR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - APORT2Y Master Disable
    #[inline(always)]
    pub fn aport2ymasterdis(&self) -> Aport2ymasterdisR {
        Aport2ymasterdisR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - APORT3X Master Disable
    #[inline(always)]
    pub fn aport3xmasterdis(&self) -> Aport3xmasterdisR {
        Aport3xmasterdisR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - APORT3Y Master Disable
    #[inline(always)]
    pub fn aport3ymasterdis(&self) -> Aport3ymasterdisR {
        Aport3ymasterdisR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - APORT4X Master Disable
    #[inline(always)]
    pub fn aport4xmasterdis(&self) -> Aport4xmasterdisR {
        Aport4xmasterdisR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - APORT4Y Master Disable
    #[inline(always)]
    pub fn aport4ymasterdis(&self) -> Aport4ymasterdisR {
        Aport4ymasterdisR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APORTMASTERDIS")
            .field("aport1xmasterdis", &self.aport1xmasterdis())
            .field("aport1ymasterdis", &self.aport1ymasterdis())
            .field("aport2xmasterdis", &self.aport2xmasterdis())
            .field("aport2ymasterdis", &self.aport2ymasterdis())
            .field("aport3xmasterdis", &self.aport3xmasterdis())
            .field("aport3ymasterdis", &self.aport3ymasterdis())
            .field("aport4xmasterdis", &self.aport4xmasterdis())
            .field("aport4ymasterdis", &self.aport4ymasterdis())
            .finish()
    }
}
impl W {
    ///Bit 2 - APORT1X Master Disable
    #[inline(always)]
    #[must_use]
    pub fn aport1xmasterdis(&mut self) -> Aport1xmasterdisW<APORTMASTERDISrs> {
        Aport1xmasterdisW::new(self, 2)
    }
    ///Bit 3 - APORT1Y Master Disable
    #[inline(always)]
    #[must_use]
    pub fn aport1ymasterdis(&mut self) -> Aport1ymasterdisW<APORTMASTERDISrs> {
        Aport1ymasterdisW::new(self, 3)
    }
    ///Bit 4 - APORT2X Master Disable
    #[inline(always)]
    #[must_use]
    pub fn aport2xmasterdis(&mut self) -> Aport2xmasterdisW<APORTMASTERDISrs> {
        Aport2xmasterdisW::new(self, 4)
    }
    ///Bit 5 - APORT2Y Master Disable
    #[inline(always)]
    #[must_use]
    pub fn aport2ymasterdis(&mut self) -> Aport2ymasterdisW<APORTMASTERDISrs> {
        Aport2ymasterdisW::new(self, 5)
    }
    ///Bit 6 - APORT3X Master Disable
    #[inline(always)]
    #[must_use]
    pub fn aport3xmasterdis(&mut self) -> Aport3xmasterdisW<APORTMASTERDISrs> {
        Aport3xmasterdisW::new(self, 6)
    }
    ///Bit 7 - APORT3Y Master Disable
    #[inline(always)]
    #[must_use]
    pub fn aport3ymasterdis(&mut self) -> Aport3ymasterdisW<APORTMASTERDISrs> {
        Aport3ymasterdisW::new(self, 7)
    }
    ///Bit 8 - APORT4X Master Disable
    #[inline(always)]
    #[must_use]
    pub fn aport4xmasterdis(&mut self) -> Aport4xmasterdisW<APORTMASTERDISrs> {
        Aport4xmasterdisW::new(self, 8)
    }
    ///Bit 9 - APORT4Y Master Disable
    #[inline(always)]
    #[must_use]
    pub fn aport4ymasterdis(&mut self) -> Aport4ymasterdisW<APORTMASTERDISrs> {
        Aport4ymasterdisW::new(self, 9)
    }
}
///APORT Bus Master Disable Register
///
///You can [`read`](crate::Reg::read) this register and get [`aportmasterdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aportmasterdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct APORTMASTERDISrs;
impl crate::RegisterSpec for APORTMASTERDISrs {
    type Ux = u32;
}
///`read()` method returns [`aportmasterdis::R`](R) reader structure
impl crate::Readable for APORTMASTERDISrs {}
///`write(|w| ..)` method takes [`aportmasterdis::W`](W) writer structure
impl crate::Writable for APORTMASTERDISrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APORTMASTERDIS to value 0
impl crate::Resettable for APORTMASTERDISrs {
    const RESET_VALUE: u32 = 0;
}
