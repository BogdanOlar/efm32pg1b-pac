#[doc = "Register `DDATA0BIG` reader"]
pub type R = crate::R<DDATA0BIGrs>;
#[doc = "Register `DDATA0BIG` writer"]
pub type W = crate::W<DDATA0BIGrs>;
#[doc = "Field `DDATA0BIG` reader - Double Data 0 Big Endian Access"]
pub type Ddata0bigR = crate::FieldReader<u32>;
#[doc = "Field `DDATA0BIG` writer - Double Data 0 Big Endian Access"]
pub type Ddata0bigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Big Endian Access"]
    #[inline(always)]
    pub fn ddata0big(&self) -> Ddata0bigR {
        Ddata0bigR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Big Endian Access"]
    #[inline(always)]
    #[must_use]
    pub fn ddata0big(&mut self) -> Ddata0bigW<DDATA0BIGrs> {
        Ddata0bigW::new(self, 0)
    }
}
#[doc = "DDATA0 Register Big Endian Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddata0big::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddata0big::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDATA0BIGrs;
impl crate::RegisterSpec for DDATA0BIGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata0big::R`](R) reader structure"]
impl crate::Readable for DDATA0BIGrs {}
#[doc = "`write(|w| ..)` method takes [`ddata0big::W`](W) writer structure"]
impl crate::Writable for DDATA0BIGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDATA0BIG to value 0"]
impl crate::Resettable for DDATA0BIGrs {
    const RESET_VALUE: u32 = 0;
}
