#[doc = "Register `TXDOUBLEX` reader"]
pub type R = crate::R<TXDOUBLEXrs>;
#[doc = "Register `TXDOUBLEX` writer"]
pub type W = crate::W<TXDOUBLEXrs>;
#[doc = "Field `TXDATA0` reader - TX Data"]
pub type Txdata0R = crate::FieldReader<u16>;
#[doc = "Field `TXDATA0` writer - TX Data"]
pub type Txdata0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `UBRXAT0` reader - Unblock RX After Transmission"]
pub type Ubrxat0R = crate::BitReader;
#[doc = "Field `UBRXAT0` writer - Unblock RX After Transmission"]
pub type Ubrxat0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIAT0` reader - Set TXTRI After Transmission"]
pub type Txtriat0R = crate::BitReader;
#[doc = "Field `TXTRIAT0` writer - Set TXTRI After Transmission"]
pub type Txtriat0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBREAK0` reader - Transmit Data as Break"]
pub type Txbreak0R = crate::BitReader;
#[doc = "Field `TXBREAK0` writer - Transmit Data as Break"]
pub type Txbreak0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDISAT0` reader - Clear TXEN After Transmission"]
pub type Txdisat0R = crate::BitReader;
#[doc = "Field `TXDISAT0` writer - Clear TXEN After Transmission"]
pub type Txdisat0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXENAT0` reader - Enable RX After Transmission"]
pub type Rxenat0R = crate::BitReader;
#[doc = "Field `RXENAT0` writer - Enable RX After Transmission"]
pub type Rxenat0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDATA1` reader - TX Data"]
pub type Txdata1R = crate::FieldReader<u16>;
#[doc = "Field `TXDATA1` writer - TX Data"]
pub type Txdata1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `UBRXAT1` reader - Unblock RX After Transmission"]
pub type Ubrxat1R = crate::BitReader;
#[doc = "Field `UBRXAT1` writer - Unblock RX After Transmission"]
pub type Ubrxat1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIAT1` reader - Set TXTRI After Transmission"]
pub type Txtriat1R = crate::BitReader;
#[doc = "Field `TXTRIAT1` writer - Set TXTRI After Transmission"]
pub type Txtriat1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBREAK1` reader - Transmit Data as Break"]
pub type Txbreak1R = crate::BitReader;
#[doc = "Field `TXBREAK1` writer - Transmit Data as Break"]
pub type Txbreak1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDISAT1` reader - Clear TXEN After Transmission"]
pub type Txdisat1R = crate::BitReader;
#[doc = "Field `TXDISAT1` writer - Clear TXEN After Transmission"]
pub type Txdisat1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXENAT1` reader - Enable RX After Transmission"]
pub type Rxenat1R = crate::BitReader;
#[doc = "Field `RXENAT1` writer - Enable RX After Transmission"]
pub type Rxenat1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    pub fn txdata0(&self) -> Txdata0R {
        Txdata0R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat0(&self) -> Ubrxat0R {
        Ubrxat0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat0(&self) -> Txtriat0R {
        Txtriat0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak0(&self) -> Txbreak0R {
        Txbreak0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat0(&self) -> Txdisat0R {
        Txdisat0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat0(&self) -> Rxenat0R {
        Rxenat0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - TX Data"]
    #[inline(always)]
    pub fn txdata1(&self) -> Txdata1R {
        Txdata1R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat1(&self) -> Ubrxat1R {
        Ubrxat1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat1(&self) -> Txtriat1R {
        Txtriat1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak1(&self) -> Txbreak1R {
        Txbreak1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat1(&self) -> Txdisat1R {
        Txdisat1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat1(&self) -> Rxenat1R {
        Rxenat1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata0(&mut self) -> Txdata0W<TXDOUBLEXrs> {
        Txdata0W::new(self, 0)
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ubrxat0(&mut self) -> Ubrxat0W<TXDOUBLEXrs> {
        Ubrxat0W::new(self, 11)
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txtriat0(&mut self) -> Txtriat0W<TXDOUBLEXrs> {
        Txtriat0W::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Data as Break"]
    #[inline(always)]
    #[must_use]
    pub fn txbreak0(&mut self) -> Txbreak0W<TXDOUBLEXrs> {
        Txbreak0W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txdisat0(&mut self) -> Txdisat0W<TXDOUBLEXrs> {
        Txdisat0W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn rxenat0(&mut self) -> Rxenat0W<TXDOUBLEXrs> {
        Rxenat0W::new(self, 15)
    }
    #[doc = "Bits 16:24 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata1(&mut self) -> Txdata1W<TXDOUBLEXrs> {
        Txdata1W::new(self, 16)
    }
    #[doc = "Bit 27 - Unblock RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ubrxat1(&mut self) -> Ubrxat1W<TXDOUBLEXrs> {
        Ubrxat1W::new(self, 27)
    }
    #[doc = "Bit 28 - Set TXTRI After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txtriat1(&mut self) -> Txtriat1W<TXDOUBLEXrs> {
        Txtriat1W::new(self, 28)
    }
    #[doc = "Bit 29 - Transmit Data as Break"]
    #[inline(always)]
    #[must_use]
    pub fn txbreak1(&mut self) -> Txbreak1W<TXDOUBLEXrs> {
        Txbreak1W::new(self, 29)
    }
    #[doc = "Bit 30 - Clear TXEN After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txdisat1(&mut self) -> Txdisat1W<TXDOUBLEXrs> {
        Txdisat1W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn rxenat1(&mut self) -> Rxenat1W<TXDOUBLEXrs> {
        Rxenat1W::new(self, 31)
    }
}
#[doc = "TX Buffer Double Data Extended Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdoublex::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdoublex::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDOUBLEXrs;
impl crate::RegisterSpec for TXDOUBLEXrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdoublex::R`](R) reader structure"]
impl crate::Readable for TXDOUBLEXrs {}
#[doc = "`write(|w| ..)` method takes [`txdoublex::W`](W) writer structure"]
impl crate::Writable for TXDOUBLEXrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDOUBLEX to value 0"]
impl crate::Resettable for TXDOUBLEXrs {
    const RESET_VALUE: u32 = 0;
}
