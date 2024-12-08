///Register `DDATA0BIG` reader
pub type R = crate::R<DDATA0BIGrs>;
///Register `DDATA0BIG` writer
pub type W = crate::W<DDATA0BIGrs>;
///Field `DDATA0BIG` reader - Double Data 0 Big Endian Access
pub type Ddata0bigR = crate::FieldReader<u32>;
///Field `DDATA0BIG` writer - Double Data 0 Big Endian Access
pub type Ddata0bigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Double Data 0 Big Endian Access
    #[inline(always)]
    pub fn ddata0big(&self) -> Ddata0bigR {
        Ddata0bigR::new(self.bits)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DDATA0BIGrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Double Data 0 Big Endian Access
    #[inline(always)]
    #[must_use]
    pub fn ddata0big(&mut self) -> Ddata0bigW<DDATA0BIGrs> {
        Ddata0bigW::new(self, 0)
    }
}
///DDATA0 Register Big Endian Access
///
///You can [`read`](crate::Reg::read) this register and get [`ddata0big::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata0big::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct DDATA0BIGrs;
impl crate::RegisterSpec for DDATA0BIGrs {
    type Ux = u32;
}
///`read()` method returns [`ddata0big::R`](R) reader structure
impl crate::Readable for DDATA0BIGrs {}
///`write(|w| ..)` method takes [`ddata0big::W`](W) writer structure
impl crate::Writable for DDATA0BIGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDATA0BIG to value 0
impl crate::Resettable for DDATA0BIGrs {
    const RESET_VALUE: u32 = 0;
}
