#[doc = "Register `HFCLKSTATUS` reader"]
pub type R = crate::R<HFCLKSTATUSrs>;
#[doc = "HFCLK Selected\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELECTED {
    #[doc = "1: HFRCO is selected as HFCLK clock source"]
    Hfrco = 1,
    #[doc = "2: HFXO is selected as HFCLK clock source"]
    Hfxo = 2,
    #[doc = "3: LFRCO is selected as HFCLK clock source"]
    Lfrco = 3,
    #[doc = "4: LFXO is selected as HFCLK clock source"]
    Lfxo = 4,
}
impl From<SELECTED> for u8 {
    #[inline(always)]
    fn from(variant: SELECTED) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SELECTED {
    type Ux = u8;
}
impl crate::IsEnum for SELECTED {}
#[doc = "Field `SELECTED` reader - HFCLK Selected"]
pub type SelectedR = crate::FieldReader<SELECTED>;
impl SelectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SELECTED> {
        match self.bits {
            1 => Some(SELECTED::Hfrco),
            2 => Some(SELECTED::Hfxo),
            3 => Some(SELECTED::Lfrco),
            4 => Some(SELECTED::Lfxo),
            _ => None,
        }
    }
    #[doc = "HFRCO is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == SELECTED::Hfrco
    }
    #[doc = "HFXO is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == SELECTED::Hfxo
    }
    #[doc = "LFRCO is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == SELECTED::Lfrco
    }
    #[doc = "LFXO is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == SELECTED::Lfxo
    }
}
impl R {
    #[doc = "Bits 0:2 - HFCLK Selected"]
    #[inline(always)]
    pub fn selected(&self) -> SelectedR {
        SelectedR::new((self.bits & 7) as u8)
    }
}
#[doc = "HFCLK Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfclkstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFCLKSTATUSrs;
impl crate::RegisterSpec for HFCLKSTATUSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfclkstatus::R`](R) reader structure"]
impl crate::Readable for HFCLKSTATUSrs {}
#[doc = "`reset()` method sets HFCLKSTATUS to value 0x01"]
impl crate::Resettable for HFCLKSTATUSrs {
    const RESET_VALUE: u32 = 0x01;
}
