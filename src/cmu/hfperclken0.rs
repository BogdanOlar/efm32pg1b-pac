///Register `HFPERCLKEN0` reader
pub type R = crate::R<HFPERCLKEN0rs>;
///Register `HFPERCLKEN0` writer
pub type W = crate::W<HFPERCLKEN0rs>;
///Field `TIMER0` reader - Timer 0 Clock Enable
pub type Timer0R = crate::BitReader;
///Field `TIMER0` writer - Timer 0 Clock Enable
pub type Timer0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMER1` reader - Timer 1 Clock Enable
pub type Timer1R = crate::BitReader;
///Field `TIMER1` writer - Timer 1 Clock Enable
pub type Timer1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART0` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable
pub type Usart0R = crate::BitReader;
///Field `USART0` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable
pub type Usart0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable
pub type Usart1R = crate::BitReader;
///Field `USART1` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable
pub type Usart1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACMP0` reader - Analog Comparator 0 Clock Enable
pub type Acmp0R = crate::BitReader;
///Field `ACMP0` writer - Analog Comparator 0 Clock Enable
pub type Acmp0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACMP1` reader - Analog Comparator 1 Clock Enable
pub type Acmp1R = crate::BitReader;
///Field `ACMP1` writer - Analog Comparator 1 Clock Enable
pub type Acmp1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYOTIMER` reader - CRYOTIMER Clock Enable
pub type CryotimerR = crate::BitReader;
///Field `CRYOTIMER` writer - CRYOTIMER Clock Enable
pub type CryotimerW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C0` reader - I2C 0 Clock Enable
pub type I2c0R = crate::BitReader;
///Field `I2C0` writer - I2C 0 Clock Enable
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC0` reader - Analog to Digital Converter 0 Clock Enable
pub type Adc0R = crate::BitReader;
///Field `ADC0` writer - Analog to Digital Converter 0 Clock Enable
pub type Adc0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDAC0` reader - Current Digital to Analog Converter 0 Clock Enable
pub type Idac0R = crate::BitReader;
///Field `IDAC0` writer - Current Digital to Analog Converter 0 Clock Enable
pub type Idac0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Timer 0 Clock Enable
    #[inline(always)]
    pub fn timer0(&self) -> Timer0R {
        Timer0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer 1 Clock Enable
    #[inline(always)]
    pub fn timer1(&self) -> Timer1R {
        Timer1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable
    #[inline(always)]
    pub fn usart0(&self) -> Usart0R {
        Usart0R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable
    #[inline(always)]
    pub fn usart1(&self) -> Usart1R {
        Usart1R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog Comparator 0 Clock Enable
    #[inline(always)]
    pub fn acmp0(&self) -> Acmp0R {
        Acmp0R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Analog Comparator 1 Clock Enable
    #[inline(always)]
    pub fn acmp1(&self) -> Acmp1R {
        Acmp1R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CRYOTIMER Clock Enable
    #[inline(always)]
    pub fn cryotimer(&self) -> CryotimerR {
        CryotimerR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C 0 Clock Enable
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog to Digital Converter 0 Clock Enable
    #[inline(always)]
    pub fn adc0(&self) -> Adc0R {
        Adc0R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Current Digital to Analog Converter 0 Clock Enable
    #[inline(always)]
    pub fn idac0(&self) -> Idac0R {
        Idac0R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFPERCLKEN0")
            .field("timer0", &self.timer0())
            .field("timer1", &self.timer1())
            .field("usart0", &self.usart0())
            .field("usart1", &self.usart1())
            .field("acmp0", &self.acmp0())
            .field("acmp1", &self.acmp1())
            .field("cryotimer", &self.cryotimer())
            .field("i2c0", &self.i2c0())
            .field("adc0", &self.adc0())
            .field("idac0", &self.idac0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timer 0 Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn timer0(&mut self) -> Timer0W<HFPERCLKEN0rs> {
        Timer0W::new(self, 0)
    }
    ///Bit 1 - Timer 1 Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn timer1(&mut self) -> Timer1W<HFPERCLKEN0rs> {
        Timer1W::new(self, 1)
    }
    ///Bit 2 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> Usart0W<HFPERCLKEN0rs> {
        Usart0W::new(self, 2)
    }
    ///Bit 3 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> Usart1W<HFPERCLKEN0rs> {
        Usart1W::new(self, 3)
    }
    ///Bit 4 - Analog Comparator 0 Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn acmp0(&mut self) -> Acmp0W<HFPERCLKEN0rs> {
        Acmp0W::new(self, 4)
    }
    ///Bit 5 - Analog Comparator 1 Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn acmp1(&mut self) -> Acmp1W<HFPERCLKEN0rs> {
        Acmp1W::new(self, 5)
    }
    ///Bit 6 - CRYOTIMER Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn cryotimer(&mut self) -> CryotimerW<HFPERCLKEN0rs> {
        CryotimerW::new(self, 6)
    }
    ///Bit 7 - I2C 0 Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2c0W<HFPERCLKEN0rs> {
        I2c0W::new(self, 7)
    }
    ///Bit 8 - Analog to Digital Converter 0 Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> Adc0W<HFPERCLKEN0rs> {
        Adc0W::new(self, 8)
    }
    ///Bit 9 - Current Digital to Analog Converter 0 Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn idac0(&mut self) -> Idac0W<HFPERCLKEN0rs> {
        Idac0W::new(self, 9)
    }
}
///High Frequency Peripheral Clock Enable Register 0
///
///You can [`read`](crate::Reg::read) this register and get [`hfperclken0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfperclken0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HFPERCLKEN0rs;
impl crate::RegisterSpec for HFPERCLKEN0rs {
    type Ux = u32;
}
///`read()` method returns [`hfperclken0::R`](R) reader structure
impl crate::Readable for HFPERCLKEN0rs {}
///`write(|w| ..)` method takes [`hfperclken0::W`](W) writer structure
impl crate::Writable for HFPERCLKEN0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HFPERCLKEN0 to value 0
impl crate::Resettable for HFPERCLKEN0rs {
    const RESET_VALUE: u32 = 0;
}
