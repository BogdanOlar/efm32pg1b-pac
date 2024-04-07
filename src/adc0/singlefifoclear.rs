#[doc = "Register `SINGLEFIFOCLEAR` writer"]
pub type W = crate::W<SINGLEFIFOCLEARrs>;
#[doc = "Field `SINGLEFIFOCLEAR` writer - Clear Single FIFO Content"]
pub type SinglefifoclearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear Single FIFO Content"]
    #[inline(always)]
    #[must_use]
    pub fn singlefifoclear(&mut self) -> SinglefifoclearW<SINGLEFIFOCLEARrs> {
        SinglefifoclearW::new(self, 0)
    }
}
#[doc = "Single FIFO Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`singlefifoclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLEFIFOCLEARrs;
impl crate::RegisterSpec for SINGLEFIFOCLEARrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`singlefifoclear::W`](W) writer structure"]
impl crate::Writable for SINGLEFIFOCLEARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SINGLEFIFOCLEAR to value 0"]
impl crate::Resettable for SINGLEFIFOCLEARrs {
    const RESET_VALUE: u32 = 0;
}
