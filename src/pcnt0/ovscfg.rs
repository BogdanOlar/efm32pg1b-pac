#[doc = "Register `OVSCFG` reader"]
pub type R = crate::R<OVSCFGrs>;
#[doc = "Register `OVSCFG` writer"]
pub type W = crate::W<OVSCFGrs>;
#[doc = "Field `FILTLEN` reader - Configure Filter Length for Inputs S0IN and S1IN"]
pub type FiltlenR = crate::FieldReader;
#[doc = "Field `FILTLEN` writer - Configure Filter Length for Inputs S0IN and S1IN"]
pub type FiltlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FLUTTERRM` reader - Flutter Remove"]
pub type FlutterrmR = crate::BitReader;
#[doc = "Field `FLUTTERRM` writer - Flutter Remove"]
pub type FlutterrmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Configure Filter Length for Inputs S0IN and S1IN"]
    #[inline(always)]
    pub fn filtlen(&self) -> FiltlenR {
        FiltlenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - Flutter Remove"]
    #[inline(always)]
    pub fn flutterrm(&self) -> FlutterrmR {
        FlutterrmR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OVSCFG")
            .field("filtlen", &self.filtlen())
            .field("flutterrm", &self.flutterrm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configure Filter Length for Inputs S0IN and S1IN"]
    #[inline(always)]
    #[must_use]
    pub fn filtlen(&mut self) -> FiltlenW<OVSCFGrs> {
        FiltlenW::new(self, 0)
    }
    #[doc = "Bit 12 - Flutter Remove"]
    #[inline(always)]
    #[must_use]
    pub fn flutterrm(&mut self) -> FlutterrmW<OVSCFGrs> {
        FlutterrmW::new(self, 12)
    }
}
#[doc = "Oversampling Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ovscfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ovscfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OVSCFGrs;
impl crate::RegisterSpec for OVSCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovscfg::R`](R) reader structure"]
impl crate::Readable for OVSCFGrs {}
#[doc = "`write(|w| ..)` method takes [`ovscfg::W`](W) writer structure"]
impl crate::Writable for OVSCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OVSCFG to value 0"]
impl crate::Resettable for OVSCFGrs {
    const RESET_VALUE: u32 = 0;
}
