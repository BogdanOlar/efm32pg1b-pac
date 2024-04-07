#[doc = "Register `LFECLKSEL` reader"]
pub type R = crate::R<LFECLKSELrs>;
#[doc = "Register `LFECLKSEL` writer"]
pub type W = crate::W<LFECLKSELrs>;
#[doc = "Clock Select for LFE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFE {
    #[doc = "0: LFECLK is disabled"]
    Disabled = 0,
    #[doc = "1: LFRCO selected as LFECLK"]
    Lfrco = 1,
    #[doc = "2: LFXO selected as LFECLK"]
    Lfxo = 2,
    #[doc = "4: ULFRCO selected as LFECLK"]
    Ulfrco = 4,
}
impl From<LFE> for u8 {
    #[inline(always)]
    fn from(variant: LFE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFE {
    type Ux = u8;
}
impl crate::IsEnum for LFE {}
#[doc = "Field `LFE` reader - Clock Select for LFE"]
pub type LfeR = crate::FieldReader<LFE>;
impl LfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LFE> {
        match self.bits {
            0 => Some(LFE::Disabled),
            1 => Some(LFE::Lfrco),
            2 => Some(LFE::Lfxo),
            4 => Some(LFE::Ulfrco),
            _ => None,
        }
    }
    #[doc = "LFECLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFE::Disabled
    }
    #[doc = "LFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFE::Lfrco
    }
    #[doc = "LFXO selected as LFECLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFE::Lfxo
    }
    #[doc = "ULFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFE::Ulfrco
    }
}
#[doc = "Field `LFE` writer - Clock Select for LFE"]
pub type LfeW<'a, REG> = crate::FieldWriter<'a, REG, 3, LFE>;
impl<'a, REG> LfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFECLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFE::Disabled)
    }
    #[doc = "LFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFE::Lfrco)
    }
    #[doc = "LFXO selected as LFECLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(LFE::Lfxo)
    }
    #[doc = "ULFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFE::Ulfrco)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFE"]
    #[inline(always)]
    pub fn lfe(&self) -> LfeR {
        LfeR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFE"]
    #[inline(always)]
    #[must_use]
    pub fn lfe(&mut self) -> LfeW<LFECLKSELrs> {
        LfeW::new(self, 0)
    }
}
#[doc = "Low Frequency E Clock Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfeclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfeclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFECLKSELrs;
impl crate::RegisterSpec for LFECLKSELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfeclksel::R`](R) reader structure"]
impl crate::Readable for LFECLKSELrs {}
#[doc = "`write(|w| ..)` method takes [`lfeclksel::W`](W) writer structure"]
impl crate::Writable for LFECLKSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFECLKSEL to value 0"]
impl crate::Resettable for LFECLKSELrs {
    const RESET_VALUE: u32 = 0;
}
