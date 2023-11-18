#[doc = "Register `PWRCFG` reader"]
pub type R = crate::R<PWRCFG_SPEC>;
#[doc = "Register `PWRCFG` writer"]
pub type W = crate::W<PWRCFG_SPEC>;
#[doc = "Field `PWRCFG` reader - Power Configuration"]
pub type PWRCFG_R = crate::FieldReader<PWRCFG_A>;
#[doc = "Power Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRCFG_A {
    #[doc = "0: Power up configuration. Works with any external configuration."]
    STARTUP = 0,
    #[doc = "2: DCDC is enabled and routed to DVDD."]
    DCDCTODVDD = 2,
}
impl From<PWRCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRCFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWRCFG_A {
    type Ux = u8;
}
impl PWRCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWRCFG_A> {
        match self.bits {
            0 => Some(PWRCFG_A::STARTUP),
            2 => Some(PWRCFG_A::DCDCTODVDD),
            _ => None,
        }
    }
    #[doc = "Power up configuration. Works with any external configuration."]
    #[inline(always)]
    pub fn is_startup(&self) -> bool {
        *self == PWRCFG_A::STARTUP
    }
    #[doc = "DCDC is enabled and routed to DVDD."]
    #[inline(always)]
    pub fn is_dcdctodvdd(&self) -> bool {
        *self == PWRCFG_A::DCDCTODVDD
    }
}
#[doc = "Field `PWRCFG` writer - Power Configuration"]
pub type PWRCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, PWRCFG_A>;
impl<'a, REG, const O: u8> PWRCFG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Power up configuration. Works with any external configuration."]
    #[inline(always)]
    pub fn startup(self) -> &'a mut crate::W<REG> {
        self.variant(PWRCFG_A::STARTUP)
    }
    #[doc = "DCDC is enabled and routed to DVDD."]
    #[inline(always)]
    pub fn dcdctodvdd(self) -> &'a mut crate::W<REG> {
        self.variant(PWRCFG_A::DCDCTODVDD)
    }
}
impl R {
    #[doc = "Bits 0:3 - Power Configuration"]
    #[inline(always)]
    pub fn pwrcfg(&self) -> PWRCFG_R {
        PWRCFG_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRCFG")
            .field("pwrcfg", &format_args!("{}", self.pwrcfg().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PWRCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Power Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pwrcfg(&mut self) -> PWRCFG_W<PWRCFG_SPEC, 0> {
        PWRCFG_W::new(self)
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
#[doc = "Power Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRCFG_SPEC;
impl crate::RegisterSpec for PWRCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcfg::R`](R) reader structure"]
impl crate::Readable for PWRCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwrcfg::W`](W) writer structure"]
impl crate::Writable for PWRCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCFG to value 0"]
impl crate::Resettable for PWRCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
