#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("hfrcoens", &format_args!("{}", self.hfrcoens().bit()))
            .field("hfrcordy", &format_args!("{}", self.hfrcordy().bit()))
            .field("hfxoens", &format_args!("{}", self.hfxoens().bit()))
            .field("hfxordy", &format_args!("{}", self.hfxordy().bit()))
            .field("auxhfrcoens", &format_args!("{}", self.auxhfrcoens().bit()))
            .field("auxhfrcordy", &format_args!("{}", self.auxhfrcordy().bit()))
            .field("lfrcoens", &format_args!("{}", self.lfrcoens().bit()))
            .field("lfrcordy", &format_args!("{}", self.lfrcordy().bit()))
            .field("lfxoens", &format_args!("{}", self.lfxoens().bit()))
            .field("lfxordy", &format_args!("{}", self.lfxordy().bit()))
            .field("calrdy", &format_args!("{}", self.calrdy().bit()))
            .field("hfxoreq", &format_args!("{}", self.hfxoreq().bit()))
            .field(
                "hfxopeakdetrdy",
                &format_args!("{}", self.hfxopeakdetrdy().bit()),
            )
            .field(
                "hfxoshuntoptrdy",
                &format_args!("{}", self.hfxoshuntoptrdy().bit()),
            )
            .field("hfxoamphigh", &format_args!("{}", self.hfxoamphigh().bit()))
            .field("hfxoamplow", &format_args!("{}", self.hfxoamplow().bit()))
            .field("hfxoregilow", &format_args!("{}", self.hfxoregilow().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0x0001_0003"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0003;
}
