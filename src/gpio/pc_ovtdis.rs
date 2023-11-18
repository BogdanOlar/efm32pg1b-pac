#[doc = "Register `PC_OVTDIS` reader"]
pub type R = crate::R<PC_OVTDIS_SPEC>;
#[doc = "Register `PC_OVTDIS` writer"]
pub type W = crate::W<PC_OVTDIS_SPEC>;
#[doc = "Field `OVTDIS` reader - Disable Over Voltage Capability"]
pub type OVTDIS_R = crate::FieldReader<u16>;
#[doc = "Field `OVTDIS` writer - Disable Over Voltage Capability"]
pub type OVTDIS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis(&self) -> OVTDIS_R {
        OVTDIS_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PC_OVTDIS")
            .field("ovtdis", &format_args!("{}", self.ovtdis().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PC_OVTDIS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis(&mut self) -> OVTDIS_W<PC_OVTDIS_SPEC, 0> {
        OVTDIS_W::new(self)
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
#[doc = "Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_ovtdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_ovtdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC_OVTDIS_SPEC;
impl crate::RegisterSpec for PC_OVTDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_ovtdis::R`](R) reader structure"]
impl crate::Readable for PC_OVTDIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc_ovtdis::W`](W) writer structure"]
impl crate::Writable for PC_OVTDIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PC_OVTDIS to value 0"]
impl crate::Resettable for PC_OVTDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
