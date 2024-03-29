#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUSrs>;
#[doc = "Field `HFRCOENS` reader - HFRCO Enable Status"]
pub type HFRCOENS_R = crate::BitReader;
#[doc = "Field `HFRCORDY` reader - HFRCO Ready"]
pub type HFRCORDY_R = crate::BitReader;
#[doc = "Field `HFXOENS` reader - HFXO Enable Status"]
pub type HFXOENS_R = crate::BitReader;
#[doc = "Field `HFXORDY` reader - HFXO Ready"]
pub type HFXORDY_R = crate::BitReader;
#[doc = "Field `AUXHFRCOENS` reader - AUXHFRCO Enable Status"]
pub type AUXHFRCOENS_R = crate::BitReader;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCO Ready"]
pub type AUXHFRCORDY_R = crate::BitReader;
#[doc = "Field `LFRCOENS` reader - LFRCO Enable Status"]
pub type LFRCOENS_R = crate::BitReader;
#[doc = "Field `LFRCORDY` reader - LFRCO Ready"]
pub type LFRCORDY_R = crate::BitReader;
#[doc = "Field `LFXOENS` reader - LFXO Enable Status"]
pub type LFXOENS_R = crate::BitReader;
#[doc = "Field `LFXORDY` reader - LFXO Ready"]
pub type LFXORDY_R = crate::BitReader;
#[doc = "Field `CALRDY` reader - Calibration Ready"]
pub type CALRDY_R = crate::BitReader;
#[doc = "Field `HFXOREQ` reader - HFXO is Required By Hardware"]
pub type HFXOREQ_R = crate::BitReader;
#[doc = "Field `HFXOPEAKDETRDY` reader - HFXO Peak Detection Ready"]
pub type HFXOPEAKDETRDY_R = crate::BitReader;
#[doc = "Field `HFXOSHUNTOPTRDY` reader - HFXO Shunt Current Optimization Ready"]
pub type HFXOSHUNTOPTRDY_R = crate::BitReader;
#[doc = "Field `HFXOAMPHIGH` reader - HFXO Oscillation Amplitude is Too High"]
pub type HFXOAMPHIGH_R = crate::BitReader;
#[doc = "Field `HFXOAMPLOW` reader - HFXO Amplitude Tuning Value Too Low"]
pub type HFXOAMPLOW_R = crate::BitReader;
#[doc = "Field `HFXOREGILOW` reader - HFXO Regulator Shunt Current Too Low"]
pub type HFXOREGILOW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HFRCO Enable Status"]
    #[inline(always)]
    pub fn hfrcoens(&self) -> HFRCOENS_R {
        HFRCOENS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFRCO Ready"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HFXO Enable Status"]
    #[inline(always)]
    pub fn hfxoens(&self) -> HFXOENS_R {
        HFXOENS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HFXO Ready"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable Status"]
    #[inline(always)]
    pub fn auxhfrcoens(&self) -> AUXHFRCOENS_R {
        AUXHFRCOENS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AUXHFRCO Ready"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LFRCO Enable Status"]
    #[inline(always)]
    pub fn lfrcoens(&self) -> LFRCOENS_R {
        LFRCOENS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LFRCO Ready"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LFXO Enable Status"]
    #[inline(always)]
    pub fn lfxoens(&self) -> LFXOENS_R {
        LFXOENS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LFXO Ready"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Calibration Ready"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - HFXO is Required By Hardware"]
    #[inline(always)]
    pub fn hfxoreq(&self) -> HFXOREQ_R {
        HFXOREQ_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HFXO Peak Detection Ready"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HFXOPEAKDETRDY_R {
        HFXOPEAKDETRDY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - HFXO Shunt Current Optimization Ready"]
    #[inline(always)]
    pub fn hfxoshuntoptrdy(&self) -> HFXOSHUNTOPTRDY_R {
        HFXOSHUNTOPTRDY_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - HFXO Oscillation Amplitude is Too High"]
    #[inline(always)]
    pub fn hfxoamphigh(&self) -> HFXOAMPHIGH_R {
        HFXOAMPHIGH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HFXO Amplitude Tuning Value Too Low"]
    #[inline(always)]
    pub fn hfxoamplow(&self) -> HFXOAMPLOW_R {
        HFXOAMPLOW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - HFXO Regulator Shunt Current Too Low"]
    #[inline(always)]
    pub fn hfxoregilow(&self) -> HFXOREGILOW_R {
        HFXOREGILOW_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUSrs {}
#[doc = "`reset()` method sets STATUS to value 0x0001_0003"]
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0x0001_0003;
}
