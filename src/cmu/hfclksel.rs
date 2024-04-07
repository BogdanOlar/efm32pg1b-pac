#[doc = "Register `HFCLKSEL` writer"]
pub type W = crate::W<HFCLKSELrs>;
#[doc = "HFCLK Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HF {
    #[doc = "1: Select HFRCO as HFCLK"]
    Hfrco = 1,
    #[doc = "2: Select HFXO as HFCLK"]
    Hfxo = 2,
    #[doc = "3: Select LFRCO as HFCLK"]
    Lfrco = 3,
    #[doc = "4: Select LFXO as HFCLK"]
    Lfxo = 4,
}
impl From<HF> for u8 {
    #[inline(always)]
    fn from(variant: HF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HF {
    type Ux = u8;
}
impl crate::IsEnum for HF {}
#[doc = "Field `HF` writer - HFCLK Select"]
pub type HfW<'a, REG> = crate::FieldWriter<'a, REG, 3, HF>;
impl<'a, REG> HfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select HFRCO as HFCLK"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(HF::Hfrco)
    }
    #[doc = "Select HFXO as HFCLK"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(HF::Hfxo)
    }
    #[doc = "Select LFRCO as HFCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(HF::Lfrco)
    }
    #[doc = "Select LFXO as HFCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(HF::Lfxo)
    }
}
impl W {
    #[doc = "Bits 0:2 - HFCLK Select"]
    #[inline(always)]
    #[must_use]
    pub fn hf(&mut self) -> HfW<HFCLKSELrs> {
        HfW::new(self, 0)
    }
}
#[doc = "High Frequency Clock Select Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfclksel::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFCLKSELrs;
impl crate::RegisterSpec for HFCLKSELrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hfclksel::W`](W) writer structure"]
impl crate::Writable for HFCLKSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFCLKSEL to value 0"]
impl crate::Resettable for HFCLKSELrs {
    const RESET_VALUE: u32 = 0;
}
