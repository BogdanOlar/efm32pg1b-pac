#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Field `AUTOTRI` reader - Automatic Transmitter Tristate"]
pub type AUTOTRI_R = crate::BitReader;
#[doc = "Field `AUTOTRI` writer - Automatic Transmitter Tristate"]
pub type AUTOTRI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATABITS` reader - Data-Bit Mode"]
pub type DATABITS_R = crate::BitReader;
#[doc = "Field `DATABITS` writer - Data-Bit Mode"]
pub type DATABITS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY` reader - Parity-Bit Mode"]
pub type PARITY_R = crate::FieldReader<PARITY>;
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
impl PARITY_R {
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
pub type PARITY_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PARITY>;
impl<'a, REG> PARITY_W<'a, REG>
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
pub type STOPBITS_R = crate::BitReader;
#[doc = "Field `STOPBITS` writer - Stop-Bit Mode"]
pub type STOPBITS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INV` reader - Invert Input and Output"]
pub type INV_R = crate::BitReader;
#[doc = "Field `INV` writer - Invert Input and Output"]
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSDMA` reader - Clear RX DMA on Error"]
pub type ERRSDMA_R = crate::BitReader;
#[doc = "Field `ERRSDMA` writer - Clear RX DMA on Error"]
pub type ERRSDMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBK` reader - Loopback Enable"]
pub type LOOPBK_R = crate::BitReader;
#[doc = "Field `LOOPBK` writer - Loopback Enable"]
pub type LOOPBK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFUBRX` reader - Start-Frame UnBlock RX"]
pub type SFUBRX_R = crate::BitReader;
#[doc = "Field `SFUBRX` writer - Start-Frame UnBlock RX"]
pub type SFUBRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPM` reader - Multi-Processor Mode"]
pub type MPM_R = crate::BitReader;
#[doc = "Field `MPM` writer - Multi-Processor Mode"]
pub type MPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAB` reader - Multi-Processor Address-Bit"]
pub type MPAB_R = crate::BitReader;
#[doc = "Field `MPAB` writer - Multi-Processor Address-Bit"]
pub type MPAB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT8DV` reader - Bit 8 Default Value"]
pub type BIT8DV_R = crate::BitReader;
#[doc = "Field `BIT8DV` writer - Bit 8 Default Value"]
pub type BIT8DV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAWU` reader - RX DMA Wakeup"]
pub type RXDMAWU_R = crate::BitReader;
#[doc = "Field `RXDMAWU` writer - RX DMA Wakeup"]
pub type RXDMAWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAWU` reader - TX DMA Wakeup"]
pub type TXDMAWU_R = crate::BitReader;
#[doc = "Field `TXDMAWU` writer - TX DMA Wakeup"]
pub type TXDMAWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDELAY` reader - TX Delay Transmission"]
pub type TXDELAY_R = crate::FieldReader<TXDELAY>;
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
impl TXDELAY_R {
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
pub type TXDELAY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TXDELAY>;
impl<'a, REG> TXDELAY_W<'a, REG>
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
    pub fn autotri(&self) -> AUTOTRI_R {
        AUTOTRI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&self) -> DATABITS_R {
        DATABITS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&self) -> STOPBITS_R {
        STOPBITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Invert Input and Output"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear RX DMA on Error"]
    #[inline(always)]
    pub fn errsdma(&self) -> ERRSDMA_R {
        ERRSDMA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&self) -> LOOPBK_R {
        LOOPBK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start-Frame UnBlock RX"]
    #[inline(always)]
    pub fn sfubrx(&self) -> SFUBRX_R {
        SFUBRX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&self) -> MPM_R {
        MPM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&self) -> MPAB_R {
        MPAB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&self) -> BIT8DV_R {
        BIT8DV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RX DMA Wakeup"]
    #[inline(always)]
    pub fn rxdmawu(&self) -> RXDMAWU_R {
        RXDMAWU_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX DMA Wakeup"]
    #[inline(always)]
    pub fn txdmawu(&self) -> TXDMAWU_R {
        TXDMAWU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&self) -> TXDELAY_R {
        TXDELAY_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Automatic Transmitter Tristate"]
    #[inline(always)]
    #[must_use]
    pub fn autotri(&mut self) -> AUTOTRI_W<CTRLrs> {
        AUTOTRI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn databits(&mut self) -> DATABITS_W<CTRLrs> {
        DATABITS_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Parity-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<CTRLrs> {
        PARITY_W::new(self, 2)
    }
    #[doc = "Bit 4 - Stop-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn stopbits(&mut self) -> STOPBITS_W<CTRLrs> {
        STOPBITS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Invert Input and Output"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<CTRLrs> {
        INV_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear RX DMA on Error"]
    #[inline(always)]
    #[must_use]
    pub fn errsdma(&mut self) -> ERRSDMA_W<CTRLrs> {
        ERRSDMA_W::new(self, 6)
    }
    #[doc = "Bit 7 - Loopback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn loopbk(&mut self) -> LOOPBK_W<CTRLrs> {
        LOOPBK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Start-Frame UnBlock RX"]
    #[inline(always)]
    #[must_use]
    pub fn sfubrx(&mut self) -> SFUBRX_W<CTRLrs> {
        SFUBRX_W::new(self, 8)
    }
    #[doc = "Bit 9 - Multi-Processor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mpm(&mut self) -> MPM_W<CTRLrs> {
        MPM_W::new(self, 9)
    }
    #[doc = "Bit 10 - Multi-Processor Address-Bit"]
    #[inline(always)]
    #[must_use]
    pub fn mpab(&mut self) -> MPAB_W<CTRLrs> {
        MPAB_W::new(self, 10)
    }
    #[doc = "Bit 11 - Bit 8 Default Value"]
    #[inline(always)]
    #[must_use]
    pub fn bit8dv(&mut self) -> BIT8DV_W<CTRLrs> {
        BIT8DV_W::new(self, 11)
    }
    #[doc = "Bit 12 - RX DMA Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmawu(&mut self) -> RXDMAWU_W<CTRLrs> {
        RXDMAWU_W::new(self, 12)
    }
    #[doc = "Bit 13 - TX DMA Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn txdmawu(&mut self) -> TXDMAWU_W<CTRLrs> {
        TXDMAWU_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - TX Delay Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txdelay(&mut self) -> TXDELAY_W<CTRLrs> {
        TXDELAY_W::new(self, 14)
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
