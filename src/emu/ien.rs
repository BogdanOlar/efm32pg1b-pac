#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `VMONAVDDFALL` reader - VMONAVDDFALL Interrupt Enable"]
pub type VMONAVDDFALL_R = crate::BitReader;
#[doc = "Field `VMONAVDDFALL` writer - VMONAVDDFALL Interrupt Enable"]
pub type VMONAVDDFALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONAVDDRISE` reader - VMONAVDDRISE Interrupt Enable"]
pub type VMONAVDDRISE_R = crate::BitReader;
#[doc = "Field `VMONAVDDRISE` writer - VMONAVDDRISE Interrupt Enable"]
pub type VMONAVDDRISE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONALTAVDDFALL` reader - VMONALTAVDDFALL Interrupt Enable"]
pub type VMONALTAVDDFALL_R = crate::BitReader;
#[doc = "Field `VMONALTAVDDFALL` writer - VMONALTAVDDFALL Interrupt Enable"]
pub type VMONALTAVDDFALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONALTAVDDRISE` reader - VMONALTAVDDRISE Interrupt Enable"]
pub type VMONALTAVDDRISE_R = crate::BitReader;
#[doc = "Field `VMONALTAVDDRISE` writer - VMONALTAVDDRISE Interrupt Enable"]
pub type VMONALTAVDDRISE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONDVDDFALL` reader - VMONDVDDFALL Interrupt Enable"]
pub type VMONDVDDFALL_R = crate::BitReader;
#[doc = "Field `VMONDVDDFALL` writer - VMONDVDDFALL Interrupt Enable"]
pub type VMONDVDDFALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONDVDDRISE` reader - VMONDVDDRISE Interrupt Enable"]
pub type VMONDVDDRISE_R = crate::BitReader;
#[doc = "Field `VMONDVDDRISE` writer - VMONDVDDRISE Interrupt Enable"]
pub type VMONDVDDRISE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONIO0FALL` reader - VMONIO0FALL Interrupt Enable"]
pub type VMONIO0FALL_R = crate::BitReader;
#[doc = "Field `VMONIO0FALL` writer - VMONIO0FALL Interrupt Enable"]
pub type VMONIO0FALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONIO0RISE` reader - VMONIO0RISE Interrupt Enable"]
pub type VMONIO0RISE_R = crate::BitReader;
#[doc = "Field `VMONIO0RISE` writer - VMONIO0RISE Interrupt Enable"]
pub type VMONIO0RISE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONFVDDFALL` reader - VMONFVDDFALL Interrupt Enable"]
pub type VMONFVDDFALL_R = crate::BitReader;
#[doc = "Field `VMONFVDDFALL` writer - VMONFVDDFALL Interrupt Enable"]
pub type VMONFVDDFALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMONFVDDRISE` reader - VMONFVDDRISE Interrupt Enable"]
pub type VMONFVDDRISE_R = crate::BitReader;
#[doc = "Field `VMONFVDDRISE` writer - VMONFVDDRISE Interrupt Enable"]
pub type VMONFVDDRISE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PFETOVERCURRENTLIMIT` reader - PFETOVERCURRENTLIMIT Interrupt Enable"]
pub type PFETOVERCURRENTLIMIT_R = crate::BitReader;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - PFETOVERCURRENTLIMIT Interrupt Enable"]
pub type PFETOVERCURRENTLIMIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NFETOVERCURRENTLIMIT` reader - NFETOVERCURRENTLIMIT Interrupt Enable"]
pub type NFETOVERCURRENTLIMIT_R = crate::BitReader;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - NFETOVERCURRENTLIMIT Interrupt Enable"]
pub type NFETOVERCURRENTLIMIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCDCLPRUNNING` reader - DCDCLPRUNNING Interrupt Enable"]
pub type DCDCLPRUNNING_R = crate::BitReader;
#[doc = "Field `DCDCLPRUNNING` writer - DCDCLPRUNNING Interrupt Enable"]
pub type DCDCLPRUNNING_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCDCLNRUNNING` reader - DCDCLNRUNNING Interrupt Enable"]
pub type DCDCLNRUNNING_R = crate::BitReader;
#[doc = "Field `DCDCLNRUNNING` writer - DCDCLNRUNNING Interrupt Enable"]
pub type DCDCLNRUNNING_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCDCINBYPASS` reader - DCDCINBYPASS Interrupt Enable"]
pub type DCDCINBYPASS_R = crate::BitReader;
#[doc = "Field `DCDCINBYPASS` writer - DCDCINBYPASS Interrupt Enable"]
pub type DCDCINBYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM23WAKEUP` reader - EM23WAKEUP Interrupt Enable"]
pub type EM23WAKEUP_R = crate::BitReader;
#[doc = "Field `EM23WAKEUP` writer - EM23WAKEUP Interrupt Enable"]
pub type EM23WAKEUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEMP` reader - TEMP Interrupt Enable"]
pub type TEMP_R = crate::BitReader;
#[doc = "Field `TEMP` writer - TEMP Interrupt Enable"]
pub type TEMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEMPLOW` reader - TEMPLOW Interrupt Enable"]
pub type TEMPLOW_R = crate::BitReader;
#[doc = "Field `TEMPLOW` writer - TEMPLOW Interrupt Enable"]
pub type TEMPLOW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEMPHIGH` reader - TEMPHIGH Interrupt Enable"]
pub type TEMPHIGH_R = crate::BitReader;
#[doc = "Field `TEMPHIGH` writer - TEMPHIGH Interrupt Enable"]
pub type TEMPHIGH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - VMONAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddfall(&self) -> VMONAVDDFALL_R {
        VMONAVDDFALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VMONAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddrise(&self) -> VMONAVDDRISE_R {
        VMONAVDDRISE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VMONALTAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddfall(&self) -> VMONALTAVDDFALL_R {
        VMONALTAVDDFALL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VMONALTAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddrise(&self) -> VMONALTAVDDRISE_R {
        VMONALTAVDDRISE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VMONDVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddfall(&self) -> VMONDVDDFALL_R {
        VMONDVDDFALL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VMONDVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddrise(&self) -> VMONDVDDRISE_R {
        VMONDVDDRISE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VMONIO0FALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0fall(&self) -> VMONIO0FALL_R {
        VMONIO0FALL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VMONIO0RISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0rise(&self) -> VMONIO0RISE_R {
        VMONIO0RISE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 14 - VMONFVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonfvddfall(&self) -> VMONFVDDFALL_R {
        VMONFVDDFALL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VMONFVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonfvddrise(&self) -> VMONFVDDRISE_R {
        VMONFVDDRISE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&self) -> PFETOVERCURRENTLIMIT_R {
        PFETOVERCURRENTLIMIT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&self) -> NFETOVERCURRENTLIMIT_R {
        NFETOVERCURRENTLIMIT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DCDCLPRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclprunning(&self) -> DCDCLPRUNNING_R {
        DCDCLPRUNNING_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DCDCLNRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclnrunning(&self) -> DCDCLNRUNNING_R {
        DCDCLNRUNNING_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DCDCINBYPASS Interrupt Enable"]
    #[inline(always)]
    pub fn dcdcinbypass(&self) -> DCDCINBYPASS_R {
        DCDCINBYPASS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - EM23WAKEUP Interrupt Enable"]
    #[inline(always)]
    pub fn em23wakeup(&self) -> EM23WAKEUP_R {
        EM23WAKEUP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - TEMP Interrupt Enable"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TEMPLOW Interrupt Enable"]
    #[inline(always)]
    pub fn templow(&self) -> TEMPLOW_R {
        TEMPLOW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TEMPHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn temphigh(&self) -> TEMPHIGH_R {
        TEMPHIGH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field(
                "vmonavddfall",
                &format_args!("{}", self.vmonavddfall().bit()),
            )
            .field(
                "vmonavddrise",
                &format_args!("{}", self.vmonavddrise().bit()),
            )
            .field(
                "vmonaltavddfall",
                &format_args!("{}", self.vmonaltavddfall().bit()),
            )
            .field(
                "vmonaltavddrise",
                &format_args!("{}", self.vmonaltavddrise().bit()),
            )
            .field(
                "vmondvddfall",
                &format_args!("{}", self.vmondvddfall().bit()),
            )
            .field(
                "vmondvddrise",
                &format_args!("{}", self.vmondvddrise().bit()),
            )
            .field("vmonio0fall", &format_args!("{}", self.vmonio0fall().bit()))
            .field("vmonio0rise", &format_args!("{}", self.vmonio0rise().bit()))
            .field(
                "vmonfvddfall",
                &format_args!("{}", self.vmonfvddfall().bit()),
            )
            .field(
                "vmonfvddrise",
                &format_args!("{}", self.vmonfvddrise().bit()),
            )
            .field(
                "pfetovercurrentlimit",
                &format_args!("{}", self.pfetovercurrentlimit().bit()),
            )
            .field(
                "nfetovercurrentlimit",
                &format_args!("{}", self.nfetovercurrentlimit().bit()),
            )
            .field(
                "dcdclprunning",
                &format_args!("{}", self.dcdclprunning().bit()),
            )
            .field(
                "dcdclnrunning",
                &format_args!("{}", self.dcdclnrunning().bit()),
            )
            .field(
                "dcdcinbypass",
                &format_args!("{}", self.dcdcinbypass().bit()),
            )
            .field("em23wakeup", &format_args!("{}", self.em23wakeup().bit()))
            .field("temp", &format_args!("{}", self.temp().bit()))
            .field("templow", &format_args!("{}", self.templow().bit()))
            .field("temphigh", &format_args!("{}", self.temphigh().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<IEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - VMONAVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddfall(&mut self) -> VMONAVDDFALL_W<IEN_SPEC, 0> {
        VMONAVDDFALL_W::new(self)
    }
    #[doc = "Bit 1 - VMONAVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddrise(&mut self) -> VMONAVDDRISE_W<IEN_SPEC, 1> {
        VMONAVDDRISE_W::new(self)
    }
    #[doc = "Bit 2 - VMONALTAVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddfall(&mut self) -> VMONALTAVDDFALL_W<IEN_SPEC, 2> {
        VMONALTAVDDFALL_W::new(self)
    }
    #[doc = "Bit 3 - VMONALTAVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddrise(&mut self) -> VMONALTAVDDRISE_W<IEN_SPEC, 3> {
        VMONALTAVDDRISE_W::new(self)
    }
    #[doc = "Bit 4 - VMONDVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddfall(&mut self) -> VMONDVDDFALL_W<IEN_SPEC, 4> {
        VMONDVDDFALL_W::new(self)
    }
    #[doc = "Bit 5 - VMONDVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddrise(&mut self) -> VMONDVDDRISE_W<IEN_SPEC, 5> {
        VMONDVDDRISE_W::new(self)
    }
    #[doc = "Bit 6 - VMONIO0FALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0fall(&mut self) -> VMONIO0FALL_W<IEN_SPEC, 6> {
        VMONIO0FALL_W::new(self)
    }
    #[doc = "Bit 7 - VMONIO0RISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0rise(&mut self) -> VMONIO0RISE_W<IEN_SPEC, 7> {
        VMONIO0RISE_W::new(self)
    }
    #[doc = "Bit 14 - VMONFVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddfall(&mut self) -> VMONFVDDFALL_W<IEN_SPEC, 14> {
        VMONFVDDFALL_W::new(self)
    }
    #[doc = "Bit 15 - VMONFVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddrise(&mut self) -> VMONFVDDRISE_W<IEN_SPEC, 15> {
        VMONFVDDRISE_W::new(self)
    }
    #[doc = "Bit 16 - PFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfetovercurrentlimit(&mut self) -> PFETOVERCURRENTLIMIT_W<IEN_SPEC, 16> {
        PFETOVERCURRENTLIMIT_W::new(self)
    }
    #[doc = "Bit 17 - NFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfetovercurrentlimit(&mut self) -> NFETOVERCURRENTLIMIT_W<IEN_SPEC, 17> {
        NFETOVERCURRENTLIMIT_W::new(self)
    }
    #[doc = "Bit 18 - DCDCLPRUNNING Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclprunning(&mut self) -> DCDCLPRUNNING_W<IEN_SPEC, 18> {
        DCDCLPRUNNING_W::new(self)
    }
    #[doc = "Bit 19 - DCDCLNRUNNING Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclnrunning(&mut self) -> DCDCLNRUNNING_W<IEN_SPEC, 19> {
        DCDCLNRUNNING_W::new(self)
    }
    #[doc = "Bit 20 - DCDCINBYPASS Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcinbypass(&mut self) -> DCDCINBYPASS_W<IEN_SPEC, 20> {
        DCDCINBYPASS_W::new(self)
    }
    #[doc = "Bit 24 - EM23WAKEUP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em23wakeup(&mut self) -> EM23WAKEUP_W<IEN_SPEC, 24> {
        EM23WAKEUP_W::new(self)
    }
    #[doc = "Bit 29 - TEMP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn temp(&mut self) -> TEMP_W<IEN_SPEC, 29> {
        TEMP_W::new(self)
    }
    #[doc = "Bit 30 - TEMPLOW Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn templow(&mut self) -> TEMPLOW_W<IEN_SPEC, 30> {
        TEMPLOW_W::new(self)
    }
    #[doc = "Bit 31 - TEMPHIGH Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn temphigh(&mut self) -> TEMPHIGH_W<IEN_SPEC, 31> {
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
