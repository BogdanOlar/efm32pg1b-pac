#[doc = "Register `LFRCOCTRL` reader"]
pub type R = crate::R<LFRCOCTRL_SPEC>;
#[doc = "Register `LFRCOCTRL` writer"]
pub type W = crate::W<LFRCOCTRL_SPEC>;
#[doc = "Field `TUNING` reader - LFRCO Tuning Value"]
pub type TUNING_R = crate::FieldReader<u16>;
#[doc = "Field `TUNING` writer - LFRCO Tuning Value"]
pub type TUNING_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `ENVREF` reader - Enable Duty Cycling of Vref"]
pub type ENVREF_R = crate::BitReader;
#[doc = "Field `ENVREF` writer - Enable Duty Cycling of Vref"]
pub type ENVREF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENCHOP` reader - Enable Comparator Chopping"]
pub type ENCHOP_R = crate::BitReader;
#[doc = "Field `ENCHOP` writer - Enable Comparator Chopping"]
pub type ENCHOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENDEM` reader - Enable Dynamic Element Matching"]
pub type ENDEM_R = crate::BitReader;
#[doc = "Field `ENDEM` writer - Enable Dynamic Element Matching"]
pub type ENDEM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMEOUT` reader - LFRCO Timeout"]
pub type TIMEOUT_R = crate::FieldReader<TIMEOUT_A>;
#[doc = "LFRCO Timeout\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUT_A {
    #[doc = "0: Timeout period of 2 cycles"]
    _2CYCLES = 0,
    #[doc = "1: Timeout period of 16 cycles"]
    _16CYCLES = 1,
    #[doc = "2: Timeout period of 32 cycles"]
    _32CYCLES = 2,
}
impl From<TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMEOUT_A {
    type Ux = u8;
}
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIMEOUT_A> {
        match self.bits {
            0 => Some(TIMEOUT_A::_2CYCLES),
            1 => Some(TIMEOUT_A::_16CYCLES),
            2 => Some(TIMEOUT_A::_32CYCLES),
            _ => None,
        }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == TIMEOUT_A::_2CYCLES
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == TIMEOUT_A::_16CYCLES
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == TIMEOUT_A::_32CYCLES
    }
}
#[doc = "Field `TIMEOUT` writer - LFRCO Timeout"]
pub type TIMEOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, TIMEOUT_A>;
impl<'a, REG, const O: u8> TIMEOUT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT_A::_32CYCLES)
    }
}
#[doc = "Field `GMCCURTUNE` reader - Tuning of Gmc Current"]
pub type GMCCURTUNE_R = crate::FieldReader;
#[doc = "Field `GMCCURTUNE` writer - Tuning of Gmc Current"]
pub type GMCCURTUNE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:8 - LFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Enable Duty Cycling of Vref"]
    #[inline(always)]
    pub fn envref(&self) -> ENVREF_R {
        ENVREF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Comparator Chopping"]
    #[inline(always)]
    pub fn enchop(&self) -> ENCHOP_R {
        ENCHOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Dynamic Element Matching"]
    #[inline(always)]
    pub fn endem(&self) -> ENDEM_R {
        ENDEM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:25 - LFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Tuning of Gmc Current"]
    #[inline(always)]
    pub fn gmccurtune(&self) -> GMCCURTUNE_R {
        GMCCURTUNE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFRCOCTRL")
            .field("tuning", &format_args!("{}", self.tuning().bits()))
            .field("envref", &format_args!("{}", self.envref().bit()))
            .field("enchop", &format_args!("{}", self.enchop().bit()))
            .field("endem", &format_args!("{}", self.endem().bit()))
            .field("timeout", &format_args!("{}", self.timeout().bits()))
            .field("gmccurtune", &format_args!("{}", self.gmccurtune().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<LFRCOCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - LFRCO Tuning Value"]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TUNING_W<LFRCOCTRL_SPEC, 0> {
        TUNING_W::new(self)
    }
    #[doc = "Bit 16 - Enable Duty Cycling of Vref"]
    #[inline(always)]
    #[must_use]
    pub fn envref(&mut self) -> ENVREF_W<LFRCOCTRL_SPEC, 16> {
        ENVREF_W::new(self)
    }
    #[doc = "Bit 17 - Enable Comparator Chopping"]
    #[inline(always)]
    #[must_use]
    pub fn enchop(&mut self) -> ENCHOP_W<LFRCOCTRL_SPEC, 17> {
        ENCHOP_W::new(self)
    }
    #[doc = "Bit 18 - Enable Dynamic Element Matching"]
    #[inline(always)]
    #[must_use]
    pub fn endem(&mut self) -> ENDEM_W<LFRCOCTRL_SPEC, 18> {
        ENDEM_W::new(self)
    }
    #[doc = "Bits 24:25 - LFRCO Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<LFRCOCTRL_SPEC, 24> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bits 28:31 - Tuning of Gmc Current"]
    #[inline(always)]
    #[must_use]
    pub fn gmccurtune(&mut self) -> GMCCURTUNE_W<LFRCOCTRL_SPEC, 28> {
        GMCCURTUNE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfrcoctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfrcoctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFRCOCTRL_SPEC;
impl crate::RegisterSpec for LFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfrcoctrl::R`](R) reader structure"]
impl crate::Readable for LFRCOCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfrcoctrl::W`](W) writer structure"]
impl crate::Writable for LFRCOCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFRCOCTRL to value 0x8106_0100"]
impl crate::Resettable for LFRCOCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8106_0100;
}
