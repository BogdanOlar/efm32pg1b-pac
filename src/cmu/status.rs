///Register `STATUS` reader
pub type R = crate::R<STATUSrs>;
///Field `HFRCOENS` reader - HFRCO Enable Status
pub type HfrcoensR = crate::BitReader;
///Field `HFRCORDY` reader - HFRCO Ready
pub type HfrcordyR = crate::BitReader;
///Field `HFXOENS` reader - HFXO Enable Status
pub type HfxoensR = crate::BitReader;
///Field `HFXORDY` reader - HFXO Ready
pub type HfxordyR = crate::BitReader;
///Field `AUXHFRCOENS` reader - AUXHFRCO Enable Status
pub type AuxhfrcoensR = crate::BitReader;
///Field `AUXHFRCORDY` reader - AUXHFRCO Ready
pub type AuxhfrcordyR = crate::BitReader;
///Field `LFRCOENS` reader - LFRCO Enable Status
pub type LfrcoensR = crate::BitReader;
///Field `LFRCORDY` reader - LFRCO Ready
pub type LfrcordyR = crate::BitReader;
///Field `LFXOENS` reader - LFXO Enable Status
pub type LfxoensR = crate::BitReader;
///Field `LFXORDY` reader - LFXO Ready
pub type LfxordyR = crate::BitReader;
///Field `CALRDY` reader - Calibration Ready
pub type CalrdyR = crate::BitReader;
///Field `HFXOREQ` reader - HFXO is Required By Hardware
pub type HfxoreqR = crate::BitReader;
///Field `HFXOPEAKDETRDY` reader - HFXO Peak Detection Ready
pub type HfxopeakdetrdyR = crate::BitReader;
///Field `HFXOSHUNTOPTRDY` reader - HFXO Shunt Current Optimization Ready
pub type HfxoshuntoptrdyR = crate::BitReader;
///Field `HFXOAMPHIGH` reader - HFXO Oscillation Amplitude is Too High
pub type HfxoamphighR = crate::BitReader;
///Field `HFXOAMPLOW` reader - HFXO Amplitude Tuning Value Too Low
pub type HfxoamplowR = crate::BitReader;
///Field `HFXOREGILOW` reader - HFXO Regulator Shunt Current Too Low
pub type HfxoregilowR = crate::BitReader;
impl R {
    ///Bit 0 - HFRCO Enable Status
    #[inline(always)]
    pub fn hfrcoens(&self) -> HfrcoensR {
        HfrcoensR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HFRCO Ready
    #[inline(always)]
    pub fn hfrcordy(&self) -> HfrcordyR {
        HfrcordyR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HFXO Enable Status
    #[inline(always)]
    pub fn hfxoens(&self) -> HfxoensR {
        HfxoensR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HFXO Ready
    #[inline(always)]
    pub fn hfxordy(&self) -> HfxordyR {
        HfxordyR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AUXHFRCO Enable Status
    #[inline(always)]
    pub fn auxhfrcoens(&self) -> AuxhfrcoensR {
        AuxhfrcoensR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AUXHFRCO Ready
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AuxhfrcordyR {
        AuxhfrcordyR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LFRCO Enable Status
    #[inline(always)]
    pub fn lfrcoens(&self) -> LfrcoensR {
        LfrcoensR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LFRCO Ready
    #[inline(always)]
    pub fn lfrcordy(&self) -> LfrcordyR {
        LfrcordyR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LFXO Enable Status
    #[inline(always)]
    pub fn lfxoens(&self) -> LfxoensR {
        LfxoensR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LFXO Ready
    #[inline(always)]
    pub fn lfxordy(&self) -> LfxordyR {
        LfxordyR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Calibration Ready
    #[inline(always)]
    pub fn calrdy(&self) -> CalrdyR {
        CalrdyR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 21 - HFXO is Required By Hardware
    #[inline(always)]
    pub fn hfxoreq(&self) -> HfxoreqR {
        HfxoreqR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - HFXO Peak Detection Ready
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HfxopeakdetrdyR {
        HfxopeakdetrdyR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - HFXO Shunt Current Optimization Ready
    #[inline(always)]
    pub fn hfxoshuntoptrdy(&self) -> HfxoshuntoptrdyR {
        HfxoshuntoptrdyR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - HFXO Oscillation Amplitude is Too High
    #[inline(always)]
    pub fn hfxoamphigh(&self) -> HfxoamphighR {
        HfxoamphighR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - HFXO Amplitude Tuning Value Too Low
    #[inline(always)]
    pub fn hfxoamplow(&self) -> HfxoamplowR {
        HfxoamplowR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - HFXO Regulator Shunt Current Too Low
    #[inline(always)]
    pub fn hfxoregilow(&self) -> HfxoregilowR {
        HfxoregilowR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("hfrcoens", &self.hfrcoens())
            .field("hfrcordy", &self.hfrcordy())
            .field("hfxoens", &self.hfxoens())
            .field("hfxordy", &self.hfxordy())
            .field("auxhfrcoens", &self.auxhfrcoens())
            .field("auxhfrcordy", &self.auxhfrcordy())
            .field("lfrcoens", &self.lfrcoens())
            .field("lfrcordy", &self.lfrcordy())
            .field("lfxoens", &self.lfxoens())
            .field("lfxordy", &self.lfxordy())
            .field("calrdy", &self.calrdy())
            .field("hfxoreq", &self.hfxoreq())
            .field("hfxopeakdetrdy", &self.hfxopeakdetrdy())
            .field("hfxoshuntoptrdy", &self.hfxoshuntoptrdy())
            .field("hfxoamphigh", &self.hfxoamphigh())
            .field("hfxoamplow", &self.hfxoamplow())
            .field("hfxoregilow", &self.hfxoregilow())
            .finish()
    }
}
///Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUSrs {}
///`reset()` method sets STATUS to value 0x0001_0003
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0x0001_0003;
}
