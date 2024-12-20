///Register `DDATA3` reader
pub type R = crate::R<DDATA3rs>;
///Register `DDATA3` writer
pub type W = crate::W<DDATA3rs>;
///Field `DDATA3` reader - Double Data 0 Access
pub type Ddata3R = crate::FieldReader<u32>;
///Field `DDATA3` writer - Double Data 0 Access
pub type Ddata3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Double Data 0 Access
    #[inline(always)]
    pub fn ddata3(&self) -> Ddata3R {
        Ddata3R::new(self.bits)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DDATA3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Double Data 0 Access
    #[inline(always)]
    #[must_use]
    pub fn ddata3(&mut self) -> Ddata3W<DDATA3rs> {
        Ddata3W::new(self, 0)
    }
}
///DDATA3 Register Access
///
///You can [`read`](crate::Reg::read) this register and get [`ddata3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct DDATA3rs;
impl crate::RegisterSpec for DDATA3rs {
    type Ux = u32;
}
///`read()` method returns [`ddata3::R`](R) reader structure
impl crate::Readable for DDATA3rs {}
///`write(|w| ..)` method takes [`ddata3::W`](W) writer structure
impl crate::Writable for DDATA3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDATA3 to value 0
impl crate::Resettable for DDATA3rs {
    const RESET_VALUE: u32 = 0;
}
