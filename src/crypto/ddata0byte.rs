///Register `DDATA0BYTE` reader
pub type R = crate::R<DDATA0BYTErs>;
///Register `DDATA0BYTE` writer
pub type W = crate::W<DDATA0BYTErs>;
///Field `DDATA0BYTE` reader - Ddata 0 Byte Access
pub type Ddata0byteR = crate::FieldReader;
///Field `DDATA0BYTE` writer - Ddata 0 Byte Access
pub type Ddata0byteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Ddata 0 Byte Access
    #[inline(always)]
    pub fn ddata0byte(&self) -> Ddata0byteR {
        Ddata0byteR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DDATA0BYTErs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Ddata 0 Byte Access
    #[inline(always)]
    #[must_use]
    pub fn ddata0byte(&mut self) -> Ddata0byteW<DDATA0BYTErs> {
        Ddata0byteW::new(self, 0)
    }
}
///DDATA0 Register Byte Access
///
///You can [`read`](crate::Reg::read) this register and get [`ddata0byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata0byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct DDATA0BYTErs;
impl crate::RegisterSpec for DDATA0BYTErs {
    type Ux = u32;
}
///`read()` method returns [`ddata0byte::R`](R) reader structure
impl crate::Readable for DDATA0BYTErs {}
///`write(|w| ..)` method takes [`ddata0byte::W`](W) writer structure
impl crate::Writable for DDATA0BYTErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDATA0BYTE to value 0
impl crate::Resettable for DDATA0BYTErs {
    const RESET_VALUE: u32 = 0;
}
