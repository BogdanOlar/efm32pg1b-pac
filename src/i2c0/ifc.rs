///Register `IFC` writer
pub type W = crate::W<IFCrs>;
///Field `START` writer - Clear START Interrupt Flag
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTART` writer - Clear RSTART Interrupt Flag
pub type RstartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDR` writer - Clear ADDR Interrupt Flag
pub type AddrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXC` writer - Clear TXC Interrupt Flag
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACK` writer - Clear ACK Interrupt Flag
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACK` writer - Clear NACK Interrupt Flag
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTOP` writer - Clear MSTOP Interrupt Flag
pub type MstopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARBLOST` writer - Clear ARBLOST Interrupt Flag
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSERR` writer - Clear BUSERR Interrupt Flag
pub type BuserrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSHOLD` writer - Clear BUSHOLD Interrupt Flag
pub type BusholdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOF` writer - Clear TXOF Interrupt Flag
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXUF` writer - Clear RXUF Interrupt Flag
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BITO` writer - Clear BITO Interrupt Flag
pub type BitoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLTO` writer - Clear CLTO Interrupt Flag
pub type CltoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSTOP` writer - Clear SSTOP Interrupt Flag
pub type SstopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFULL` writer - Clear RXFULL Interrupt Flag
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLERR` writer - Clear CLERR Interrupt Flag
pub type ClerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear START Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<IFCrs> {
        StartW::new(self, 0)
    }
    ///Bit 1 - Clear RSTART Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn rstart(&mut self) -> RstartW<IFCrs> {
        RstartW::new(self, 1)
    }
    ///Bit 2 - Clear ADDR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<IFCrs> {
        AddrW::new(self, 2)
    }
    ///Bit 3 - Clear TXC Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<IFCrs> {
        TxcW::new(self, 3)
    }
    ///Bit 6 - Clear ACK Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<IFCrs> {
        AckW::new(self, 6)
    }
    ///Bit 7 - Clear NACK Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<IFCrs> {
        NackW::new(self, 7)
    }
    ///Bit 8 - Clear MSTOP Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn mstop(&mut self) -> MstopW<IFCrs> {
        MstopW::new(self, 8)
    }
    ///Bit 9 - Clear ARBLOST Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn arblost(&mut self) -> ArblostW<IFCrs> {
        ArblostW::new(self, 9)
    }
    ///Bit 10 - Clear BUSERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BuserrW<IFCrs> {
        BuserrW::new(self, 10)
    }
    ///Bit 11 - Clear BUSHOLD Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn bushold(&mut self) -> BusholdW<IFCrs> {
        BusholdW::new(self, 11)
    }
    ///Bit 12 - Clear TXOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TxofW<IFCrs> {
        TxofW::new(self, 12)
    }
    ///Bit 13 - Clear RXUF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RxufW<IFCrs> {
        RxufW::new(self, 13)
    }
    ///Bit 14 - Clear BITO Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn bito(&mut self) -> BitoW<IFCrs> {
        BitoW::new(self, 14)
    }
    ///Bit 15 - Clear CLTO Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn clto(&mut self) -> CltoW<IFCrs> {
        CltoW::new(self, 15)
    }
    ///Bit 16 - Clear SSTOP Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn sstop(&mut self) -> SstopW<IFCrs> {
        SstopW::new(self, 16)
    }
    ///Bit 17 - Clear RXFULL Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RxfullW<IFCrs> {
        RxfullW::new(self, 17)
    }
    ///Bit 18 - Clear CLERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn clerr(&mut self) -> ClerrW<IFCrs> {
        ClerrW::new(self, 18)
    }
}
///Interrupt Flag Clear Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFCrs;
impl crate::RegisterSpec for IFCrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifc::W`](W) writer structure
impl crate::Writable for IFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IFC to value 0
impl crate::Resettable for IFCrs {
    const RESET_VALUE: u32 = 0;
}
