///Register `STATUS` reader
pub type R = crate::R<STATUSrs>;
///Field `VMONRDY` reader - VMON Ready
pub type VmonrdyR = crate::BitReader;
///Field `VMONAVDD` reader - VMON AVDD Channel
pub type VmonavddR = crate::BitReader;
///Field `VMONALTAVDD` reader - Alternate VMON AVDD Channel
pub type VmonaltavddR = crate::BitReader;
///Field `VMONDVDD` reader - VMON DVDD Channel
pub type VmondvddR = crate::BitReader;
///Field `VMONIO0` reader - VMON IOVDD0 Channel
pub type Vmonio0R = crate::BitReader;
///Field `VMONFVDD` reader - VMON VDDFLASH Channel
pub type VmonfvddR = crate::BitReader;
///Field `EM4IORET` reader - IO Retention Status
pub type Em4ioretR = crate::BitReader;
impl R {
    ///Bit 0 - VMON Ready
    #[inline(always)]
    pub fn vmonrdy(&self) -> VmonrdyR {
        VmonrdyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VMON AVDD Channel
    #[inline(always)]
    pub fn vmonavdd(&self) -> VmonavddR {
        VmonavddR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Alternate VMON AVDD Channel
    #[inline(always)]
    pub fn vmonaltavdd(&self) -> VmonaltavddR {
        VmonaltavddR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VMON DVDD Channel
    #[inline(always)]
    pub fn vmondvdd(&self) -> VmondvddR {
        VmondvddR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VMON IOVDD0 Channel
    #[inline(always)]
    pub fn vmonio0(&self) -> Vmonio0R {
        Vmonio0R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - VMON VDDFLASH Channel
    #[inline(always)]
    pub fn vmonfvdd(&self) -> VmonfvddR {
        VmonfvddR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 20 - IO Retention Status
    #[inline(always)]
    pub fn em4ioret(&self) -> Em4ioretR {
        Em4ioretR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("vmonrdy", &self.vmonrdy())
            .field("vmonavdd", &self.vmonavdd())
            .field("vmonaltavdd", &self.vmonaltavdd())
            .field("vmondvdd", &self.vmondvdd())
            .field("vmonio0", &self.vmonio0())
            .field("vmonfvdd", &self.vmonfvdd())
            .field("em4ioret", &self.em4ioret())
            .finish()
    }
}
///Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUSrs {}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0;
}
