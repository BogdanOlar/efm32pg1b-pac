#[doc = "Register `PINLOCKN` reader"]
pub type R = crate::R<PINLOCKNrs>;
#[doc = "Register `PINLOCKN` writer"]
pub type W = crate::W<PINLOCKNrs>;
#[doc = "Field `PINS_PINLOCKN` reader - Unlocked Pins for pins 0:15"]
pub type PinsPinlocknR = crate::FieldReader<u16>;
#[doc = "Field `PINS_PINLOCKN` writer - Unlocked Pins for pins 0:15"]
pub type PinsPinlocknW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Unlocked Pins for pins 0:15"]
    #[inline(always)]
    pub fn pins_pinlockn(&self) -> PinsPinlocknR {
        PinsPinlocknR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Unlocked Pins for pins 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn pins_pinlockn(&mut self) -> PinsPinlocknW<PINLOCKNrs> {
        PinsPinlocknW::new(self, 0)
    }
}
#[doc = "Port Unlocked Pins Register. Shows unlocked pins in the port. To lock pin n, clear bit n. The pin is then locked until reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinlockn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinlockn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINLOCKNrs;
impl crate::RegisterSpec for PINLOCKNrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinlockn::R`](R) reader structure"]
impl crate::Readable for PINLOCKNrs {}
#[doc = "`write(|w| ..)` method takes [`pinlockn::W`](W) writer structure"]
impl crate::Writable for PINLOCKNrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINLOCKN to value 0xffff"]
impl crate::Resettable for PINLOCKNrs {
    const RESET_VALUE: u32 = 0xffff;
}
