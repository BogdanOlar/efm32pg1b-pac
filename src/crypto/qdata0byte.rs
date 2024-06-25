#[doc = "Register `QDATA0BYTE` reader"]
pub type R = crate::R<QDATA0BYTErs>;
#[doc = "Register `QDATA0BYTE` writer"]
pub type W = crate::W<QDATA0BYTErs>;
#[doc = "Field `QDATA0BYTE` reader - Qdata 0 Byte Access"]
pub type Qdata0byteR = crate::FieldReader;
#[doc = "Field `QDATA0BYTE` writer - Qdata 0 Byte Access"]
pub type Qdata0byteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Qdata 0 Byte Access"]
    #[inline(always)]
    pub fn qdata0byte(&self) -> Qdata0byteR {
        Qdata0byteR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for crate::generic::Reg<QDATA0BYTErs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Qdata 0 Byte Access"]
    #[inline(always)]
    #[must_use]
    pub fn qdata0byte(&mut self) -> Qdata0byteW<QDATA0BYTErs> {
        Qdata0byteW::new(self, 0)
    }
}
#[doc = "QDATA0 Register Byte Access\n\nYou can [`read`](crate::Reg::read) this register and get [`qdata0byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdata0byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct QDATA0BYTErs;
impl crate::RegisterSpec for QDATA0BYTErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdata0byte::R`](R) reader structure"]
impl crate::Readable for QDATA0BYTErs {}
#[doc = "`write(|w| ..)` method takes [`qdata0byte::W`](W) writer structure"]
impl crate::Writable for QDATA0BYTErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QDATA0BYTE to value 0"]
impl crate::Resettable for QDATA0BYTErs {
    const RESET_VALUE: u32 = 0;
}
