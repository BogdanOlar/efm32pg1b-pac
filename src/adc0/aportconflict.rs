///Register `APORTCONFLICT` reader
pub type R = crate::R<APORTCONFLICTrs>;
///Field `APORT0XCONFLICT` reader - 1 If the Bus Connected to APORT0X is in Conflict With Another Peripheral
pub type Aport0xconflictR = crate::BitReader;
///Field `APORT0YCONFLICT` reader - 1 If the Bus Connected to APORT0Y is in Conflict With Another Peripheral
pub type Aport0yconflictR = crate::BitReader;
///Field `APORT1XCONFLICT` reader - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral
pub type Aport1xconflictR = crate::BitReader;
///Field `APORT1YCONFLICT` reader - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral
pub type Aport1yconflictR = crate::BitReader;
///Field `APORT2XCONFLICT` reader - 1 If the Bus Connected to APORT2X is in Conflict With Another Peripheral
pub type Aport2xconflictR = crate::BitReader;
///Field `APORT2YCONFLICT` reader - 1 If the Bus Connected to APORT2Y is in Conflict With Another Peripheral
pub type Aport2yconflictR = crate::BitReader;
///Field `APORT3XCONFLICT` reader - 1 If the Bus Connected to APORT3X is in Conflict With Another Peripheral
pub type Aport3xconflictR = crate::BitReader;
///Field `APORT3YCONFLICT` reader - 1 If the Bus Connected to APORT3Y is in Conflict With Another Peripheral
pub type Aport3yconflictR = crate::BitReader;
///Field `APORT4XCONFLICT` reader - 1 If the Bus Connected to APORT4X is in Conflict With Another Peripheral
pub type Aport4xconflictR = crate::BitReader;
///Field `APORT4YCONFLICT` reader - 1 If the Bus Connected to APORT4Y is in Conflict With Another Peripheral
pub type Aport4yconflictR = crate::BitReader;
impl R {
    ///Bit 0 - 1 If the Bus Connected to APORT0X is in Conflict With Another Peripheral
    #[inline(always)]
    pub fn aport0xconflict(&self) -> Aport0xconflictR {
        Aport0xconflictR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1 If the Bus Connected to APORT0Y is in Conflict With Another Peripheral
    #[inline(always)]
    pub fn aport0yconflict(&self) -> Aport0yconflictR {
        Aport0yconflictR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral
    #[inline(always)]
    pub fn aport1xconflict(&self) -> Aport1xconflictR {
        Aport1xconflictR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral
    #[inline(always)]
    pub fn aport1yconflict(&self) -> Aport1yconflictR {
        Aport1yconflictR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - 1 If the Bus Connected to APORT2X is in Conflict With Another Peripheral
    #[inline(always)]
    pub fn aport2xconflict(&self) -> Aport2xconflictR {
        Aport2xconflictR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - 1 If the Bus Connected to APORT2Y is in Conflict With Another Peripheral
    #[inline(always)]
    pub fn aport2yconflict(&self) -> Aport2yconflictR {
        Aport2yconflictR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - 1 If the Bus Connected to APORT3X is in Conflict With Another Peripheral
    #[inline(always)]
    pub fn aport3xconflict(&self) -> Aport3xconflictR {
        Aport3xconflictR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - 1 If the Bus Connected to APORT3Y is in Conflict With Another Peripheral
    #[inline(always)]
    pub fn aport3yconflict(&self) -> Aport3yconflictR {
        Aport3yconflictR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - 1 If the Bus Connected to APORT4X is in Conflict With Another Peripheral
    #[inline(always)]
    pub fn aport4xconflict(&self) -> Aport4xconflictR {
        Aport4xconflictR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - 1 If the Bus Connected to APORT4Y is in Conflict With Another Peripheral
    #[inline(always)]
    pub fn aport4yconflict(&self) -> Aport4yconflictR {
        Aport4yconflictR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APORTCONFLICT")
            .field("aport0xconflict", &self.aport0xconflict())
            .field("aport0yconflict", &self.aport0yconflict())
            .field("aport1xconflict", &self.aport1xconflict())
            .field("aport1yconflict", &self.aport1yconflict())
            .field("aport2xconflict", &self.aport2xconflict())
            .field("aport2yconflict", &self.aport2yconflict())
            .field("aport3xconflict", &self.aport3xconflict())
            .field("aport3yconflict", &self.aport3yconflict())
            .field("aport4xconflict", &self.aport4xconflict())
            .field("aport4yconflict", &self.aport4yconflict())
            .finish()
    }
}
///APORT Conflict Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`aportconflict::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct APORTCONFLICTrs;
impl crate::RegisterSpec for APORTCONFLICTrs {
    type Ux = u32;
}
///`read()` method returns [`aportconflict::R`](R) reader structure
impl crate::Readable for APORTCONFLICTrs {}
///`reset()` method sets APORTCONFLICT to value 0
impl crate::Resettable for APORTCONFLICTrs {
    const RESET_VALUE: u32 = 0;
}
