///Register `PWRCFG` reader
pub type R = crate::R<PWRCFGrs>;
///Register `PWRCFG` writer
pub type W = crate::W<PWRCFGrs>;
///Power Configuration
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRCFG {
    ///0: Power up configuration. Works with any external configuration.
    Startup = 0,
    ///2: DCDC is enabled and routed to DVDD.
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
///Field `PWRCFG` reader - Power Configuration
pub type PwrcfgR = crate::FieldReader<PWRCFG>;
impl PwrcfgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWRCFG> {
        match self.bits {
            0 => Some(PWRCFG::Startup),
            2 => Some(PWRCFG::Dcdctodvdd),
            _ => None,
        }
    }
    ///Power up configuration. Works with any external configuration.
    #[inline(always)]
    pub fn is_startup(&self) -> bool {
        *self == PWRCFG::Startup
    }
    ///DCDC is enabled and routed to DVDD.
    #[inline(always)]
    pub fn is_dcdctodvdd(&self) -> bool {
        *self == PWRCFG::Dcdctodvdd
    }
}
///Field `PWRCFG` writer - Power Configuration
pub type PwrcfgW<'a, REG> = crate::FieldWriter<'a, REG, 4, PWRCFG>;
impl<'a, REG> PwrcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Power up configuration. Works with any external configuration.
    #[inline(always)]
    pub fn startup(self) -> &'a mut crate::W<REG> {
        self.variant(PWRCFG::Startup)
    }
    ///DCDC is enabled and routed to DVDD.
    #[inline(always)]
    pub fn dcdctodvdd(self) -> &'a mut crate::W<REG> {
        self.variant(PWRCFG::Dcdctodvdd)
    }
}
impl R {
    ///Bits 0:3 - Power Configuration
    #[inline(always)]
    pub fn pwrcfg(&self) -> PwrcfgR {
        PwrcfgR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRCFG")
            .field("pwrcfg", &self.pwrcfg())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Power Configuration
    #[inline(always)]
    #[must_use]
    pub fn pwrcfg(&mut self) -> PwrcfgW<PWRCFGrs> {
        PwrcfgW::new(self, 0)
    }
}
///Power Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`pwrcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PWRCFGrs;
impl crate::RegisterSpec for PWRCFGrs {
    type Ux = u32;
}
///`read()` method returns [`pwrcfg::R`](R) reader structure
impl crate::Readable for PWRCFGrs {}
///`write(|w| ..)` method takes [`pwrcfg::W`](W) writer structure
impl crate::Writable for PWRCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWRCFG to value 0
impl crate::Resettable for PWRCFGrs {
    const RESET_VALUE: u32 = 0;
}
