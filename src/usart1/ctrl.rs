#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNC` reader - USART Synchronous Mode"]
pub type SYNC_R = crate::BitReader;
#[doc = "Field `SYNC` writer - USART Synchronous Mode"]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `LOOPBK` reader - Loopback Enable"]
pub type LOOPBK_R = crate::BitReader;
#[doc = "Field `LOOPBK` writer - Loopback Enable"]
pub type LOOPBK_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CCEN` reader - Collision Check Enable"]
pub type CCEN_R = crate::BitReader;
#[doc = "Field `CCEN` writer - Collision Check Enable"]
pub type CCEN_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `MPM` reader - Multi-Processor Mode"]
pub type MPM_R = crate::BitReader;
#[doc = "Field `MPM` writer - Multi-Processor Mode"]
pub type MPM_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `MPAB` reader - Multi-Processor Address-Bit"]
pub type MPAB_R = crate::BitReader;
#[doc = "Field `MPAB` writer - Multi-Processor Address-Bit"]
pub type MPAB_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `OVS` reader - Oversampling"]
pub type OVS_R = crate::FieldReader<OVS_A>;
#[doc = "Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVS_A {
    #[doc = "0: Regular UART mode with 16X oversampling in asynchronous mode"]
    X16 = 0,
    #[doc = "1: Double speed with 8X oversampling in asynchronous mode"]
    X8 = 1,
    #[doc = "2: 6X oversampling in asynchronous mode"]
    X6 = 2,
    #[doc = "3: Quadruple speed with 4X oversampling in asynchronous mode"]
    X4 = 3,
}
impl From<OVS_A> for u8 {
    #[inline(always)]
    fn from(variant: OVS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVS_A {
    type Ux = u8;
}
impl OVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVS_A {
        match self.bits {
            0 => OVS_A::X16,
            1 => OVS_A::X8,
            2 => OVS_A::X6,
            3 => OVS_A::X4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == OVS_A::X16
    }
    #[doc = "Checks if the value of the field is `X8`"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == OVS_A::X8
    }
    #[doc = "Checks if the value of the field is `X6`"]
    #[inline(always)]
    pub fn is_x6(&self) -> bool {
        *self == OVS_A::X6
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == OVS_A::X4
    }
}
#[doc = "Field `OVS` writer - Oversampling"]
pub type OVS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CTRL_SPEC, 2, O, OVS_A>;
impl<'a, const O: u8> OVS_W<'a, O> {
    #[doc = "Regular UART mode with 16X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut W {
        self.variant(OVS_A::X16)
    }
    #[doc = "Double speed with 8X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut W {
        self.variant(OVS_A::X8)
    }
    #[doc = "6X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x6(self) -> &'a mut W {
        self.variant(OVS_A::X6)
    }
    #[doc = "Quadruple speed with 4X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(OVS_A::X4)
    }
}
#[doc = "Field `CLKPOL` reader - Clock Polarity"]
pub type CLKPOL_R = crate::BitReader;
#[doc = "Field `CLKPOL` writer - Clock Polarity"]
pub type CLKPOL_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CLKPHA` reader - Clock Edge for Setup/Sample"]
pub type CLKPHA_R = crate::BitReader;
#[doc = "Field `CLKPHA` writer - Clock Edge for Setup/Sample"]
pub type CLKPHA_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MSBF_R = crate::BitReader;
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MSBF_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CSMA` reader - Action on Slave-Select in Master Mode"]
pub type CSMA_R = crate::BitReader;
#[doc = "Field `CSMA` writer - Action on Slave-Select in Master Mode"]
pub type CSMA_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `TXBIL` reader - TX Buffer Interrupt Level"]
pub type TXBIL_R = crate::BitReader;
#[doc = "Field `TXBIL` writer - TX Buffer Interrupt Level"]
pub type TXBIL_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `RXINV` reader - Receiver Input Invert"]
pub type RXINV_R = crate::BitReader;
#[doc = "Field `RXINV` writer - Receiver Input Invert"]
pub type RXINV_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `TXINV` reader - Transmitter Output Invert"]
pub type TXINV_R = crate::BitReader;
#[doc = "Field `TXINV` writer - Transmitter Output Invert"]
pub type TXINV_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CSINV` reader - Chip Select Invert"]
pub type CSINV_R = crate::BitReader;
#[doc = "Field `CSINV` writer - Chip Select Invert"]
pub type CSINV_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `AUTOCS` reader - Automatic Chip Select"]
pub type AUTOCS_R = crate::BitReader;
#[doc = "Field `AUTOCS` writer - Automatic Chip Select"]
pub type AUTOCS_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `AUTOTRI` reader - Automatic TX Tristate"]
pub type AUTOTRI_R = crate::BitReader;
#[doc = "Field `AUTOTRI` writer - Automatic TX Tristate"]
pub type AUTOTRI_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `SCMODE` reader - SmartCard Mode"]
pub type SCMODE_R = crate::BitReader;
#[doc = "Field `SCMODE` writer - SmartCard Mode"]
pub type SCMODE_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `SCRETRANS` reader - SmartCard Retransmit"]
pub type SCRETRANS_R = crate::BitReader;
#[doc = "Field `SCRETRANS` writer - SmartCard Retransmit"]
pub type SCRETRANS_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `SKIPPERRF` reader - Skip Parity Error Frames"]
pub type SKIPPERRF_R = crate::BitReader;
#[doc = "Field `SKIPPERRF` writer - Skip Parity Error Frames"]
pub type SKIPPERRF_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `BIT8DV` reader - Bit 8 Default Value"]
pub type BIT8DV_R = crate::BitReader;
#[doc = "Field `BIT8DV` writer - Bit 8 Default Value"]
pub type BIT8DV_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `ERRSDMA` reader - Halt DMA on Error"]
pub type ERRSDMA_R = crate::BitReader;
#[doc = "Field `ERRSDMA` writer - Halt DMA on Error"]
pub type ERRSDMA_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `ERRSRX` reader - Disable RX on Error"]
pub type ERRSRX_R = crate::BitReader;
#[doc = "Field `ERRSRX` writer - Disable RX on Error"]
pub type ERRSRX_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `ERRSTX` reader - Disable TX on Error"]
pub type ERRSTX_R = crate::BitReader;
#[doc = "Field `ERRSTX` writer - Disable TX on Error"]
pub type ERRSTX_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `SSSEARLY` reader - Synchronous Slave Setup Early"]
pub type SSSEARLY_R = crate::BitReader;
#[doc = "Field `SSSEARLY` writer - Synchronous Slave Setup Early"]
pub type SSSEARLY_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `BYTESWAP` reader - Byteswap in Double Accesses"]
pub type BYTESWAP_R = crate::BitReader;
#[doc = "Field `BYTESWAP` writer - Byteswap in Double Accesses"]
pub type BYTESWAP_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `AUTOTX` reader - Always Transmit When RX Not Full"]
pub type AUTOTX_R = crate::BitReader;
#[doc = "Field `AUTOTX` writer - Always Transmit When RX Not Full"]
pub type AUTOTX_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `MVDIS` reader - Majority Vote Disable"]
pub type MVDIS_R = crate::BitReader;
#[doc = "Field `MVDIS` writer - Majority Vote Disable"]
pub type MVDIS_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `SMSDELAY` reader - Synchronous Master Sample Delay"]
pub type SMSDELAY_R = crate::BitReader;
#[doc = "Field `SMSDELAY` writer - Synchronous Master Sample Delay"]
pub type SMSDELAY_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
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
    pub fn sync(&mut self) -> SYNC_W<0> {
        SYNC_W::new(self)
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn loopbk(&mut self) -> LOOPBK_W<1> {
        LOOPBK_W::new(self)
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccen(&mut self) -> CCEN_W<2> {
        CCEN_W::new(self)
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mpm(&mut self) -> MPM_W<3> {
        MPM_W::new(self)
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline(always)]
    #[must_use]
    pub fn mpab(&mut self) -> MPAB_W<4> {
        MPAB_W::new(self)
    }
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline(always)]
    #[must_use]
    pub fn ovs(&mut self) -> OVS_W<5> {
        OVS_W::new(self)
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> CLKPOL_W<8> {
        CLKPOL_W::new(self)
    }
    #[doc = "Bit 9 - Clock Edge for Setup/Sample"]
    #[inline(always)]
    #[must_use]
    pub fn clkpha(&mut self) -> CLKPHA_W<9> {
        CLKPHA_W::new(self)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<10> {
        MSBF_W::new(self)
    }
    #[doc = "Bit 11 - Action on Slave-Select in Master Mode"]
    #[inline(always)]
    #[must_use]
    pub fn csma(&mut self) -> CSMA_W<11> {
        CSMA_W::new(self)
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline(always)]
    #[must_use]
    pub fn txbil(&mut self) -> TXBIL_W<12> {
        TXBIL_W::new(self)
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<13> {
        RXINV_W::new(self)
    }
    #[doc = "Bit 14 - Transmitter Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<14> {
        TXINV_W::new(self)
    }
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline(always)]
    #[must_use]
    pub fn csinv(&mut self) -> CSINV_W<15> {
        CSINV_W::new(self)
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn autocs(&mut self) -> AUTOCS_W<16> {
        AUTOCS_W::new(self)
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    #[must_use]
    pub fn autotri(&mut self) -> AUTOTRI_W<17> {
        AUTOTRI_W::new(self)
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline(always)]
    #[must_use]
    pub fn scmode(&mut self) -> SCMODE_W<18> {
        SCMODE_W::new(self)
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline(always)]
    #[must_use]
    pub fn scretrans(&mut self) -> SCRETRANS_W<19> {
        SCRETRANS_W::new(self)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    #[must_use]
    pub fn skipperrf(&mut self) -> SKIPPERRF_W<20> {
        SKIPPERRF_W::new(self)
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline(always)]
    #[must_use]
    pub fn bit8dv(&mut self) -> BIT8DV_W<21> {
        BIT8DV_W::new(self)
    }
    #[doc = "Bit 22 - Halt DMA on Error"]
    #[inline(always)]
    #[must_use]
    pub fn errsdma(&mut self) -> ERRSDMA_W<22> {
        ERRSDMA_W::new(self)
    }
    #[doc = "Bit 23 - Disable RX on Error"]
    #[inline(always)]
    #[must_use]
    pub fn errsrx(&mut self) -> ERRSRX_W<23> {
        ERRSRX_W::new(self)
    }
    #[doc = "Bit 24 - Disable TX on Error"]
    #[inline(always)]
    #[must_use]
    pub fn errstx(&mut self) -> ERRSTX_W<24> {
        ERRSTX_W::new(self)
    }
    #[doc = "Bit 25 - Synchronous Slave Setup Early"]
    #[inline(always)]
    #[must_use]
    pub fn sssearly(&mut self) -> SSSEARLY_W<25> {
        SSSEARLY_W::new(self)
    }
    #[doc = "Bit 28 - Byteswap in Double Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn byteswap(&mut self) -> BYTESWAP_W<28> {
        BYTESWAP_W::new(self)
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline(always)]
    #[must_use]
    pub fn autotx(&mut self) -> AUTOTX_W<29> {
        AUTOTX_W::new(self)
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mvdis(&mut self) -> MVDIS_W<30> {
        MVDIS_W::new(self)
    }
    #[doc = "Bit 31 - Synchronous Master Sample Delay"]
    #[inline(always)]
    #[must_use]
    pub fn smsdelay(&mut self) -> SMSDELAY_W<31> {
        SMSDELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
