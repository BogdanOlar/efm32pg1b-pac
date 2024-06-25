#[doc = "Register `DOUT_TGL` writer"]
pub type W = crate::W<DOUT_TGLrs>;
#[doc = "Field `PINS_DOUT_TGL` writer - Data Out Toggle for pins 0:15"]
pub type PinsDoutTglW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<DOUT_TGLrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Out Toggle for pins 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn pins_dout_tgl(&mut self) -> PinsDoutTglW<DOUT_TGLrs> {
        PinsDoutTglW::new(self, 0)
    }
}
#[doc = "Port Data Out Toggle Register. Write bits to 1 to toggle corresponding bits in GPIO_Px_DOUT. Bits written to 0 will have no effect.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_tgl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUT_TGLrs;
impl crate::RegisterSpec for DOUT_TGLrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout_tgl::W`](W) writer structure"]
impl crate::Writable for DOUT_TGLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT_TGL to value 0"]
impl crate::Resettable for DOUT_TGLrs {
    const RESET_VALUE: u32 = 0;
}
