#[doc = "Register `IEN` reader"]
pub type R = crate::R<IENrs>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IENrs>;
#[doc = "Field `VMONAVDDFALL` reader - VMONAVDDFALL Interrupt Enable"]
pub type VmonavddfallR = crate::BitReader;
#[doc = "Field `VMONAVDDFALL` writer - VMONAVDDFALL Interrupt Enable"]
pub type VmonavddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONAVDDRISE` reader - VMONAVDDRISE Interrupt Enable"]
pub type VmonavddriseR = crate::BitReader;
#[doc = "Field `VMONAVDDRISE` writer - VMONAVDDRISE Interrupt Enable"]
pub type VmonavddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDFALL` reader - VMONALTAVDDFALL Interrupt Enable"]
pub type VmonaltavddfallR = crate::BitReader;
#[doc = "Field `VMONALTAVDDFALL` writer - VMONALTAVDDFALL Interrupt Enable"]
pub type VmonaltavddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDRISE` reader - VMONALTAVDDRISE Interrupt Enable"]
pub type VmonaltavddriseR = crate::BitReader;
#[doc = "Field `VMONALTAVDDRISE` writer - VMONALTAVDDRISE Interrupt Enable"]
pub type VmonaltavddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDFALL` reader - VMONDVDDFALL Interrupt Enable"]
pub type VmondvddfallR = crate::BitReader;
#[doc = "Field `VMONDVDDFALL` writer - VMONDVDDFALL Interrupt Enable"]
pub type VmondvddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDRISE` reader - VMONDVDDRISE Interrupt Enable"]
pub type VmondvddriseR = crate::BitReader;
#[doc = "Field `VMONDVDDRISE` writer - VMONDVDDRISE Interrupt Enable"]
pub type VmondvddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0FALL` reader - VMONIO0FALL Interrupt Enable"]
pub type Vmonio0fallR = crate::BitReader;
#[doc = "Field `VMONIO0FALL` writer - VMONIO0FALL Interrupt Enable"]
pub type Vmonio0fallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0RISE` reader - VMONIO0RISE Interrupt Enable"]
pub type Vmonio0riseR = crate::BitReader;
#[doc = "Field `VMONIO0RISE` writer - VMONIO0RISE Interrupt Enable"]
pub type Vmonio0riseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONFVDDFALL` reader - VMONFVDDFALL Interrupt Enable"]
pub type VmonfvddfallR = crate::BitReader;
#[doc = "Field `VMONFVDDFALL` writer - VMONFVDDFALL Interrupt Enable"]
pub type VmonfvddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONFVDDRISE` reader - VMONFVDDRISE Interrupt Enable"]
pub type VmonfvddriseR = crate::BitReader;
#[doc = "Field `VMONFVDDRISE` writer - VMONFVDDRISE Interrupt Enable"]
pub type VmonfvddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFETOVERCURRENTLIMIT` reader - PFETOVERCURRENTLIMIT Interrupt Enable"]
pub type PfetovercurrentlimitR = crate::BitReader;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - PFETOVERCURRENTLIMIT Interrupt Enable"]
pub type PfetovercurrentlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFETOVERCURRENTLIMIT` reader - NFETOVERCURRENTLIMIT Interrupt Enable"]
pub type NfetovercurrentlimitR = crate::BitReader;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - NFETOVERCURRENTLIMIT Interrupt Enable"]
pub type NfetovercurrentlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLPRUNNING` reader - DCDCLPRUNNING Interrupt Enable"]
pub type DcdclprunningR = crate::BitReader;
#[doc = "Field `DCDCLPRUNNING` writer - DCDCLPRUNNING Interrupt Enable"]
pub type DcdclprunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLNRUNNING` reader - DCDCLNRUNNING Interrupt Enable"]
pub type DcdclnrunningR = crate::BitReader;
#[doc = "Field `DCDCLNRUNNING` writer - DCDCLNRUNNING Interrupt Enable"]
pub type DcdclnrunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCINBYPASS` reader - DCDCINBYPASS Interrupt Enable"]
pub type DcdcinbypassR = crate::BitReader;
#[doc = "Field `DCDCINBYPASS` writer - DCDCINBYPASS Interrupt Enable"]
pub type DcdcinbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23WAKEUP` reader - EM23WAKEUP Interrupt Enable"]
pub type Em23wakeupR = crate::BitReader;
#[doc = "Field `EM23WAKEUP` writer - EM23WAKEUP Interrupt Enable"]
pub type Em23wakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP` reader - TEMP Interrupt Enable"]
pub type TempR = crate::BitReader;
#[doc = "Field `TEMP` writer - TEMP Interrupt Enable"]
pub type TempW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPLOW` reader - TEMPLOW Interrupt Enable"]
pub type TemplowR = crate::BitReader;
#[doc = "Field `TEMPLOW` writer - TEMPLOW Interrupt Enable"]
pub type TemplowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPHIGH` reader - TEMPHIGH Interrupt Enable"]
pub type TemphighR = crate::BitReader;
#[doc = "Field `TEMPHIGH` writer - TEMPHIGH Interrupt Enable"]
pub type TemphighW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VMONAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddfall(&self) -> VmonavddfallR {
        VmonavddfallR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VMONAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddrise(&self) -> VmonavddriseR {
        VmonavddriseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VMONALTAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddfall(&self) -> VmonaltavddfallR {
        VmonaltavddfallR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VMONALTAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddrise(&self) -> VmonaltavddriseR {
        VmonaltavddriseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VMONDVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddfall(&self) -> VmondvddfallR {
        VmondvddfallR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VMONDVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddrise(&self) -> VmondvddriseR {
        VmondvddriseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VMONIO0FALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0fall(&self) -> Vmonio0fallR {
        Vmonio0fallR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VMONIO0RISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0rise(&self) -> Vmonio0riseR {
        Vmonio0riseR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 14 - VMONFVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonfvddfall(&self) -> VmonfvddfallR {
        VmonfvddfallR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VMONFVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonfvddrise(&self) -> VmonfvddriseR {
        VmonfvddriseR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&self) -> PfetovercurrentlimitR {
        PfetovercurrentlimitR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&self) -> NfetovercurrentlimitR {
        NfetovercurrentlimitR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DCDCLPRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclprunning(&self) -> DcdclprunningR {
        DcdclprunningR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DCDCLNRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclnrunning(&self) -> DcdclnrunningR {
        DcdclnrunningR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DCDCINBYPASS Interrupt Enable"]
    #[inline(always)]
    pub fn dcdcinbypass(&self) -> DcdcinbypassR {
        DcdcinbypassR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - EM23WAKEUP Interrupt Enable"]
    #[inline(always)]
    pub fn em23wakeup(&self) -> Em23wakeupR {
        Em23wakeupR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - TEMP Interrupt Enable"]
    #[inline(always)]
    pub fn temp(&self) -> TempR {
        TempR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TEMPLOW Interrupt Enable"]
    #[inline(always)]
    pub fn templow(&self) -> TemplowR {
        TemplowR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TEMPHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn temphigh(&self) -> TemphighR {
        TemphighR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("vmonavddfall", &self.vmonavddfall())
            .field("vmonavddrise", &self.vmonavddrise())
            .field("vmonaltavddfall", &self.vmonaltavddfall())
            .field("vmonaltavddrise", &self.vmonaltavddrise())
            .field("vmondvddfall", &self.vmondvddfall())
            .field("vmondvddrise", &self.vmondvddrise())
            .field("vmonio0fall", &self.vmonio0fall())
            .field("vmonio0rise", &self.vmonio0rise())
            .field("vmonfvddfall", &self.vmonfvddfall())
            .field("vmonfvddrise", &self.vmonfvddrise())
            .field("pfetovercurrentlimit", &self.pfetovercurrentlimit())
            .field("nfetovercurrentlimit", &self.nfetovercurrentlimit())
            .field("dcdclprunning", &self.dcdclprunning())
            .field("dcdclnrunning", &self.dcdclnrunning())
            .field("dcdcinbypass", &self.dcdcinbypass())
            .field("em23wakeup", &self.em23wakeup())
            .field("temp", &self.temp())
            .field("templow", &self.templow())
            .field("temphigh", &self.temphigh())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VMONAVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddfall(&mut self) -> VmonavddfallW<IENrs> {
        VmonavddfallW::new(self, 0)
    }
    #[doc = "Bit 1 - VMONAVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddrise(&mut self) -> VmonavddriseW<IENrs> {
        VmonavddriseW::new(self, 1)
    }
    #[doc = "Bit 2 - VMONALTAVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddfall(&mut self) -> VmonaltavddfallW<IENrs> {
        VmonaltavddfallW::new(self, 2)
    }
    #[doc = "Bit 3 - VMONALTAVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddrise(&mut self) -> VmonaltavddriseW<IENrs> {
        VmonaltavddriseW::new(self, 3)
    }
    #[doc = "Bit 4 - VMONDVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddfall(&mut self) -> VmondvddfallW<IENrs> {
        VmondvddfallW::new(self, 4)
    }
    #[doc = "Bit 5 - VMONDVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddrise(&mut self) -> VmondvddriseW<IENrs> {
        VmondvddriseW::new(self, 5)
    }
    #[doc = "Bit 6 - VMONIO0FALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0fall(&mut self) -> Vmonio0fallW<IENrs> {
        Vmonio0fallW::new(self, 6)
    }
    #[doc = "Bit 7 - VMONIO0RISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0rise(&mut self) -> Vmonio0riseW<IENrs> {
        Vmonio0riseW::new(self, 7)
    }
    #[doc = "Bit 14 - VMONFVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddfall(&mut self) -> VmonfvddfallW<IENrs> {
        VmonfvddfallW::new(self, 14)
    }
    #[doc = "Bit 15 - VMONFVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddrise(&mut self) -> VmonfvddriseW<IENrs> {
        VmonfvddriseW::new(self, 15)
    }
    #[doc = "Bit 16 - PFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfetovercurrentlimit(&mut self) -> PfetovercurrentlimitW<IENrs> {
        PfetovercurrentlimitW::new(self, 16)
    }
    #[doc = "Bit 17 - NFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfetovercurrentlimit(&mut self) -> NfetovercurrentlimitW<IENrs> {
        NfetovercurrentlimitW::new(self, 17)
    }
    #[doc = "Bit 18 - DCDCLPRUNNING Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclprunning(&mut self) -> DcdclprunningW<IENrs> {
        DcdclprunningW::new(self, 18)
    }
    #[doc = "Bit 19 - DCDCLNRUNNING Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclnrunning(&mut self) -> DcdclnrunningW<IENrs> {
        DcdclnrunningW::new(self, 19)
    }
    #[doc = "Bit 20 - DCDCINBYPASS Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcinbypass(&mut self) -> DcdcinbypassW<IENrs> {
        DcdcinbypassW::new(self, 20)
    }
    #[doc = "Bit 24 - EM23WAKEUP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em23wakeup(&mut self) -> Em23wakeupW<IENrs> {
        Em23wakeupW::new(self, 24)
    }
    #[doc = "Bit 29 - TEMP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn temp(&mut self) -> TempW<IENrs> {
        TempW::new(self, 29)
    }
    #[doc = "Bit 30 - TEMPLOW Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn templow(&mut self) -> TemplowW<IENrs> {
        TemplowW::new(self, 30)
    }
    #[doc = "Bit 31 - TEMPHIGH Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn temphigh(&mut self) -> TemphighW<IENrs> {
        TemphighW::new(self, 31)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IENrs {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
