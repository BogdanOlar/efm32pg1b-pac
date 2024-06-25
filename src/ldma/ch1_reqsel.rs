#[doc = "Register `CH1_REQSEL` reader"]
pub type R = crate::R<CH1_REQSELrs>;
#[doc = "Register `CH1_REQSEL` writer"]
pub type W = crate::W<CH1_REQSELrs>;
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SigselR = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SigselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCESEL {
    #[doc = "0: No source selected"]
    None = 0,
    #[doc = "1: Peripheral Reflex System"]
    Prs = 1,
    #[doc = "8: Analog to Digital Converter 0"]
    Adc0 = 8,
    #[doc = "12: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    Usart0 = 12,
    #[doc = "13: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    Usart1 = 13,
    #[doc = "16: Low Energy UART 0"]
    Leuart0 = 16,
    #[doc = "20: I2C 0"]
    I2c0 = 20,
    #[doc = "24: Timer 0"]
    Timer0 = 24,
    #[doc = "25: Timer 1"]
    Timer1 = 25,
    #[doc = "48: Memory System Controller"]
    Msc = 48,
    #[doc = "49: Advanced Encryption Standard Accelerator"]
    Crypto = 49,
}
impl From<SOURCESEL> for u8 {
    #[inline(always)]
    fn from(variant: SOURCESEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SOURCESEL {
    type Ux = u8;
}
impl crate::IsEnum for SOURCESEL {}
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SourceselR = crate::FieldReader<SOURCESEL>;
impl SourceselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SOURCESEL> {
        match self.bits {
            0 => Some(SOURCESEL::None),
            1 => Some(SOURCESEL::Prs),
            8 => Some(SOURCESEL::Adc0),
            12 => Some(SOURCESEL::Usart0),
            13 => Some(SOURCESEL::Usart1),
            16 => Some(SOURCESEL::Leuart0),
            20 => Some(SOURCESEL::I2c0),
            24 => Some(SOURCESEL::Timer0),
            25 => Some(SOURCESEL::Timer1),
            48 => Some(SOURCESEL::Msc),
            49 => Some(SOURCESEL::Crypto),
            _ => None,
        }
    }
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SOURCESEL::None
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == SOURCESEL::Prs
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESEL::Adc0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == SOURCESEL::Usart0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == SOURCESEL::Usart1
    }
    #[doc = "Low Energy UART 0"]
    #[inline(always)]
    pub fn is_leuart0(&self) -> bool {
        *self == SOURCESEL::Leuart0
    }
    #[doc = "I2C 0"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == SOURCESEL::I2c0
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == SOURCESEL::Timer0
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == SOURCESEL::Timer1
    }
    #[doc = "Memory System Controller"]
    #[inline(always)]
    pub fn is_msc(&self) -> bool {
        *self == SOURCESEL::Msc
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline(always)]
    pub fn is_crypto(&self) -> bool {
        *self == SOURCESEL::Crypto
    }
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SourceselW<'a, REG> = crate::FieldWriter<'a, REG, 6, SOURCESEL>;
impl<'a, REG> SourceselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::None)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Prs)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Adc0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Usart0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Usart1)
    }
    #[doc = "Low Energy UART 0"]
    #[inline(always)]
    pub fn leuart0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Leuart0)
    }
    #[doc = "I2C 0"]
    #[inline(always)]
    pub fn i2c0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::I2c0)
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Timer0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Timer1)
    }
    #[doc = "Memory System Controller"]
    #[inline(always)]
    pub fn msc(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Msc)
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline(always)]
    pub fn crypto(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Crypto)
    }
}
impl R {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SigselR {
        SigselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SourceselR {
        SourceselR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH1_REQSEL")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SigselW<CH1_REQSELrs> {
        SigselW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SourceselW<CH1_REQSELrs> {
        SourceselW::new(self, 16)
    }
}
#[doc = "Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_reqsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_reqsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH1_REQSELrs;
impl crate::RegisterSpec for CH1_REQSELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_reqsel::R`](R) reader structure"]
impl crate::Readable for CH1_REQSELrs {}
#[doc = "`write(|w| ..)` method takes [`ch1_reqsel::W`](W) writer structure"]
impl crate::Writable for CH1_REQSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1_REQSEL to value 0"]
impl crate::Resettable for CH1_REQSELrs {
    const RESET_VALUE: u32 = 0;
}
