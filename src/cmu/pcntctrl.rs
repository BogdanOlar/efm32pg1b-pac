#[doc = "Register `PCNTCTRL` reader"]
pub type R = crate::R<PCNTCTRLrs>;
#[doc = "Register `PCNTCTRL` writer"]
pub type W = crate::W<PCNTCTRLrs>;
#[doc = "Field `PCNT0CLKEN` reader - PCNT0 Clock Enable"]
pub type Pcnt0clkenR = crate::BitReader;
#[doc = "Field `PCNT0CLKEN` writer - PCNT0 Clock Enable"]
pub type Pcnt0clkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT0CLKSEL` reader - PCNT0 Clock Select"]
pub type Pcnt0clkselR = crate::BitReader;
#[doc = "Field `PCNT0CLKSEL` writer - PCNT0 Clock Select"]
pub type Pcnt0clkselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    pub fn pcnt0clken(&self) -> Pcnt0clkenR {
        Pcnt0clkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    pub fn pcnt0clksel(&self) -> Pcnt0clkselR {
        Pcnt0clkselR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNTCTRL")
            .field("pcnt0clken", &self.pcnt0clken())
            .field("pcnt0clksel", &self.pcnt0clksel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0clken(&mut self) -> Pcnt0clkenW<PCNTCTRLrs> {
        Pcnt0clkenW::new(self, 0)
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0clksel(&mut self) -> Pcnt0clkselW<PCNTCTRLrs> {
        Pcnt0clkselW::new(self, 1)
    }
}
#[doc = "PCNT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcntctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCNTCTRLrs;
impl crate::RegisterSpec for PCNTCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcntctrl::R`](R) reader structure"]
impl crate::Readable for PCNTCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`pcntctrl::W`](W) writer structure"]
impl crate::Writable for PCNTCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCNTCTRL to value 0"]
impl crate::Resettable for PCNTCTRLrs {
    const RESET_VALUE: u32 = 0;
}
