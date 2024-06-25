#[doc = "Register `DATA0XORBYTE` reader"]
pub type R = crate::R<DATA0XORBYTErs>;
#[doc = "Register `DATA0XORBYTE` writer"]
pub type W = crate::W<DATA0XORBYTErs>;
#[doc = "Field `DATA0XORBYTE` reader - Data 0 XOR Byte Access"]
pub type Data0xorbyteR = crate::FieldReader;
#[doc = "Field `DATA0XORBYTE` writer - Data 0 XOR Byte Access"]
pub type Data0xorbyteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 0 XOR Byte Access"]
    #[inline(always)]
    pub fn data0xorbyte(&self) -> Data0xorbyteR {
        Data0xorbyteR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DATA0XORBYTErs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 XOR Byte Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0xorbyte(&mut self) -> Data0xorbyteW<DATA0XORBYTErs> {
        Data0xorbyteW::new(self, 0)
    }
}
#[doc = "DATA0 Register Byte XOR Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0xorbyte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0xorbyte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct DATA0XORBYTErs;
impl crate::RegisterSpec for DATA0XORBYTErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0xorbyte::R`](R) reader structure"]
impl crate::Readable for DATA0XORBYTErs {}
#[doc = "`write(|w| ..)` method takes [`data0xorbyte::W`](W) writer structure"]
impl crate::Writable for DATA0XORBYTErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA0XORBYTE to value 0"]
impl crate::Resettable for DATA0XORBYTErs {
    const RESET_VALUE: u32 = 0;
}
