///Register `DATA0BYTE15` reader
pub type R = crate::R<DATA0BYTE15rs>;
///Register `DATA0BYTE15` writer
pub type W = crate::W<DATA0BYTE15rs>;
///Field `DATA0BYTE15` reader - Data 0 Byte 15 Access
pub type Data0byte15R = crate::FieldReader;
///Field `DATA0BYTE15` writer - Data 0 Byte 15 Access
pub type Data0byte15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 0 Byte 15 Access
    #[inline(always)]
    pub fn data0byte15(&self) -> Data0byte15R {
        Data0byte15R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA0BYTE15")
            .field("data0byte15", &self.data0byte15())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 0 Byte 15 Access
    #[inline(always)]
    #[must_use]
    pub fn data0byte15(&mut self) -> Data0byte15W<DATA0BYTE15rs> {
        Data0byte15W::new(self, 0)
    }
}
///DATA0 Register Byte 15 Access
///
///You can [`read`](crate::Reg::read) this register and get [`data0byte15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0byte15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DATA0BYTE15rs;
impl crate::RegisterSpec for DATA0BYTE15rs {
    type Ux = u32;
}
///`read()` method returns [`data0byte15::R`](R) reader structure
impl crate::Readable for DATA0BYTE15rs {}
///`write(|w| ..)` method takes [`data0byte15::W`](W) writer structure
impl crate::Writable for DATA0BYTE15rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA0BYTE15 to value 0
impl crate::Resettable for DATA0BYTE15rs {
    const RESET_VALUE: u32 = 0;
}
