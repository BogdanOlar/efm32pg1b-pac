#[doc = "Register `PWRCFG` reader"]
pub type R = crate::R<PWRCFGrs>;
#[doc = "Register `PWRCFG` writer"]
pub type W = crate::W<PWRCFGrs>;
#[doc = "Power Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRCFG {
    #[doc = "0: Power up configuration. Works with any external configuration."]
    Startup = 0,
    #[doc = "2: DCDC is enabled and routed to DVDD."]
    Dcdctodvdd = 2,
}
impl From<PWRCFG> for u8 {
    #[inline(always)]
    fn from(variant: PWRCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWRCFG {
    type Ux = u8;
}
impl crate::IsEnum for PWRCFG {}
#[doc = "Field `PWRCFG` reader - Power Configuration"]
pub type PwrcfgR = crate::FieldReader<PWRCFG>;
impl PwrcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWRCFG> {
        match self.bits {
            0 => Some(PWRCFG::Startup),
            2 => Some(PWRCFG::Dcdctodvdd),
            _ => None,
        }
    }
    #[doc = "Power up configuration. Works with any external configuration."]
    #[inline(always)]
    pub fn is_startup(&self) -> bool {
        *self == PWRCFG::Startup
    }
    #[doc = "DCDC is enabled and routed to DVDD."]
    #[inline(always)]
    pub fn is_dcdctodvdd(&self) -> bool {
        *self == PWRCFG::Dcdctodvdd
    }
}
#[doc = "Field `PWRCFG` writer - Power Configuration"]
pub type PwrcfgW<'a, REG> = crate::FieldWriter<'a, REG, 4, PWRCFG>;
impl<'a, REG> PwrcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Power up configuration. Works with any external configuration."]
    #[inline(always)]
    pub fn startup(self) -> &'a mut crate::W<REG> {
        self.variant(PWRCFG::Startup)
    }
    #[doc = "DCDC is enabled and routed to DVDD."]
    #[inline(always)]
    pub fn dcdctodvdd(self) -> &'a mut crate::W<REG> {
        self.variant(PWRCFG::Dcdctodvdd)
    }
}
impl R {
    #[doc = "Bits 0:3 - Power Configuration"]
    #[inline(always)]
    pub fn pwrcfg(&self) -> PwrcfgR {
        PwrcfgR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Power Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pwrcfg(&mut self) -> PwrcfgW<PWRCFGrs> {
        PwrcfgW::new(self, 0)
    }
}
#[doc = "Power Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRCFGrs;
impl crate::RegisterSpec for PWRCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcfg::R`](R) reader structure"]
impl crate::Readable for PWRCFGrs {}
#[doc = "`write(|w| ..)` method takes [`pwrcfg::W`](W) writer structure"]
impl crate::Writable for PWRCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCFG to value 0"]
impl crate::Resettable for PWRCFGrs {
    const RESET_VALUE: u32 = 0;
}
