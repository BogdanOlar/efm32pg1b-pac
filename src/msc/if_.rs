///Register `IF` reader
pub type R = crate::R<IFrs>;
///Field `ERASE` reader - Erase Done Interrupt Read Flag
pub type EraseR = crate::BitReader;
///Field `WRITE` reader - Write Done Interrupt Read Flag
pub type WriteR = crate::BitReader;
///Field `CHOF` reader - Cache Hits Overflow Interrupt Flag
pub type ChofR = crate::BitReader;
///Field `CMOF` reader - Cache Misses Overflow Interrupt Flag
pub type CmofR = crate::BitReader;
///Field `PWRUPF` reader - Flash Power Up Sequence Complete Flag
pub type PwrupfR = crate::BitReader;
///Field `ICACHERR` reader - ICache RAM Parity Error Flag
pub type IcacherrR = crate::BitReader;
impl R {
    ///Bit 0 - Erase Done Interrupt Read Flag
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Write Done Interrupt Read Flag
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Cache Hits Overflow Interrupt Flag
    #[inline(always)]
    pub fn chof(&self) -> ChofR {
        ChofR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Cache Misses Overflow Interrupt Flag
    #[inline(always)]
    pub fn cmof(&self) -> CmofR {
        CmofR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Flash Power Up Sequence Complete Flag
    #[inline(always)]
    pub fn pwrupf(&self) -> PwrupfR {
        PwrupfR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ICache RAM Parity Error Flag
    #[inline(always)]
    pub fn icacherr(&self) -> IcacherrR {
        IcacherrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("erase", &self.erase())
            .field("write", &self.write())
            .field("chof", &self.chof())
            .field("cmof", &self.cmof())
            .field("pwrupf", &self.pwrupf())
            .field("icacherr", &self.icacherr())
            .finish()
    }
}
///Interrupt Flag Register
///
///You can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
///`read()` method returns [`if_::R`](R) reader structure
impl crate::Readable for IFrs {}
///`reset()` method sets IF to value 0
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0;
}
