#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFCrs>;
#[doc = "Field `VMONAVDDFALL` writer - Clear VMONAVDDFALL Interrupt Flag"]
pub type VMONAVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONAVDDRISE` writer - Clear VMONAVDDRISE Interrupt Flag"]
pub type VMONAVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDFALL` writer - Clear VMONALTAVDDFALL Interrupt Flag"]
pub type VMONALTAVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDRISE` writer - Clear VMONALTAVDDRISE Interrupt Flag"]
pub type VMONALTAVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDFALL` writer - Clear VMONDVDDFALL Interrupt Flag"]
pub type VMONDVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDRISE` writer - Clear VMONDVDDRISE Interrupt Flag"]
pub type VMONDVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0FALL` writer - Clear VMONIO0FALL Interrupt Flag"]
pub type VMONIO0FALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0RISE` writer - Clear VMONIO0RISE Interrupt Flag"]
pub type VMONIO0RISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONFVDDFALL` writer - Clear VMONFVDDFALL Interrupt Flag"]
pub type VMONFVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONFVDDRISE` writer - Clear VMONFVDDRISE Interrupt Flag"]
pub type VMONFVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - Clear PFETOVERCURRENTLIMIT Interrupt Flag"]
pub type PFETOVERCURRENTLIMIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - Clear NFETOVERCURRENTLIMIT Interrupt Flag"]
pub type NFETOVERCURRENTLIMIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLPRUNNING` writer - Clear DCDCLPRUNNING Interrupt Flag"]
pub type DCDCLPRUNNING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLNRUNNING` writer - Clear DCDCLNRUNNING Interrupt Flag"]
pub type DCDCLNRUNNING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCINBYPASS` writer - Clear DCDCINBYPASS Interrupt Flag"]
pub type DCDCINBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23WAKEUP` writer - Clear EM23WAKEUP Interrupt Flag"]
pub type EM23WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP` writer - Clear TEMP Interrupt Flag"]
pub type TEMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPLOW` writer - Clear TEMPLOW Interrupt Flag"]
pub type TEMPLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPHIGH` writer - Clear TEMPHIGH Interrupt Flag"]
pub type TEMPHIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear VMONAVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddfall(&mut self) -> VMONAVDDFALL_W<IFCrs> {
        VMONAVDDFALL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear VMONAVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddrise(&mut self) -> VMONAVDDRISE_W<IFCrs> {
        VMONAVDDRISE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear VMONALTAVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddfall(&mut self) -> VMONALTAVDDFALL_W<IFCrs> {
        VMONALTAVDDFALL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear VMONALTAVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddrise(&mut self) -> VMONALTAVDDRISE_W<IFCrs> {
        VMONALTAVDDRISE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear VMONDVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddfall(&mut self) -> VMONDVDDFALL_W<IFCrs> {
        VMONDVDDFALL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear VMONDVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddrise(&mut self) -> VMONDVDDRISE_W<IFCrs> {
        VMONDVDDRISE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear VMONIO0FALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0fall(&mut self) -> VMONIO0FALL_W<IFCrs> {
        VMONIO0FALL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear VMONIO0RISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0rise(&mut self) -> VMONIO0RISE_W<IFCrs> {
        VMONIO0RISE_W::new(self, 7)
    }
    #[doc = "Bit 14 - Clear VMONFVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddfall(&mut self) -> VMONFVDDFALL_W<IFCrs> {
        VMONFVDDFALL_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear VMONFVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddrise(&mut self) -> VMONFVDDRISE_W<IFCrs> {
        VMONFVDDRISE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear PFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pfetovercurrentlimit(&mut self) -> PFETOVERCURRENTLIMIT_W<IFCrs> {
        PFETOVERCURRENTLIMIT_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear NFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nfetovercurrentlimit(&mut self) -> NFETOVERCURRENTLIMIT_W<IFCrs> {
        NFETOVERCURRENTLIMIT_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear DCDCLPRUNNING Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclprunning(&mut self) -> DCDCLPRUNNING_W<IFCrs> {
        DCDCLPRUNNING_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear DCDCLNRUNNING Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclnrunning(&mut self) -> DCDCLNRUNNING_W<IFCrs> {
        DCDCLNRUNNING_W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear DCDCINBYPASS Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcinbypass(&mut self) -> DCDCINBYPASS_W<IFCrs> {
        DCDCINBYPASS_W::new(self, 20)
    }
    #[doc = "Bit 24 - Clear EM23WAKEUP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn em23wakeup(&mut self) -> EM23WAKEUP_W<IFCrs> {
        EM23WAKEUP_W::new(self, 24)
    }
    #[doc = "Bit 29 - Clear TEMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn temp(&mut self) -> TEMP_W<IFCrs> {
        TEMP_W::new(self, 29)
    }
    #[doc = "Bit 30 - Clear TEMPLOW Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn templow(&mut self) -> TEMPLOW_W<IFCrs> {
        TEMPLOW_W::new(self, 30)
    }
    #[doc = "Bit 31 - Clear TEMPHIGH Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn temphigh(&mut self) -> TEMPHIGH_W<IFCrs> {
        TEMPHIGH_W::new(self, 31)
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
pub struct IFCrs;
impl crate::RegisterSpec for IFCrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFCrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFCrs {
    const RESET_VALUE: u32 = 0;
}
