#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFSrs>;
#[doc = "Field `START` writer - Set START Interrupt Flag"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTART` writer - Set RSTART Interrupt Flag"]
pub type RstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` writer - Set ADDR Interrupt Flag"]
pub type AddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` writer - Set TXC Interrupt Flag"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` writer - Set ACK Interrupt Flag"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Set NACK Interrupt Flag"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTOP` writer - Set MSTOP Interrupt Flag"]
pub type MstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` writer - Set ARBLOST Interrupt Flag"]
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERR` writer - Set BUSERR Interrupt Flag"]
pub type BuserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSHOLD` writer - Set BUSHOLD Interrupt Flag"]
pub type BusholdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Set TXOF Interrupt Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Set RXUF Interrupt Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITO` writer - Set BITO Interrupt Flag"]
pub type BitoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTO` writer - Set CLTO Interrupt Flag"]
pub type CltoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTOP` writer - Set SSTOP Interrupt Flag"]
pub type SstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` writer - Set RXFULL Interrupt Flag"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLERR` writer - Set CLERR Interrupt Flag"]
pub type ClerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set START Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<IFSrs> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Set RSTART Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rstart(&mut self) -> RstartW<IFSrs> {
        RstartW::new(self, 1)
    }
    #[doc = "Bit 2 - Set ADDR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<IFSrs> {
        AddrW::new(self, 2)
    }
    #[doc = "Bit 3 - Set TXC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<IFSrs> {
        TxcW::new(self, 3)
    }
    #[doc = "Bit 6 - Set ACK Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<IFSrs> {
        AckW::new(self, 6)
    }
    #[doc = "Bit 7 - Set NACK Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<IFSrs> {
        NackW::new(self, 7)
    }
    #[doc = "Bit 8 - Set MSTOP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mstop(&mut self) -> MstopW<IFSrs> {
        MstopW::new(self, 8)
    }
    #[doc = "Bit 9 - Set ARBLOST Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn arblost(&mut self) -> ArblostW<IFSrs> {
        ArblostW::new(self, 9)
    }
    #[doc = "Bit 10 - Set BUSERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BuserrW<IFSrs> {
        BuserrW::new(self, 10)
    }
    #[doc = "Bit 11 - Set BUSHOLD Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bushold(&mut self) -> BusholdW<IFSrs> {
        BusholdW::new(self, 11)
    }
    #[doc = "Bit 12 - Set TXOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TxofW<IFSrs> {
        TxofW::new(self, 12)
    }
    #[doc = "Bit 13 - Set RXUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RxufW<IFSrs> {
        RxufW::new(self, 13)
    }
    #[doc = "Bit 14 - Set BITO Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bito(&mut self) -> BitoW<IFSrs> {
        BitoW::new(self, 14)
    }
    #[doc = "Bit 15 - Set CLTO Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn clto(&mut self) -> CltoW<IFSrs> {
        CltoW::new(self, 15)
    }
    #[doc = "Bit 16 - Set SSTOP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sstop(&mut self) -> SstopW<IFSrs> {
        SstopW::new(self, 16)
    }
    #[doc = "Bit 17 - Set RXFULL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RxfullW<IFSrs> {
        RxfullW::new(self, 17)
    }
    #[doc = "Bit 18 - Set CLERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn clerr(&mut self) -> ClerrW<IFSrs> {
        ClerrW::new(self, 18)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
