#[doc = "Register `SWLEVEL` reader"]
pub type R = crate::R<SWLEVELrs>;
#[doc = "Register `SWLEVEL` writer"]
pub type W = crate::W<SWLEVELrs>;
#[doc = "Field `CH0LEVEL` reader - Channel 0 Software Level"]
pub type CH0LEVEL_R = crate::BitReader;
#[doc = "Field `CH0LEVEL` writer - Channel 0 Software Level"]
pub type CH0LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1LEVEL` reader - Channel 1 Software Level"]
pub type CH1LEVEL_R = crate::BitReader;
#[doc = "Field `CH1LEVEL` writer - Channel 1 Software Level"]
pub type CH1LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2LEVEL` reader - Channel 2 Software Level"]
pub type CH2LEVEL_R = crate::BitReader;
#[doc = "Field `CH2LEVEL` writer - Channel 2 Software Level"]
pub type CH2LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3LEVEL` reader - Channel 3 Software Level"]
pub type CH3LEVEL_R = crate::BitReader;
#[doc = "Field `CH3LEVEL` writer - Channel 3 Software Level"]
pub type CH3LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4LEVEL` reader - Channel 4 Software Level"]
pub type CH4LEVEL_R = crate::BitReader;
#[doc = "Field `CH4LEVEL` writer - Channel 4 Software Level"]
pub type CH4LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5LEVEL` reader - Channel 5 Software Level"]
pub type CH5LEVEL_R = crate::BitReader;
#[doc = "Field `CH5LEVEL` writer - Channel 5 Software Level"]
pub type CH5LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6LEVEL` reader - Channel 6 Software Level"]
pub type CH6LEVEL_R = crate::BitReader;
#[doc = "Field `CH6LEVEL` writer - Channel 6 Software Level"]
pub type CH6LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7LEVEL` reader - Channel 7 Software Level"]
pub type CH7LEVEL_R = crate::BitReader;
#[doc = "Field `CH7LEVEL` writer - Channel 7 Software Level"]
pub type CH7LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8LEVEL` reader - Channel 8 Software Level"]
pub type CH8LEVEL_R = crate::BitReader;
#[doc = "Field `CH8LEVEL` writer - Channel 8 Software Level"]
pub type CH8LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9LEVEL` reader - Channel 9 Software Level"]
pub type CH9LEVEL_R = crate::BitReader;
#[doc = "Field `CH9LEVEL` writer - Channel 9 Software Level"]
pub type CH9LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10LEVEL` reader - Channel 10 Software Level"]
pub type CH10LEVEL_R = crate::BitReader;
#[doc = "Field `CH10LEVEL` writer - Channel 10 Software Level"]
pub type CH10LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11LEVEL` reader - Channel 11 Software Level"]
pub type CH11LEVEL_R = crate::BitReader;
#[doc = "Field `CH11LEVEL` writer - Channel 11 Software Level"]
pub type CH11LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 0 Software Level"]
    #[inline(always)]
    pub fn ch0level(&self) -> CH0LEVEL_R {
        CH0LEVEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Level"]
    #[inline(always)]
    pub fn ch1level(&self) -> CH1LEVEL_R {
        CH1LEVEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Level"]
    #[inline(always)]
    pub fn ch2level(&self) -> CH2LEVEL_R {
        CH2LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Level"]
    #[inline(always)]
    pub fn ch3level(&self) -> CH3LEVEL_R {
        CH3LEVEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Level"]
    #[inline(always)]
    pub fn ch4level(&self) -> CH4LEVEL_R {
        CH4LEVEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Level"]
    #[inline(always)]
    pub fn ch5level(&self) -> CH5LEVEL_R {
        CH5LEVEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Software Level"]
    #[inline(always)]
    pub fn ch6level(&self) -> CH6LEVEL_R {
        CH6LEVEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Software Level"]
    #[inline(always)]
    pub fn ch7level(&self) -> CH7LEVEL_R {
        CH7LEVEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Software Level"]
    #[inline(always)]
    pub fn ch8level(&self) -> CH8LEVEL_R {
        CH8LEVEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Software Level"]
    #[inline(always)]
    pub fn ch9level(&self) -> CH9LEVEL_R {
        CH9LEVEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Software Level"]
    #[inline(always)]
    pub fn ch10level(&self) -> CH10LEVEL_R {
        CH10LEVEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Software Level"]
    #[inline(always)]
    pub fn ch11level(&self) -> CH11LEVEL_R {
        CH11LEVEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch0level(&mut self) -> CH0LEVEL_W<SWLEVELrs> {
        CH0LEVEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch1level(&mut self) -> CH1LEVEL_W<SWLEVELrs> {
        CH1LEVEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch2level(&mut self) -> CH2LEVEL_W<SWLEVELrs> {
        CH2LEVEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch3level(&mut self) -> CH3LEVEL_W<SWLEVELrs> {
        CH3LEVEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch4level(&mut self) -> CH4LEVEL_W<SWLEVELrs> {
        CH4LEVEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch5level(&mut self) -> CH5LEVEL_W<SWLEVELrs> {
        CH5LEVEL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch6level(&mut self) -> CH6LEVEL_W<SWLEVELrs> {
        CH6LEVEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch7level(&mut self) -> CH7LEVEL_W<SWLEVELrs> {
        CH7LEVEL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 8 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch8level(&mut self) -> CH8LEVEL_W<SWLEVELrs> {
        CH8LEVEL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 9 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch9level(&mut self) -> CH9LEVEL_W<SWLEVELrs> {
        CH9LEVEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 10 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch10level(&mut self) -> CH10LEVEL_W<SWLEVELrs> {
        CH10LEVEL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 11 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch11level(&mut self) -> CH11LEVEL_W<SWLEVELrs> {
        CH11LEVEL_W::new(self, 11)
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
#[doc = "Software Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swlevel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swlevel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWLEVELrs;
impl crate::RegisterSpec for SWLEVELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swlevel::R`](R) reader structure"]
impl crate::Readable for SWLEVELrs {}
#[doc = "`write(|w| ..)` method takes [`swlevel::W`](W) writer structure"]
impl crate::Writable for SWLEVELrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWLEVEL to value 0"]
impl crate::Resettable for SWLEVELrs {
    const RESET_VALUE: u32 = 0;
}
