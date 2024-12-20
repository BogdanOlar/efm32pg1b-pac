///Register `DDATA0` reader
pub type R = crate::R<DDATA0rs>;
///Register `DDATA0` writer
pub type W = crate::W<DDATA0rs>;
///Field `DDATA0` reader - Double Data 0 Access
pub type Ddata0R = crate::FieldReader<u32>;
///Field `DDATA0` writer - Double Data 0 Access
pub type Ddata0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Double Data 0 Access
    #[inline(always)]
    pub fn ddata0(&self) -> Ddata0R {
        Ddata0R::new(self.bits)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DDATA0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Double Data 0 Access
    #[inline(always)]
    #[must_use]
    pub fn ddata0(&mut self) -> Ddata0W<DDATA0rs> {
        Ddata0W::new(self, 0)
    }
}
///DDATA0 Register Access
///
///You can [`read`](crate::Reg::read) this register and get [`ddata0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct DDATA0rs;
impl crate::RegisterSpec for DDATA0rs {
    type Ux = u32;
}
///`read()` method returns [`ddata0::R`](R) reader structure
impl crate::Readable for DDATA0rs {}
///`write(|w| ..)` method takes [`ddata0::W`](W) writer structure
impl crate::Writable for DDATA0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDATA0 to value 0
impl crate::Resettable for DDATA0rs {
    const RESET_VALUE: u32 = 0;
}
