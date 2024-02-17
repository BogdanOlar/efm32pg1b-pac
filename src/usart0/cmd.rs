#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMDrs>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RXDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TXDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTEREN` writer - Master Enable"]
pub type MASTEREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTERDIS` writer - Master Disable"]
pub type MASTERDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBLOCKEN` writer - Receiver Block Enable"]
pub type RXBLOCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBLOCKDIS` writer - Receiver Block Disable"]
pub type RXBLOCKDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIEN` writer - Transmitter Tristate Enable"]
pub type TXTRIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIDIS` writer - Transmitter Tristate Disable"]
pub type TXTRIDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARTX` writer - Clear TX"]
pub type CLEARTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARRX` writer - Clear RX"]
pub type CLEARRX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<CMDrs> {
        RXEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RXDIS_W<CMDrs> {
        RXDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<CMDrs> {
        TXEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TXDIS_W<CMDrs> {
        TXDIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Master Enable"]
    #[inline(always)]
    #[must_use]
    pub fn masteren(&mut self) -> MASTEREN_W<CMDrs> {
        MASTEREN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn masterdis(&mut self) -> MASTERDIS_W<CMDrs> {
        MASTERDIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receiver Block Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxblocken(&mut self) -> RXBLOCKEN_W<CMDrs> {
        RXBLOCKEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receiver Block Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxblockdis(&mut self) -> RXBLOCKDIS_W<CMDrs> {
        RXBLOCKDIS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Transmitter Tristate Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txtrien(&mut self) -> TXTRIEN_W<CMDrs> {
        TXTRIEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Transmitter Tristate Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txtridis(&mut self) -> TXTRIDIS_W<CMDrs> {
        TXTRIDIS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear TX"]
    #[inline(always)]
    #[must_use]
    pub fn cleartx(&mut self) -> CLEARTX_W<CMDrs> {
        CLEARTX_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear RX"]
    #[inline(always)]
    #[must_use]
    pub fn clearrx(&mut self) -> CLEARRX_W<CMDrs> {
        CLEARRX_W::new(self, 11)
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
#[doc = "Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMDrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMDrs {
    const RESET_VALUE: u32 = 0;
}
