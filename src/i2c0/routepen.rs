#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPEN_SPEC>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPEN_SPEC>;
#[doc = "Field `SDAPEN` reader - SDA Pin Enable"]
pub type SDAPEN_R = crate::BitReader;
#[doc = "Field `SDAPEN` writer - SDA Pin Enable"]
pub type SDAPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCLPEN` reader - SCL Pin Enable"]
pub type SCLPEN_R = crate::BitReader;
#[doc = "Field `SCLPEN` writer - SCL Pin Enable"]
pub type SCLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SDA Pin Enable"]
    #[inline(always)]
    pub fn sdapen(&self) -> SDAPEN_R {
        SDAPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL Pin Enable"]
    #[inline(always)]
    pub fn sclpen(&self) -> SCLPEN_R {
        SCLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTEPEN")
            .field("sdapen", &format_args!("{}", self.sdapen().bit()))
            .field("sclpen", &format_args!("{}", self.sclpen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ROUTEPEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - SDA Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdapen(&mut self) -> SDAPEN_W<ROUTEPEN_SPEC, 0> {
        SDAPEN_W::new(self)
    }
    #[doc = "Bit 1 - SCL Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sclpen(&mut self) -> SCLPEN_W<ROUTEPEN_SPEC, 1> {
        SCLPEN_W::new(self)
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
#[doc = "I/O Routing Pin Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTEPEN_SPEC;
impl crate::RegisterSpec for ROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for ROUTEPEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for ROUTEPEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
