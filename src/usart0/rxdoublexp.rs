///Register `RXDOUBLEXP` reader
pub type R = crate::R<RXDOUBLEXPrs>;
///Field `RXDATAP0` reader - RX Data 0 Peek
pub type Rxdatap0R = crate::FieldReader<u16>;
///Field `PERRP0` reader - Data Parity Error 0 Peek
pub type Perrp0R = crate::BitReader;
///Field `FERRP0` reader - Data Framing Error 0 Peek
pub type Ferrp0R = crate::BitReader;
///Field `RXDATAP1` reader - RX Data 1 Peek
pub type Rxdatap1R = crate::FieldReader<u16>;
///Field `PERRP1` reader - Data Parity Error 1 Peek
pub type Perrp1R = crate::BitReader;
///Field `FERRP1` reader - Data Framing Error 1 Peek
pub type Ferrp1R = crate::BitReader;
impl R {
    ///Bits 0:8 - RX Data 0 Peek
    #[inline(always)]
    pub fn rxdatap0(&self) -> Rxdatap0R {
        Rxdatap0R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 14 - Data Parity Error 0 Peek
    #[inline(always)]
    pub fn perrp0(&self) -> Perrp0R {
        Perrp0R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Data Framing Error 0 Peek
    #[inline(always)]
    pub fn ferrp0(&self) -> Ferrp0R {
        Ferrp0R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:24 - RX Data 1 Peek
    #[inline(always)]
    pub fn rxdatap1(&self) -> Rxdatap1R {
        Rxdatap1R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bit 30 - Data Parity Error 1 Peek
    #[inline(always)]
    pub fn perrp1(&self) -> Perrp1R {
        Perrp1R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Data Framing Error 1 Peek
    #[inline(always)]
    pub fn ferrp1(&self) -> Ferrp1R {
        Ferrp1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDOUBLEXP")
            .field("rxdatap0", &self.rxdatap0())
            .field("perrp0", &self.perrp0())
            .field("ferrp0", &self.ferrp0())
            .field("rxdatap1", &self.rxdatap1())
            .field("perrp1", &self.perrp1())
            .field("ferrp1", &self.ferrp1())
            .finish()
    }
}
///RX Buffer Double Data Extended Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdoublexp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RXDOUBLEXPrs;
impl crate::RegisterSpec for RXDOUBLEXPrs {
    type Ux = u32;
}
///`read()` method returns [`rxdoublexp::R`](R) reader structure
impl crate::Readable for RXDOUBLEXPrs {}
///`reset()` method sets RXDOUBLEXP to value 0
impl crate::Resettable for RXDOUBLEXPrs {
    const RESET_VALUE: u32 = 0;
}
