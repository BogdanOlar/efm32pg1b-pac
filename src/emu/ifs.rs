#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFSrs>;
#[doc = "Field `VMONAVDDFALL` writer - Set VMONAVDDFALL Interrupt Flag"]
pub type VmonavddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONAVDDRISE` writer - Set VMONAVDDRISE Interrupt Flag"]
pub type VmonavddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDFALL` writer - Set VMONALTAVDDFALL Interrupt Flag"]
pub type VmonaltavddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDRISE` writer - Set VMONALTAVDDRISE Interrupt Flag"]
pub type VmonaltavddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDFALL` writer - Set VMONDVDDFALL Interrupt Flag"]
pub type VmondvddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDRISE` writer - Set VMONDVDDRISE Interrupt Flag"]
pub type VmondvddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0FALL` writer - Set VMONIO0FALL Interrupt Flag"]
pub type Vmonio0fallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0RISE` writer - Set VMONIO0RISE Interrupt Flag"]
pub type Vmonio0riseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONFVDDFALL` writer - Set VMONFVDDFALL Interrupt Flag"]
pub type VmonfvddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONFVDDRISE` writer - Set VMONFVDDRISE Interrupt Flag"]
pub type VmonfvddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - Set PFETOVERCURRENTLIMIT Interrupt Flag"]
pub type PfetovercurrentlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - Set NFETOVERCURRENTLIMIT Interrupt Flag"]
pub type NfetovercurrentlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLPRUNNING` writer - Set DCDCLPRUNNING Interrupt Flag"]
pub type DcdclprunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLNRUNNING` writer - Set DCDCLNRUNNING Interrupt Flag"]
pub type DcdclnrunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCINBYPASS` writer - Set DCDCINBYPASS Interrupt Flag"]
pub type DcdcinbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23WAKEUP` writer - Set EM23WAKEUP Interrupt Flag"]
pub type Em23wakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP` writer - Set TEMP Interrupt Flag"]
pub type TempW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPLOW` writer - Set TEMPLOW Interrupt Flag"]
pub type TemplowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPHIGH` writer - Set TEMPHIGH Interrupt Flag"]
pub type TemphighW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set VMONAVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddfall(&mut self) -> VmonavddfallW<IFSrs> {
        VmonavddfallW::new(self, 0)
    }
    #[doc = "Bit 1 - Set VMONAVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddrise(&mut self) -> VmonavddriseW<IFSrs> {
        VmonavddriseW::new(self, 1)
    }
    #[doc = "Bit 2 - Set VMONALTAVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddfall(&mut self) -> VmonaltavddfallW<IFSrs> {
        VmonaltavddfallW::new(self, 2)
    }
    #[doc = "Bit 3 - Set VMONALTAVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddrise(&mut self) -> VmonaltavddriseW<IFSrs> {
        VmonaltavddriseW::new(self, 3)
    }
    #[doc = "Bit 4 - Set VMONDVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddfall(&mut self) -> VmondvddfallW<IFSrs> {
        VmondvddfallW::new(self, 4)
    }
    #[doc = "Bit 5 - Set VMONDVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddrise(&mut self) -> VmondvddriseW<IFSrs> {
        VmondvddriseW::new(self, 5)
    }
    #[doc = "Bit 6 - Set VMONIO0FALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0fall(&mut self) -> Vmonio0fallW<IFSrs> {
        Vmonio0fallW::new(self, 6)
    }
    #[doc = "Bit 7 - Set VMONIO0RISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0rise(&mut self) -> Vmonio0riseW<IFSrs> {
        Vmonio0riseW::new(self, 7)
    }
    #[doc = "Bit 14 - Set VMONFVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddfall(&mut self) -> VmonfvddfallW<IFSrs> {
        VmonfvddfallW::new(self, 14)
    }
    #[doc = "Bit 15 - Set VMONFVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddrise(&mut self) -> VmonfvddriseW<IFSrs> {
        VmonfvddriseW::new(self, 15)
    }
    #[doc = "Bit 16 - Set PFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pfetovercurrentlimit(&mut self) -> PfetovercurrentlimitW<IFSrs> {
        PfetovercurrentlimitW::new(self, 16)
    }
    #[doc = "Bit 17 - Set NFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nfetovercurrentlimit(&mut self) -> NfetovercurrentlimitW<IFSrs> {
        NfetovercurrentlimitW::new(self, 17)
    }
    #[doc = "Bit 18 - Set DCDCLPRUNNING Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclprunning(&mut self) -> DcdclprunningW<IFSrs> {
        DcdclprunningW::new(self, 18)
    }
    #[doc = "Bit 19 - Set DCDCLNRUNNING Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclnrunning(&mut self) -> DcdclnrunningW<IFSrs> {
        DcdclnrunningW::new(self, 19)
    }
    #[doc = "Bit 20 - Set DCDCINBYPASS Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcinbypass(&mut self) -> DcdcinbypassW<IFSrs> {
        DcdcinbypassW::new(self, 20)
    }
    #[doc = "Bit 24 - Set EM23WAKEUP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn em23wakeup(&mut self) -> Em23wakeupW<IFSrs> {
        Em23wakeupW::new(self, 24)
    }
    #[doc = "Bit 29 - Set TEMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn temp(&mut self) -> TempW<IFSrs> {
        TempW::new(self, 29)
    }
    #[doc = "Bit 30 - Set TEMPLOW Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn templow(&mut self) -> TemplowW<IFSrs> {
        TemplowW::new(self, 30)
    }
    #[doc = "Bit 31 - Set TEMPHIGH Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn temphigh(&mut self) -> TemphighW<IFSrs> {
        TemphighW::new(self, 31)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
