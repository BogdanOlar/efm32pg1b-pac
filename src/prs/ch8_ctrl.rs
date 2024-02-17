#[doc = "Register `CH8_CTRL` reader"]
pub type R = crate::R<CH8_CTRLrs>;
#[doc = "Register `CH8_CTRL` writer"]
pub type W = crate::W<CH8_CTRLrs>;
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SIGSEL_R = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SOURCESEL_R = crate::FieldReader<SOURCESEL>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCESEL {
    #[doc = "0: No source selected"]
    None = 0,
    #[doc = "1: Peripheral Reflex System"]
    Prsl = 1,
    #[doc = "2: Peripheral Reflex System"]
    Prsh = 2,
    #[doc = "6: Analog Comparator 0"]
    Acmp0 = 6,
    #[doc = "7: Analog Comparator 1"]
    Acmp1 = 7,
    #[doc = "8: Analog to Digital Converter 0"]
    Adc0 = 8,
    #[doc = "16: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    Usart0 = 16,
    #[doc = "17: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    Usart1 = 17,
    #[doc = "28: Timer 0"]
    Timer0 = 28,
    #[doc = "29: Timer 1"]
    Timer1 = 29,
    #[doc = "41: Real-Time Counter and Calendar"]
    Rtcc = 41,
    #[doc = "48: General purpose Input/Output"]
    Gpiol = 48,
    #[doc = "49: General purpose Input/Output"]
    Gpioh = 49,
    #[doc = "52: Low Energy Timer 0"]
    Letimer0 = 52,
    #[doc = "54: Pulse Counter 0"]
    Pcnt0 = 54,
    #[doc = "60: CRYOTIMER"]
    Cryotimer = 60,
    #[doc = "61: Clock Management Unit"]
    Cmu = 61,
    #[doc = "67: `1000011`"]
    Cm4 = 67,
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
impl SOURCESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SOURCESEL> {
        match self.bits {
            0 => Some(SOURCESEL::None),
            1 => Some(SOURCESEL::Prsl),
            2 => Some(SOURCESEL::Prsh),
            6 => Some(SOURCESEL::Acmp0),
            7 => Some(SOURCESEL::Acmp1),
            8 => Some(SOURCESEL::Adc0),
            16 => Some(SOURCESEL::Usart0),
            17 => Some(SOURCESEL::Usart1),
            28 => Some(SOURCESEL::Timer0),
            29 => Some(SOURCESEL::Timer1),
            41 => Some(SOURCESEL::Rtcc),
            48 => Some(SOURCESEL::Gpiol),
            49 => Some(SOURCESEL::Gpioh),
            52 => Some(SOURCESEL::Letimer0),
            54 => Some(SOURCESEL::Pcnt0),
            60 => Some(SOURCESEL::Cryotimer),
            61 => Some(SOURCESEL::Cmu),
            67 => Some(SOURCESEL::Cm4),
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
    pub fn is_prsl(&self) -> bool {
        *self == SOURCESEL::Prsl
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn is_prsh(&self) -> bool {
        *self == SOURCESEL::Prsh
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == SOURCESEL::Acmp0
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == SOURCESEL::Acmp1
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
    #[doc = "Real-Time Counter and Calendar"]
    #[inline(always)]
    pub fn is_rtcc(&self) -> bool {
        *self == SOURCESEL::Rtcc
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn is_gpiol(&self) -> bool {
        *self == SOURCESEL::Gpiol
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn is_gpioh(&self) -> bool {
        *self == SOURCESEL::Gpioh
    }
    #[doc = "Low Energy Timer 0"]
    #[inline(always)]
    pub fn is_letimer0(&self) -> bool {
        *self == SOURCESEL::Letimer0
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == SOURCESEL::Pcnt0
    }
    #[doc = "CRYOTIMER"]
    #[inline(always)]
    pub fn is_cryotimer(&self) -> bool {
        *self == SOURCESEL::Cryotimer
    }
    #[doc = "Clock Management Unit"]
    #[inline(always)]
    pub fn is_cmu(&self) -> bool {
        *self == SOURCESEL::Cmu
    }
    #[doc = "`1000011`"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == SOURCESEL::Cm4
    }
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SOURCESEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7, SOURCESEL>;
impl<'a, REG> SOURCESEL_W<'a, REG>
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
    pub fn prsl(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Prsl)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prsh(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Prsh)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Acmp0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Acmp1)
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
    #[doc = "Real-Time Counter and Calendar"]
    #[inline(always)]
    pub fn rtcc(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Rtcc)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpiol(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Gpiol)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpioh(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Gpioh)
    }
    #[doc = "Low Energy Timer 0"]
    #[inline(always)]
    pub fn letimer0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Letimer0)
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn pcnt0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Pcnt0)
    }
    #[doc = "CRYOTIMER"]
    #[inline(always)]
    pub fn cryotimer(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Cryotimer)
    }
    #[doc = "Clock Management Unit"]
    #[inline(always)]
    pub fn cmu(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Cmu)
    }
    #[doc = "`1000011`"]
    #[inline(always)]
    pub fn cm4(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Cm4)
    }
}
#[doc = "Field `EDSEL` reader - Edge Detect Select"]
pub type EDSEL_R = crate::FieldReader<EDSEL>;
#[doc = "Edge Detect Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDSEL {
    #[doc = "0: Signal is left as it is"]
    Off = 0,
    #[doc = "1: A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    Posedge = 1,
    #[doc = "2: A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    Negedge = 2,
    #[doc = "3: A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    Bothedges = 3,
}
impl From<EDSEL> for u8 {
    #[inline(always)]
    fn from(variant: EDSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EDSEL {
    type Ux = u8;
}
impl EDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EDSEL {
        match self.bits {
            0 => EDSEL::Off,
            1 => EDSEL::Posedge,
            2 => EDSEL::Negedge,
            3 => EDSEL::Bothedges,
            _ => unreachable!(),
        }
    }
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EDSEL::Off
    }
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == EDSEL::Posedge
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == EDSEL::Negedge
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == EDSEL::Bothedges
    }
}
#[doc = "Field `EDSEL` writer - Edge Detect Select"]
pub type EDSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EDSEL>;
impl<'a, REG> EDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL::Off)
    }
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL::Posedge)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL::Negedge)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL::Bothedges)
    }
}
#[doc = "Field `STRETCH` reader - Stretch Channel Output"]
pub type STRETCH_R = crate::BitReader;
#[doc = "Field `STRETCH` writer - Stretch Channel Output"]
pub type STRETCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INV` reader - Invert Channel"]
pub type INV_R = crate::BitReader;
#[doc = "Field `INV` writer - Invert Channel"]
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORPREV` reader - Or Previous"]
pub type ORPREV_R = crate::BitReader;
#[doc = "Field `ORPREV` writer - Or Previous"]
pub type ORPREV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANDNEXT` reader - And Next"]
pub type ANDNEXT_R = crate::BitReader;
#[doc = "Field `ANDNEXT` writer - And Next"]
pub type ANDNEXT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCREFL` reader - Asynchronous Reflex"]
pub type ASYNCREFL_R = crate::BitReader;
#[doc = "Field `ASYNCREFL` writer - Asynchronous Reflex"]
pub type ASYNCREFL_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SIGSEL_W<CH8_CTRLrs> {
        SIGSEL_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SOURCESEL_W<CH8_CTRLrs> {
        SOURCESEL_W::new(self, 8)
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline(always)]
    #[must_use]
    pub fn edsel(&mut self) -> EDSEL_W<CH8_CTRLrs> {
        EDSEL_W::new(self, 20)
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline(always)]
    #[must_use]
    pub fn stretch(&mut self) -> STRETCH_W<CH8_CTRLrs> {
        STRETCH_W::new(self, 25)
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<CH8_CTRLrs> {
        INV_W::new(self, 26)
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline(always)]
    #[must_use]
    pub fn orprev(&mut self) -> ORPREV_W<CH8_CTRLrs> {
        ORPREV_W::new(self, 27)
    }
    #[doc = "Bit 28 - And Next"]
    #[inline(always)]
    #[must_use]
    pub fn andnext(&mut self) -> ANDNEXT_W<CH8_CTRLrs> {
        ANDNEXT_W::new(self, 28)
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline(always)]
    #[must_use]
    pub fn asyncrefl(&mut self) -> ASYNCREFL_W<CH8_CTRLrs> {
        ASYNCREFL_W::new(self, 30)
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
#[doc = "Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH8_CTRLrs;
impl crate::RegisterSpec for CH8_CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch8_ctrl::R`](R) reader structure"]
impl crate::Readable for CH8_CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ch8_ctrl::W`](W) writer structure"]
impl crate::Writable for CH8_CTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH8_CTRL to value 0"]
impl crate::Resettable for CH8_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
