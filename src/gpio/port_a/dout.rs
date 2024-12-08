///Register `DOUT` reader
pub type R = crate::R<DOUTrs>;
///Register `DOUT` writer
pub type W = crate::W<DOUTrs>;
///Field `PINS_DOUT` reader - Data Out for pins 0:15
pub type PinsDoutR = crate::FieldReader<u16>;
///Field `PINS_DOUT` writer - Data Out for pins 0:15
pub type PinsDoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Data Out for pins 0:15
    #[inline(always)]
    pub fn pins_dout(&self) -> PinsDoutR {
        PinsDoutR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT")
            .field("pins_dout", &self.pins_dout())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Data Out for pins 0:15
    #[inline(always)]
    #[must_use]
    pub fn pins_dout(&mut self) -> PinsDoutW<DOUTrs> {
        PinsDoutW::new(self, 0)
    }
}
///Port Data Out Register
///
///You can [`read`](crate::Reg::read) this register and get [`dout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DOUTrs;
impl crate::RegisterSpec for DOUTrs {
    type Ux = u32;
}
///`read()` method returns [`dout::R`](R) reader structure
impl crate::Readable for DOUTrs {}
///`write(|w| ..)` method takes [`dout::W`](W) writer structure
impl crate::Writable for DOUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOUT to value 0
impl crate::Resettable for DOUTrs {
    const RESET_VALUE: u32 = 0;
}
