#[doc = "Register `PULSECTRL` reader"]
pub type R = crate::R<PULSECTRLrs>;
#[doc = "Register `PULSECTRL` writer"]
pub type W = crate::W<PULSECTRLrs>;
#[doc = "Field `PULSEW` reader - Pulse Width"]
pub type PulsewR = crate::FieldReader;
#[doc = "Field `PULSEW` writer - Pulse Width"]
pub type PulsewW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PULSEEN` reader - Pulse Generator/Extender Enable"]
pub type PulseenR = crate::BitReader;
#[doc = "Field `PULSEEN` writer - Pulse Generator/Extender Enable"]
pub type PulseenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULSEFILT` reader - Pulse Filter"]
pub type PulsefiltR = crate::BitReader;
#[doc = "Field `PULSEFILT` writer - Pulse Filter"]
pub type PulsefiltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Pulse Width"]
    #[inline(always)]
    pub fn pulsew(&self) -> PulsewR {
        PulsewR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    pub fn pulseen(&self) -> PulseenR {
        PulseenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pulse Filter"]
    #[inline(always)]
    pub fn pulsefilt(&self) -> PulsefiltR {
        PulsefiltR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn pulsew(&mut self) -> PulsewW<PULSECTRLrs> {
        PulsewW::new(self, 0)
    }
    #[doc = "Bit 4 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pulseen(&mut self) -> PulseenW<PULSECTRLrs> {
        PulseenW::new(self, 4)
    }
    #[doc = "Bit 5 - Pulse Filter"]
    #[inline(always)]
    #[must_use]
    pub fn pulsefilt(&mut self) -> PulsefiltW<PULSECTRLrs> {
        PulsefiltW::new(self, 5)
    }
}
#[doc = "Pulse Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulsectrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulsectrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PULSECTRLrs;
impl crate::RegisterSpec for PULSECTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pulsectrl::R`](R) reader structure"]
impl crate::Readable for PULSECTRLrs {}
#[doc = "`write(|w| ..)` method takes [`pulsectrl::W`](W) writer structure"]
impl crate::Writable for PULSECTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PULSECTRL to value 0"]
impl crate::Resettable for PULSECTRLrs {
    const RESET_VALUE: u32 = 0;
}
