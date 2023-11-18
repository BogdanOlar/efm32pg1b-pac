#[doc = "Register `DATA0BYTE` reader"]
pub type R = crate::R<DATA0BYTE_SPEC>;
#[doc = "Register `DATA0BYTE` writer"]
pub type W = crate::W<DATA0BYTE_SPEC>;
#[doc = "Field `DATA0BYTE` reader - Data 0 Byte Access"]
pub type DATA0BYTE_R = crate::FieldReader;
#[doc = "Field `DATA0BYTE` writer - Data 0 Byte Access"]
pub type DATA0BYTE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte Access"]
    #[inline(always)]
    pub fn data0byte(&self) -> DATA0BYTE_R {
        DATA0BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DATA0BYTE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0byte(&mut self) -> DATA0BYTE_W<DATA0BYTE_SPEC, 0> {
        DATA0BYTE_W::new(self)
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
#[doc = "DATA0 Register Byte Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0byte::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0byte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA0BYTE_SPEC;
impl crate::RegisterSpec for DATA0BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0byte::R`](R) reader structure"]
impl crate::Readable for DATA0BYTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data0byte::W`](W) writer structure"]
impl crate::Writable for DATA0BYTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA0BYTE to value 0"]
impl crate::Resettable for DATA0BYTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
