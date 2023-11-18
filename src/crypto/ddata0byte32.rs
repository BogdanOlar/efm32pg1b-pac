#[doc = "Register `DDATA0BYTE32` reader"]
pub type R = crate::R<DDATA0BYTE32_SPEC>;
#[doc = "Register `DDATA0BYTE32` writer"]
pub type W = crate::W<DDATA0BYTE32_SPEC>;
#[doc = "Field `DDATA0BYTE32` reader - Ddata 0 Byte 32 Access"]
pub type DDATA0BYTE32_R = crate::FieldReader;
#[doc = "Field `DDATA0BYTE32` writer - Ddata 0 Byte 32 Access"]
pub type DDATA0BYTE32_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Ddata 0 Byte 32 Access"]
    #[inline(always)]
    pub fn ddata0byte32(&self) -> DDATA0BYTE32_R {
        DDATA0BYTE32_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDATA0BYTE32")
            .field(
                "ddata0byte32",
                &format_args!("{}", self.ddata0byte32().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DDATA0BYTE32_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Ddata 0 Byte 32 Access"]
    #[inline(always)]
    #[must_use]
    pub fn ddata0byte32(&mut self) -> DDATA0BYTE32_W<DDATA0BYTE32_SPEC, 0> {
        DDATA0BYTE32_W::new(self)
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
#[doc = "DDATA0 Register Byte 32 Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddata0byte32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddata0byte32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDATA0BYTE32_SPEC;
impl crate::RegisterSpec for DDATA0BYTE32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata0byte32::R`](R) reader structure"]
impl crate::Readable for DDATA0BYTE32_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddata0byte32::W`](W) writer structure"]
impl crate::Writable for DDATA0BYTE32_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDATA0BYTE32 to value 0"]
impl crate::Resettable for DDATA0BYTE32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
