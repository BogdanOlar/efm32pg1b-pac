#[doc = "Register `RET18_REG` reader"]
pub type R = crate::R<RET18_REGrs>;
#[doc = "Register `RET18_REG` writer"]
pub type W = crate::W<RET18_REGrs>;
#[doc = "Field `REG` reader - General Purpose Retention Register"]
pub type REG_R = crate::FieldReader<u32>;
#[doc = "Field `REG` writer - General Purpose Retention Register"]
pub type REG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Retention Register"]
    #[inline(always)]
    pub fn reg(&self) -> REG_R {
        REG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - General Purpose Retention Register"]
    #[inline(always)]
    #[must_use]
    pub fn reg(&mut self) -> REG_W<RET18_REGrs> {
        REG_W::new(self, 0)
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
#[doc = "Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret18_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret18_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RET18_REGrs;
impl crate::RegisterSpec for RET18_REGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ret18_reg::R`](R) reader structure"]
impl crate::Readable for RET18_REGrs {}
#[doc = "`write(|w| ..)` method takes [`ret18_reg::W`](W) writer structure"]
impl crate::Writable for RET18_REGrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RET18_REG to value 0"]
impl crate::Resettable for RET18_REGrs {
    const RESET_VALUE: u32 = 0;
}
