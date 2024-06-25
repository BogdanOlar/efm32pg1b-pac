#[doc = "Register `CH3_CTRL` reader"]
pub type R = crate::R<CH3_CTRLrs>;
#[doc = "Register `CH3_CTRL` writer"]
pub type W = crate::W<CH3_CTRLrs>;
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SigselR = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SigselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
impl crate::IsEnum for SOURCESEL {}
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SourceselR = crate::FieldReader<SOURCESEL>;
impl SourceselR {
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
pub type SourceselW<'a, REG> = crate::FieldWriter<'a, REG, 7, SOURCESEL>;
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
#[doc = "Edge Detect Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
impl crate::IsEnum for EDSEL {}
#[doc = "Field `EDSEL` reader - Edge Detect Select"]
pub type EdselR = crate::FieldReader<EDSEL>;
impl EdselR {
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
pub type EdselW<'a, REG> = crate::FieldWriter<'a, REG, 2, EDSEL, crate::Safe>;
impl<'a, REG> EdselW<'a, REG>
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
pub type StretchR = crate::BitReader;
#[doc = "Field `STRETCH` writer - Stretch Channel Output"]
pub type StretchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INV` reader - Invert Channel"]
pub type InvR = crate::BitReader;
#[doc = "Field `INV` writer - Invert Channel"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORPREV` reader - Or Previous"]
pub type OrprevR = crate::BitReader;
#[doc = "Field `ORPREV` writer - Or Previous"]
pub type OrprevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANDNEXT` reader - And Next"]
pub type AndnextR = crate::BitReader;
#[doc = "Field `ANDNEXT` writer - And Next"]
pub type AndnextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCREFL` reader - Asynchronous Reflex"]
pub type AsyncreflR = crate::BitReader;
#[doc = "Field `ASYNCREFL` writer - Asynchronous Reflex"]
pub type AsyncreflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SigselR {
        SigselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SourceselR {
        SourceselR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&self) -> EdselR {
        EdselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline(always)]
    pub fn stretch(&self) -> StretchR {
        StretchR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline(always)]
    pub fn orprev(&self) -> OrprevR {
        OrprevR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - And Next"]
    #[inline(always)]
    pub fn andnext(&self) -> AndnextR {
        AndnextR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline(always)]
    pub fn asyncrefl(&self) -> AsyncreflR {
        AsyncreflR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH3_CTRL")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("asyncrefl", &self.asyncrefl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SigselW<CH3_CTRLrs> {
        SigselW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SourceselW<CH3_CTRLrs> {
        SourceselW::new(self, 8)
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline(always)]
    #[must_use]
    pub fn edsel(&mut self) -> EdselW<CH3_CTRLrs> {
        EdselW::new(self, 20)
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline(always)]
    #[must_use]
    pub fn stretch(&mut self) -> StretchW<CH3_CTRLrs> {
        StretchW::new(self, 25)
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> InvW<CH3_CTRLrs> {
        InvW::new(self, 26)
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline(always)]
    #[must_use]
    pub fn orprev(&mut self) -> OrprevW<CH3_CTRLrs> {
        OrprevW::new(self, 27)
    }
    #[doc = "Bit 28 - And Next"]
    #[inline(always)]
    #[must_use]
    pub fn andnext(&mut self) -> AndnextW<CH3_CTRLrs> {
        AndnextW::new(self, 28)
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline(always)]
    #[must_use]
    pub fn asyncrefl(&mut self) -> AsyncreflW<CH3_CTRLrs> {
        AsyncreflW::new(self, 30)
    }
}
#[doc = "Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH3_CTRLrs;
impl crate::RegisterSpec for CH3_CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_ctrl::R`](R) reader structure"]
impl crate::Readable for CH3_CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ch3_ctrl::W`](W) writer structure"]
impl crate::Writable for CH3_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH3_CTRL to value 0"]
impl crate::Resettable for CH3_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
