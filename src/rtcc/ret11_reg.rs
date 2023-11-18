#[doc = "Register `RET11_REG` reader"]
pub type R = crate::R<RET11_REG_SPEC>;
#[doc = "Register `RET11_REG` writer"]
pub type W = crate::W<RET11_REG_SPEC>;
#[doc = "Field `REG` reader - General Purpose Retention Register"]
pub type REG_R = crate::FieldReader<u32>;
#[doc = "Field `REG` writer - General Purpose Retention Register"]
pub type REG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Retention Register"]
    #[inline(always)]
    pub fn reg(&self) -> REG_R {
        REG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RET11_REG")
            .field("reg", &format_args!("{}", self.reg().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RET11_REG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - General Purpose Retention Register"]
    #[inline(always)]
    #[must_use]
    pub fn reg(&mut self) -> REG_W<RET11_REG_SPEC, 0> {
        REG_W::new(self)
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
#[doc = "Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret11_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret11_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RET11_REG_SPEC;
impl crate::RegisterSpec for RET11_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ret11_reg::R`](R) reader structure"]
impl crate::Readable for RET11_REG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ret11_reg::W`](W) writer structure"]
impl crate::Writable for RET11_REG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RET11_REG to value 0"]
impl crate::Resettable for RET11_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
