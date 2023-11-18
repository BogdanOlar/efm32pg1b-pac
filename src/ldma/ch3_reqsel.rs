#[doc = "Register `CH3_REQSEL` reader"]
pub type R = crate::R<CH3_REQSEL_SPEC>;
#[doc = "Register `CH3_REQSEL` writer"]
pub type W = crate::W<CH3_REQSEL_SPEC>;
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SIGSEL_R = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SIGSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SOURCESEL_R = crate::FieldReader<SOURCESEL_A>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCESEL_A {
    #[doc = "0: No source selected"]
    NONE = 0,
    #[doc = "1: Peripheral Reflex System"]
    PRS = 1,
    #[doc = "8: Analog to Digital Converter 0"]
    ADC0 = 8,
    #[doc = "12: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0 = 12,
    #[doc = "13: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1 = 13,
    #[doc = "16: Low Energy UART 0"]
    LEUART0 = 16,
    #[doc = "20: I2C 0"]
    I2C0 = 20,
    #[doc = "24: Timer 0"]
    TIMER0 = 24,
    #[doc = "25: Timer 1"]
    TIMER1 = 25,
    #[doc = "48: Memory System Controller"]
    MSC = 48,
    #[doc = "49: Advanced Encryption Standard Accelerator"]
    CRYPTO = 49,
}
impl From<SOURCESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCESEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SOURCESEL_A {
    type Ux = u8;
}
impl SOURCESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SOURCESEL_A> {
        match self.bits {
            0 => Some(SOURCESEL_A::NONE),
            1 => Some(SOURCESEL_A::PRS),
            8 => Some(SOURCESEL_A::ADC0),
            12 => Some(SOURCESEL_A::USART0),
            13 => Some(SOURCESEL_A::USART1),
            16 => Some(SOURCESEL_A::LEUART0),
            20 => Some(SOURCESEL_A::I2C0),
            24 => Some(SOURCESEL_A::TIMER0),
            25 => Some(SOURCESEL_A::TIMER1),
            48 => Some(SOURCESEL_A::MSC),
            49 => Some(SOURCESEL_A::CRYPTO),
            _ => None,
        }
    }
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SOURCESEL_A::NONE
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == SOURCESEL_A::PRS
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESEL_A::ADC0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == SOURCESEL_A::USART0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == SOURCESEL_A::USART1
    }
    #[doc = "Low Energy UART 0"]
    #[inline(always)]
    pub fn is_leuart0(&self) -> bool {
        *self == SOURCESEL_A::LEUART0
    }
    #[doc = "I2C 0"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == SOURCESEL_A::I2C0
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == SOURCESEL_A::TIMER0
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == SOURCESEL_A::TIMER1
    }
    #[doc = "Memory System Controller"]
    #[inline(always)]
    pub fn is_msc(&self) -> bool {
        *self == SOURCESEL_A::MSC
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline(always)]
    pub fn is_crypto(&self) -> bool {
        *self == SOURCESEL_A::CRYPTO
    }
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SOURCESEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, SOURCESEL_A>;
impl<'a, REG, const O: u8> SOURCESEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::NONE)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::PRS)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::ADC0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::USART0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::USART1)
    }
    #[doc = "Low Energy UART 0"]
    #[inline(always)]
    pub fn leuart0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::LEUART0)
    }
    #[doc = "I2C 0"]
    #[inline(always)]
    pub fn i2c0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::I2C0)
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::TIMER0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::TIMER1)
    }
    #[doc = "Memory System Controller"]
    #[inline(always)]
    pub fn msc(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::MSC)
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline(always)]
    pub fn crypto(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::CRYPTO)
    }
}
impl R {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SOURCESEL_R {
        SOURCESEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH3_REQSEL")
            .field("sigsel", &format_args!("{}", self.sigsel().bits()))
            .field("sourcesel", &format_args!("{}", self.sourcesel().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CH3_REQSEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SIGSEL_W<CH3_REQSEL_SPEC, 0> {
        SIGSEL_W::new(self)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SOURCESEL_W<CH3_REQSEL_SPEC, 16> {
        SOURCESEL_W::new(self)
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
#[doc = "Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_reqsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_reqsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH3_REQSEL_SPEC;
impl crate::RegisterSpec for CH3_REQSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_reqsel::R`](R) reader structure"]
impl crate::Readable for CH3_REQSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch3_reqsel::W`](W) writer structure"]
impl crate::Writable for CH3_REQSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3_REQSEL to value 0"]
impl crate::Resettable for CH3_REQSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
