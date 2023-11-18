#[doc = "Register `HFCLKSTATUS` reader"]
pub type R = crate::R<HFCLKSTATUS_SPEC>;
#[doc = "Field `SELECTED` reader - HFCLK Selected"]
pub type SELECTED_R = crate::FieldReader<SELECTED_A>;
#[doc = "HFCLK Selected\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELECTED_A {
    #[doc = "1: HFRCO is selected as HFCLK clock source"]
    HFRCO = 1,
    #[doc = "2: HFXO is selected as HFCLK clock source"]
    HFXO = 2,
    #[doc = "3: LFRCO is selected as HFCLK clock source"]
    LFRCO = 3,
    #[doc = "4: LFXO is selected as HFCLK clock source"]
    LFXO = 4,
}
impl From<SELECTED_A> for u8 {
    #[inline(always)]
    fn from(variant: SELECTED_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SELECTED_A {
    type Ux = u8;
}
impl SELECTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SELECTED_A> {
        match self.bits {
            1 => Some(SELECTED_A::HFRCO),
            2 => Some(SELECTED_A::HFXO),
            3 => Some(SELECTED_A::LFRCO),
            4 => Some(SELECTED_A::LFXO),
            _ => None,
        }
    }
    #[doc = "HFRCO is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == SELECTED_A::HFRCO
    }
    #[doc = "HFXO is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == SELECTED_A::HFXO
    }
    #[doc = "LFRCO is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == SELECTED_A::LFRCO
    }
    #[doc = "LFXO is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == SELECTED_A::LFXO
    }
}
impl R {
    #[doc = "Bits 0:2 - HFCLK Selected"]
    #[inline(always)]
    pub fn selected(&self) -> SELECTED_R {
        SELECTED_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFCLKSTATUS")
            .field("selected", &format_args!("{}", self.selected().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HFCLKSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "HFCLK Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfclkstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFCLKSTATUS_SPEC;
impl crate::RegisterSpec for HFCLKSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfclkstatus::R`](R) reader structure"]
impl crate::Readable for HFCLKSTATUS_SPEC {}
#[doc = "`reset()` method sets HFCLKSTATUS to value 0x01"]
impl crate::Resettable for HFCLKSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
