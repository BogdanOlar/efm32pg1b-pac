///Register `CH1_CTRL` reader
pub type R = crate::R<CH1_CTRLrs>;
///Register `CH1_CTRL` writer
pub type W = crate::W<CH1_CTRLrs>;
///Field `SIGSEL` reader - Signal Select
pub type SigselR = crate::FieldReader;
///Field `SIGSEL` writer - Signal Select
pub type SigselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Source Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCESEL {
    ///0: No source selected
    None = 0,
    ///1: Peripheral Reflex System
    Prsl = 1,
    ///2: Peripheral Reflex System
    Prsh = 2,
    ///6: Analog Comparator 0
    Acmp0 = 6,
    ///7: Analog Comparator 1
    Acmp1 = 7,
    ///8: Analog to Digital Converter 0
    Adc0 = 8,
    ///16: Universal Synchronous/Asynchronous Receiver/Transmitter 0
    Usart0 = 16,
    ///17: Universal Synchronous/Asynchronous Receiver/Transmitter 1
    Usart1 = 17,
    ///28: Timer 0
    Timer0 = 28,
    ///29: Timer 1
    Timer1 = 29,
    ///41: Real-Time Counter and Calendar
    Rtcc = 41,
    ///48: General purpose Input/Output
    Gpiol = 48,
    ///49: General purpose Input/Output
    Gpioh = 49,
    ///52: Low Energy Timer 0
    Letimer0 = 52,
    ///54: Pulse Counter 0
    Pcnt0 = 54,
    ///60: CRYOTIMER
    Cryotimer = 60,
    ///61: Clock Management Unit
    Cmu = 61,
    ///67: `1000011`
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
///Field `SOURCESEL` reader - Source Select
pub type SourceselR = crate::FieldReader<SOURCESEL>;
impl SourceselR {
    ///Get enumerated values variant
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
    ///No source selected
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SOURCESEL::None
    }
    ///Peripheral Reflex System
    #[inline(always)]
    pub fn is_prsl(&self) -> bool {
        *self == SOURCESEL::Prsl
    }
    ///Peripheral Reflex System
    #[inline(always)]
    pub fn is_prsh(&self) -> bool {
        *self == SOURCESEL::Prsh
    }
    ///Analog Comparator 0
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == SOURCESEL::Acmp0
    }
    ///Analog Comparator 1
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == SOURCESEL::Acmp1
    }
    ///Analog to Digital Converter 0
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESEL::Adc0
    }
    ///Universal Synchronous/Asynchronous Receiver/Transmitter 0
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == SOURCESEL::Usart0
    }
    ///Universal Synchronous/Asynchronous Receiver/Transmitter 1
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == SOURCESEL::Usart1
    }
    ///Timer 0
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == SOURCESEL::Timer0
    }
    ///Timer 1
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == SOURCESEL::Timer1
    }
    ///Real-Time Counter and Calendar
    #[inline(always)]
    pub fn is_rtcc(&self) -> bool {
        *self == SOURCESEL::Rtcc
    }
    ///General purpose Input/Output
    #[inline(always)]
    pub fn is_gpiol(&self) -> bool {
        *self == SOURCESEL::Gpiol
    }
    ///General purpose Input/Output
    #[inline(always)]
    pub fn is_gpioh(&self) -> bool {
        *self == SOURCESEL::Gpioh
    }
    ///Low Energy Timer 0
    #[inline(always)]
    pub fn is_letimer0(&self) -> bool {
        *self == SOURCESEL::Letimer0
    }
    ///Pulse Counter 0
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == SOURCESEL::Pcnt0
    }
    ///CRYOTIMER
    #[inline(always)]
    pub fn is_cryotimer(&self) -> bool {
        *self == SOURCESEL::Cryotimer
    }
    ///Clock Management Unit
    #[inline(always)]
    pub fn is_cmu(&self) -> bool {
        *self == SOURCESEL::Cmu
    }
    ///`1000011`
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == SOURCESEL::Cm4
    }
}
///Field `SOURCESEL` writer - Source Select
pub type SourceselW<'a, REG> = crate::FieldWriter<'a, REG, 7, SOURCESEL>;
impl<'a, REG> SourceselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No source selected
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::None)
    }
    ///Peripheral Reflex System
    #[inline(always)]
    pub fn prsl(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Prsl)
    }
    ///Peripheral Reflex System
    #[inline(always)]
    pub fn prsh(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Prsh)
    }
    ///Analog Comparator 0
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Acmp0)
    }
    ///Analog Comparator 1
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Acmp1)
    }
    ///Analog to Digital Converter 0
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Adc0)
    }
    ///Universal Synchronous/Asynchronous Receiver/Transmitter 0
    #[inline(always)]
    pub fn usart0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Usart0)
    }
    ///Universal Synchronous/Asynchronous Receiver/Transmitter 1
    #[inline(always)]
    pub fn usart1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Usart1)
    }
    ///Timer 0
    #[inline(always)]
    pub fn timer0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Timer0)
    }
    ///Timer 1
    #[inline(always)]
    pub fn timer1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Timer1)
    }
    ///Real-Time Counter and Calendar
    #[inline(always)]
    pub fn rtcc(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Rtcc)
    }
    ///General purpose Input/Output
    #[inline(always)]
    pub fn gpiol(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Gpiol)
    }
    ///General purpose Input/Output
    #[inline(always)]
    pub fn gpioh(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Gpioh)
    }
    ///Low Energy Timer 0
    #[inline(always)]
    pub fn letimer0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Letimer0)
    }
    ///Pulse Counter 0
    #[inline(always)]
    pub fn pcnt0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Pcnt0)
    }
    ///CRYOTIMER
    #[inline(always)]
    pub fn cryotimer(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Cryotimer)
    }
    ///Clock Management Unit
    #[inline(always)]
    pub fn cmu(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Cmu)
    }
    ///`1000011`
    #[inline(always)]
    pub fn cm4(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL::Cm4)
    }
}
///Edge Detect Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDSEL {
    ///0: Signal is left as it is
    Off = 0,
    ///1: A one HFCLK cycle pulse is generated for every positive edge of the incoming signal
    Posedge = 1,
    ///2: A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal
    Negedge = 2,
    ///3: A one HFCLK clock cycle pulse is generated for every edge of the incoming signal
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
///Field `EDSEL` reader - Edge Detect Select
pub type EdselR = crate::FieldReader<EDSEL>;
impl EdselR {
    ///Get enumerated values variant
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
    ///Signal is left as it is
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EDSEL::Off
    }
    ///A one HFCLK cycle pulse is generated for every positive edge of the incoming signal
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == EDSEL::Posedge
    }
    ///A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == EDSEL::Negedge
    }
    ///A one HFCLK clock cycle pulse is generated for every edge of the incoming signal
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == EDSEL::Bothedges
    }
}
///Field `EDSEL` writer - Edge Detect Select
pub type EdselW<'a, REG> = crate::FieldWriter<'a, REG, 2, EDSEL, crate::Safe>;
impl<'a, REG> EdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Signal is left as it is
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL::Off)
    }
    ///A one HFCLK cycle pulse is generated for every positive edge of the incoming signal
    #[inline(always)]
    pub fn posedge(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL::Posedge)
    }
    ///A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal
    #[inline(always)]
    pub fn negedge(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL::Negedge)
    }
    ///A one HFCLK clock cycle pulse is generated for every edge of the incoming signal
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL::Bothedges)
    }
}
///Field `STRETCH` reader - Stretch Channel Output
pub type StretchR = crate::BitReader;
///Field `STRETCH` writer - Stretch Channel Output
pub type StretchW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INV` reader - Invert Channel
pub type InvR = crate::BitReader;
///Field `INV` writer - Invert Channel
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ORPREV` reader - Or Previous
pub type OrprevR = crate::BitReader;
///Field `ORPREV` writer - Or Previous
pub type OrprevW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANDNEXT` reader - And Next
pub type AndnextR = crate::BitReader;
///Field `ANDNEXT` writer - And Next
pub type AndnextW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASYNCREFL` reader - Asynchronous Reflex
pub type AsyncreflR = crate::BitReader;
///Field `ASYNCREFL` writer - Asynchronous Reflex
pub type AsyncreflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Signal Select
    #[inline(always)]
    pub fn sigsel(&self) -> SigselR {
        SigselR::new((self.bits & 7) as u8)
    }
    ///Bits 8:14 - Source Select
    #[inline(always)]
    pub fn sourcesel(&self) -> SourceselR {
        SourceselR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 20:21 - Edge Detect Select
    #[inline(always)]
    pub fn edsel(&self) -> EdselR {
        EdselR::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 25 - Stretch Channel Output
    #[inline(always)]
    pub fn stretch(&self) -> StretchR {
        StretchR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Invert Channel
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Or Previous
    #[inline(always)]
    pub fn orprev(&self) -> OrprevR {
        OrprevR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - And Next
    #[inline(always)]
    pub fn andnext(&self) -> AndnextR {
        AndnextR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Asynchronous Reflex
    #[inline(always)]
    pub fn asyncrefl(&self) -> AsyncreflR {
        AsyncreflR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH1_CTRL")
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
    ///Bits 0:2 - Signal Select
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SigselW<CH1_CTRLrs> {
        SigselW::new(self, 0)
    }
    ///Bits 8:14 - Source Select
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SourceselW<CH1_CTRLrs> {
        SourceselW::new(self, 8)
    }
    ///Bits 20:21 - Edge Detect Select
    #[inline(always)]
    #[must_use]
    pub fn edsel(&mut self) -> EdselW<CH1_CTRLrs> {
        EdselW::new(self, 20)
    }
    ///Bit 25 - Stretch Channel Output
    #[inline(always)]
    #[must_use]
    pub fn stretch(&mut self) -> StretchW<CH1_CTRLrs> {
        StretchW::new(self, 25)
    }
    ///Bit 26 - Invert Channel
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> InvW<CH1_CTRLrs> {
        InvW::new(self, 26)
    }
    ///Bit 27 - Or Previous
    #[inline(always)]
    #[must_use]
    pub fn orprev(&mut self) -> OrprevW<CH1_CTRLrs> {
        OrprevW::new(self, 27)
    }
    ///Bit 28 - And Next
    #[inline(always)]
    #[must_use]
    pub fn andnext(&mut self) -> AndnextW<CH1_CTRLrs> {
        AndnextW::new(self, 28)
    }
    ///Bit 30 - Asynchronous Reflex
    #[inline(always)]
    #[must_use]
    pub fn asyncrefl(&mut self) -> AsyncreflW<CH1_CTRLrs> {
        AsyncreflW::new(self, 30)
    }
}
///Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CH1_CTRLrs;
impl crate::RegisterSpec for CH1_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ch1_ctrl::R`](R) reader structure
impl crate::Readable for CH1_CTRLrs {}
///`write(|w| ..)` method takes [`ch1_ctrl::W`](W) writer structure
impl crate::Writable for CH1_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH1_CTRL to value 0
impl crate::Resettable for CH1_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
