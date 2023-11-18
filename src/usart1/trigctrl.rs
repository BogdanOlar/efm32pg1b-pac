#[doc = "Register `TRIGCTRL` reader"]
pub type R = crate::R<TRIGCTRL_SPEC>;
#[doc = "Register `TRIGCTRL` writer"]
pub type W = crate::W<TRIGCTRL_SPEC>;
#[doc = "Field `RXTEN` reader - Receive Trigger Enable"]
pub type RXTEN_R = crate::BitReader;
#[doc = "Field `RXTEN` writer - Receive Trigger Enable"]
pub type RXTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXTEN` reader - Transmit Trigger Enable"]
pub type TXTEN_R = crate::BitReader;
#[doc = "Field `TXTEN` writer - Transmit Trigger Enable"]
pub type TXTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTOTXTEN` reader - AUTOTX Trigger Enable"]
pub type AUTOTXTEN_R = crate::BitReader;
#[doc = "Field `AUTOTXTEN` writer - AUTOTX Trigger Enable"]
pub type AUTOTXTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXARX0EN` reader - Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL"]
pub type TXARX0EN_R = crate::BitReader;
#[doc = "Field `TXARX0EN` writer - Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL"]
pub type TXARX0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXARX1EN` reader - Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL"]
pub type TXARX1EN_R = crate::BitReader;
#[doc = "Field `TXARX1EN` writer - Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL"]
pub type TXARX1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXARX2EN` reader - Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL"]
pub type TXARX2EN_R = crate::BitReader;
#[doc = "Field `TXARX2EN` writer - Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL"]
pub type TXARX2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXATX0EN` reader - Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times"]
pub type RXATX0EN_R = crate::BitReader;
#[doc = "Field `RXATX0EN` writer - Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times"]
pub type RXATX0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXATX1EN` reader - Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times"]
pub type RXATX1EN_R = crate::BitReader;
#[doc = "Field `RXATX1EN` writer - Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times"]
pub type RXATX1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXATX2EN` reader - Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times"]
pub type RXATX2EN_R = crate::BitReader;
#[doc = "Field `RXATX2EN` writer - Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times"]
pub type RXATX2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSEL` reader - Trigger PRS Channel Select"]
pub type TSEL_R = crate::FieldReader<TSEL_A>;
#[doc = "Trigger PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL_A {
    #[doc = "0: PRS Channel 0 selected"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected"]
    PRSCH11 = 11,
}
impl From<TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSEL_A {
    type Ux = u8;
}
impl TSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSEL_A> {
        match self.bits {
            0 => Some(TSEL_A::PRSCH0),
            1 => Some(TSEL_A::PRSCH1),
            2 => Some(TSEL_A::PRSCH2),
            3 => Some(TSEL_A::PRSCH3),
            4 => Some(TSEL_A::PRSCH4),
            5 => Some(TSEL_A::PRSCH5),
            6 => Some(TSEL_A::PRSCH6),
            7 => Some(TSEL_A::PRSCH7),
            8 => Some(TSEL_A::PRSCH8),
            9 => Some(TSEL_A::PRSCH9),
            10 => Some(TSEL_A::PRSCH10),
            11 => Some(TSEL_A::PRSCH11),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TSEL_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TSEL_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TSEL_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TSEL_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TSEL_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TSEL_A::PRSCH5
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == TSEL_A::PRSCH6
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == TSEL_A::PRSCH7
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == TSEL_A::PRSCH8
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == TSEL_A::PRSCH9
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == TSEL_A::PRSCH10
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == TSEL_A::PRSCH11
    }
}
#[doc = "Field `TSEL` writer - Trigger PRS Channel Select"]
pub type TSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, TSEL_A>;
impl<'a, REG, const O: u8> TSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL_A::PRSCH11)
    }
}
impl R {
    #[doc = "Bit 4 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RXTEN_R {
        RXTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TXTEN_R {
        TXTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&self) -> AUTOTXTEN_R {
        AUTOTXTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL"]
    #[inline(always)]
    pub fn txarx0en(&self) -> TXARX0EN_R {
        TXARX0EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL"]
    #[inline(always)]
    pub fn txarx1en(&self) -> TXARX1EN_R {
        TXARX1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL"]
    #[inline(always)]
    pub fn txarx2en(&self) -> TXARX2EN_R {
        TXARX2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times"]
    #[inline(always)]
    pub fn rxatx0en(&self) -> RXATX0EN_R {
        RXATX0EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times"]
    #[inline(always)]
    pub fn rxatx1en(&self) -> RXATX1EN_R {
        RXATX1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times"]
    #[inline(always)]
    pub fn rxatx2en(&self) -> RXATX2EN_R {
        RXATX2EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Trigger PRS Channel Select"]
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIGCTRL")
            .field("rxten", &format_args!("{}", self.rxten().bit()))
            .field("txten", &format_args!("{}", self.txten().bit()))
            .field("autotxten", &format_args!("{}", self.autotxten().bit()))
            .field("txarx0en", &format_args!("{}", self.txarx0en().bit()))
            .field("txarx1en", &format_args!("{}", self.txarx1en().bit()))
            .field("txarx2en", &format_args!("{}", self.txarx2en().bit()))
            .field("rxatx0en", &format_args!("{}", self.rxatx0en().bit()))
            .field("rxatx1en", &format_args!("{}", self.rxatx1en().bit()))
            .field("rxatx2en", &format_args!("{}", self.rxatx2en().bit()))
            .field("tsel", &format_args!("{}", self.tsel().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TRIGCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 4 - Receive Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxten(&mut self) -> RXTEN_W<TRIGCTRL_SPEC, 4> {
        RXTEN_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txten(&mut self) -> TXTEN_W<TRIGCTRL_SPEC, 5> {
        TXTEN_W::new(self)
    }
    #[doc = "Bit 6 - AUTOTX Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autotxten(&mut self) -> AUTOTXTEN_W<TRIGCTRL_SPEC, 6> {
        AUTOTXTEN_W::new(self)
    }
    #[doc = "Bit 7 - Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL"]
    #[inline(always)]
    #[must_use]
    pub fn txarx0en(&mut self) -> TXARX0EN_W<TRIGCTRL_SPEC, 7> {
        TXARX0EN_W::new(self)
    }
    #[doc = "Bit 8 - Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL"]
    #[inline(always)]
    #[must_use]
    pub fn txarx1en(&mut self) -> TXARX1EN_W<TRIGCTRL_SPEC, 8> {
        TXARX1EN_W::new(self)
    }
    #[doc = "Bit 9 - Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL"]
    #[inline(always)]
    #[must_use]
    pub fn txarx2en(&mut self) -> TXARX2EN_W<TRIGCTRL_SPEC, 9> {
        TXARX2EN_W::new(self)
    }
    #[doc = "Bit 10 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times"]
    #[inline(always)]
    #[must_use]
    pub fn rxatx0en(&mut self) -> RXATX0EN_W<TRIGCTRL_SPEC, 10> {
        RXATX0EN_W::new(self)
    }
    #[doc = "Bit 11 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times"]
    #[inline(always)]
    #[must_use]
    pub fn rxatx1en(&mut self) -> RXATX1EN_W<TRIGCTRL_SPEC, 11> {
        RXATX1EN_W::new(self)
    }
    #[doc = "Bit 12 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times"]
    #[inline(always)]
    #[must_use]
    pub fn rxatx2en(&mut self) -> RXATX2EN_W<TRIGCTRL_SPEC, 12> {
        RXATX2EN_W::new(self)
    }
    #[doc = "Bits 16:19 - Trigger PRS Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn tsel(&mut self) -> TSEL_W<TRIGCTRL_SPEC, 16> {
        TSEL_W::new(self)
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
#[doc = "USART Trigger Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIGCTRL_SPEC;
impl crate::RegisterSpec for TRIGCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigctrl::R`](R) reader structure"]
impl crate::Readable for TRIGCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trigctrl::W`](W) writer structure"]
impl crate::Writable for TRIGCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIGCTRL to value 0"]
impl crate::Resettable for TRIGCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
