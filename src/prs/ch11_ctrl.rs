#[doc = "Register `CH11_CTRL` reader"]
pub type R = crate::R<CH11_CTRL_SPEC>;
#[doc = "Register `CH11_CTRL` writer"]
pub type W = crate::W<CH11_CTRL_SPEC>;
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SIGSEL_R = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SIGSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SOURCESEL_R = crate::FieldReader<SOURCESEL_A>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCESEL_A {
    #[doc = "0: No source selected"]
    NONE = 0,
    #[doc = "1: Peripheral Reflex System"]
    PRSL = 1,
    #[doc = "2: Peripheral Reflex System"]
    PRSH = 2,
    #[doc = "6: Analog Comparator 0"]
    ACMP0 = 6,
    #[doc = "7: Analog Comparator 1"]
    ACMP1 = 7,
    #[doc = "8: Analog to Digital Converter 0"]
    ADC0 = 8,
    #[doc = "16: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0 = 16,
    #[doc = "17: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1 = 17,
    #[doc = "28: Timer 0"]
    TIMER0 = 28,
    #[doc = "29: Timer 1"]
    TIMER1 = 29,
    #[doc = "41: Real-Time Counter and Calendar"]
    RTCC = 41,
    #[doc = "48: General purpose Input/Output"]
    GPIOL = 48,
    #[doc = "49: General purpose Input/Output"]
    GPIOH = 49,
    #[doc = "52: Low Energy Timer 0"]
    LETIMER0 = 52,
    #[doc = "54: Pulse Counter 0"]
    PCNT0 = 54,
    #[doc = "60: CRYOTIMER"]
    CRYOTIMER = 60,
    #[doc = "61: Clock Management Unit"]
    CMU = 61,
    #[doc = "67: `1000011`"]
    CM4 = 67,
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
            1 => Some(SOURCESEL_A::PRSL),
            2 => Some(SOURCESEL_A::PRSH),
            6 => Some(SOURCESEL_A::ACMP0),
            7 => Some(SOURCESEL_A::ACMP1),
            8 => Some(SOURCESEL_A::ADC0),
            16 => Some(SOURCESEL_A::USART0),
            17 => Some(SOURCESEL_A::USART1),
            28 => Some(SOURCESEL_A::TIMER0),
            29 => Some(SOURCESEL_A::TIMER1),
            41 => Some(SOURCESEL_A::RTCC),
            48 => Some(SOURCESEL_A::GPIOL),
            49 => Some(SOURCESEL_A::GPIOH),
            52 => Some(SOURCESEL_A::LETIMER0),
            54 => Some(SOURCESEL_A::PCNT0),
            60 => Some(SOURCESEL_A::CRYOTIMER),
            61 => Some(SOURCESEL_A::CMU),
            67 => Some(SOURCESEL_A::CM4),
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
    pub fn is_prsl(&self) -> bool {
        *self == SOURCESEL_A::PRSL
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn is_prsh(&self) -> bool {
        *self == SOURCESEL_A::PRSH
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == SOURCESEL_A::ACMP0
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == SOURCESEL_A::ACMP1
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
    #[doc = "Real-Time Counter and Calendar"]
    #[inline(always)]
    pub fn is_rtcc(&self) -> bool {
        *self == SOURCESEL_A::RTCC
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn is_gpiol(&self) -> bool {
        *self == SOURCESEL_A::GPIOL
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn is_gpioh(&self) -> bool {
        *self == SOURCESEL_A::GPIOH
    }
    #[doc = "Low Energy Timer 0"]
    #[inline(always)]
    pub fn is_letimer0(&self) -> bool {
        *self == SOURCESEL_A::LETIMER0
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == SOURCESEL_A::PCNT0
    }
    #[doc = "CRYOTIMER"]
    #[inline(always)]
    pub fn is_cryotimer(&self) -> bool {
        *self == SOURCESEL_A::CRYOTIMER
    }
    #[doc = "Clock Management Unit"]
    #[inline(always)]
    pub fn is_cmu(&self) -> bool {
        *self == SOURCESEL_A::CMU
    }
    #[doc = "`1000011`"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == SOURCESEL_A::CM4
    }
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SOURCESEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O, SOURCESEL_A>;
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
    pub fn prsl(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::PRSL)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prsh(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::PRSH)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::ACMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::ACMP1)
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
    #[doc = "Real-Time Counter and Calendar"]
    #[inline(always)]
    pub fn rtcc(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::RTCC)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpiol(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::GPIOL)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpioh(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::GPIOH)
    }
    #[doc = "Low Energy Timer 0"]
    #[inline(always)]
    pub fn letimer0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::LETIMER0)
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn pcnt0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::PCNT0)
    }
    #[doc = "CRYOTIMER"]
    #[inline(always)]
    pub fn cryotimer(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::CRYOTIMER)
    }
    #[doc = "Clock Management Unit"]
    #[inline(always)]
    pub fn cmu(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::CMU)
    }
    #[doc = "`1000011`"]
    #[inline(always)]
    pub fn cm4(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::CM4)
    }
}
#[doc = "Field `EDSEL` reader - Edge Detect Select"]
pub type EDSEL_R = crate::FieldReader<EDSEL_A>;
#[doc = "Edge Detect Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDSEL_A {
    #[doc = "0: Signal is left as it is"]
    OFF = 0,
    #[doc = "1: A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    POSEDGE = 1,
    #[doc = "2: A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    NEGEDGE = 2,
    #[doc = "3: A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    BOTHEDGES = 3,
}
impl From<EDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EDSEL_A {
    type Ux = u8;
}
impl EDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EDSEL_A {
        match self.bits {
            0 => EDSEL_A::OFF,
            1 => EDSEL_A::POSEDGE,
            2 => EDSEL_A::NEGEDGE,
            3 => EDSEL_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EDSEL_A::OFF
    }
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == EDSEL_A::POSEDGE
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == EDSEL_A::NEGEDGE
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == EDSEL_A::BOTHEDGES
    }
}
#[doc = "Field `EDSEL` writer - Edge Detect Select"]
pub type EDSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, EDSEL_A>;
impl<'a, REG, const O: u8> EDSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL_A::OFF)
    }
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL_A::POSEDGE)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL_A::NEGEDGE)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL_A::BOTHEDGES)
    }
}
#[doc = "Field `STRETCH` reader - Stretch Channel Output"]
pub type STRETCH_R = crate::BitReader;
#[doc = "Field `STRETCH` writer - Stretch Channel Output"]
pub type STRETCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INV` reader - Invert Channel"]
pub type INV_R = crate::BitReader;
#[doc = "Field `INV` writer - Invert Channel"]
pub type INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ORPREV` reader - Or Previous"]
pub type ORPREV_R = crate::BitReader;
#[doc = "Field `ORPREV` writer - Or Previous"]
pub type ORPREV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ANDNEXT` reader - And Next"]
pub type ANDNEXT_R = crate::BitReader;
#[doc = "Field `ANDNEXT` writer - And Next"]
pub type ANDNEXT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ASYNCREFL` reader - Asynchronous Reflex"]
pub type ASYNCREFL_R = crate::BitReader;
#[doc = "Field `ASYNCREFL` writer - Asynchronous Reflex"]
pub type ASYNCREFL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SOURCESEL_R {
        SOURCESEL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&self) -> EDSEL_R {
        EDSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline(always)]
    pub fn stretch(&self) -> STRETCH_R {
        STRETCH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline(always)]
    pub fn orprev(&self) -> ORPREV_R {
        ORPREV_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - And Next"]
    #[inline(always)]
    pub fn andnext(&self) -> ANDNEXT_R {
        ANDNEXT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline(always)]
    pub fn asyncrefl(&self) -> ASYNCREFL_R {
        ASYNCREFL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH11_CTRL")
            .field("sigsel", &format_args!("{}", self.sigsel().bits()))
            .field("sourcesel", &format_args!("{}", self.sourcesel().bits()))
            .field("edsel", &format_args!("{}", self.edsel().bits()))
            .field("stretch", &format_args!("{}", self.stretch().bit()))
            .field("inv", &format_args!("{}", self.inv().bit()))
            .field("orprev", &format_args!("{}", self.orprev().bit()))
            .field("andnext", &format_args!("{}", self.andnext().bit()))
            .field("asyncrefl", &format_args!("{}", self.asyncrefl().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CH11_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SIGSEL_W<CH11_CTRL_SPEC, 0> {
        SIGSEL_W::new(self)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SOURCESEL_W<CH11_CTRL_SPEC, 8> {
        SOURCESEL_W::new(self)
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline(always)]
    #[must_use]
    pub fn edsel(&mut self) -> EDSEL_W<CH11_CTRL_SPEC, 20> {
        EDSEL_W::new(self)
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline(always)]
    #[must_use]
    pub fn stretch(&mut self) -> STRETCH_W<CH11_CTRL_SPEC, 25> {
        STRETCH_W::new(self)
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<CH11_CTRL_SPEC, 26> {
        INV_W::new(self)
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline(always)]
    #[must_use]
    pub fn orprev(&mut self) -> ORPREV_W<CH11_CTRL_SPEC, 27> {
        ORPREV_W::new(self)
    }
    #[doc = "Bit 28 - And Next"]
    #[inline(always)]
    #[must_use]
    pub fn andnext(&mut self) -> ANDNEXT_W<CH11_CTRL_SPEC, 28> {
        ANDNEXT_W::new(self)
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline(always)]
    #[must_use]
    pub fn asyncrefl(&mut self) -> ASYNCREFL_W<CH11_CTRL_SPEC, 30> {
        ASYNCREFL_W::new(self)
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
#[doc = "Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch11_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH11_CTRL_SPEC;
impl crate::RegisterSpec for CH11_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch11_ctrl::R`](R) reader structure"]
impl crate::Readable for CH11_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch11_ctrl::W`](W) writer structure"]
impl crate::Writable for CH11_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH11_CTRL to value 0"]
impl crate::Resettable for CH11_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
