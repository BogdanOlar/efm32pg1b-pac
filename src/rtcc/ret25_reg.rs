#[doc = "Register `RET25_REG` reader"]
pub type R = crate::R<RET25_REGrs>;
#[doc = "Register `RET25_REG` writer"]
pub type W = crate::W<RET25_REGrs>;
#[doc = "Field `REG` reader - General Purpose Retention Register"]
pub type RegR = crate::FieldReader<u32>;
#[doc = "Field `REG` writer - General Purpose Retention Register"]
pub type RegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Retention Register"]
    #[inline(always)]
    pub fn reg(&self) -> RegR {
        RegR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RET25_REG")
            .field("reg", &self.reg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - General Purpose Retention Register"]
    #[inline(always)]
    #[must_use]
    pub fn reg(&mut self) -> RegW<RET25_REGrs> {
        RegW::new(self, 0)
    }
}
#[doc = "Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret25_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret25_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RET25_REGrs;
impl crate::RegisterSpec for RET25_REGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ret25_reg::R`](R) reader structure"]
impl crate::Readable for RET25_REGrs {}
#[doc = "`write(|w| ..)` method takes [`ret25_reg::W`](W) writer structure"]
impl crate::Writable for RET25_REGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RET25_REG to value 0"]
impl crate::Resettable for RET25_REGrs {
    const RESET_VALUE: u32 = 0;
}
