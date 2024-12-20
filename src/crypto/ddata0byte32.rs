///Register `DDATA0BYTE32` reader
pub type R = crate::R<DDATA0BYTE32rs>;
///Register `DDATA0BYTE32` writer
pub type W = crate::W<DDATA0BYTE32rs>;
///Field `DDATA0BYTE32` reader - Ddata 0 Byte 32 Access
pub type Ddata0byte32R = crate::FieldReader;
///Field `DDATA0BYTE32` writer - Ddata 0 Byte 32 Access
pub type Ddata0byte32W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Ddata 0 Byte 32 Access
    #[inline(always)]
    pub fn ddata0byte32(&self) -> Ddata0byte32R {
        Ddata0byte32R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDATA0BYTE32")
            .field("ddata0byte32", &self.ddata0byte32())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Ddata 0 Byte 32 Access
    #[inline(always)]
    #[must_use]
    pub fn ddata0byte32(&mut self) -> Ddata0byte32W<DDATA0BYTE32rs> {
        Ddata0byte32W::new(self, 0)
    }
}
///DDATA0 Register Byte 32 Access
///
///You can [`read`](crate::Reg::read) this register and get [`ddata0byte32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata0byte32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DDATA0BYTE32rs;
impl crate::RegisterSpec for DDATA0BYTE32rs {
    type Ux = u32;
}
///`read()` method returns [`ddata0byte32::R`](R) reader structure
impl crate::Readable for DDATA0BYTE32rs {}
///`write(|w| ..)` method takes [`ddata0byte32::W`](W) writer structure
impl crate::Writable for DDATA0BYTE32rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDATA0BYTE32 to value 0
impl crate::Resettable for DDATA0BYTE32rs {
    const RESET_VALUE: u32 = 0;
}
