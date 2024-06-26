#[doc = "Register `LFBCLKSEL` reader"]
pub type R = crate::R<LFBCLKSELrs>;
#[doc = "Register `LFBCLKSEL` writer"]
pub type W = crate::W<LFBCLKSELrs>;
#[doc = "Clock Select for LFB\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFB {
    #[doc = "0: LFBCLK is disabled"]
    Disabled = 0,
    #[doc = "1: LFRCO selected as LFBCLK"]
    Lfrco = 1,
    #[doc = "2: LFXO selected as LFBCLK"]
    Lfxo = 2,
    #[doc = "3: HFCLK divided by two/four is selected as LFBCLK"]
    Hfclkle = 3,
    #[doc = "4: ULFRCO selected as LFBCLK"]
    Ulfrco = 4,
}
impl From<LFB> for u8 {
    #[inline(always)]
    fn from(variant: LFB) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFB {
    type Ux = u8;
}
impl crate::IsEnum for LFB {}
#[doc = "Field `LFB` reader - Clock Select for LFB"]
pub type LfbR = crate::FieldReader<LFB>;
impl LfbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LFB> {
        match self.bits {
            0 => Some(LFB::Disabled),
            1 => Some(LFB::Lfrco),
            2 => Some(LFB::Lfxo),
            3 => Some(LFB::Hfclkle),
            4 => Some(LFB::Ulfrco),
            _ => None,
        }
    }
    #[doc = "LFBCLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFB::Disabled
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFB::Lfrco
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFB::Lfxo
    }
    #[doc = "HFCLK divided by two/four is selected as LFBCLK"]
    #[inline(always)]
    pub fn is_hfclkle(&self) -> bool {
        *self == LFB::Hfclkle
    }
    #[doc = "ULFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFB::Ulfrco
    }
}
#[doc = "Field `LFB` writer - Clock Select for LFB"]
pub type LfbW<'a, REG> = crate::FieldWriter<'a, REG, 3, LFB>;
impl<'a, REG> LfbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFBCLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFB::Disabled)
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFB::Lfrco)
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(LFB::Lfxo)
    }
    #[doc = "HFCLK divided by two/four is selected as LFBCLK"]
    #[inline(always)]
    pub fn hfclkle(self) -> &'a mut crate::W<REG> {
        self.variant(LFB::Hfclkle)
    }
    #[doc = "ULFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFB::Ulfrco)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFB"]
    #[inline(always)]
    pub fn lfb(&self) -> LfbR {
        LfbR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFBCLKSEL")
            .field("lfb", &self.lfb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFB"]
    #[inline(always)]
    #[must_use]
    pub fn lfb(&mut self) -> LfbW<LFBCLKSELrs> {
        LfbW::new(self, 0)
    }
}
#[doc = "Low Frequency B Clock Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfbclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfbclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFBCLKSELrs;
impl crate::RegisterSpec for LFBCLKSELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfbclksel::R`](R) reader structure"]
impl crate::Readable for LFBCLKSELrs {}
#[doc = "`write(|w| ..)` method takes [`lfbclksel::W`](W) writer structure"]
impl crate::Writable for LFBCLKSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFBCLKSEL to value 0"]
impl crate::Resettable for LFBCLKSELrs {
    const RESET_VALUE: u32 = 0;
}
