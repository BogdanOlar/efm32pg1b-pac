#[doc = "Register `DATA0BYTE` reader"]
pub type R = crate::R<DATA0BYTErs>;
#[doc = "Register `DATA0BYTE` writer"]
pub type W = crate::W<DATA0BYTErs>;
#[doc = "Field `DATA0BYTE` reader - Data 0 Byte Access"]
pub type Data0byteR = crate::FieldReader;
#[doc = "Field `DATA0BYTE` writer - Data 0 Byte Access"]
pub type Data0byteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte Access"]
    #[inline(always)]
    pub fn data0byte(&self) -> Data0byteR {
        Data0byteR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DATA0BYTErs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0byte(&mut self) -> Data0byteW<DATA0BYTErs> {
        Data0byteW::new(self, 0)
    }
}
#[doc = "DATA0 Register Byte Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct DATA0BYTErs;
impl crate::RegisterSpec for DATA0BYTErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0byte::R`](R) reader structure"]
impl crate::Readable for DATA0BYTErs {}
#[doc = "`write(|w| ..)` method takes [`data0byte::W`](W) writer structure"]
impl crate::Writable for DATA0BYTErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA0BYTE to value 0"]
impl crate::Resettable for DATA0BYTErs {
    const RESET_VALUE: u32 = 0;
}
