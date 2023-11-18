#[doc = "Register `HFPERCLKEN0` reader"]
pub type R = crate::R<HFPERCLKEN0_SPEC>;
#[doc = "Register `HFPERCLKEN0` writer"]
pub type W = crate::W<HFPERCLKEN0_SPEC>;
#[doc = "Field `TIMER0` reader - Timer 0 Clock Enable"]
pub type TIMER0_R = crate::BitReader;
#[doc = "Field `TIMER0` writer - Timer 0 Clock Enable"]
pub type TIMER0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER1` reader - Timer 1 Clock Enable"]
pub type TIMER1_R = crate::BitReader;
#[doc = "Field `TIMER1` writer - Timer 1 Clock Enable"]
pub type TIMER1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART0` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type USART0_R = crate::BitReader;
#[doc = "Field `USART0` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type USART0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type USART1_R = crate::BitReader;
#[doc = "Field `USART1` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type USART1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACMP0` reader - Analog Comparator 0 Clock Enable"]
pub type ACMP0_R = crate::BitReader;
#[doc = "Field `ACMP0` writer - Analog Comparator 0 Clock Enable"]
pub type ACMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACMP1` reader - Analog Comparator 1 Clock Enable"]
pub type ACMP1_R = crate::BitReader;
#[doc = "Field `ACMP1` writer - Analog Comparator 1 Clock Enable"]
pub type ACMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRYOTIMER` reader - CRYOTIMER Clock Enable"]
pub type CRYOTIMER_R = crate::BitReader;
#[doc = "Field `CRYOTIMER` writer - CRYOTIMER Clock Enable"]
pub type CRYOTIMER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C0` reader - I2C 0 Clock Enable"]
pub type I2C0_R = crate::BitReader;
#[doc = "Field `I2C0` writer - I2C 0 Clock Enable"]
pub type I2C0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC0` reader - Analog to Digital Converter 0 Clock Enable"]
pub type ADC0_R = crate::BitReader;
#[doc = "Field `ADC0` writer - Analog to Digital Converter 0 Clock Enable"]
pub type ADC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDAC0` reader - Current Digital to Analog Converter 0 Clock Enable"]
pub type IDAC0_R = crate::BitReader;
#[doc = "Field `IDAC0` writer - Current Digital to Analog Converter 0 Clock Enable"]
pub type IDAC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog Comparator 0 Clock Enable"]
    #[inline(always)]
    pub fn acmp0(&self) -> ACMP0_R {
        ACMP0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog Comparator 1 Clock Enable"]
    #[inline(always)]
    pub fn acmp1(&self) -> ACMP1_R {
        ACMP1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CRYOTIMER Clock Enable"]
    #[inline(always)]
    pub fn cryotimer(&self) -> CRYOTIMER_R {
        CRYOTIMER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C 0 Clock Enable"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Current Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn idac0(&self) -> IDAC0_R {
        IDAC0_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFPERCLKEN0")
            .field("timer0", &format_args!("{}", self.timer0().bit()))
            .field("timer1", &format_args!("{}", self.timer1().bit()))
            .field("usart0", &format_args!("{}", self.usart0().bit()))
            .field("usart1", &format_args!("{}", self.usart1().bit()))
            .field("acmp0", &format_args!("{}", self.acmp0().bit()))
            .field("acmp1", &format_args!("{}", self.acmp1().bit()))
            .field("cryotimer", &format_args!("{}", self.cryotimer().bit()))
            .field("i2c0", &format_args!("{}", self.i2c0().bit()))
            .field("adc0", &format_args!("{}", self.adc0().bit()))
            .field("idac0", &format_args!("{}", self.idac0().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HFPERCLKEN0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer0(&mut self) -> TIMER0_W<HFPERCLKEN0_SPEC, 0> {
        TIMER0_W::new(self)
    }
    #[doc = "Bit 1 - Timer 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer1(&mut self) -> TIMER1_W<HFPERCLKEN0_SPEC, 1> {
        TIMER1_W::new(self)
    }
    #[doc = "Bit 2 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> USART0_W<HFPERCLKEN0_SPEC, 2> {
        USART0_W::new(self)
    }
    #[doc = "Bit 3 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<HFPERCLKEN0_SPEC, 3> {
        USART1_W::new(self)
    }
    #[doc = "Bit 4 - Analog Comparator 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0(&mut self) -> ACMP0_W<HFPERCLKEN0_SPEC, 4> {
        ACMP0_W::new(self)
    }
    #[doc = "Bit 5 - Analog Comparator 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1(&mut self) -> ACMP1_W<HFPERCLKEN0_SPEC, 5> {
        ACMP1_W::new(self)
    }
    #[doc = "Bit 6 - CRYOTIMER Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cryotimer(&mut self) -> CRYOTIMER_W<HFPERCLKEN0_SPEC, 6> {
        CRYOTIMER_W::new(self)
    }
    #[doc = "Bit 7 - I2C 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<HFPERCLKEN0_SPEC, 7> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 8 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> ADC0_W<HFPERCLKEN0_SPEC, 8> {
        ADC0_W::new(self)
    }
    #[doc = "Bit 9 - Current Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn idac0(&mut self) -> IDAC0_W<HFPERCLKEN0_SPEC, 9> {
        IDAC0_W::new(self)
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
#[doc = "High Frequency Peripheral Clock Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfperclken0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfperclken0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFPERCLKEN0_SPEC;
impl crate::RegisterSpec for HFPERCLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfperclken0::R`](R) reader structure"]
impl crate::Readable for HFPERCLKEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfperclken0::W`](W) writer structure"]
impl crate::Writable for HFPERCLKEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFPERCLKEN0 to value 0"]
impl crate::Resettable for HFPERCLKEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
