#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Field `SYNC` reader - USART Synchronous Mode"]
pub type SYNC_R = crate::BitReader;
#[doc = "Field `SYNC` writer - USART Synchronous Mode"]
pub type SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBK` reader - Loopback Enable"]
pub type LOOPBK_R = crate::BitReader;
#[doc = "Field `LOOPBK` writer - Loopback Enable"]
pub type LOOPBK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCEN` reader - Collision Check Enable"]
pub type CCEN_R = crate::BitReader;
#[doc = "Field `CCEN` writer - Collision Check Enable"]
pub type CCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPM` reader - Multi-Processor Mode"]
pub type MPM_R = crate::BitReader;
#[doc = "Field `MPM` writer - Multi-Processor Mode"]
pub type MPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAB` reader - Multi-Processor Address-Bit"]
pub type MPAB_R = crate::BitReader;
#[doc = "Field `MPAB` writer - Multi-Processor Address-Bit"]
pub type MPAB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVS` reader - Oversampling"]
pub type OVS_R = crate::FieldReader<OVS>;
#[doc = "Oversampling\n\nValue on reset: 0"]
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
impl OVS_R {
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
pub type OVS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, OVS>;
impl<'a, REG> OVS_W<'a, REG>
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
pub type CLKPOL_R = crate::BitReader;
#[doc = "Field `CLKPOL` writer - Clock Polarity"]
pub type CLKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPHA` reader - Clock Edge for Setup/Sample"]
pub type CLKPHA_R = crate::BitReader;
#[doc = "Field `CLKPHA` writer - Clock Edge for Setup/Sample"]
pub type CLKPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MSBF_R = crate::BitReader;
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MSBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMA` reader - Action on Slave-Select in Master Mode"]
pub type CSMA_R = crate::BitReader;
#[doc = "Field `CSMA` writer - Action on Slave-Select in Master Mode"]
pub type CSMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBIL` reader - TX Buffer Interrupt Level"]
pub type TXBIL_R = crate::BitReader;
#[doc = "Field `TXBIL` writer - TX Buffer Interrupt Level"]
pub type TXBIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - Receiver Input Invert"]
pub type RXINV_R = crate::BitReader;
#[doc = "Field `RXINV` writer - Receiver Input Invert"]
pub type RXINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINV` reader - Transmitter Output Invert"]
pub type TXINV_R = crate::BitReader;
#[doc = "Field `TXINV` writer - Transmitter Output Invert"]
pub type TXINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSINV` reader - Chip Select Invert"]
pub type CSINV_R = crate::BitReader;
#[doc = "Field `CSINV` writer - Chip Select Invert"]
pub type CSINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCS` reader - Automatic Chip Select"]
pub type AUTOCS_R = crate::BitReader;
#[doc = "Field `AUTOCS` writer - Automatic Chip Select"]
pub type AUTOCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOTRI` reader - Automatic TX Tristate"]
pub type AUTOTRI_R = crate::BitReader;
#[doc = "Field `AUTOTRI` writer - Automatic TX Tristate"]
pub type AUTOTRI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMODE` reader - SmartCard Mode"]
pub type SCMODE_R = crate::BitReader;
#[doc = "Field `SCMODE` writer - SmartCard Mode"]
pub type SCMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCRETRANS` reader - SmartCard Retransmit"]
pub type SCRETRANS_R = crate::BitReader;
#[doc = "Field `SCRETRANS` writer - SmartCard Retransmit"]
pub type SCRETRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SKIPPERRF` reader - Skip Parity Error Frames"]
pub type SKIPPERRF_R = crate::BitReader;
#[doc = "Field `SKIPPERRF` writer - Skip Parity Error Frames"]
pub type SKIPPERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT8DV` reader - Bit 8 Default Value"]
pub type BIT8DV_R = crate::BitReader;
#[doc = "Field `BIT8DV` writer - Bit 8 Default Value"]
pub type BIT8DV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSDMA` reader - Halt DMA on Error"]
pub type ERRSDMA_R = crate::BitReader;
#[doc = "Field `ERRSDMA` writer - Halt DMA on Error"]
pub type ERRSDMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSRX` reader - Disable RX on Error"]
pub type ERRSRX_R = crate::BitReader;
#[doc = "Field `ERRSRX` writer - Disable RX on Error"]
pub type ERRSRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSTX` reader - Disable TX on Error"]
pub type ERRSTX_R = crate::BitReader;
#[doc = "Field `ERRSTX` writer - Disable TX on Error"]
pub type ERRSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSEARLY` reader - Synchronous Slave Setup Early"]
pub type SSSEARLY_R = crate::BitReader;
#[doc = "Field `SSSEARLY` writer - Synchronous Slave Setup Early"]
pub type SSSEARLY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTESWAP` reader - Byteswap in Double Accesses"]
pub type BYTESWAP_R = crate::BitReader;
#[doc = "Field `BYTESWAP` writer - Byteswap in Double Accesses"]
pub type BYTESWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOTX` reader - Always Transmit When RX Not Full"]
pub type AUTOTX_R = crate::BitReader;
#[doc = "Field `AUTOTX` writer - Always Transmit When RX Not Full"]
pub type AUTOTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MVDIS` reader - Majority Vote Disable"]
pub type MVDIS_R = crate::BitReader;
#[doc = "Field `MVDIS` writer - Majority Vote Disable"]
pub type MVDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSDELAY` reader - Synchronous Master Sample Delay"]
pub type SMSDELAY_R = crate::BitReader;
#[doc = "Field `SMSDELAY` writer - Synchronous Master Sample Delay"]
pub type SMSDELAY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USART Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&self) -> LOOPBK_R {
        LOOPBK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline(always)]
    pub fn ccen(&self) -> CCEN_R {
        CCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&self) -> MPM_R {
        MPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&self) -> MPAB_R {
        MPAB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock Edge for Setup/Sample"]
    #[inline(always)]
    pub fn clkpha(&self) -> CLKPHA_R {
        CLKPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Action on Slave-Select in Master Mode"]
    #[inline(always)]
    pub fn csma(&self) -> CSMA_R {
        CSMA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline(always)]
    pub fn txbil(&self) -> TXBIL_R {
        TXBIL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmitter Output Invert"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline(always)]
    pub fn csinv(&self) -> CSINV_R {
        CSINV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline(always)]
    pub fn autocs(&self) -> AUTOCS_R {
        AUTOCS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    pub fn autotri(&self) -> AUTOTRI_R {
        AUTOTRI_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline(always)]
    pub fn scmode(&self) -> SCMODE_R {
        SCMODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline(always)]
    pub fn scretrans(&self) -> SCRETRANS_R {
        SCRETRANS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    pub fn skipperrf(&self) -> SKIPPERRF_R {
        SKIPPERRF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&self) -> BIT8DV_R {
        BIT8DV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Halt DMA on Error"]
    #[inline(always)]
    pub fn errsdma(&self) -> ERRSDMA_R {
        ERRSDMA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Disable RX on Error"]
    #[inline(always)]
    pub fn errsrx(&self) -> ERRSRX_R {
        ERRSRX_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable TX on Error"]
    #[inline(always)]
    pub fn errstx(&self) -> ERRSTX_R {
        ERRSTX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Synchronous Slave Setup Early"]
    #[inline(always)]
    pub fn sssearly(&self) -> SSSEARLY_R {
        SSSEARLY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Byteswap in Double Accesses"]
    #[inline(always)]
    pub fn byteswap(&self) -> BYTESWAP_R {
        BYTESWAP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline(always)]
    pub fn autotx(&self) -> AUTOTX_R {
        AUTOTX_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    pub fn mvdis(&self) -> MVDIS_R {
        MVDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Synchronous Master Sample Delay"]
    #[inline(always)]
    pub fn smsdelay(&self) -> SMSDELAY_R {
        SMSDELAY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART Synchronous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<CTRLrs> {
        SYNC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn loopbk(&mut self) -> LOOPBK_W<CTRLrs> {
        LOOPBK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccen(&mut self) -> CCEN_W<CTRLrs> {
        CCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mpm(&mut self) -> MPM_W<CTRLrs> {
        MPM_W::new(self, 3)
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline(always)]
    #[must_use]
    pub fn mpab(&mut self) -> MPAB_W<CTRLrs> {
        MPAB_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline(always)]
    #[must_use]
    pub fn ovs(&mut self) -> OVS_W<CTRLrs> {
        OVS_W::new(self, 5)
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> CLKPOL_W<CTRLrs> {
        CLKPOL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock Edge for Setup/Sample"]
    #[inline(always)]
    #[must_use]
    pub fn clkpha(&mut self) -> CLKPHA_W<CTRLrs> {
        CLKPHA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<CTRLrs> {
        MSBF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Action on Slave-Select in Master Mode"]
    #[inline(always)]
    #[must_use]
    pub fn csma(&mut self) -> CSMA_W<CTRLrs> {
        CSMA_W::new(self, 11)
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline(always)]
    #[must_use]
    pub fn txbil(&mut self) -> TXBIL_W<CTRLrs> {
        TXBIL_W::new(self, 12)
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<CTRLrs> {
        RXINV_W::new(self, 13)
    }
    #[doc = "Bit 14 - Transmitter Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<CTRLrs> {
        TXINV_W::new(self, 14)
    }
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline(always)]
    #[must_use]
    pub fn csinv(&mut self) -> CSINV_W<CTRLrs> {
        CSINV_W::new(self, 15)
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn autocs(&mut self) -> AUTOCS_W<CTRLrs> {
        AUTOCS_W::new(self, 16)
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    #[must_use]
    pub fn autotri(&mut self) -> AUTOTRI_W<CTRLrs> {
        AUTOTRI_W::new(self, 17)
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline(always)]
    #[must_use]
    pub fn scmode(&mut self) -> SCMODE_W<CTRLrs> {
        SCMODE_W::new(self, 18)
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline(always)]
    #[must_use]
    pub fn scretrans(&mut self) -> SCRETRANS_W<CTRLrs> {
        SCRETRANS_W::new(self, 19)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    #[must_use]
    pub fn skipperrf(&mut self) -> SKIPPERRF_W<CTRLrs> {
        SKIPPERRF_W::new(self, 20)
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline(always)]
    #[must_use]
    pub fn bit8dv(&mut self) -> BIT8DV_W<CTRLrs> {
        BIT8DV_W::new(self, 21)
    }
    #[doc = "Bit 22 - Halt DMA on Error"]
    #[inline(always)]
    #[must_use]
    pub fn errsdma(&mut self) -> ERRSDMA_W<CTRLrs> {
        ERRSDMA_W::new(self, 22)
    }
    #[doc = "Bit 23 - Disable RX on Error"]
    #[inline(always)]
    #[must_use]
    pub fn errsrx(&mut self) -> ERRSRX_W<CTRLrs> {
        ERRSRX_W::new(self, 23)
    }
    #[doc = "Bit 24 - Disable TX on Error"]
    #[inline(always)]
    #[must_use]
    pub fn errstx(&mut self) -> ERRSTX_W<CTRLrs> {
        ERRSTX_W::new(self, 24)
    }
    #[doc = "Bit 25 - Synchronous Slave Setup Early"]
    #[inline(always)]
    #[must_use]
    pub fn sssearly(&mut self) -> SSSEARLY_W<CTRLrs> {
        SSSEARLY_W::new(self, 25)
    }
    #[doc = "Bit 28 - Byteswap in Double Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn byteswap(&mut self) -> BYTESWAP_W<CTRLrs> {
        BYTESWAP_W::new(self, 28)
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline(always)]
    #[must_use]
    pub fn autotx(&mut self) -> AUTOTX_W<CTRLrs> {
        AUTOTX_W::new(self, 29)
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mvdis(&mut self) -> MVDIS_W<CTRLrs> {
        MVDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Synchronous Master Sample Delay"]
    #[inline(always)]
    #[must_use]
    pub fn smsdelay(&mut self) -> SMSDELAY_W<CTRLrs> {
        SMSDELAY_W::new(self, 31)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
