#[doc = "Register `DDATA1BYTE` reader"]
pub type R = crate::R<DDATA1BYTErs>;
#[doc = "Register `DDATA1BYTE` writer"]
pub type W = crate::W<DDATA1BYTErs>;
#[doc = "Field `DDATA1BYTE` reader - Ddata 1 Byte Access"]
pub type Ddata1byteR = crate::FieldReader;
#[doc = "Field `DDATA1BYTE` writer - Ddata 1 Byte Access"]
pub type Ddata1byteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Ddata 1 Byte Access"]
    #[inline(always)]
    pub fn ddata1byte(&self) -> Ddata1byteR {
        Ddata1byteR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DDATA1BYTErs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Ddata 1 Byte Access"]
    #[inline(always)]
    #[must_use]
    pub fn ddata1byte(&mut self) -> Ddata1byteW<DDATA1BYTErs> {
        Ddata1byteW::new(self, 0)
    }
}
#[doc = "DDATA1 Register Byte Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata1byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata1byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct DDATA1BYTErs;
impl crate::RegisterSpec for DDATA1BYTErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata1byte::R`](R) reader structure"]
impl crate::Readable for DDATA1BYTErs {}
#[doc = "`write(|w| ..)` method takes [`ddata1byte::W`](W) writer structure"]
impl crate::Writable for DDATA1BYTErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDATA1BYTE to value 0"]
impl crate::Resettable for DDATA1BYTErs {
    const RESET_VALUE: u32 = 0;
}
