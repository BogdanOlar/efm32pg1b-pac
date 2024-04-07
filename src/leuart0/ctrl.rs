#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Field `AUTOTRI` reader - Automatic Transmitter Tristate"]
pub type AutotriR = crate::BitReader;
#[doc = "Field `AUTOTRI` writer - Automatic Transmitter Tristate"]
pub type AutotriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATABITS` reader - Data-Bit Mode"]
pub type DatabitsR = crate::BitReader;
#[doc = "Field `DATABITS` writer - Data-Bit Mode"]
pub type DatabitsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Parity-Bit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PARITY {
    #[doc = "0: Parity bits are not used"]
    None = 0,
    #[doc = "2: Even parity are used. Parity bits are automatically generated and checked by hardware."]
    Even = 2,
    #[doc = "3: Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    Odd = 3,
}
impl From<PARITY> for u8 {
    #[inline(always)]
    fn from(variant: PARITY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PARITY {
    type Ux = u8;
}
impl crate::IsEnum for PARITY {}
#[doc = "Field `PARITY` reader - Parity-Bit Mode"]
pub type ParityR = crate::FieldReader<PARITY>;
impl ParityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PARITY> {
        match self.bits {
            0 => Some(PARITY::None),
            2 => Some(PARITY::Even),
            3 => Some(PARITY::Odd),
            _ => None,
        }
    }
    #[doc = "Parity bits are not used"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PARITY::None
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARITY::Even
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARITY::Odd
    }
}
#[doc = "Field `PARITY` writer - Parity-Bit Mode"]
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 2, PARITY>;
impl<'a, REG> ParityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Parity bits are not used"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY::None)
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY::Even)
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY::Odd)
    }
}
#[doc = "Field `STOPBITS` reader - Stop-Bit Mode"]
pub type StopbitsR = crate::BitReader;
#[doc = "Field `STOPBITS` writer - Stop-Bit Mode"]
pub type StopbitsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INV` reader - Invert Input and Output"]
pub type InvR = crate::BitReader;
#[doc = "Field `INV` writer - Invert Input and Output"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSDMA` reader - Clear RX DMA on Error"]
pub type ErrsdmaR = crate::BitReader;
#[doc = "Field `ERRSDMA` writer - Clear RX DMA on Error"]
pub type ErrsdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBK` reader - Loopback Enable"]
pub type LoopbkR = crate::BitReader;
#[doc = "Field `LOOPBK` writer - Loopback Enable"]
pub type LoopbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFUBRX` reader - Start-Frame UnBlock RX"]
pub type SfubrxR = crate::BitReader;
#[doc = "Field `SFUBRX` writer - Start-Frame UnBlock RX"]
pub type SfubrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPM` reader - Multi-Processor Mode"]
pub type MpmR = crate::BitReader;
#[doc = "Field `MPM` writer - Multi-Processor Mode"]
pub type MpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAB` reader - Multi-Processor Address-Bit"]
pub type MpabR = crate::BitReader;
#[doc = "Field `MPAB` writer - Multi-Processor Address-Bit"]
pub type MpabW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT8DV` reader - Bit 8 Default Value"]
pub type Bit8dvR = crate::BitReader;
#[doc = "Field `BIT8DV` writer - Bit 8 Default Value"]
pub type Bit8dvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAWU` reader - RX DMA Wakeup"]
pub type RxdmawuR = crate::BitReader;
#[doc = "Field `RXDMAWU` writer - RX DMA Wakeup"]
pub type RxdmawuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAWU` reader - TX DMA Wakeup"]
pub type TxdmawuR = crate::BitReader;
#[doc = "Field `TXDMAWU` writer - TX DMA Wakeup"]
pub type TxdmawuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TX Delay Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXDELAY {
    #[doc = "0: Frames are transmitted immediately"]
    None = 0,
    #[doc = "1: Transmission of new frames are delayed by a single bit period"]
    Single = 1,
    #[doc = "2: Transmission of new frames are delayed by two bit periods"]
    Double = 2,
    #[doc = "3: Transmission of new frames are delayed by three bit periods"]
    Triple = 3,
}
impl From<TXDELAY> for u8 {
    #[inline(always)]
    fn from(variant: TXDELAY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXDELAY {
    type Ux = u8;
}
impl crate::IsEnum for TXDELAY {}
#[doc = "Field `TXDELAY` reader - TX Delay Transmission"]
pub type TxdelayR = crate::FieldReader<TXDELAY>;
impl TxdelayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXDELAY {
        match self.bits {
            0 => TXDELAY::None,
            1 => TXDELAY::Single,
            2 => TXDELAY::Double,
            3 => TXDELAY::Triple,
            _ => unreachable!(),
        }
    }
    #[doc = "Frames are transmitted immediately"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TXDELAY::None
    }
    #[doc = "Transmission of new frames are delayed by a single bit period"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TXDELAY::Single
    }
    #[doc = "Transmission of new frames are delayed by two bit periods"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == TXDELAY::Double
    }
    #[doc = "Transmission of new frames are delayed by three bit periods"]
    #[inline(always)]
    pub fn is_triple(&self) -> bool {
        *self == TXDELAY::Triple
    }
}
#[doc = "Field `TXDELAY` writer - TX Delay Transmission"]
pub type TxdelayW<'a, REG> = crate::FieldWriter<'a, REG, 2, TXDELAY, crate::Safe>;
impl<'a, REG> TxdelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frames are transmitted immediately"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::None)
    }
    #[doc = "Transmission of new frames are delayed by a single bit period"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Single)
    }
    #[doc = "Transmission of new frames are delayed by two bit periods"]
    #[inline(always)]
    pub fn double(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Double)
    }
    #[doc = "Transmission of new frames are delayed by three bit periods"]
    #[inline(always)]
    pub fn triple(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Triple)
    }
}
impl R {
    #[doc = "Bit 0 - Automatic Transmitter Tristate"]
    #[inline(always)]
    pub fn autotri(&self) -> AutotriR {
        AutotriR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&self) -> DatabitsR {
        DatabitsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&self) -> StopbitsR {
        StopbitsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Invert Input and Output"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear RX DMA on Error"]
    #[inline(always)]
    pub fn errsdma(&self) -> ErrsdmaR {
        ErrsdmaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&self) -> LoopbkR {
        LoopbkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start-Frame UnBlock RX"]
    #[inline(always)]
    pub fn sfubrx(&self) -> SfubrxR {
        SfubrxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&self) -> MpmR {
        MpmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&self) -> MpabR {
        MpabR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&self) -> Bit8dvR {
        Bit8dvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RX DMA Wakeup"]
    #[inline(always)]
    pub fn rxdmawu(&self) -> RxdmawuR {
        RxdmawuR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX DMA Wakeup"]
    #[inline(always)]
    pub fn txdmawu(&self) -> TxdmawuR {
        TxdmawuR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&self) -> TxdelayR {
        TxdelayR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Automatic Transmitter Tristate"]
    #[inline(always)]
    #[must_use]
    pub fn autotri(&mut self) -> AutotriW<CTRLrs> {
        AutotriW::new(self, 0)
    }
    #[doc = "Bit 1 - Data-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn databits(&mut self) -> DatabitsW<CTRLrs> {
        DatabitsW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Parity-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<CTRLrs> {
        ParityW::new(self, 2)
    }
    #[doc = "Bit 4 - Stop-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn stopbits(&mut self) -> StopbitsW<CTRLrs> {
        StopbitsW::new(self, 4)
    }
    #[doc = "Bit 5 - Invert Input and Output"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> InvW<CTRLrs> {
        InvW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear RX DMA on Error"]
    #[inline(always)]
    #[must_use]
    pub fn errsdma(&mut self) -> ErrsdmaW<CTRLrs> {
        ErrsdmaW::new(self, 6)
    }
    #[doc = "Bit 7 - Loopback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn loopbk(&mut self) -> LoopbkW<CTRLrs> {
        LoopbkW::new(self, 7)
    }
    #[doc = "Bit 8 - Start-Frame UnBlock RX"]
    #[inline(always)]
    #[must_use]
    pub fn sfubrx(&mut self) -> SfubrxW<CTRLrs> {
        SfubrxW::new(self, 8)
    }
    #[doc = "Bit 9 - Multi-Processor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mpm(&mut self) -> MpmW<CTRLrs> {
        MpmW::new(self, 9)
    }
    #[doc = "Bit 10 - Multi-Processor Address-Bit"]
    #[inline(always)]
    #[must_use]
    pub fn mpab(&mut self) -> MpabW<CTRLrs> {
        MpabW::new(self, 10)
    }
    #[doc = "Bit 11 - Bit 8 Default Value"]
    #[inline(always)]
    #[must_use]
    pub fn bit8dv(&mut self) -> Bit8dvW<CTRLrs> {
        Bit8dvW::new(self, 11)
    }
    #[doc = "Bit 12 - RX DMA Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmawu(&mut self) -> RxdmawuW<CTRLrs> {
        RxdmawuW::new(self, 12)
    }
    #[doc = "Bit 13 - TX DMA Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn txdmawu(&mut self) -> TxdmawuW<CTRLrs> {
        TxdmawuW::new(self, 13)
    }
    #[doc = "Bits 14:15 - TX Delay Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txdelay(&mut self) -> TxdelayW<CTRLrs> {
        TxdelayW::new(self, 14)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
