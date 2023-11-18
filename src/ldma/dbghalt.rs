#[doc = "Register `DBGHALT` reader"]
pub type R = crate::R<DBGHALT_SPEC>;
#[doc = "Register `DBGHALT` writer"]
pub type W = crate::W<DBGHALT_SPEC>;
#[doc = "Field `DBGHALT` reader - DMA Debug Halt"]
pub type DBGHALT_R = crate::FieldReader;
#[doc = "Field `DBGHALT` writer - DMA Debug Halt"]
pub type DBGHALT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DMA Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGHALT")
            .field("dbghalt", &format_args!("{}", self.dbghalt().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DBGHALT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Debug Halt"]
    #[inline(always)]
    #[must_use]
    pub fn dbghalt(&mut self) -> DBGHALT_W<DBGHALT_SPEC, 0> {
        DBGHALT_W::new(self)
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
#[doc = "DMA Channel Debug Halt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbghalt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbghalt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGHALT_SPEC;
impl crate::RegisterSpec for DBGHALT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbghalt::R`](R) reader structure"]
impl crate::Readable for DBGHALT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbghalt::W`](W) writer structure"]
impl crate::Writable for DBGHALT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGHALT to value 0"]
impl crate::Resettable for DBGHALT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
