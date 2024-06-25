#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMDrs>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBLOCKEN` writer - Receiver Block Enable"]
pub type RxblockenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBLOCKDIS` writer - Receiver Block Disable"]
pub type RxblockdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARTX` writer - Clear TX"]
pub type CleartxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARRX` writer - Clear RX"]
pub type ClearrxW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CMDrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RxenW<CMDrs> {
        RxenW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RxdisW<CMDrs> {
        RxdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TxenW<CMDrs> {
        TxenW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TxdisW<CMDrs> {
        TxdisW::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Block Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxblocken(&mut self) -> RxblockenW<CMDrs> {
        RxblockenW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Block Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxblockdis(&mut self) -> RxblockdisW<CMDrs> {
        RxblockdisW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear TX"]
    #[inline(always)]
    #[must_use]
    pub fn cleartx(&mut self) -> CleartxW<CMDrs> {
        CleartxW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear RX"]
    #[inline(always)]
    #[must_use]
    pub fn clearrx(&mut self) -> ClearrxW<CMDrs> {
        ClearrxW::new(self, 7)
    }
}
#[doc = "Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMDrs {
    const RESET_VALUE: u32 = 0;
}
