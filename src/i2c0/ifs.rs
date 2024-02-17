#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFSrs>;
#[doc = "Field `START` writer - Set START Interrupt Flag"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTART` writer - Set RSTART Interrupt Flag"]
pub type RSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` writer - Set ADDR Interrupt Flag"]
pub type ADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` writer - Set TXC Interrupt Flag"]
pub type TXC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` writer - Set ACK Interrupt Flag"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Set NACK Interrupt Flag"]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTOP` writer - Set MSTOP Interrupt Flag"]
pub type MSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` writer - Set ARBLOST Interrupt Flag"]
pub type ARBLOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERR` writer - Set BUSERR Interrupt Flag"]
pub type BUSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSHOLD` writer - Set BUSHOLD Interrupt Flag"]
pub type BUSHOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Set TXOF Interrupt Flag"]
pub type TXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Set RXUF Interrupt Flag"]
pub type RXUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITO` writer - Set BITO Interrupt Flag"]
pub type BITO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTO` writer - Set CLTO Interrupt Flag"]
pub type CLTO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTOP` writer - Set SSTOP Interrupt Flag"]
pub type SSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` writer - Set RXFULL Interrupt Flag"]
pub type RXFULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLERR` writer - Set CLERR Interrupt Flag"]
pub type CLERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set START Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<IFSrs> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set RSTART Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rstart(&mut self) -> RSTART_W<IFSrs> {
        RSTART_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set ADDR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<IFSrs> {
        ADDR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set TXC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<IFSrs> {
        TXC_W::new(self, 3)
    }
    #[doc = "Bit 6 - Set ACK Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<IFSrs> {
        ACK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set NACK Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<IFSrs> {
        NACK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set MSTOP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mstop(&mut self) -> MSTOP_W<IFSrs> {
        MSTOP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set ARBLOST Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn arblost(&mut self) -> ARBLOST_W<IFSrs> {
        ARBLOST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set BUSERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BUSERR_W<IFSrs> {
        BUSERR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set BUSHOLD Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bushold(&mut self) -> BUSHOLD_W<IFSrs> {
        BUSHOLD_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set TXOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<IFSrs> {
        TXOF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set RXUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<IFSrs> {
        RXUF_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set BITO Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bito(&mut self) -> BITO_W<IFSrs> {
        BITO_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set CLTO Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn clto(&mut self) -> CLTO_W<IFSrs> {
        CLTO_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set SSTOP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sstop(&mut self) -> SSTOP_W<IFSrs> {
        SSTOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set RXFULL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<IFSrs> {
        RXFULL_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set CLERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn clerr(&mut self) -> CLERR_W<IFSrs> {
        CLERR_W::new(self, 18)
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
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFSrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
