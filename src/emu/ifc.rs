#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFCrs>;
#[doc = "Field `VMONAVDDFALL` writer - Clear VMONAVDDFALL Interrupt Flag"]
pub type VmonavddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONAVDDRISE` writer - Clear VMONAVDDRISE Interrupt Flag"]
pub type VmonavddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDFALL` writer - Clear VMONALTAVDDFALL Interrupt Flag"]
pub type VmonaltavddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDRISE` writer - Clear VMONALTAVDDRISE Interrupt Flag"]
pub type VmonaltavddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDFALL` writer - Clear VMONDVDDFALL Interrupt Flag"]
pub type VmondvddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDRISE` writer - Clear VMONDVDDRISE Interrupt Flag"]
pub type VmondvddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0FALL` writer - Clear VMONIO0FALL Interrupt Flag"]
pub type Vmonio0fallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0RISE` writer - Clear VMONIO0RISE Interrupt Flag"]
pub type Vmonio0riseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONFVDDFALL` writer - Clear VMONFVDDFALL Interrupt Flag"]
pub type VmonfvddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONFVDDRISE` writer - Clear VMONFVDDRISE Interrupt Flag"]
pub type VmonfvddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - Clear PFETOVERCURRENTLIMIT Interrupt Flag"]
pub type PfetovercurrentlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - Clear NFETOVERCURRENTLIMIT Interrupt Flag"]
pub type NfetovercurrentlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLPRUNNING` writer - Clear DCDCLPRUNNING Interrupt Flag"]
pub type DcdclprunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLNRUNNING` writer - Clear DCDCLNRUNNING Interrupt Flag"]
pub type DcdclnrunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCINBYPASS` writer - Clear DCDCINBYPASS Interrupt Flag"]
pub type DcdcinbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23WAKEUP` writer - Clear EM23WAKEUP Interrupt Flag"]
pub type Em23wakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP` writer - Clear TEMP Interrupt Flag"]
pub type TempW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPLOW` writer - Clear TEMPLOW Interrupt Flag"]
pub type TemplowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPHIGH` writer - Clear TEMPHIGH Interrupt Flag"]
pub type TemphighW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear VMONAVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddfall(&mut self) -> VmonavddfallW<IFCrs> {
        VmonavddfallW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear VMONAVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddrise(&mut self) -> VmonavddriseW<IFCrs> {
        VmonavddriseW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear VMONALTAVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddfall(&mut self) -> VmonaltavddfallW<IFCrs> {
        VmonaltavddfallW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear VMONALTAVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddrise(&mut self) -> VmonaltavddriseW<IFCrs> {
        VmonaltavddriseW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear VMONDVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddfall(&mut self) -> VmondvddfallW<IFCrs> {
        VmondvddfallW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear VMONDVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddrise(&mut self) -> VmondvddriseW<IFCrs> {
        VmondvddriseW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear VMONIO0FALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0fall(&mut self) -> Vmonio0fallW<IFCrs> {
        Vmonio0fallW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear VMONIO0RISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0rise(&mut self) -> Vmonio0riseW<IFCrs> {
        Vmonio0riseW::new(self, 7)
    }
    #[doc = "Bit 14 - Clear VMONFVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddfall(&mut self) -> VmonfvddfallW<IFCrs> {
        VmonfvddfallW::new(self, 14)
    }
    #[doc = "Bit 15 - Clear VMONFVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddrise(&mut self) -> VmonfvddriseW<IFCrs> {
        VmonfvddriseW::new(self, 15)
    }
    #[doc = "Bit 16 - Clear PFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pfetovercurrentlimit(&mut self) -> PfetovercurrentlimitW<IFCrs> {
        PfetovercurrentlimitW::new(self, 16)
    }
    #[doc = "Bit 17 - Clear NFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nfetovercurrentlimit(&mut self) -> NfetovercurrentlimitW<IFCrs> {
        NfetovercurrentlimitW::new(self, 17)
    }
    #[doc = "Bit 18 - Clear DCDCLPRUNNING Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclprunning(&mut self) -> DcdclprunningW<IFCrs> {
        DcdclprunningW::new(self, 18)
    }
    #[doc = "Bit 19 - Clear DCDCLNRUNNING Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclnrunning(&mut self) -> DcdclnrunningW<IFCrs> {
        DcdclnrunningW::new(self, 19)
    }
    #[doc = "Bit 20 - Clear DCDCINBYPASS Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcinbypass(&mut self) -> DcdcinbypassW<IFCrs> {
        DcdcinbypassW::new(self, 20)
    }
    #[doc = "Bit 24 - Clear EM23WAKEUP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn em23wakeup(&mut self) -> Em23wakeupW<IFCrs> {
        Em23wakeupW::new(self, 24)
    }
    #[doc = "Bit 29 - Clear TEMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn temp(&mut self) -> TempW<IFCrs> {
        TempW::new(self, 29)
    }
    #[doc = "Bit 30 - Clear TEMPLOW Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn templow(&mut self) -> TemplowW<IFCrs> {
        TemplowW::new(self, 30)
    }
    #[doc = "Bit 31 - Clear TEMPHIGH Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn temphigh(&mut self) -> TemphighW<IFCrs> {
        TemphighW::new(self, 31)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCrs;
impl crate::RegisterSpec for IFCrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFCrs {
    const RESET_VALUE: u32 = 0;
}
