#[doc = "Register `LFACLKSEL` reader"]
pub type R = crate::R<LFACLKSELrs>;
#[doc = "Register `LFACLKSEL` writer"]
pub type W = crate::W<LFACLKSELrs>;
#[doc = "Clock Select for LFA\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFA {
    #[doc = "0: LFACLK is disabled"]
    Disabled = 0,
    #[doc = "1: LFRCO selected as LFACLK"]
    Lfrco = 1,
    #[doc = "2: LFXO selected as LFACLK"]
    Lfxo = 2,
    #[doc = "4: ULFRCO selected as LFACLK"]
    Ulfrco = 4,
}
impl From<LFA> for u8 {
    #[inline(always)]
    fn from(variant: LFA) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFA {
    type Ux = u8;
}
impl crate::IsEnum for LFA {}
#[doc = "Field `LFA` reader - Clock Select for LFA"]
pub type LfaR = crate::FieldReader<LFA>;
impl LfaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LFA> {
        match self.bits {
            0 => Some(LFA::Disabled),
            1 => Some(LFA::Lfrco),
            2 => Some(LFA::Lfxo),
            4 => Some(LFA::Ulfrco),
            _ => None,
        }
    }
    #[doc = "LFACLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFA::Disabled
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFA::Lfrco
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFA::Lfxo
    }
    #[doc = "ULFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFA::Ulfrco
    }
}
#[doc = "Field `LFA` writer - Clock Select for LFA"]
pub type LfaW<'a, REG> = crate::FieldWriter<'a, REG, 3, LFA>;
impl<'a, REG> LfaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFACLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFA::Disabled)
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFA::Lfrco)
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(LFA::Lfxo)
    }
    #[doc = "ULFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFA::Ulfrco)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFA"]
    #[inline(always)]
    pub fn lfa(&self) -> LfaR {
        LfaR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFACLKSEL")
            .field("lfa", &self.lfa())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFA"]
    #[inline(always)]
    #[must_use]
    pub fn lfa(&mut self) -> LfaW<LFACLKSELrs> {
        LfaW::new(self, 0)
    }
}
#[doc = "Low Frequency A Clock Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfaclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfaclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFACLKSELrs;
impl crate::RegisterSpec for LFACLKSELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfaclksel::R`](R) reader structure"]
impl crate::Readable for LFACLKSELrs {}
#[doc = "`write(|w| ..)` method takes [`lfaclksel::W`](W) writer structure"]
impl crate::Writable for LFACLKSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFACLKSEL to value 0"]
impl crate::Resettable for LFACLKSELrs {
    const RESET_VALUE: u32 = 0;
}
