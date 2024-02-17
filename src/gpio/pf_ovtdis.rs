#[doc = "Register `PF_OVTDIS` reader"]
pub type R = crate::R<PF_OVTDISrs>;
#[doc = "Register `PF_OVTDIS` writer"]
pub type W = crate::W<PF_OVTDISrs>;
#[doc = "Field `OVTDIS` reader - Disable Over Voltage Capability"]
pub type OVTDIS_R = crate::FieldReader<u16>;
#[doc = "Field `OVTDIS` writer - Disable Over Voltage Capability"]
pub type OVTDIS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis(&self) -> OVTDIS_R {
        OVTDIS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis(&mut self) -> OVTDIS_W<PF_OVTDISrs> {
        OVTDIS_W::new(self, 0)
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
#[doc = "Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_ovtdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_ovtdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PF_OVTDISrs;
impl crate::RegisterSpec for PF_OVTDISrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_ovtdis::R`](R) reader structure"]
impl crate::Readable for PF_OVTDISrs {}
#[doc = "`write(|w| ..)` method takes [`pf_ovtdis::W`](W) writer structure"]
impl crate::Writable for PF_OVTDISrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PF_OVTDIS to value 0"]
impl crate::Resettable for PF_OVTDISrs {
    const RESET_VALUE: u32 = 0;
}
