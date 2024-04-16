#[doc = "Register `DOUT` reader"]
pub type R = crate::R<DOUTrs>;
#[doc = "Register `DOUT` writer"]
pub type W = crate::W<DOUTrs>;
#[doc = "Field `PINS_DOUT` reader - Data Out for pins 0:15"]
pub type PinsDoutR = crate::FieldReader<u16>;
#[doc = "Field `PINS_DOUT` writer - Data Out for pins 0:15"]
pub type PinsDoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Out for pins 0:15"]
    #[inline(always)]
    pub fn pins_dout(&self) -> PinsDoutR {
        PinsDoutR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Out for pins 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn pins_dout(&mut self) -> PinsDoutW<DOUTrs> {
        PinsDoutW::new(self, 0)
    }
}
#[doc = "Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTrs;
impl crate::RegisterSpec for DOUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout::R`](R) reader structure"]
impl crate::Readable for DOUTrs {}
#[doc = "`write(|w| ..)` method takes [`dout::W`](W) writer structure"]
impl crate::Writable for DOUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT to value 0"]
impl crate::Resettable for DOUTrs {
    const RESET_VALUE: u32 = 0;
}
