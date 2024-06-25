#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Field `SYNC` reader - USART Synchronous Mode"]
pub type SyncR = crate::BitReader;
#[doc = "Field `SYNC` writer - USART Synchronous Mode"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBK` reader - Loopback Enable"]
pub type LoopbkR = crate::BitReader;
#[doc = "Field `LOOPBK` writer - Loopback Enable"]
pub type LoopbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCEN` reader - Collision Check Enable"]
pub type CcenR = crate::BitReader;
#[doc = "Field `CCEN` writer - Collision Check Enable"]
pub type CcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPM` reader - Multi-Processor Mode"]
pub type MpmR = crate::BitReader;
#[doc = "Field `MPM` writer - Multi-Processor Mode"]
pub type MpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAB` reader - Multi-Processor Address-Bit"]
pub type MpabR = crate::BitReader;
#[doc = "Field `MPAB` writer - Multi-Processor Address-Bit"]
pub type MpabW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Oversampling\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVS {
    #[doc = "0: Regular UART mode with 16X oversampling in asynchronous mode"]
    X16 = 0,
    #[doc = "1: Double speed with 8X oversampling in asynchronous mode"]
    X8 = 1,
    #[doc = "2: 6X oversampling in asynchronous mode"]
    X6 = 2,
    #[doc = "3: Quadruple speed with 4X oversampling in asynchronous mode"]
    X4 = 3,
}
impl From<OVS> for u8 {
    #[inline(always)]
    fn from(variant: OVS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVS {
    type Ux = u8;
}
impl crate::IsEnum for OVS {}
#[doc = "Field `OVS` reader - Oversampling"]
pub type OvsR = crate::FieldReader<OVS>;
impl OvsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVS {
        match self.bits {
            0 => OVS::X16,
            1 => OVS::X8,
            2 => OVS::X6,
            3 => OVS::X4,
            _ => unreachable!(),
        }
    }
    #[doc = "Regular UART mode with 16X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == OVS::X16
    }
    #[doc = "Double speed with 8X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == OVS::X8
    }
    #[doc = "6X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn is_x6(&self) -> bool {
        *self == OVS::X6
    }
    #[doc = "Quadruple speed with 4X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == OVS::X4
    }
}
#[doc = "Field `OVS` writer - Oversampling"]
pub type OvsW<'a, REG> = crate::FieldWriter<'a, REG, 2, OVS, crate::Safe>;
impl<'a, REG> OvsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Regular UART mode with 16X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut crate::W<REG> {
        self.variant(OVS::X16)
    }
    #[doc = "Double speed with 8X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut crate::W<REG> {
        self.variant(OVS::X8)
    }
    #[doc = "6X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x6(self) -> &'a mut crate::W<REG> {
        self.variant(OVS::X6)
    }
    #[doc = "Quadruple speed with 4X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(OVS::X4)
    }
}
#[doc = "Field `CLKPOL` reader - Clock Polarity"]
pub type ClkpolR = crate::BitReader;
#[doc = "Field `CLKPOL` writer - Clock Polarity"]
pub type ClkpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPHA` reader - Clock Edge for Setup/Sample"]
pub type ClkphaR = crate::BitReader;
#[doc = "Field `CLKPHA` writer - Clock Edge for Setup/Sample"]
pub type ClkphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MsbfR = crate::BitReader;
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MsbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMA` reader - Action on Slave-Select in Master Mode"]
pub type CsmaR = crate::BitReader;
#[doc = "Field `CSMA` writer - Action on Slave-Select in Master Mode"]
pub type CsmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBIL` reader - TX Buffer Interrupt Level"]
pub type TxbilR = crate::BitReader;
#[doc = "Field `TXBIL` writer - TX Buffer Interrupt Level"]
pub type TxbilW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - Receiver Input Invert"]
pub type RxinvR = crate::BitReader;
#[doc = "Field `RXINV` writer - Receiver Input Invert"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINV` reader - Transmitter Output Invert"]
pub type TxinvR = crate::BitReader;
#[doc = "Field `TXINV` writer - Transmitter Output Invert"]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSINV` reader - Chip Select Invert"]
pub type CsinvR = crate::BitReader;
#[doc = "Field `CSINV` writer - Chip Select Invert"]
pub type CsinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCS` reader - Automatic Chip Select"]
pub type AutocsR = crate::BitReader;
#[doc = "Field `AUTOCS` writer - Automatic Chip Select"]
pub type AutocsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOTRI` reader - Automatic TX Tristate"]
pub type AutotriR = crate::BitReader;
#[doc = "Field `AUTOTRI` writer - Automatic TX Tristate"]
pub type AutotriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMODE` reader - SmartCard Mode"]
pub type ScmodeR = crate::BitReader;
#[doc = "Field `SCMODE` writer - SmartCard Mode"]
pub type ScmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCRETRANS` reader - SmartCard Retransmit"]
pub type ScretransR = crate::BitReader;
#[doc = "Field `SCRETRANS` writer - SmartCard Retransmit"]
pub type ScretransW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SKIPPERRF` reader - Skip Parity Error Frames"]
pub type SkipperrfR = crate::BitReader;
#[doc = "Field `SKIPPERRF` writer - Skip Parity Error Frames"]
pub type SkipperrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT8DV` reader - Bit 8 Default Value"]
pub type Bit8dvR = crate::BitReader;
#[doc = "Field `BIT8DV` writer - Bit 8 Default Value"]
pub type Bit8dvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSDMA` reader - Halt DMA on Error"]
pub type ErrsdmaR = crate::BitReader;
#[doc = "Field `ERRSDMA` writer - Halt DMA on Error"]
pub type ErrsdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSRX` reader - Disable RX on Error"]
pub type ErrsrxR = crate::BitReader;
#[doc = "Field `ERRSRX` writer - Disable RX on Error"]
pub type ErrsrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSTX` reader - Disable TX on Error"]
pub type ErrstxR = crate::BitReader;
#[doc = "Field `ERRSTX` writer - Disable TX on Error"]
pub type ErrstxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSEARLY` reader - Synchronous Slave Setup Early"]
pub type SssearlyR = crate::BitReader;
#[doc = "Field `SSSEARLY` writer - Synchronous Slave Setup Early"]
pub type SssearlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTESWAP` reader - Byteswap in Double Accesses"]
pub type ByteswapR = crate::BitReader;
#[doc = "Field `BYTESWAP` writer - Byteswap in Double Accesses"]
pub type ByteswapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOTX` reader - Always Transmit When RX Not Full"]
pub type AutotxR = crate::BitReader;
#[doc = "Field `AUTOTX` writer - Always Transmit When RX Not Full"]
pub type AutotxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MVDIS` reader - Majority Vote Disable"]
pub type MvdisR = crate::BitReader;
#[doc = "Field `MVDIS` writer - Majority Vote Disable"]
pub type MvdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSDELAY` reader - Synchronous Master Sample Delay"]
pub type SmsdelayR = crate::BitReader;
#[doc = "Field `SMSDELAY` writer - Synchronous Master Sample Delay"]
pub type SmsdelayW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USART Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&self) -> LoopbkR {
        LoopbkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline(always)]
    pub fn ccen(&self) -> CcenR {
        CcenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&self) -> MpmR {
        MpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&self) -> MpabR {
        MpabR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline(always)]
    pub fn ovs(&self) -> OvsR {
        OvsR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> ClkpolR {
        ClkpolR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock Edge for Setup/Sample"]
    #[inline(always)]
    pub fn clkpha(&self) -> ClkphaR {
        ClkphaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MsbfR {
        MsbfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Action on Slave-Select in Master Mode"]
    #[inline(always)]
    pub fn csma(&self) -> CsmaR {
        CsmaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline(always)]
    pub fn txbil(&self) -> TxbilR {
        TxbilR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmitter Output Invert"]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline(always)]
    pub fn csinv(&self) -> CsinvR {
        CsinvR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline(always)]
    pub fn autocs(&self) -> AutocsR {
        AutocsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    pub fn autotri(&self) -> AutotriR {
        AutotriR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline(always)]
    pub fn scmode(&self) -> ScmodeR {
        ScmodeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline(always)]
    pub fn scretrans(&self) -> ScretransR {
        ScretransR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    pub fn skipperrf(&self) -> SkipperrfR {
        SkipperrfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&self) -> Bit8dvR {
        Bit8dvR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Halt DMA on Error"]
    #[inline(always)]
    pub fn errsdma(&self) -> ErrsdmaR {
        ErrsdmaR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Disable RX on Error"]
    #[inline(always)]
    pub fn errsrx(&self) -> ErrsrxR {
        ErrsrxR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable TX on Error"]
    #[inline(always)]
    pub fn errstx(&self) -> ErrstxR {
        ErrstxR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Synchronous Slave Setup Early"]
    #[inline(always)]
    pub fn sssearly(&self) -> SssearlyR {
        SssearlyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Byteswap in Double Accesses"]
    #[inline(always)]
    pub fn byteswap(&self) -> ByteswapR {
        ByteswapR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline(always)]
    pub fn autotx(&self) -> AutotxR {
        AutotxR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    pub fn mvdis(&self) -> MvdisR {
        MvdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Synchronous Master Sample Delay"]
    #[inline(always)]
    pub fn smsdelay(&self) -> SmsdelayR {
        SmsdelayR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("sync", &self.sync())
            .field("loopbk", &self.loopbk())
            .field("ccen", &self.ccen())
            .field("mpm", &self.mpm())
            .field("mpab", &self.mpab())
            .field("ovs", &self.ovs())
            .field("clkpol", &self.clkpol())
            .field("clkpha", &self.clkpha())
            .field("msbf", &self.msbf())
            .field("csma", &self.csma())
            .field("txbil", &self.txbil())
            .field("rxinv", &self.rxinv())
            .field("txinv", &self.txinv())
            .field("csinv", &self.csinv())
            .field("autocs", &self.autocs())
            .field("autotri", &self.autotri())
            .field("scmode", &self.scmode())
            .field("scretrans", &self.scretrans())
            .field("skipperrf", &self.skipperrf())
            .field("bit8dv", &self.bit8dv())
            .field("errsdma", &self.errsdma())
            .field("errsrx", &self.errsrx())
            .field("errstx", &self.errstx())
            .field("sssearly", &self.sssearly())
            .field("byteswap", &self.byteswap())
            .field("autotx", &self.autotx())
            .field("mvdis", &self.mvdis())
            .field("smsdelay", &self.smsdelay())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - USART Synchronous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SyncW<CTRLrs> {
        SyncW::new(self, 0)
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn loopbk(&mut self) -> LoopbkW<CTRLrs> {
        LoopbkW::new(self, 1)
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccen(&mut self) -> CcenW<CTRLrs> {
        CcenW::new(self, 2)
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mpm(&mut self) -> MpmW<CTRLrs> {
        MpmW::new(self, 3)
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline(always)]
    #[must_use]
    pub fn mpab(&mut self) -> MpabW<CTRLrs> {
        MpabW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline(always)]
    #[must_use]
    pub fn ovs(&mut self) -> OvsW<CTRLrs> {
        OvsW::new(self, 5)
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> ClkpolW<CTRLrs> {
        ClkpolW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock Edge for Setup/Sample"]
    #[inline(always)]
    #[must_use]
    pub fn clkpha(&mut self) -> ClkphaW<CTRLrs> {
        ClkphaW::new(self, 9)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MsbfW<CTRLrs> {
        MsbfW::new(self, 10)
    }
    #[doc = "Bit 11 - Action on Slave-Select in Master Mode"]
    #[inline(always)]
    #[must_use]
    pub fn csma(&mut self) -> CsmaW<CTRLrs> {
        CsmaW::new(self, 11)
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline(always)]
    #[must_use]
    pub fn txbil(&mut self) -> TxbilW<CTRLrs> {
        TxbilW::new(self, 12)
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RxinvW<CTRLrs> {
        RxinvW::new(self, 13)
    }
    #[doc = "Bit 14 - Transmitter Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TxinvW<CTRLrs> {
        TxinvW::new(self, 14)
    }
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline(always)]
    #[must_use]
    pub fn csinv(&mut self) -> CsinvW<CTRLrs> {
        CsinvW::new(self, 15)
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn autocs(&mut self) -> AutocsW<CTRLrs> {
        AutocsW::new(self, 16)
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    #[must_use]
    pub fn autotri(&mut self) -> AutotriW<CTRLrs> {
        AutotriW::new(self, 17)
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline(always)]
    #[must_use]
    pub fn scmode(&mut self) -> ScmodeW<CTRLrs> {
        ScmodeW::new(self, 18)
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline(always)]
    #[must_use]
    pub fn scretrans(&mut self) -> ScretransW<CTRLrs> {
        ScretransW::new(self, 19)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    #[must_use]
    pub fn skipperrf(&mut self) -> SkipperrfW<CTRLrs> {
        SkipperrfW::new(self, 20)
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline(always)]
    #[must_use]
    pub fn bit8dv(&mut self) -> Bit8dvW<CTRLrs> {
        Bit8dvW::new(self, 21)
    }
    #[doc = "Bit 22 - Halt DMA on Error"]
    #[inline(always)]
    #[must_use]
    pub fn errsdma(&mut self) -> ErrsdmaW<CTRLrs> {
        ErrsdmaW::new(self, 22)
    }
    #[doc = "Bit 23 - Disable RX on Error"]
    #[inline(always)]
    #[must_use]
    pub fn errsrx(&mut self) -> ErrsrxW<CTRLrs> {
        ErrsrxW::new(self, 23)
    }
    #[doc = "Bit 24 - Disable TX on Error"]
    #[inline(always)]
    #[must_use]
    pub fn errstx(&mut self) -> ErrstxW<CTRLrs> {
        ErrstxW::new(self, 24)
    }
    #[doc = "Bit 25 - Synchronous Slave Setup Early"]
    #[inline(always)]
    #[must_use]
    pub fn sssearly(&mut self) -> SssearlyW<CTRLrs> {
        SssearlyW::new(self, 25)
    }
    #[doc = "Bit 28 - Byteswap in Double Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn byteswap(&mut self) -> ByteswapW<CTRLrs> {
        ByteswapW::new(self, 28)
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline(always)]
    #[must_use]
    pub fn autotx(&mut self) -> AutotxW<CTRLrs> {
        AutotxW::new(self, 29)
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mvdis(&mut self) -> MvdisW<CTRLrs> {
        MvdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Synchronous Master Sample Delay"]
    #[inline(always)]
    #[must_use]
    pub fn smsdelay(&mut self) -> SmsdelayW<CTRLrs> {
        SmsdelayW::new(self, 31)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
