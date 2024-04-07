#[doc = "Register `HFPERCLKEN0` reader"]
pub type R = crate::R<HFPERCLKEN0rs>;
#[doc = "Register `HFPERCLKEN0` writer"]
pub type W = crate::W<HFPERCLKEN0rs>;
#[doc = "Field `TIMER0` reader - Timer 0 Clock Enable"]
pub type Timer0R = crate::BitReader;
#[doc = "Field `TIMER0` writer - Timer 0 Clock Enable"]
pub type Timer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1` reader - Timer 1 Clock Enable"]
pub type Timer1R = crate::BitReader;
#[doc = "Field `TIMER1` writer - Timer 1 Clock Enable"]
pub type Timer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART0` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type Usart0R = crate::BitReader;
#[doc = "Field `USART0` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type Usart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type Usart1R = crate::BitReader;
#[doc = "Field `USART1` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type Usart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP0` reader - Analog Comparator 0 Clock Enable"]
pub type Acmp0R = crate::BitReader;
#[doc = "Field `ACMP0` writer - Analog Comparator 0 Clock Enable"]
pub type Acmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1` reader - Analog Comparator 1 Clock Enable"]
pub type Acmp1R = crate::BitReader;
#[doc = "Field `ACMP1` writer - Analog Comparator 1 Clock Enable"]
pub type Acmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYOTIMER` reader - CRYOTIMER Clock Enable"]
pub type CryotimerR = crate::BitReader;
#[doc = "Field `CRYOTIMER` writer - CRYOTIMER Clock Enable"]
pub type CryotimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0` reader - I2C 0 Clock Enable"]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C0` writer - I2C 0 Clock Enable"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0` reader - Analog to Digital Converter 0 Clock Enable"]
pub type Adc0R = crate::BitReader;
#[doc = "Field `ADC0` writer - Analog to Digital Converter 0 Clock Enable"]
pub type Adc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDAC0` reader - Current Digital to Analog Converter 0 Clock Enable"]
pub type Idac0R = crate::BitReader;
#[doc = "Field `IDAC0` writer - Current Digital to Analog Converter 0 Clock Enable"]
pub type Idac0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn timer0(&self) -> Timer0R {
        Timer0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn timer1(&self) -> Timer1R {
        Timer1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn usart0(&self) -> Usart0R {
        Usart0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn usart1(&self) -> Usart1R {
        Usart1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog Comparator 0 Clock Enable"]
    #[inline(always)]
    pub fn acmp0(&self) -> Acmp0R {
        Acmp0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog Comparator 1 Clock Enable"]
    #[inline(always)]
    pub fn acmp1(&self) -> Acmp1R {
        Acmp1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CRYOTIMER Clock Enable"]
    #[inline(always)]
    pub fn cryotimer(&self) -> CryotimerR {
        CryotimerR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C 0 Clock Enable"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn adc0(&self) -> Adc0R {
        Adc0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Current Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn idac0(&self) -> Idac0R {
        Idac0R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer0(&mut self) -> Timer0W<HFPERCLKEN0rs> {
        Timer0W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer1(&mut self) -> Timer1W<HFPERCLKEN0rs> {
        Timer1W::new(self, 1)
    }
    #[doc = "Bit 2 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> Usart0W<HFPERCLKEN0rs> {
        Usart0W::new(self, 2)
    }
    #[doc = "Bit 3 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> Usart1W<HFPERCLKEN0rs> {
        Usart1W::new(self, 3)
    }
    #[doc = "Bit 4 - Analog Comparator 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0(&mut self) -> Acmp0W<HFPERCLKEN0rs> {
        Acmp0W::new(self, 4)
    }
    #[doc = "Bit 5 - Analog Comparator 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1(&mut self) -> Acmp1W<HFPERCLKEN0rs> {
        Acmp1W::new(self, 5)
    }
    #[doc = "Bit 6 - CRYOTIMER Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cryotimer(&mut self) -> CryotimerW<HFPERCLKEN0rs> {
        CryotimerW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2c0W<HFPERCLKEN0rs> {
        I2c0W::new(self, 7)
    }
    #[doc = "Bit 8 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> Adc0W<HFPERCLKEN0rs> {
        Adc0W::new(self, 8)
    }
    #[doc = "Bit 9 - Current Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn idac0(&mut self) -> Idac0W<HFPERCLKEN0rs> {
        Idac0W::new(self, 9)
    }
}
#[doc = "High Frequency Peripheral Clock Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfperclken0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfperclken0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFPERCLKEN0rs;
impl crate::RegisterSpec for HFPERCLKEN0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfperclken0::R`](R) reader structure"]
impl crate::Readable for HFPERCLKEN0rs {}
#[doc = "`write(|w| ..)` method takes [`hfperclken0::W`](W) writer structure"]
impl crate::Writable for HFPERCLKEN0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFPERCLKEN0 to value 0"]
impl crate::Resettable for HFPERCLKEN0rs {
    const RESET_VALUE: u32 = 0;
}
