///Register `STATUS` reader
pub type R = crate::R<STATUSrs>;
///Field `ANYBUSY` reader - Any DMA Channel Busy
pub type AnybusyR = crate::BitReader;
///Field `ANYREQ` reader - Any DMA Channel Request Pending
pub type AnyreqR = crate::BitReader;
///Field `CHGRANT` reader - Granted Channel Number
pub type ChgrantR = crate::FieldReader;
///Field `CHERROR` reader - Errant Channel Number
pub type CherrorR = crate::FieldReader;
///Field `FIFOLEVEL` reader - FIFO Level
pub type FifolevelR = crate::FieldReader;
///Field `CHNUM` reader - Number of Channels
pub type ChnumR = crate::FieldReader;
impl R {
    ///Bit 0 - Any DMA Channel Busy
    #[inline(always)]
    pub fn anybusy(&self) -> AnybusyR {
        AnybusyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Any DMA Channel Request Pending
    #[inline(always)]
    pub fn anyreq(&self) -> AnyreqR {
        AnyreqR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 3:5 - Granted Channel Number
    #[inline(always)]
    pub fn chgrant(&self) -> ChgrantR {
        ChgrantR::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 8:10 - Errant Channel Number
    #[inline(always)]
    pub fn cherror(&self) -> CherrorR {
        CherrorR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 16:20 - FIFO Level
    #[inline(always)]
    pub fn fifolevel(&self) -> FifolevelR {
        FifolevelR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:28 - Number of Channels
    #[inline(always)]
    pub fn chnum(&self) -> ChnumR {
        ChnumR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("anybusy", &self.anybusy())
            .field("anyreq", &self.anyreq())
            .field("chgrant", &self.chgrant())
            .field("cherror", &self.cherror())
            .field("fifolevel", &self.fifolevel())
            .field("chnum", &self.chnum())
            .finish()
    }
}
///DMA Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUSrs {}
///`reset()` method sets STATUS to value 0x0810_0000
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0x0810_0000;
}
