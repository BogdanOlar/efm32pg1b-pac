#[doc = "Register `SWPULSE` writer"]
pub type W = crate::W<SWPULSErs>;
#[doc = "Field `CH0PULSE` writer - Channel 0 Pulse Generation"]
pub type Ch0pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1PULSE` writer - Channel 1 Pulse Generation"]
pub type Ch1pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2PULSE` writer - Channel 2 Pulse Generation"]
pub type Ch2pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3PULSE` writer - Channel 3 Pulse Generation"]
pub type Ch3pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4PULSE` writer - Channel 4 Pulse Generation"]
pub type Ch4pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5PULSE` writer - Channel 5 Pulse Generation"]
pub type Ch5pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6PULSE` writer - Channel 6 Pulse Generation"]
pub type Ch6pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7PULSE` writer - Channel 7 Pulse Generation"]
pub type Ch7pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8PULSE` writer - Channel 8 Pulse Generation"]
pub type Ch8pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9PULSE` writer - Channel 9 Pulse Generation"]
pub type Ch9pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10PULSE` writer - Channel 10 Pulse Generation"]
pub type Ch10pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11PULSE` writer - Channel 11 Pulse Generation"]
pub type Ch11pulseW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SWPULSErs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch0pulse(&mut self) -> Ch0pulseW<SWPULSErs> {
        Ch0pulseW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pulse(&mut self) -> Ch1pulseW<SWPULSErs> {
        Ch1pulseW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pulse(&mut self) -> Ch2pulseW<SWPULSErs> {
        Ch2pulseW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pulse(&mut self) -> Ch3pulseW<SWPULSErs> {
        Ch3pulseW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch4pulse(&mut self) -> Ch4pulseW<SWPULSErs> {
        Ch4pulseW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch5pulse(&mut self) -> Ch5pulseW<SWPULSErs> {
        Ch5pulseW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch6pulse(&mut self) -> Ch6pulseW<SWPULSErs> {
        Ch6pulseW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch7pulse(&mut self) -> Ch7pulseW<SWPULSErs> {
        Ch7pulseW::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 8 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch8pulse(&mut self) -> Ch8pulseW<SWPULSErs> {
        Ch8pulseW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 9 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch9pulse(&mut self) -> Ch9pulseW<SWPULSErs> {
        Ch9pulseW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 10 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch10pulse(&mut self) -> Ch10pulseW<SWPULSErs> {
        Ch10pulseW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 11 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch11pulse(&mut self) -> Ch11pulseW<SWPULSErs> {
        Ch11pulseW::new(self, 11)
    }
}
#[doc = "Software Pulse Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpulse::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWPULSErs;
impl crate::RegisterSpec for SWPULSErs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swpulse::W`](W) writer structure"]
impl crate::Writable for SWPULSErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWPULSE to value 0"]
impl crate::Resettable for SWPULSErs {
    const RESET_VALUE: u32 = 0;
}
