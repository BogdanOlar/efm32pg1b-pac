#[doc = "Register `DATA0BYTE12` reader"]
pub type R = crate::R<DATA0BYTE12_SPEC>;
#[doc = "Register `DATA0BYTE12` writer"]
pub type W = crate::W<DATA0BYTE12_SPEC>;
#[doc = "Field `DATA0BYTE12` reader - Data 0 Byte 12 Access"]
pub type DATA0BYTE12_R = crate::FieldReader;
#[doc = "Field `DATA0BYTE12` writer - Data 0 Byte 12 Access"]
pub type DATA0BYTE12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte 12 Access"]
    #[inline(always)]
    pub fn data0byte12(&self) -> DATA0BYTE12_R {
        DATA0BYTE12_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA0BYTE12")
            .field(
                "data0byte12",
                &format_args!("{}", self.data0byte12().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DATA0BYTE12_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte 12 Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0byte12(&mut self) -> DATA0BYTE12_W<DATA0BYTE12_SPEC, 0> {
        DATA0BYTE12_W::new(self)
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
#[doc = "DATA0 Register Byte 12 Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0byte12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0byte12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA0BYTE12_SPEC;
impl crate::RegisterSpec for DATA0BYTE12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0byte12::R`](R) reader structure"]
impl crate::Readable for DATA0BYTE12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data0byte12::W`](W) writer structure"]
impl crate::Writable for DATA0BYTE12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA0BYTE12 to value 0"]
impl crate::Resettable for DATA0BYTE12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
