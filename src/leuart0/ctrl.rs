///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Field `AUTOTRI` reader - Automatic Transmitter Tristate
pub type AutotriR = crate::BitReader;
///Field `AUTOTRI` writer - Automatic Transmitter Tristate
pub type AutotriW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATABITS` reader - Data-Bit Mode
pub type DatabitsR = crate::BitReader;
///Field `DATABITS` writer - Data-Bit Mode
pub type DatabitsW<'a, REG> = crate::BitWriter<'a, REG>;
///Parity-Bit Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PARITY {
    ///0: Parity bits are not used
    None = 0,
    ///2: Even parity are used. Parity bits are automatically generated and checked by hardware.
    Even = 2,
    ///3: Odd parity is used. Parity bits are automatically generated and checked by hardware.
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
///Field `PARITY` reader - Parity-Bit Mode
pub type ParityR = crate::FieldReader<PARITY>;
impl ParityR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PARITY> {
        match self.bits {
            0 => Some(PARITY::None),
            2 => Some(PARITY::Even),
            3 => Some(PARITY::Odd),
            _ => None,
        }
    }
    ///Parity bits are not used
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PARITY::None
    }
    ///Even parity are used. Parity bits are automatically generated and checked by hardware.
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARITY::Even
    }
    ///Odd parity is used. Parity bits are automatically generated and checked by hardware.
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARITY::Odd
    }
}
///Field `PARITY` writer - Parity-Bit Mode
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 2, PARITY>;
impl<'a, REG> ParityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Parity bits are not used
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY::None)
    }
    ///Even parity are used. Parity bits are automatically generated and checked by hardware.
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY::Even)
    }
    ///Odd parity is used. Parity bits are automatically generated and checked by hardware.
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY::Odd)
    }
}
///Field `STOPBITS` reader - Stop-Bit Mode
pub type StopbitsR = crate::BitReader;
///Field `STOPBITS` writer - Stop-Bit Mode
pub type StopbitsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INV` reader - Invert Input and Output
pub type InvR = crate::BitReader;
///Field `INV` writer - Invert Input and Output
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRSDMA` reader - Clear RX DMA on Error
pub type ErrsdmaR = crate::BitReader;
///Field `ERRSDMA` writer - Clear RX DMA on Error
pub type ErrsdmaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOOPBK` reader - Loopback Enable
pub type LoopbkR = crate::BitReader;
///Field `LOOPBK` writer - Loopback Enable
pub type LoopbkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SFUBRX` reader - Start-Frame UnBlock RX
pub type SfubrxR = crate::BitReader;
///Field `SFUBRX` writer - Start-Frame UnBlock RX
pub type SfubrxW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPM` reader - Multi-Processor Mode
pub type MpmR = crate::BitReader;
///Field `MPM` writer - Multi-Processor Mode
pub type MpmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPAB` reader - Multi-Processor Address-Bit
pub type MpabR = crate::BitReader;
///Field `MPAB` writer - Multi-Processor Address-Bit
pub type MpabW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIT8DV` reader - Bit 8 Default Value
pub type Bit8dvR = crate::BitReader;
///Field `BIT8DV` writer - Bit 8 Default Value
pub type Bit8dvW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXDMAWU` reader - RX DMA Wakeup
pub type RxdmawuR = crate::BitReader;
///Field `RXDMAWU` writer - RX DMA Wakeup
pub type RxdmawuW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXDMAWU` reader - TX DMA Wakeup
pub type TxdmawuR = crate::BitReader;
///Field `TXDMAWU` writer - TX DMA Wakeup
pub type TxdmawuW<'a, REG> = crate::BitWriter<'a, REG>;
///TX Delay Transmission
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXDELAY {
    ///0: Frames are transmitted immediately
    None = 0,
    ///1: Transmission of new frames are delayed by a single bit period
    Single = 1,
    ///2: Transmission of new frames are delayed by two bit periods
    Double = 2,
    ///3: Transmission of new frames are delayed by three bit periods
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
///Field `TXDELAY` reader - TX Delay Transmission
pub type TxdelayR = crate::FieldReader<TXDELAY>;
impl TxdelayR {
    ///Get enumerated values variant
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
    ///Frames are transmitted immediately
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TXDELAY::None
    }
    ///Transmission of new frames are delayed by a single bit period
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TXDELAY::Single
    }
    ///Transmission of new frames are delayed by two bit periods
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == TXDELAY::Double
    }
    ///Transmission of new frames are delayed by three bit periods
    #[inline(always)]
    pub fn is_triple(&self) -> bool {
        *self == TXDELAY::Triple
    }
}
///Field `TXDELAY` writer - TX Delay Transmission
pub type TxdelayW<'a, REG> = crate::FieldWriter<'a, REG, 2, TXDELAY, crate::Safe>;
impl<'a, REG> TxdelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Frames are transmitted immediately
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::None)
    }
    ///Transmission of new frames are delayed by a single bit period
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Single)
    }
    ///Transmission of new frames are delayed by two bit periods
    #[inline(always)]
    pub fn double(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Double)
    }
    ///Transmission of new frames are delayed by three bit periods
    #[inline(always)]
    pub fn triple(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY::Triple)
    }
}
impl R {
    ///Bit 0 - Automatic Transmitter Tristate
    #[inline(always)]
    pub fn autotri(&self) -> AutotriR {
        AutotriR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data-Bit Mode
    #[inline(always)]
    pub fn databits(&self) -> DatabitsR {
        DatabitsR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Parity-Bit Mode
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - Stop-Bit Mode
    #[inline(always)]
    pub fn stopbits(&self) -> StopbitsR {
        StopbitsR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Invert Input and Output
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Clear RX DMA on Error
    #[inline(always)]
    pub fn errsdma(&self) -> ErrsdmaR {
        ErrsdmaR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Loopback Enable
    #[inline(always)]
    pub fn loopbk(&self) -> LoopbkR {
        LoopbkR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Start-Frame UnBlock RX
    #[inline(always)]
    pub fn sfubrx(&self) -> SfubrxR {
        SfubrxR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Multi-Processor Mode
    #[inline(always)]
    pub fn mpm(&self) -> MpmR {
        MpmR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Multi-Processor Address-Bit
    #[inline(always)]
    pub fn mpab(&self) -> MpabR {
        MpabR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Bit 8 Default Value
    #[inline(always)]
    pub fn bit8dv(&self) -> Bit8dvR {
        Bit8dvR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RX DMA Wakeup
    #[inline(always)]
    pub fn rxdmawu(&self) -> RxdmawuR {
        RxdmawuR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TX DMA Wakeup
    #[inline(always)]
    pub fn txdmawu(&self) -> TxdmawuR {
        TxdmawuR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - TX Delay Transmission
    #[inline(always)]
    pub fn txdelay(&self) -> TxdelayR {
        TxdelayR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("autotri", &self.autotri())
            .field("databits", &self.databits())
            .field("parity", &self.parity())
            .field("stopbits", &self.stopbits())
            .field("inv", &self.inv())
            .field("errsdma", &self.errsdma())
            .field("loopbk", &self.loopbk())
            .field("sfubrx", &self.sfubrx())
            .field("mpm", &self.mpm())
            .field("mpab", &self.mpab())
            .field("bit8dv", &self.bit8dv())
            .field("rxdmawu", &self.rxdmawu())
            .field("txdmawu", &self.txdmawu())
            .field("txdelay", &self.txdelay())
            .finish()
    }
}
impl W {
    ///Bit 0 - Automatic Transmitter Tristate
    #[inline(always)]
    #[must_use]
    pub fn autotri(&mut self) -> AutotriW<CTRLrs> {
        AutotriW::new(self, 0)
    }
    ///Bit 1 - Data-Bit Mode
    #[inline(always)]
    #[must_use]
    pub fn databits(&mut self) -> DatabitsW<CTRLrs> {
        DatabitsW::new(self, 1)
    }
    ///Bits 2:3 - Parity-Bit Mode
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<CTRLrs> {
        ParityW::new(self, 2)
    }
    ///Bit 4 - Stop-Bit Mode
    #[inline(always)]
    #[must_use]
    pub fn stopbits(&mut self) -> StopbitsW<CTRLrs> {
        StopbitsW::new(self, 4)
    }
    ///Bit 5 - Invert Input and Output
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> InvW<CTRLrs> {
        InvW::new(self, 5)
    }
    ///Bit 6 - Clear RX DMA on Error
    #[inline(always)]
    #[must_use]
    pub fn errsdma(&mut self) -> ErrsdmaW<CTRLrs> {
        ErrsdmaW::new(self, 6)
    }
    ///Bit 7 - Loopback Enable
    #[inline(always)]
    #[must_use]
    pub fn loopbk(&mut self) -> LoopbkW<CTRLrs> {
        LoopbkW::new(self, 7)
    }
    ///Bit 8 - Start-Frame UnBlock RX
    #[inline(always)]
    #[must_use]
    pub fn sfubrx(&mut self) -> SfubrxW<CTRLrs> {
        SfubrxW::new(self, 8)
    }
    ///Bit 9 - Multi-Processor Mode
    #[inline(always)]
    #[must_use]
    pub fn mpm(&mut self) -> MpmW<CTRLrs> {
        MpmW::new(self, 9)
    }
    ///Bit 10 - Multi-Processor Address-Bit
    #[inline(always)]
    #[must_use]
    pub fn mpab(&mut self) -> MpabW<CTRLrs> {
        MpabW::new(self, 10)
    }
    ///Bit 11 - Bit 8 Default Value
    #[inline(always)]
    #[must_use]
    pub fn bit8dv(&mut self) -> Bit8dvW<CTRLrs> {
        Bit8dvW::new(self, 11)
    }
    ///Bit 12 - RX DMA Wakeup
    #[inline(always)]
    #[must_use]
    pub fn rxdmawu(&mut self) -> RxdmawuW<CTRLrs> {
        RxdmawuW::new(self, 12)
    }
    ///Bit 13 - TX DMA Wakeup
    #[inline(always)]
    #[must_use]
    pub fn txdmawu(&mut self) -> TxdmawuW<CTRLrs> {
        TxdmawuW::new(self, 13)
    }
    ///Bits 14:15 - TX Delay Transmission
    #[inline(always)]
    #[must_use]
    pub fn txdelay(&mut self) -> TxdelayW<CTRLrs> {
        TxdelayW::new(self, 14)
    }
}
///Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CTRLrs {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL to value 0
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
