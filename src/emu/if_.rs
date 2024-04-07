#[doc = "Register `IF` reader"]
pub type R = crate::R<IFrs>;
#[doc = "Field `VMONAVDDFALL` reader - VMON AVDD Channel Fall"]
pub type VmonavddfallR = crate::BitReader;
#[doc = "Field `VMONAVDDRISE` reader - VMON AVDD Channel Rise"]
pub type VmonavddriseR = crate::BitReader;
#[doc = "Field `VMONALTAVDDFALL` reader - Alternate VMON AVDD Channel Fall"]
pub type VmonaltavddfallR = crate::BitReader;
#[doc = "Field `VMONALTAVDDRISE` reader - Alternate VMON AVDD Channel Rise"]
pub type VmonaltavddriseR = crate::BitReader;
#[doc = "Field `VMONDVDDFALL` reader - VMON DVDD Channel Fall"]
pub type VmondvddfallR = crate::BitReader;
#[doc = "Field `VMONDVDDRISE` reader - VMON DVDD Channel Rise"]
pub type VmondvddriseR = crate::BitReader;
#[doc = "Field `VMONIO0FALL` reader - VMON IOVDD0 Channel Fall"]
pub type Vmonio0fallR = crate::BitReader;
#[doc = "Field `VMONIO0RISE` reader - VMON IOVDD0 Channel Rise"]
pub type Vmonio0riseR = crate::BitReader;
#[doc = "Field `VMONFVDDFALL` reader - VMON VDDFLASH Channel Fall"]
pub type VmonfvddfallR = crate::BitReader;
#[doc = "Field `VMONFVDDRISE` reader - VMON VDDFLASH Channel Rise"]
pub type VmonfvddriseR = crate::BitReader;
#[doc = "Field `PFETOVERCURRENTLIMIT` reader - PFET Current Limit Hit"]
pub type PfetovercurrentlimitR = crate::BitReader;
#[doc = "Field `NFETOVERCURRENTLIMIT` reader - NFET Current Limit Hit"]
pub type NfetovercurrentlimitR = crate::BitReader;
#[doc = "Field `DCDCLPRUNNING` reader - LP Mode is Running"]
pub type DcdclprunningR = crate::BitReader;
#[doc = "Field `DCDCLNRUNNING` reader - LN Mode is Running"]
pub type DcdclnrunningR = crate::BitReader;
#[doc = "Field `DCDCINBYPASS` reader - DCDC is in Bypass"]
pub type DcdcinbypassR = crate::BitReader;
#[doc = "Field `EM23WAKEUP` reader - Wakeup IRQ From EM2 and EM3"]
pub type Em23wakeupR = crate::BitReader;
#[doc = "Field `TEMP` reader - New Temperature Measurement Valid"]
pub type TempR = crate::BitReader;
#[doc = "Field `TEMPLOW` reader - Temperature Low Limit Reached"]
pub type TemplowR = crate::BitReader;
#[doc = "Field `TEMPHIGH` reader - Temperature High Limit Reached"]
pub type TemphighR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - VMON AVDD Channel Fall"]
    #[inline(always)]
    pub fn vmonavddfall(&self) -> VmonavddfallR {
        VmonavddfallR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VMON AVDD Channel Rise"]
    #[inline(always)]
    pub fn vmonavddrise(&self) -> VmonavddriseR {
        VmonavddriseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alternate VMON AVDD Channel Fall"]
    #[inline(always)]
    pub fn vmonaltavddfall(&self) -> VmonaltavddfallR {
        VmonaltavddfallR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Alternate VMON AVDD Channel Rise"]
    #[inline(always)]
    pub fn vmonaltavddrise(&self) -> VmonaltavddriseR {
        VmonaltavddriseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VMON DVDD Channel Fall"]
    #[inline(always)]
    pub fn vmondvddfall(&self) -> VmondvddfallR {
        VmondvddfallR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VMON DVDD Channel Rise"]
    #[inline(always)]
    pub fn vmondvddrise(&self) -> VmondvddriseR {
        VmondvddriseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VMON IOVDD0 Channel Fall"]
    #[inline(always)]
    pub fn vmonio0fall(&self) -> Vmonio0fallR {
        Vmonio0fallR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VMON IOVDD0 Channel Rise"]
    #[inline(always)]
    pub fn vmonio0rise(&self) -> Vmonio0riseR {
        Vmonio0riseR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 14 - VMON VDDFLASH Channel Fall"]
    #[inline(always)]
    pub fn vmonfvddfall(&self) -> VmonfvddfallR {
        VmonfvddfallR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VMON VDDFLASH Channel Rise"]
    #[inline(always)]
    pub fn vmonfvddrise(&self) -> VmonfvddriseR {
        VmonfvddriseR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PFET Current Limit Hit"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&self) -> PfetovercurrentlimitR {
        PfetovercurrentlimitR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NFET Current Limit Hit"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&self) -> NfetovercurrentlimitR {
        NfetovercurrentlimitR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LP Mode is Running"]
    #[inline(always)]
    pub fn dcdclprunning(&self) -> DcdclprunningR {
        DcdclprunningR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LN Mode is Running"]
    #[inline(always)]
    pub fn dcdclnrunning(&self) -> DcdclnrunningR {
        DcdclnrunningR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DCDC is in Bypass"]
    #[inline(always)]
    pub fn dcdcinbypass(&self) -> DcdcinbypassR {
        DcdcinbypassR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Wakeup IRQ From EM2 and EM3"]
    #[inline(always)]
    pub fn em23wakeup(&self) -> Em23wakeupR {
        Em23wakeupR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - New Temperature Measurement Valid"]
    #[inline(always)]
    pub fn temp(&self) -> TempR {
        TempR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Temperature Low Limit Reached"]
    #[inline(always)]
    pub fn templow(&self) -> TemplowR {
        TemplowR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Temperature High Limit Reached"]
    #[inline(always)]
    pub fn temphigh(&self) -> TemphighR {
        TemphighR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IFrs {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0;
}
