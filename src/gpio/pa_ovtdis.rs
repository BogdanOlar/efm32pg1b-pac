#[doc = "Register `PA_OVTDIS` reader"]
pub type R = crate::R<PA_OVTDISrs>;
#[doc = "Register `PA_OVTDIS` writer"]
pub type W = crate::W<PA_OVTDISrs>;
#[doc = "Field `OVTDIS` reader - Disable Over Voltage Capability"]
pub type OvtdisR = crate::FieldReader<u16>;
#[doc = "Field `OVTDIS` writer - Disable Over Voltage Capability"]
pub type OvtdisW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis(&self) -> OvtdisR {
        OvtdisR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis(&mut self) -> OvtdisW<PA_OVTDISrs> {
        OvtdisW::new(self, 0)
    }
}
#[doc = "Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_ovtdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_ovtdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PA_OVTDISrs;
impl crate::RegisterSpec for PA_OVTDISrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_ovtdis::R`](R) reader structure"]
impl crate::Readable for PA_OVTDISrs {}
#[doc = "`write(|w| ..)` method takes [`pa_ovtdis::W`](W) writer structure"]
impl crate::Writable for PA_OVTDISrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA_OVTDIS to value 0"]
impl crate::Resettable for PA_OVTDISrs {
    const RESET_VALUE: u32 = 0;
}
