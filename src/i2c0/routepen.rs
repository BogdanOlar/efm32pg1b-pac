#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPENrs>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPENrs>;
#[doc = "Field `SDAPEN` reader - SDA Pin Enable"]
pub type SdapenR = crate::BitReader;
#[doc = "Field `SDAPEN` writer - SDA Pin Enable"]
pub type SdapenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLPEN` reader - SCL Pin Enable"]
pub type SclpenR = crate::BitReader;
#[doc = "Field `SCLPEN` writer - SCL Pin Enable"]
pub type SclpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SDA Pin Enable"]
    #[inline(always)]
    pub fn sdapen(&self) -> SdapenR {
        SdapenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL Pin Enable"]
    #[inline(always)]
    pub fn sclpen(&self) -> SclpenR {
        SclpenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTEPEN")
            .field("sdapen", &self.sdapen())
            .field("sclpen", &self.sclpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SDA Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdapen(&mut self) -> SdapenW<ROUTEPENrs> {
        SdapenW::new(self, 0)
    }
    #[doc = "Bit 1 - SCL Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sclpen(&mut self) -> SclpenW<ROUTEPENrs> {
        SclpenW::new(self, 1)
    }
}
#[doc = "I/O Routing Pin Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTEPENrs;
impl crate::RegisterSpec for ROUTEPENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for ROUTEPENrs {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for ROUTEPENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPENrs {
    const RESET_VALUE: u32 = 0;
}
