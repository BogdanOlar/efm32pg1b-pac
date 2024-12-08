///Register `DDATA4` reader
pub type R = crate::R<DDATA4rs>;
///Register `DDATA4` writer
pub type W = crate::W<DDATA4rs>;
///Field `DDATA4` reader - Double Data 0 Access
pub type Ddata4R = crate::FieldReader<u32>;
///Field `DDATA4` writer - Double Data 0 Access
pub type Ddata4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Double Data 0 Access
    #[inline(always)]
    pub fn ddata4(&self) -> Ddata4R {
        Ddata4R::new(self.bits)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DDATA4rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Double Data 0 Access
    #[inline(always)]
    #[must_use]
    pub fn ddata4(&mut self) -> Ddata4W<DDATA4rs> {
        Ddata4W::new(self, 0)
    }
}
///DDATA4 Register Access
///
///You can [`read`](crate::Reg::read) this register and get [`ddata4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct DDATA4rs;
impl crate::RegisterSpec for DDATA4rs {
    type Ux = u32;
}
///`read()` method returns [`ddata4::R`](R) reader structure
impl crate::Readable for DDATA4rs {}
///`write(|w| ..)` method takes [`ddata4::W`](W) writer structure
impl crate::Writable for DDATA4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDATA4 to value 0
impl crate::Resettable for DDATA4rs {
    const RESET_VALUE: u32 = 0;
}
