#[doc = "Register `PA_PINLOCKN` reader"]
pub type R = crate::R<PA_PINLOCKNrs>;
#[doc = "Register `PA_PINLOCKN` writer"]
pub type W = crate::W<PA_PINLOCKNrs>;
#[doc = "Field `PINLOCKN` reader - Unlocked Pins"]
pub type PinlocknR = crate::FieldReader<u16>;
#[doc = "Field `PINLOCKN` writer - Unlocked Pins"]
pub type PinlocknW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Unlocked Pins"]
    #[inline(always)]
    pub fn pinlockn(&self) -> PinlocknR {
        PinlocknR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Unlocked Pins"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn(&mut self) -> PinlocknW<PA_PINLOCKNrs> {
        PinlocknW::new(self, 0)
    }
}
#[doc = "Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_pinlockn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_pinlockn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PA_PINLOCKNrs;
impl crate::RegisterSpec for PA_PINLOCKNrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_pinlockn::R`](R) reader structure"]
impl crate::Readable for PA_PINLOCKNrs {}
#[doc = "`write(|w| ..)` method takes [`pa_pinlockn::W`](W) writer structure"]
impl crate::Writable for PA_PINLOCKNrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA_PINLOCKN to value 0xffff"]
impl crate::Resettable for PA_PINLOCKNrs {
    const RESET_VALUE: u32 = 0xffff;
}
