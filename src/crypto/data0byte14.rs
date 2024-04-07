#[doc = "Register `DATA0BYTE14` reader"]
pub type R = crate::R<DATA0BYTE14rs>;
#[doc = "Register `DATA0BYTE14` writer"]
pub type W = crate::W<DATA0BYTE14rs>;
#[doc = "Field `DATA0BYTE14` reader - Data 0 Byte 14 Access"]
pub type Data0byte14R = crate::FieldReader;
#[doc = "Field `DATA0BYTE14` writer - Data 0 Byte 14 Access"]
pub type Data0byte14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte 14 Access"]
    #[inline(always)]
    pub fn data0byte14(&self) -> Data0byte14R {
        Data0byte14R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte 14 Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0byte14(&mut self) -> Data0byte14W<DATA0BYTE14rs> {
        Data0byte14W::new(self, 0)
    }
}
#[doc = "DATA0 Register Byte 14 Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0byte14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0byte14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA0BYTE14rs;
impl crate::RegisterSpec for DATA0BYTE14rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0byte14::R`](R) reader structure"]
impl crate::Readable for DATA0BYTE14rs {}
#[doc = "`write(|w| ..)` method takes [`data0byte14::W`](W) writer structure"]
impl crate::Writable for DATA0BYTE14rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA0BYTE14 to value 0"]
impl crate::Resettable for DATA0BYTE14rs {
    const RESET_VALUE: u32 = 0;
}
