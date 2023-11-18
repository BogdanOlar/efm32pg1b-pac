#[doc = "Register `CH5_SRC` reader"]
pub type R = crate::R<CH5_SRC_SPEC>;
#[doc = "Register `CH5_SRC` writer"]
pub type W = crate::W<CH5_SRC_SPEC>;
#[doc = "Field `SRCADDR` reader - Source Data Address"]
pub type SRCADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SRCADDR` writer - Source Data Address"]
pub type SRCADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn srcaddr(&self) -> SRCADDR_R {
        SRCADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH5_SRC")
            .field("srcaddr", &format_args!("{}", self.srcaddr().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CH5_SRC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    #[must_use]
    pub fn srcaddr(&mut self) -> SRCADDR_W<CH5_SRC_SPEC, 0> {
        SRCADDR_W::new(self)
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
#[doc = "Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_src::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_src::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH5_SRC_SPEC;
impl crate::RegisterSpec for CH5_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5_src::R`](R) reader structure"]
impl crate::Readable for CH5_SRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch5_src::W`](W) writer structure"]
impl crate::Writable for CH5_SRC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH5_SRC to value 0"]
impl crate::Resettable for CH5_SRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
