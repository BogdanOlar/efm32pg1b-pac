///Register `TXDATAX` reader
pub type R = crate::R<TXDATAXrs>;
///Register `TXDATAX` writer
pub type W = crate::W<TXDATAXrs>;
///Field `TXDATA` reader - TX Data
pub type TxdataR = crate::FieldReader<u16>;
///Field `TXDATA` writer - TX Data
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `TXBREAK` reader - Transmit Data as Break
pub type TxbreakR = crate::BitReader;
///Field `TXBREAK` writer - Transmit Data as Break
pub type TxbreakW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXDISAT` reader - Disable TX After Transmission
pub type TxdisatR = crate::BitReader;
///Field `TXDISAT` writer - Disable TX After Transmission
pub type TxdisatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXENAT` reader - Enable RX After Transmission
pub type RxenatR = crate::BitReader;
///Field `RXENAT` writer - Enable RX After Transmission
pub type RxenatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:8 - TX Data
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 13 - Transmit Data as Break
    #[inline(always)]
    pub fn txbreak(&self) -> TxbreakR {
        TxbreakR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Disable TX After Transmission
    #[inline(always)]
    pub fn txdisat(&self) -> TxdisatR {
        TxdisatR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Enable RX After Transmission
    #[inline(always)]
    pub fn rxenat(&self) -> RxenatR {
        RxenatR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXDATAX")
            .field("txdata", &self.txdata())
            .field("txbreak", &self.txbreak())
            .field("txdisat", &self.txdisat())
            .field("rxenat", &self.rxenat())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - TX Data
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TxdataW<TXDATAXrs> {
        TxdataW::new(self, 0)
    }
    ///Bit 13 - Transmit Data as Break
    #[inline(always)]
    #[must_use]
    pub fn txbreak(&mut self) -> TxbreakW<TXDATAXrs> {
        TxbreakW::new(self, 13)
    }
    ///Bit 14 - Disable TX After Transmission
    #[inline(always)]
    #[must_use]
    pub fn txdisat(&mut self) -> TxdisatW<TXDATAXrs> {
        TxdisatW::new(self, 14)
    }
    ///Bit 15 - Enable RX After Transmission
    #[inline(always)]
    #[must_use]
    pub fn rxenat(&mut self) -> RxenatW<TXDATAXrs> {
        RxenatW::new(self, 15)
    }
}
///Transmit Buffer Data Extended Register
///
///You can [`read`](crate::Reg::read) this register and get [`txdatax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdatax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TXDATAXrs;
impl crate::RegisterSpec for TXDATAXrs {
    type Ux = u32;
}
///`read()` method returns [`txdatax::R`](R) reader structure
impl crate::Readable for TXDATAXrs {}
///`write(|w| ..)` method takes [`txdatax::W`](W) writer structure
impl crate::Writable for TXDATAXrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TXDATAX to value 0
impl crate::Resettable for TXDATAXrs {
    const RESET_VALUE: u32 = 0;
}
