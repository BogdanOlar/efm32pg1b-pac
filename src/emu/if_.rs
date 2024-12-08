///Register `IF` reader
pub type R = crate::R<IFrs>;
///Field `VMONAVDDFALL` reader - VMON AVDD Channel Fall
pub type VmonavddfallR = crate::BitReader;
///Field `VMONAVDDRISE` reader - VMON AVDD Channel Rise
pub type VmonavddriseR = crate::BitReader;
///Field `VMONALTAVDDFALL` reader - Alternate VMON AVDD Channel Fall
pub type VmonaltavddfallR = crate::BitReader;
///Field `VMONALTAVDDRISE` reader - Alternate VMON AVDD Channel Rise
pub type VmonaltavddriseR = crate::BitReader;
///Field `VMONDVDDFALL` reader - VMON DVDD Channel Fall
pub type VmondvddfallR = crate::BitReader;
///Field `VMONDVDDRISE` reader - VMON DVDD Channel Rise
pub type VmondvddriseR = crate::BitReader;
///Field `VMONIO0FALL` reader - VMON IOVDD0 Channel Fall
pub type Vmonio0fallR = crate::BitReader;
///Field `VMONIO0RISE` reader - VMON IOVDD0 Channel Rise
pub type Vmonio0riseR = crate::BitReader;
///Field `VMONFVDDFALL` reader - VMON VDDFLASH Channel Fall
pub type VmonfvddfallR = crate::BitReader;
///Field `VMONFVDDRISE` reader - VMON VDDFLASH Channel Rise
pub type VmonfvddriseR = crate::BitReader;
///Field `PFETOVERCURRENTLIMIT` reader - PFET Current Limit Hit
pub type PfetovercurrentlimitR = crate::BitReader;
///Field `NFETOVERCURRENTLIMIT` reader - NFET Current Limit Hit
pub type NfetovercurrentlimitR = crate::BitReader;
///Field `DCDCLPRUNNING` reader - LP Mode is Running
pub type DcdclprunningR = crate::BitReader;
///Field `DCDCLNRUNNING` reader - LN Mode is Running
pub type DcdclnrunningR = crate::BitReader;
///Field `DCDCINBYPASS` reader - DCDC is in Bypass
pub type DcdcinbypassR = crate::BitReader;
///Field `EM23WAKEUP` reader - Wakeup IRQ From EM2 and EM3
pub type Em23wakeupR = crate::BitReader;
///Field `TEMP` reader - New Temperature Measurement Valid
pub type TempR = crate::BitReader;
///Field `TEMPLOW` reader - Temperature Low Limit Reached
pub type TemplowR = crate::BitReader;
///Field `TEMPHIGH` reader - Temperature High Limit Reached
pub type TemphighR = crate::BitReader;
impl R {
    ///Bit 0 - VMON AVDD Channel Fall
    #[inline(always)]
    pub fn vmonavddfall(&self) -> VmonavddfallR {
        VmonavddfallR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VMON AVDD Channel Rise
    #[inline(always)]
    pub fn vmonavddrise(&self) -> VmonavddriseR {
        VmonavddriseR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Alternate VMON AVDD Channel Fall
    #[inline(always)]
    pub fn vmonaltavddfall(&self) -> VmonaltavddfallR {
        VmonaltavddfallR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Alternate VMON AVDD Channel Rise
    #[inline(always)]
    pub fn vmonaltavddrise(&self) -> VmonaltavddriseR {
        VmonaltavddriseR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VMON DVDD Channel Fall
    #[inline(always)]
    pub fn vmondvddfall(&self) -> VmondvddfallR {
        VmondvddfallR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - VMON DVDD Channel Rise
    #[inline(always)]
    pub fn vmondvddrise(&self) -> VmondvddriseR {
        VmondvddriseR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - VMON IOVDD0 Channel Fall
    #[inline(always)]
    pub fn vmonio0fall(&self) -> Vmonio0fallR {
        Vmonio0fallR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - VMON IOVDD0 Channel Rise
    #[inline(always)]
    pub fn vmonio0rise(&self) -> Vmonio0riseR {
        Vmonio0riseR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 14 - VMON VDDFLASH Channel Fall
    #[inline(always)]
    pub fn vmonfvddfall(&self) -> VmonfvddfallR {
        VmonfvddfallR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - VMON VDDFLASH Channel Rise
    #[inline(always)]
    pub fn vmonfvddrise(&self) -> VmonfvddriseR {
        VmonfvddriseR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - PFET Current Limit Hit
    #[inline(always)]
    pub fn pfetovercurrentlimit(&self) -> PfetovercurrentlimitR {
        PfetovercurrentlimitR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - NFET Current Limit Hit
    #[inline(always)]
    pub fn nfetovercurrentlimit(&self) -> NfetovercurrentlimitR {
        NfetovercurrentlimitR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - LP Mode is Running
    #[inline(always)]
    pub fn dcdclprunning(&self) -> DcdclprunningR {
        DcdclprunningR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - LN Mode is Running
    #[inline(always)]
    pub fn dcdclnrunning(&self) -> DcdclnrunningR {
        DcdclnrunningR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - DCDC is in Bypass
    #[inline(always)]
    pub fn dcdcinbypass(&self) -> DcdcinbypassR {
        DcdcinbypassR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - Wakeup IRQ From EM2 and EM3
    #[inline(always)]
    pub fn em23wakeup(&self) -> Em23wakeupR {
        Em23wakeupR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 29 - New Temperature Measurement Valid
    #[inline(always)]
    pub fn temp(&self) -> TempR {
        TempR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Temperature Low Limit Reached
    #[inline(always)]
    pub fn templow(&self) -> TemplowR {
        TemplowR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Temperature High Limit Reached
    #[inline(always)]
    pub fn temphigh(&self) -> TemphighR {
        TemphighR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
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
///Interrupt Flag Register
///
///You can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
///`read()` method returns [`if_::R`](R) reader structure
impl crate::Readable for IFrs {}
///`reset()` method sets IF to value 0
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0;
}
