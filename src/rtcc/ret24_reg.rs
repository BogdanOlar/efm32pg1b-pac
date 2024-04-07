#[doc = "Register `RET24_REG` reader"]
pub type R = crate::R<RET24_REGrs>;
#[doc = "Register `RET24_REG` writer"]
pub type W = crate::W<RET24_REGrs>;
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
impl W {
    #[doc = "Bits 0:31 - General Purpose Retention Register"]
    #[inline(always)]
    #[must_use]
    pub fn reg(&mut self) -> RegW<RET24_REGrs> {
        RegW::new(self, 0)
    }
}
#[doc = "Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret24_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret24_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RET24_REGrs;
impl crate::RegisterSpec for RET24_REGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ret24_reg::R`](R) reader structure"]
impl crate::Readable for RET24_REGrs {}
#[doc = "`write(|w| ..)` method takes [`ret24_reg::W`](W) writer structure"]
impl crate::Writable for RET24_REGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RET24_REG to value 0"]
impl crate::Resettable for RET24_REGrs {
    const RESET_VALUE: u32 = 0;
}
