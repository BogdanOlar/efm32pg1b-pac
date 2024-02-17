#[doc = "Register `SWPULSE` writer"]
pub type W = crate::W<SWPULSErs>;
#[doc = "Field `CH0PULSE` writer - Channel 0 Pulse Generation"]
pub type CH0PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1PULSE` writer - Channel 1 Pulse Generation"]
pub type CH1PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2PULSE` writer - Channel 2 Pulse Generation"]
pub type CH2PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3PULSE` writer - Channel 3 Pulse Generation"]
pub type CH3PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4PULSE` writer - Channel 4 Pulse Generation"]
pub type CH4PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5PULSE` writer - Channel 5 Pulse Generation"]
pub type CH5PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6PULSE` writer - Channel 6 Pulse Generation"]
pub type CH6PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7PULSE` writer - Channel 7 Pulse Generation"]
pub type CH7PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8PULSE` writer - Channel 8 Pulse Generation"]
pub type CH8PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9PULSE` writer - Channel 9 Pulse Generation"]
pub type CH9PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10PULSE` writer - Channel 10 Pulse Generation"]
pub type CH10PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11PULSE` writer - Channel 11 Pulse Generation"]
pub type CH11PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch0pulse(&mut self) -> CH0PULSE_W<SWPULSErs> {
        CH0PULSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pulse(&mut self) -> CH1PULSE_W<SWPULSErs> {
        CH1PULSE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pulse(&mut self) -> CH2PULSE_W<SWPULSErs> {
        CH2PULSE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pulse(&mut self) -> CH3PULSE_W<SWPULSErs> {
        CH3PULSE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch4pulse(&mut self) -> CH4PULSE_W<SWPULSErs> {
        CH4PULSE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch5pulse(&mut self) -> CH5PULSE_W<SWPULSErs> {
        CH5PULSE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch6pulse(&mut self) -> CH6PULSE_W<SWPULSErs> {
        CH6PULSE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch7pulse(&mut self) -> CH7PULSE_W<SWPULSErs> {
        CH7PULSE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 8 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch8pulse(&mut self) -> CH8PULSE_W<SWPULSErs> {
        CH8PULSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 9 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch9pulse(&mut self) -> CH9PULSE_W<SWPULSErs> {
        CH9PULSE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 10 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch10pulse(&mut self) -> CH10PULSE_W<SWPULSErs> {
        CH10PULSE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 11 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch11pulse(&mut self) -> CH11PULSE_W<SWPULSErs> {
        CH11PULSE_W::new(self, 11)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software Pulse Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpulse::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWPULSErs;
impl crate::RegisterSpec for SWPULSErs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swpulse::W`](W) writer structure"]
impl crate::Writable for SWPULSErs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWPULSE to value 0"]
impl crate::Resettable for SWPULSErs {
    const RESET_VALUE: u32 = 0;
}
