#[doc = "Register `PWRCTRL` reader"]
pub type R = crate::R<PWRCTRLrs>;
#[doc = "Register `PWRCTRL` writer"]
pub type W = crate::W<PWRCTRLrs>;
#[doc = "Field `ANASW` reader - Analog Switch Selection"]
pub type AnaswR = crate::BitReader;
#[doc = "Field `ANASW` writer - Analog Switch Selection"]
pub type AnaswW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Analog Switch Selection"]
    #[inline(always)]
    pub fn anasw(&self) -> AnaswR {
        AnaswR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Analog Switch Selection"]
    #[inline(always)]
    #[must_use]
    pub fn anasw(&mut self) -> AnaswW<PWRCTRLrs> {
        AnaswW::new(self, 5)
    }
}
#[doc = "Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRCTRLrs;
impl crate::RegisterSpec for PWRCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrctrl::R`](R) reader structure"]
impl crate::Readable for PWRCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`pwrctrl::W`](W) writer structure"]
impl crate::Writable for PWRCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCTRL to value 0"]
impl crate::Resettable for PWRCTRLrs {
    const RESET_VALUE: u32 = 0;
}
