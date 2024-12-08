///Register `RSTCAUSE` reader
pub type R = crate::R<RSTCAUSErs>;
///Field `PORST` reader - Power on Reset
pub type PorstR = crate::BitReader;
///Field `AVDDBOD` reader - Brown Out Detector AVDD Reset
pub type AvddbodR = crate::BitReader;
///Field `DVDDBOD` reader - Brown Out Detector DVDD Reset
pub type DvddbodR = crate::BitReader;
///Field `DECBOD` reader - Brown Out Detector Decouple Domain Reset
pub type DecbodR = crate::BitReader;
///Field `EXTRST` reader - External Pin Reset
pub type ExtrstR = crate::BitReader;
///Field `LOCKUPRST` reader - LOCKUP Reset
pub type LockuprstR = crate::BitReader;
///Field `SYSREQRST` reader - System Request Reset
pub type SysreqrstR = crate::BitReader;
///Field `WDOGRST` reader - Watchdog Reset
pub type WdogrstR = crate::BitReader;
///Field `EM4RST` reader - EM4 Reset
pub type Em4rstR = crate::BitReader;
impl R {
    ///Bit 0 - Power on Reset
    #[inline(always)]
    pub fn porst(&self) -> PorstR {
        PorstR::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Brown Out Detector AVDD Reset
    #[inline(always)]
    pub fn avddbod(&self) -> AvddbodR {
        AvddbodR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Brown Out Detector DVDD Reset
    #[inline(always)]
    pub fn dvddbod(&self) -> DvddbodR {
        DvddbodR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Brown Out Detector Decouple Domain Reset
    #[inline(always)]
    pub fn decbod(&self) -> DecbodR {
        DecbodR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - External Pin Reset
    #[inline(always)]
    pub fn extrst(&self) -> ExtrstR {
        ExtrstR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LOCKUP Reset
    #[inline(always)]
    pub fn lockuprst(&self) -> LockuprstR {
        LockuprstR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - System Request Reset
    #[inline(always)]
    pub fn sysreqrst(&self) -> SysreqrstR {
        SysreqrstR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Watchdog Reset
    #[inline(always)]
    pub fn wdogrst(&self) -> WdogrstR {
        WdogrstR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - EM4 Reset
    #[inline(always)]
    pub fn em4rst(&self) -> Em4rstR {
        Em4rstR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTCAUSE")
            .field("porst", &self.porst())
            .field("avddbod", &self.avddbod())
            .field("dvddbod", &self.dvddbod())
            .field("decbod", &self.decbod())
            .field("extrst", &self.extrst())
            .field("lockuprst", &self.lockuprst())
            .field("sysreqrst", &self.sysreqrst())
            .field("wdogrst", &self.wdogrst())
            .field("em4rst", &self.em4rst())
            .finish()
    }
}
///Reset Cause Register
///
///You can [`read`](crate::Reg::read) this register and get [`rstcause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RSTCAUSErs;
impl crate::RegisterSpec for RSTCAUSErs {
    type Ux = u32;
}
///`read()` method returns [`rstcause::R`](R) reader structure
impl crate::Readable for RSTCAUSErs {}
///`reset()` method sets RSTCAUSE to value 0
impl crate::Resettable for RSTCAUSErs {
    const RESET_VALUE: u32 = 0;
}
