#[doc = "Register `DDATA0BYTE32` reader"]
pub type R = crate::R<DDATA0BYTE32rs>;
#[doc = "Register `DDATA0BYTE32` writer"]
pub type W = crate::W<DDATA0BYTE32rs>;
#[doc = "Field `DDATA0BYTE32` reader - Ddata 0 Byte 32 Access"]
pub type DDATA0BYTE32_R = crate::FieldReader;
#[doc = "Field `DDATA0BYTE32` writer - Ddata 0 Byte 32 Access"]
pub type DDATA0BYTE32_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Ddata 0 Byte 32 Access"]
    #[inline(always)]
    pub fn ddata0byte32(&self) -> DDATA0BYTE32_R {
        DDATA0BYTE32_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Ddata 0 Byte 32 Access"]
    #[inline(always)]
    #[must_use]
    pub fn ddata0byte32(&mut self) -> DDATA0BYTE32_W<DDATA0BYTE32rs> {
        DDATA0BYTE32_W::new(self, 0)
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
pub struct DDATA0BYTE32rs;
impl crate::RegisterSpec for DDATA0BYTE32rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata0byte32::R`](R) reader structure"]
impl crate::Readable for DDATA0BYTE32rs {}
#[doc = "`write(|w| ..)` method takes [`ddata0byte32::W`](W) writer structure"]
impl crate::Writable for DDATA0BYTE32rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDATA0BYTE32 to value 0"]
impl crate::Resettable for DDATA0BYTE32rs {
    const RESET_VALUE: u32 = 0;
}
