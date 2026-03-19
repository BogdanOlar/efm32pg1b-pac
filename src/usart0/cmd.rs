///Register `CMD` writer
pub type W = crate::W<CMDrs>;
///Field `RXEN` writer - Receiver Enable
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXDIS` writer - Receiver Disable
pub type RxdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXEN` writer - Transmitter Enable
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXDIS` writer - Transmitter Disable
pub type TxdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASTEREN` writer - Master Enable
pub type MasterenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASTERDIS` writer - Master Disable
pub type MasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXBLOCKEN` writer - Receiver Block Enable
pub type RxblockenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXBLOCKDIS` writer - Receiver Block Disable
pub type RxblockdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXTRIEN` writer - Transmitter Tristate Enable
pub type TxtrienW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXTRIDIS` writer - Transmitter Tristate Disable
pub type TxtridisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLEARTX` writer - Clear TX
pub type CleartxW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLEARRX` writer - Clear RX
pub type ClearrxW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CMDrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Receiver Enable
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<'_, CMDrs> {
        RxenW::new(self, 0)
    }
    ///Bit 1 - Receiver Disable
    #[inline(always)]
    pub fn rxdis(&mut self) -> RxdisW<'_, CMDrs> {
        RxdisW::new(self, 1)
    }
    ///Bit 2 - Transmitter Enable
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<'_, CMDrs> {
        TxenW::new(self, 2)
    }
    ///Bit 3 - Transmitter Disable
    #[inline(always)]
    pub fn txdis(&mut self) -> TxdisW<'_, CMDrs> {
        TxdisW::new(self, 3)
    }
    ///Bit 4 - Master Enable
    #[inline(always)]
    pub fn masteren(&mut self) -> MasterenW<'_, CMDrs> {
        MasterenW::new(self, 4)
    }
    ///Bit 5 - Master Disable
    #[inline(always)]
    pub fn masterdis(&mut self) -> MasterdisW<'_, CMDrs> {
        MasterdisW::new(self, 5)
    }
    ///Bit 6 - Receiver Block Enable
    #[inline(always)]
    pub fn rxblocken(&mut self) -> RxblockenW<'_, CMDrs> {
        RxblockenW::new(self, 6)
    }
    ///Bit 7 - Receiver Block Disable
    #[inline(always)]
    pub fn rxblockdis(&mut self) -> RxblockdisW<'_, CMDrs> {
        RxblockdisW::new(self, 7)
    }
    ///Bit 8 - Transmitter Tristate Enable
    #[inline(always)]
    pub fn txtrien(&mut self) -> TxtrienW<'_, CMDrs> {
        TxtrienW::new(self, 8)
    }
    ///Bit 9 - Transmitter Tristate Disable
    #[inline(always)]
    pub fn txtridis(&mut self) -> TxtridisW<'_, CMDrs> {
        TxtridisW::new(self, 9)
    }
    ///Bit 10 - Clear TX
    #[inline(always)]
    pub fn cleartx(&mut self) -> CleartxW<'_, CMDrs> {
        CleartxW::new(self, 10)
    }
    ///Bit 11 - Clear RX
    #[inline(always)]
    pub fn clearrx(&mut self) -> ClearrxW<'_, CMDrs> {
        ClearrxW::new(self, 11)
    }
}
///Command Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cmd::W`](W) writer structure
impl crate::Writable for CMDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMD to value 0
impl crate::Resettable for CMDrs {}
