#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFSrs>;
#[doc = "Field `VMONAVDDFALL` writer - Set VMONAVDDFALL Interrupt Flag"]
pub type VMONAVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONAVDDRISE` writer - Set VMONAVDDRISE Interrupt Flag"]
pub type VMONAVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDFALL` writer - Set VMONALTAVDDFALL Interrupt Flag"]
pub type VMONALTAVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDRISE` writer - Set VMONALTAVDDRISE Interrupt Flag"]
pub type VMONALTAVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDFALL` writer - Set VMONDVDDFALL Interrupt Flag"]
pub type VMONDVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDRISE` writer - Set VMONDVDDRISE Interrupt Flag"]
pub type VMONDVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0FALL` writer - Set VMONIO0FALL Interrupt Flag"]
pub type VMONIO0FALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0RISE` writer - Set VMONIO0RISE Interrupt Flag"]
pub type VMONIO0RISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONFVDDFALL` writer - Set VMONFVDDFALL Interrupt Flag"]
pub type VMONFVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONFVDDRISE` writer - Set VMONFVDDRISE Interrupt Flag"]
pub type VMONFVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - Set PFETOVERCURRENTLIMIT Interrupt Flag"]
pub type PFETOVERCURRENTLIMIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - Set NFETOVERCURRENTLIMIT Interrupt Flag"]
pub type NFETOVERCURRENTLIMIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLPRUNNING` writer - Set DCDCLPRUNNING Interrupt Flag"]
pub type DCDCLPRUNNING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLNRUNNING` writer - Set DCDCLNRUNNING Interrupt Flag"]
pub type DCDCLNRUNNING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCINBYPASS` writer - Set DCDCINBYPASS Interrupt Flag"]
pub type DCDCINBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23WAKEUP` writer - Set EM23WAKEUP Interrupt Flag"]
pub type EM23WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP` writer - Set TEMP Interrupt Flag"]
pub type TEMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPLOW` writer - Set TEMPLOW Interrupt Flag"]
pub type TEMPLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPHIGH` writer - Set TEMPHIGH Interrupt Flag"]
pub type TEMPHIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set VMONAVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddfall(&mut self) -> VMONAVDDFALL_W<IFSrs> {
        VMONAVDDFALL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set VMONAVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddrise(&mut self) -> VMONAVDDRISE_W<IFSrs> {
        VMONAVDDRISE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set VMONALTAVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddfall(&mut self) -> VMONALTAVDDFALL_W<IFSrs> {
        VMONALTAVDDFALL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set VMONALTAVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddrise(&mut self) -> VMONALTAVDDRISE_W<IFSrs> {
        VMONALTAVDDRISE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set VMONDVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddfall(&mut self) -> VMONDVDDFALL_W<IFSrs> {
        VMONDVDDFALL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set VMONDVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddrise(&mut self) -> VMONDVDDRISE_W<IFSrs> {
        VMONDVDDRISE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set VMONIO0FALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0fall(&mut self) -> VMONIO0FALL_W<IFSrs> {
        VMONIO0FALL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set VMONIO0RISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0rise(&mut self) -> VMONIO0RISE_W<IFSrs> {
        VMONIO0RISE_W::new(self, 7)
    }
    #[doc = "Bit 14 - Set VMONFVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddfall(&mut self) -> VMONFVDDFALL_W<IFSrs> {
        VMONFVDDFALL_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set VMONFVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddrise(&mut self) -> VMONFVDDRISE_W<IFSrs> {
        VMONFVDDRISE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set PFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pfetovercurrentlimit(&mut self) -> PFETOVERCURRENTLIMIT_W<IFSrs> {
        PFETOVERCURRENTLIMIT_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set NFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nfetovercurrentlimit(&mut self) -> NFETOVERCURRENTLIMIT_W<IFSrs> {
        NFETOVERCURRENTLIMIT_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set DCDCLPRUNNING Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclprunning(&mut self) -> DCDCLPRUNNING_W<IFSrs> {
        DCDCLPRUNNING_W::new(self, 18)
    }
    #[doc = "Bit 19 - Set DCDCLNRUNNING Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclnrunning(&mut self) -> DCDCLNRUNNING_W<IFSrs> {
        DCDCLNRUNNING_W::new(self, 19)
    }
    #[doc = "Bit 20 - Set DCDCINBYPASS Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcinbypass(&mut self) -> DCDCINBYPASS_W<IFSrs> {
        DCDCINBYPASS_W::new(self, 20)
    }
    #[doc = "Bit 24 - Set EM23WAKEUP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn em23wakeup(&mut self) -> EM23WAKEUP_W<IFSrs> {
        EM23WAKEUP_W::new(self, 24)
    }
    #[doc = "Bit 29 - Set TEMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn temp(&mut self) -> TEMP_W<IFSrs> {
        TEMP_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set TEMPLOW Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn templow(&mut self) -> TEMPLOW_W<IFSrs> {
        TEMPLOW_W::new(self, 30)
    }
    #[doc = "Bit 31 - Set TEMPHIGH Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn temphigh(&mut self) -> TEMPHIGH_W<IFSrs> {
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
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFSrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
