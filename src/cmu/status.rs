#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `HFRCOENS`"]
pub type HFRCOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFRCORDY`"]
pub type HFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOENS`"]
pub type HFXOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXORDY`"]
pub type HFXORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXHFRCOENS`"]
pub type AUXHFRCOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXHFRCORDY`"]
pub type AUXHFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFRCOENS`"]
pub type LFRCOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFRCORDY`"]
pub type LFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFXOENS`"]
pub type LFXOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFXORDY`"]
pub type LFXORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CALRDY`"]
pub type CALRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOREQ`"]
pub type HFXOREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOPEAKDETRDY`"]
pub type HFXOPEAKDETRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOSHUNTOPTRDY`"]
pub type HFXOSHUNTOPTRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOAMPHIGH`"]
pub type HFXOAMPHIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOAMPLOW`"]
pub type HFXOAMPLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOREGILOW`"]
pub type HFXOREGILOW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - HFRCO Enable Status"]
    #[inline(always)]
    pub fn hfrcoens(&self) -> HFRCOENS_R {
        HFRCOENS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HFRCO Ready"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HFXO Enable Status"]
    #[inline(always)]
    pub fn hfxoens(&self) -> HFXOENS_R {
        HFXOENS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HFXO Ready"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable Status"]
    #[inline(always)]
    pub fn auxhfrcoens(&self) -> AUXHFRCOENS_R {
        AUXHFRCOENS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AUXHFRCO Ready"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LFRCO Enable Status"]
    #[inline(always)]
    pub fn lfrcoens(&self) -> LFRCOENS_R {
        LFRCOENS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LFRCO Ready"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LFXO Enable Status"]
    #[inline(always)]
    pub fn lfxoens(&self) -> LFXOENS_R {
        LFXOENS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LFXO Ready"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Calibration Ready"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 21 - HFXO is Required By Hardware"]
    #[inline(always)]
    pub fn hfxoreq(&self) -> HFXOREQ_R {
        HFXOREQ_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - HFXO Peak Detection Ready"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HFXOPEAKDETRDY_R {
        HFXOPEAKDETRDY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - HFXO Shunt Current Optimization Ready"]
    #[inline(always)]
    pub fn hfxoshuntoptrdy(&self) -> HFXOSHUNTOPTRDY_R {
        HFXOSHUNTOPTRDY_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - HFXO Oscillation Amplitude is Too High"]
    #[inline(always)]
    pub fn hfxoamphigh(&self) -> HFXOAMPHIGH_R {
        HFXOAMPHIGH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - HFXO Amplitude Tuning Value Too Low"]
    #[inline(always)]
    pub fn hfxoamplow(&self) -> HFXOAMPLOW_R {
        HFXOAMPLOW_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - HFXO Regulator Shunt Current Too Low"]
    #[inline(always)]
    pub fn hfxoregilow(&self) -> HFXOREGILOW_R {
        HFXOREGILOW_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
