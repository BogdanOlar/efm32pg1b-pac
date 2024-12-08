///Register `ROUTEPEN` reader
pub type R = crate::R<ROUTEPENrs>;
///Register `ROUTEPEN` writer
pub type W = crate::W<ROUTEPENrs>;
///Field `CC0PEN` reader - CC Channel 0 Pin Enable
pub type Cc0penR = crate::BitReader;
///Field `CC0PEN` writer - CC Channel 0 Pin Enable
pub type Cc0penW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1PEN` reader - CC Channel 1 Pin Enable
pub type Cc1penR = crate::BitReader;
///Field `CC1PEN` writer - CC Channel 1 Pin Enable
pub type Cc1penW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2PEN` reader - CC Channel 2 Pin Enable
pub type Cc2penR = crate::BitReader;
///Field `CC2PEN` writer - CC Channel 2 Pin Enable
pub type Cc2penW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3PEN` reader - CC Channel 3 Pin Enable
pub type Cc3penR = crate::BitReader;
///Field `CC3PEN` writer - CC Channel 3 Pin Enable
pub type Cc3penW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDTI0PEN` reader - CC Channel 0 Complementary Dead-Time Insertion Pin Enable
pub type Cdti0penR = crate::BitReader;
///Field `CDTI0PEN` writer - CC Channel 0 Complementary Dead-Time Insertion Pin Enable
pub type Cdti0penW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDTI1PEN` reader - CC Channel 1 Complementary Dead-Time Insertion Pin Enable
pub type Cdti1penR = crate::BitReader;
///Field `CDTI1PEN` writer - CC Channel 1 Complementary Dead-Time Insertion Pin Enable
pub type Cdti1penW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDTI2PEN` reader - CC Channel 2 Complementary Dead-Time Insertion Pin Enable
pub type Cdti2penR = crate::BitReader;
///Field `CDTI2PEN` writer - CC Channel 2 Complementary Dead-Time Insertion Pin Enable
pub type Cdti2penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CC Channel 0 Pin Enable
    #[inline(always)]
    pub fn cc0pen(&self) -> Cc0penR {
        Cc0penR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CC Channel 1 Pin Enable
    #[inline(always)]
    pub fn cc1pen(&self) -> Cc1penR {
        Cc1penR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CC Channel 2 Pin Enable
    #[inline(always)]
    pub fn cc2pen(&self) -> Cc2penR {
        Cc2penR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CC Channel 3 Pin Enable
    #[inline(always)]
    pub fn cc3pen(&self) -> Cc3penR {
        Cc3penR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable
    #[inline(always)]
    pub fn cdti0pen(&self) -> Cdti0penR {
        Cdti0penR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable
    #[inline(always)]
    pub fn cdti1pen(&self) -> Cdti1penR {
        Cdti1penR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable
    #[inline(always)]
    pub fn cdti2pen(&self) -> Cdti2penR {
        Cdti2penR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTEPEN")
            .field("cc0pen", &self.cc0pen())
            .field("cc1pen", &self.cc1pen())
            .field("cc2pen", &self.cc2pen())
            .field("cc3pen", &self.cc3pen())
            .field("cdti0pen", &self.cdti0pen())
            .field("cdti1pen", &self.cdti1pen())
            .field("cdti2pen", &self.cdti2pen())
            .finish()
    }
}
impl W {
    ///Bit 0 - CC Channel 0 Pin Enable
    #[inline(always)]
    #[must_use]
    pub fn cc0pen(&mut self) -> Cc0penW<ROUTEPENrs> {
        Cc0penW::new(self, 0)
    }
    ///Bit 1 - CC Channel 1 Pin Enable
    #[inline(always)]
    #[must_use]
    pub fn cc1pen(&mut self) -> Cc1penW<ROUTEPENrs> {
        Cc1penW::new(self, 1)
    }
    ///Bit 2 - CC Channel 2 Pin Enable
    #[inline(always)]
    #[must_use]
    pub fn cc2pen(&mut self) -> Cc2penW<ROUTEPENrs> {
        Cc2penW::new(self, 2)
    }
    ///Bit 3 - CC Channel 3 Pin Enable
    #[inline(always)]
    #[must_use]
    pub fn cc3pen(&mut self) -> Cc3penW<ROUTEPENrs> {
        Cc3penW::new(self, 3)
    }
    ///Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable
    #[inline(always)]
    #[must_use]
    pub fn cdti0pen(&mut self) -> Cdti0penW<ROUTEPENrs> {
        Cdti0penW::new(self, 8)
    }
    ///Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable
    #[inline(always)]
    #[must_use]
    pub fn cdti1pen(&mut self) -> Cdti1penW<ROUTEPENrs> {
        Cdti1penW::new(self, 9)
    }
    ///Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable
    #[inline(always)]
    #[must_use]
    pub fn cdti2pen(&mut self) -> Cdti2penW<ROUTEPENrs> {
        Cdti2penW::new(self, 10)
    }
}
///I/O Routing Pin Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`routepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ROUTEPENrs;
impl crate::RegisterSpec for ROUTEPENrs {
    type Ux = u32;
}
///`read()` method returns [`routepen::R`](R) reader structure
impl crate::Readable for ROUTEPENrs {}
///`write(|w| ..)` method takes [`routepen::W`](W) writer structure
impl crate::Writable for ROUTEPENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ROUTEPEN to value 0
impl crate::Resettable for ROUTEPENrs {
    const RESET_VALUE: u32 = 0;
}
