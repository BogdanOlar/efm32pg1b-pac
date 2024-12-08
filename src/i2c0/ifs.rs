///Register `IFS` writer
pub type W = crate::W<IFSrs>;
///Field `START` writer - Set START Interrupt Flag
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTART` writer - Set RSTART Interrupt Flag
pub type RstartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDR` writer - Set ADDR Interrupt Flag
pub type AddrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXC` writer - Set TXC Interrupt Flag
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACK` writer - Set ACK Interrupt Flag
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACK` writer - Set NACK Interrupt Flag
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTOP` writer - Set MSTOP Interrupt Flag
pub type MstopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARBLOST` writer - Set ARBLOST Interrupt Flag
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSERR` writer - Set BUSERR Interrupt Flag
pub type BuserrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSHOLD` writer - Set BUSHOLD Interrupt Flag
pub type BusholdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOF` writer - Set TXOF Interrupt Flag
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXUF` writer - Set RXUF Interrupt Flag
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BITO` writer - Set BITO Interrupt Flag
pub type BitoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLTO` writer - Set CLTO Interrupt Flag
pub type CltoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSTOP` writer - Set SSTOP Interrupt Flag
pub type SstopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFULL` writer - Set RXFULL Interrupt Flag
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLERR` writer - Set CLERR Interrupt Flag
pub type ClerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFSrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set START Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<IFSrs> {
        StartW::new(self, 0)
    }
    ///Bit 1 - Set RSTART Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn rstart(&mut self) -> RstartW<IFSrs> {
        RstartW::new(self, 1)
    }
    ///Bit 2 - Set ADDR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<IFSrs> {
        AddrW::new(self, 2)
    }
    ///Bit 3 - Set TXC Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<IFSrs> {
        TxcW::new(self, 3)
    }
    ///Bit 6 - Set ACK Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<IFSrs> {
        AckW::new(self, 6)
    }
    ///Bit 7 - Set NACK Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<IFSrs> {
        NackW::new(self, 7)
    }
    ///Bit 8 - Set MSTOP Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn mstop(&mut self) -> MstopW<IFSrs> {
        MstopW::new(self, 8)
    }
    ///Bit 9 - Set ARBLOST Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn arblost(&mut self) -> ArblostW<IFSrs> {
        ArblostW::new(self, 9)
    }
    ///Bit 10 - Set BUSERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BuserrW<IFSrs> {
        BuserrW::new(self, 10)
    }
    ///Bit 11 - Set BUSHOLD Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn bushold(&mut self) -> BusholdW<IFSrs> {
        BusholdW::new(self, 11)
    }
    ///Bit 12 - Set TXOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TxofW<IFSrs> {
        TxofW::new(self, 12)
    }
    ///Bit 13 - Set RXUF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RxufW<IFSrs> {
        RxufW::new(self, 13)
    }
    ///Bit 14 - Set BITO Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn bito(&mut self) -> BitoW<IFSrs> {
        BitoW::new(self, 14)
    }
    ///Bit 15 - Set CLTO Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn clto(&mut self) -> CltoW<IFSrs> {
        CltoW::new(self, 15)
    }
    ///Bit 16 - Set SSTOP Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn sstop(&mut self) -> SstopW<IFSrs> {
        SstopW::new(self, 16)
    }
    ///Bit 17 - Set RXFULL Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RxfullW<IFSrs> {
        RxfullW::new(self, 17)
    }
    ///Bit 18 - Set CLERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn clerr(&mut self) -> ClerrW<IFSrs> {
        ClerrW::new(self, 18)
    }
}
///Interrupt Flag Set Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifs::W`](W) writer structure
impl crate::Writable for IFSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IFS to value 0
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
