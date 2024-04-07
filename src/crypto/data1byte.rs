#[doc = "Register `DATA1BYTE` reader"]
pub type R = crate::R<DATA1BYTErs>;
#[doc = "Register `DATA1BYTE` writer"]
pub type W = crate::W<DATA1BYTErs>;
#[doc = "Field `DATA1BYTE` reader - Data 1 Byte Access"]
pub type Data1byteR = crate::FieldReader;
#[doc = "Field `DATA1BYTE` writer - Data 1 Byte Access"]
pub type Data1byteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 1 Byte Access"]
    #[inline(always)]
    pub fn data1byte(&self) -> Data1byteR {
        Data1byteR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 1 Byte Access"]
    #[inline(always)]
    #[must_use]
    pub fn data1byte(&mut self) -> Data1byteW<DATA1BYTErs> {
        Data1byteW::new(self, 0)
    }
}
#[doc = "DATA1 Register Byte Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data1byte::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data1byte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA1BYTErs;
impl crate::RegisterSpec for DATA1BYTErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data1byte::R`](R) reader structure"]
impl crate::Readable for DATA1BYTErs {}
#[doc = "`write(|w| ..)` method takes [`data1byte::W`](W) writer structure"]
impl crate::Writable for DATA1BYTErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA1BYTE to value 0"]
impl crate::Resettable for DATA1BYTErs {
    const RESET_VALUE: u32 = 0;
}
