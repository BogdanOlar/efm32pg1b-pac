#[doc = "Register `DATA0BYTE12` reader"]
pub type R = crate::R<DATA0BYTE12rs>;
#[doc = "Register `DATA0BYTE12` writer"]
pub type W = crate::W<DATA0BYTE12rs>;
#[doc = "Field `DATA0BYTE12` reader - Data 0 Byte 12 Access"]
pub type Data0byte12R = crate::FieldReader;
#[doc = "Field `DATA0BYTE12` writer - Data 0 Byte 12 Access"]
pub type Data0byte12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte 12 Access"]
    #[inline(always)]
    pub fn data0byte12(&self) -> Data0byte12R {
        Data0byte12R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA0BYTE12")
            .field("data0byte12", &self.data0byte12())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte 12 Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0byte12(&mut self) -> Data0byte12W<DATA0BYTE12rs> {
        Data0byte12W::new(self, 0)
    }
}
#[doc = "DATA0 Register Byte 12 Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0byte12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0byte12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA0BYTE12rs;
impl crate::RegisterSpec for DATA0BYTE12rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0byte12::R`](R) reader structure"]
impl crate::Readable for DATA0BYTE12rs {}
#[doc = "`write(|w| ..)` method takes [`data0byte12::W`](W) writer structure"]
impl crate::Writable for DATA0BYTE12rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA0BYTE12 to value 0"]
impl crate::Resettable for DATA0BYTE12rs {
    const RESET_VALUE: u32 = 0;
}
