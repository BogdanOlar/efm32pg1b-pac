#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `VMONRDY` reader - VMON Ready"]
pub type VMONRDY_R = crate::BitReader;
#[doc = "Field `VMONAVDD` reader - VMON AVDD Channel"]
pub type VMONAVDD_R = crate::BitReader;
#[doc = "Field `VMONALTAVDD` reader - Alternate VMON AVDD Channel"]
pub type VMONALTAVDD_R = crate::BitReader;
#[doc = "Field `VMONDVDD` reader - VMON DVDD Channel"]
pub type VMONDVDD_R = crate::BitReader;
#[doc = "Field `VMONIO0` reader - VMON IOVDD0 Channel"]
pub type VMONIO0_R = crate::BitReader;
#[doc = "Field `VMONFVDD` reader - VMON VDDFLASH Channel"]
pub type VMONFVDD_R = crate::BitReader;
#[doc = "Field `EM4IORET` reader - IO Retention Status"]
pub type EM4IORET_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - VMON Ready"]
    #[inline(always)]
    pub fn vmonrdy(&self) -> VMONRDY_R {
        VMONRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VMON AVDD Channel"]
    #[inline(always)]
    pub fn vmonavdd(&self) -> VMONAVDD_R {
        VMONAVDD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alternate VMON AVDD Channel"]
    #[inline(always)]
    pub fn vmonaltavdd(&self) -> VMONALTAVDD_R {
        VMONALTAVDD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VMON DVDD Channel"]
    #[inline(always)]
    pub fn vmondvdd(&self) -> VMONDVDD_R {
        VMONDVDD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VMON IOVDD0 Channel"]
    #[inline(always)]
    pub fn vmonio0(&self) -> VMONIO0_R {
        VMONIO0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - VMON VDDFLASH Channel"]
    #[inline(always)]
    pub fn vmonfvdd(&self) -> VMONFVDD_R {
        VMONFVDD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 20 - IO Retention Status"]
    #[inline(always)]
    pub fn em4ioret(&self) -> EM4IORET_R {
        EM4IORET_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("vmonrdy", &format_args!("{}", self.vmonrdy().bit()))
            .field("vmonavdd", &format_args!("{}", self.vmonavdd().bit()))
            .field("vmonaltavdd", &format_args!("{}", self.vmonaltavdd().bit()))
            .field("vmondvdd", &format_args!("{}", self.vmondvdd().bit()))
            .field("vmonio0", &format_args!("{}", self.vmonio0().bit()))
            .field("vmonfvdd", &format_args!("{}", self.vmonfvdd().bit()))
            .field("em4ioret", &format_args!("{}", self.em4ioret().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
