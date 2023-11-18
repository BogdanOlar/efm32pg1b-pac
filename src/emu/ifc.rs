#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `VMONAVDDFALL` writer - Clear VMONAVDDFALL Interrupt Flag"]
pub type VMONAVDDFALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONAVDDRISE` writer - Clear VMONAVDDRISE Interrupt Flag"]
pub type VMONAVDDRISE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONALTAVDDFALL` writer - Clear VMONALTAVDDFALL Interrupt Flag"]
pub type VMONALTAVDDFALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONALTAVDDRISE` writer - Clear VMONALTAVDDRISE Interrupt Flag"]
pub type VMONALTAVDDRISE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONDVDDFALL` writer - Clear VMONDVDDFALL Interrupt Flag"]
pub type VMONDVDDFALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONDVDDRISE` writer - Clear VMONDVDDRISE Interrupt Flag"]
pub type VMONDVDDRISE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONIO0FALL` writer - Clear VMONIO0FALL Interrupt Flag"]
pub type VMONIO0FALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONIO0RISE` writer - Clear VMONIO0RISE Interrupt Flag"]
pub type VMONIO0RISE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONFVDDFALL` writer - Clear VMONFVDDFALL Interrupt Flag"]
pub type VMONFVDDFALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONFVDDRISE` writer - Clear VMONFVDDRISE Interrupt Flag"]
pub type VMONFVDDRISE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - Clear PFETOVERCURRENTLIMIT Interrupt Flag"]
pub type PFETOVERCURRENTLIMIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - Clear NFETOVERCURRENTLIMIT Interrupt Flag"]
pub type NFETOVERCURRENTLIMIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCDCLPRUNNING` writer - Clear DCDCLPRUNNING Interrupt Flag"]
pub type DCDCLPRUNNING_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCDCLNRUNNING` writer - Clear DCDCLNRUNNING Interrupt Flag"]
pub type DCDCLNRUNNING_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCDCINBYPASS` writer - Clear DCDCINBYPASS Interrupt Flag"]
pub type DCDCINBYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM23WAKEUP` writer - Clear EM23WAKEUP Interrupt Flag"]
pub type EM23WAKEUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEMP` writer - Clear TEMP Interrupt Flag"]
pub type TEMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEMPLOW` writer - Clear TEMPLOW Interrupt Flag"]
pub type TEMPLOW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEMPHIGH` writer - Clear TEMPHIGH Interrupt Flag"]
pub type TEMPHIGH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl core::fmt::Debug for crate::generic::Reg<IFC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear VMONAVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddfall(&mut self) -> VMONAVDDFALL_W<IFC_SPEC, 0> {
        VMONAVDDFALL_W::new(self)
    }
    #[doc = "Bit 1 - Clear VMONAVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddrise(&mut self) -> VMONAVDDRISE_W<IFC_SPEC, 1> {
        VMONAVDDRISE_W::new(self)
    }
    #[doc = "Bit 2 - Clear VMONALTAVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddfall(&mut self) -> VMONALTAVDDFALL_W<IFC_SPEC, 2> {
        VMONALTAVDDFALL_W::new(self)
    }
    #[doc = "Bit 3 - Clear VMONALTAVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddrise(&mut self) -> VMONALTAVDDRISE_W<IFC_SPEC, 3> {
        VMONALTAVDDRISE_W::new(self)
    }
    #[doc = "Bit 4 - Clear VMONDVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddfall(&mut self) -> VMONDVDDFALL_W<IFC_SPEC, 4> {
        VMONDVDDFALL_W::new(self)
    }
    #[doc = "Bit 5 - Clear VMONDVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddrise(&mut self) -> VMONDVDDRISE_W<IFC_SPEC, 5> {
        VMONDVDDRISE_W::new(self)
    }
    #[doc = "Bit 6 - Clear VMONIO0FALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0fall(&mut self) -> VMONIO0FALL_W<IFC_SPEC, 6> {
        VMONIO0FALL_W::new(self)
    }
    #[doc = "Bit 7 - Clear VMONIO0RISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0rise(&mut self) -> VMONIO0RISE_W<IFC_SPEC, 7> {
        VMONIO0RISE_W::new(self)
    }
    #[doc = "Bit 14 - Clear VMONFVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddfall(&mut self) -> VMONFVDDFALL_W<IFC_SPEC, 14> {
        VMONFVDDFALL_W::new(self)
    }
    #[doc = "Bit 15 - Clear VMONFVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddrise(&mut self) -> VMONFVDDRISE_W<IFC_SPEC, 15> {
        VMONFVDDRISE_W::new(self)
    }
    #[doc = "Bit 16 - Clear PFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pfetovercurrentlimit(&mut self) -> PFETOVERCURRENTLIMIT_W<IFC_SPEC, 16> {
        PFETOVERCURRENTLIMIT_W::new(self)
    }
    #[doc = "Bit 17 - Clear NFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nfetovercurrentlimit(&mut self) -> NFETOVERCURRENTLIMIT_W<IFC_SPEC, 17> {
        NFETOVERCURRENTLIMIT_W::new(self)
    }
    #[doc = "Bit 18 - Clear DCDCLPRUNNING Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclprunning(&mut self) -> DCDCLPRUNNING_W<IFC_SPEC, 18> {
        DCDCLPRUNNING_W::new(self)
    }
    #[doc = "Bit 19 - Clear DCDCLNRUNNING Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclnrunning(&mut self) -> DCDCLNRUNNING_W<IFC_SPEC, 19> {
        DCDCLNRUNNING_W::new(self)
    }
    #[doc = "Bit 20 - Clear DCDCINBYPASS Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcinbypass(&mut self) -> DCDCINBYPASS_W<IFC_SPEC, 20> {
        DCDCINBYPASS_W::new(self)
    }
    #[doc = "Bit 24 - Clear EM23WAKEUP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn em23wakeup(&mut self) -> EM23WAKEUP_W<IFC_SPEC, 24> {
        EM23WAKEUP_W::new(self)
    }
    #[doc = "Bit 29 - Clear TEMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn temp(&mut self) -> TEMP_W<IFC_SPEC, 29> {
        TEMP_W::new(self)
    }
    #[doc = "Bit 30 - Clear TEMPLOW Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn templow(&mut self) -> TEMPLOW_W<IFC_SPEC, 30> {
        TEMPLOW_W::new(self)
    }
    #[doc = "Bit 31 - Clear TEMPHIGH Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn temphigh(&mut self) -> TEMPHIGH_W<IFC_SPEC, 31> {
        TEMPHIGH_W::new(self)
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
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
