///Register `TRIGCTRL` reader
pub type R = crate::R<TRIGCTRLrs>;
///Register `TRIGCTRL` writer
pub type W = crate::W<TRIGCTRLrs>;
///Field `RXTEN` reader - Receive Trigger Enable
pub type RxtenR = crate::BitReader;
///Field `RXTEN` writer - Receive Trigger Enable
pub type RxtenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXTEN` reader - Transmit Trigger Enable
pub type TxtenR = crate::BitReader;
///Field `TXTEN` writer - Transmit Trigger Enable
pub type TxtenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTOTXTEN` reader - AUTOTX Trigger Enable
pub type AutotxtenR = crate::BitReader;
///Field `AUTOTXTEN` writer - AUTOTX Trigger Enable
pub type AutotxtenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXARX0EN` reader - Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL
pub type Txarx0enR = crate::BitReader;
///Field `TXARX0EN` writer - Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL
pub type Txarx0enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXARX1EN` reader - Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL
pub type Txarx1enR = crate::BitReader;
///Field `TXARX1EN` writer - Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL
pub type Txarx1enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXARX2EN` reader - Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL
pub type Txarx2enR = crate::BitReader;
///Field `TXARX2EN` writer - Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL
pub type Txarx2enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXATX0EN` reader - Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times
pub type Rxatx0enR = crate::BitReader;
///Field `RXATX0EN` writer - Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times
pub type Rxatx0enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXATX1EN` reader - Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times
pub type Rxatx1enR = crate::BitReader;
///Field `RXATX1EN` writer - Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times
pub type Rxatx1enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXATX2EN` reader - Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times
pub type Rxatx2enR = crate::BitReader;
///Field `RXATX2EN` writer - Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times
pub type Rxatx2enW<'a, REG> = crate::BitWriter<'a, REG>;
///Trigger PRS Channel Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL {
    ///0: PRS Channel 0 selected
    Prsch0 = 0,
    ///1: PRS Channel 1 selected
    Prsch1 = 1,
    ///2: PRS Channel 2 selected
    Prsch2 = 2,
    ///3: PRS Channel 3 selected
    Prsch3 = 3,
    ///4: PRS Channel 4 selected
    Prsch4 = 4,
    ///5: PRS Channel 5 selected
    Prsch5 = 5,
    ///6: PRS Channel 6 selected
    Prsch6 = 6,
    ///7: PRS Channel 7 selected
    Prsch7 = 7,
    ///8: PRS Channel 8 selected
    Prsch8 = 8,
    ///9: PRS Channel 9 selected
    Prsch9 = 9,
    ///10: PRS Channel 10 selected
    Prsch10 = 10,
    ///11: PRS Channel 11 selected
    Prsch11 = 11,
}
impl From<TSEL> for u8 {
    #[inline(always)]
    fn from(variant: TSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSEL {
    type Ux = u8;
}
impl crate::IsEnum for TSEL {}
///Field `TSEL` reader - Trigger PRS Channel Select
pub type TselR = crate::FieldReader<TSEL>;
impl TselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSEL> {
        match self.bits {
            0 => Some(TSEL::Prsch0),
            1 => Some(TSEL::Prsch1),
            2 => Some(TSEL::Prsch2),
            3 => Some(TSEL::Prsch3),
            4 => Some(TSEL::Prsch4),
            5 => Some(TSEL::Prsch5),
            6 => Some(TSEL::Prsch6),
            7 => Some(TSEL::Prsch7),
            8 => Some(TSEL::Prsch8),
            9 => Some(TSEL::Prsch9),
            10 => Some(TSEL::Prsch10),
            11 => Some(TSEL::Prsch11),
            _ => None,
        }
    }
    ///PRS Channel 0 selected
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TSEL::Prsch0
    }
    ///PRS Channel 1 selected
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TSEL::Prsch1
    }
    ///PRS Channel 2 selected
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TSEL::Prsch2
    }
    ///PRS Channel 3 selected
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TSEL::Prsch3
    }
    ///PRS Channel 4 selected
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TSEL::Prsch4
    }
    ///PRS Channel 5 selected
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TSEL::Prsch5
    }
    ///PRS Channel 6 selected
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == TSEL::Prsch6
    }
    ///PRS Channel 7 selected
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == TSEL::Prsch7
    }
    ///PRS Channel 8 selected
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == TSEL::Prsch8
    }
    ///PRS Channel 9 selected
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == TSEL::Prsch9
    }
    ///PRS Channel 10 selected
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == TSEL::Prsch10
    }
    ///PRS Channel 11 selected
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == TSEL::Prsch11
    }
}
///Field `TSEL` writer - Trigger PRS Channel Select
pub type TselW<'a, REG> = crate::FieldWriter<'a, REG, 4, TSEL>;
impl<'a, REG> TselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL::Prsch0)
    }
    ///PRS Channel 1 selected
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL::Prsch1)
    }
    ///PRS Channel 2 selected
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL::Prsch2)
    }
    ///PRS Channel 3 selected
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL::Prsch3)
    }
    ///PRS Channel 4 selected
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL::Prsch4)
    }
    ///PRS Channel 5 selected
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL::Prsch5)
    }
    ///PRS Channel 6 selected
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL::Prsch6)
    }
    ///PRS Channel 7 selected
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL::Prsch7)
    }
    ///PRS Channel 8 selected
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL::Prsch8)
    }
    ///PRS Channel 9 selected
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL::Prsch9)
    }
    ///PRS Channel 10 selected
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL::Prsch10)
    }
    ///PRS Channel 11 selected
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL::Prsch11)
    }
}
impl R {
    ///Bit 4 - Receive Trigger Enable
    #[inline(always)]
    pub fn rxten(&self) -> RxtenR {
        RxtenR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transmit Trigger Enable
    #[inline(always)]
    pub fn txten(&self) -> TxtenR {
        TxtenR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AUTOTX Trigger Enable
    #[inline(always)]
    pub fn autotxten(&self) -> AutotxtenR {
        AutotxtenR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL
    #[inline(always)]
    pub fn txarx0en(&self) -> Txarx0enR {
        Txarx0enR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL
    #[inline(always)]
    pub fn txarx1en(&self) -> Txarx1enR {
        Txarx1enR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL
    #[inline(always)]
    pub fn txarx2en(&self) -> Txarx2enR {
        Txarx2enR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times
    #[inline(always)]
    pub fn rxatx0en(&self) -> Rxatx0enR {
        Rxatx0enR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times
    #[inline(always)]
    pub fn rxatx1en(&self) -> Rxatx1enR {
        Rxatx1enR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times
    #[inline(always)]
    pub fn rxatx2en(&self) -> Rxatx2enR {
        Rxatx2enR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:19 - Trigger PRS Channel Select
    #[inline(always)]
    pub fn tsel(&self) -> TselR {
        TselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIGCTRL")
            .field("rxten", &self.rxten())
            .field("txten", &self.txten())
            .field("autotxten", &self.autotxten())
            .field("txarx0en", &self.txarx0en())
            .field("txarx1en", &self.txarx1en())
            .field("txarx2en", &self.txarx2en())
            .field("rxatx0en", &self.rxatx0en())
            .field("rxatx1en", &self.rxatx1en())
            .field("rxatx2en", &self.rxatx2en())
            .field("tsel", &self.tsel())
            .finish()
    }
}
impl W {
    ///Bit 4 - Receive Trigger Enable
    #[inline(always)]
    #[must_use]
    pub fn rxten(&mut self) -> RxtenW<TRIGCTRLrs> {
        RxtenW::new(self, 4)
    }
    ///Bit 5 - Transmit Trigger Enable
    #[inline(always)]
    #[must_use]
    pub fn txten(&mut self) -> TxtenW<TRIGCTRLrs> {
        TxtenW::new(self, 5)
    }
    ///Bit 6 - AUTOTX Trigger Enable
    #[inline(always)]
    #[must_use]
    pub fn autotxten(&mut self) -> AutotxtenW<TRIGCTRLrs> {
        AutotxtenW::new(self, 6)
    }
    ///Bit 7 - Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL
    #[inline(always)]
    #[must_use]
    pub fn txarx0en(&mut self) -> Txarx0enW<TRIGCTRLrs> {
        Txarx0enW::new(self, 7)
    }
    ///Bit 8 - Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL
    #[inline(always)]
    #[must_use]
    pub fn txarx1en(&mut self) -> Txarx1enW<TRIGCTRLrs> {
        Txarx1enW::new(self, 8)
    }
    ///Bit 9 - Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL
    #[inline(always)]
    #[must_use]
    pub fn txarx2en(&mut self) -> Txarx2enW<TRIGCTRLrs> {
        Txarx2enW::new(self, 9)
    }
    ///Bit 10 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times
    #[inline(always)]
    #[must_use]
    pub fn rxatx0en(&mut self) -> Rxatx0enW<TRIGCTRLrs> {
        Rxatx0enW::new(self, 10)
    }
    ///Bit 11 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times
    #[inline(always)]
    #[must_use]
    pub fn rxatx1en(&mut self) -> Rxatx1enW<TRIGCTRLrs> {
        Rxatx1enW::new(self, 11)
    }
    ///Bit 12 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times
    #[inline(always)]
    #[must_use]
    pub fn rxatx2en(&mut self) -> Rxatx2enW<TRIGCTRLrs> {
        Rxatx2enW::new(self, 12)
    }
    ///Bits 16:19 - Trigger PRS Channel Select
    #[inline(always)]
    #[must_use]
    pub fn tsel(&mut self) -> TselW<TRIGCTRLrs> {
        TselW::new(self, 16)
    }
}
///USART Trigger Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`trigctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TRIGCTRLrs;
impl crate::RegisterSpec for TRIGCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`trigctrl::R`](R) reader structure
impl crate::Readable for TRIGCTRLrs {}
///`write(|w| ..)` method takes [`trigctrl::W`](W) writer structure
impl crate::Writable for TRIGCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TRIGCTRL to value 0
impl crate::Resettable for TRIGCTRLrs {
    const RESET_VALUE: u32 = 0;
}
