///Register `RXDOUBLEX` reader
pub type R = crate::R<RXDOUBLEXrs>;
///Field `RXDATA0` reader - RX Data 0
pub type Rxdata0R = crate::FieldReader<u16>;
///Field `PERR0` reader - Data Parity Error 0
pub type Perr0R = crate::BitReader;
///Field `FERR0` reader - Data Framing Error 0
pub type Ferr0R = crate::BitReader;
///Field `RXDATA1` reader - RX Data 1
pub type Rxdata1R = crate::FieldReader<u16>;
///Field `PERR1` reader - Data Parity Error 1
pub type Perr1R = crate::BitReader;
///Field `FERR1` reader - Data Framing Error 1
pub type Ferr1R = crate::BitReader;
impl R {
    ///Bits 0:8 - RX Data 0
    #[inline(always)]
    pub fn rxdata0(&self) -> Rxdata0R {
        Rxdata0R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 14 - Data Parity Error 0
    #[inline(always)]
    pub fn perr0(&self) -> Perr0R {
        Perr0R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Data Framing Error 0
    #[inline(always)]
    pub fn ferr0(&self) -> Ferr0R {
        Ferr0R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:24 - RX Data 1
    #[inline(always)]
    pub fn rxdata1(&self) -> Rxdata1R {
        Rxdata1R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bit 30 - Data Parity Error 1
    #[inline(always)]
    pub fn perr1(&self) -> Perr1R {
        Perr1R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Data Framing Error 1
    #[inline(always)]
    pub fn ferr1(&self) -> Ferr1R {
        Ferr1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXDOUBLEXrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
///RX Buffer Double Data Extended Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdoublex::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct RXDOUBLEXrs;
impl crate::RegisterSpec for RXDOUBLEXrs {
    type Ux = u32;
}
///`read()` method returns [`rxdoublex::R`](R) reader structure
impl crate::Readable for RXDOUBLEXrs {}
///`reset()` method sets RXDOUBLEX to value 0
impl crate::Resettable for RXDOUBLEXrs {
    const RESET_VALUE: u32 = 0;
}
