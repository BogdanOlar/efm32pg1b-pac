///Register `REQDIS` reader
pub type R = crate::R<REQDISrs>;
///Register `REQDIS` writer
pub type W = crate::W<REQDISrs>;
///Field `REQDIS` reader - DMA Request Disables
pub type ReqdisR = crate::FieldReader;
///Field `REQDIS` writer - DMA Request Disables
pub type ReqdisW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - DMA Request Disables
    #[inline(always)]
    pub fn reqdis(&self) -> ReqdisR {
        ReqdisR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REQDIS")
            .field("reqdis", &self.reqdis())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DMA Request Disables
    #[inline(always)]
    #[must_use]
    pub fn reqdis(&mut self) -> ReqdisW<REQDISrs> {
        ReqdisW::new(self, 0)
    }
}
///DMA Channel Request Disable Register
///
///You can [`read`](crate::Reg::read) this register and get [`reqdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct REQDISrs;
impl crate::RegisterSpec for REQDISrs {
    type Ux = u32;
}
///`read()` method returns [`reqdis::R`](R) reader structure
impl crate::Readable for REQDISrs {}
///`write(|w| ..)` method takes [`reqdis::W`](W) writer structure
impl crate::Writable for REQDISrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REQDIS to value 0
impl crate::Resettable for REQDISrs {
    const RESET_VALUE: u32 = 0;
}
