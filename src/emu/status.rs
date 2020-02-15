#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `VMONRDY`"]
pub type VMONRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONAVDD`"]
pub type VMONAVDD_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONALTAVDD`"]
pub type VMONALTAVDD_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONDVDD`"]
pub type VMONDVDD_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONIO0`"]
pub type VMONIO0_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONFVDD`"]
pub type VMONFVDD_R = crate::R<bool, bool>;
#[doc = "Reader of field `EM4IORET`"]
pub type EM4IORET_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - VMON Ready"]
    #[inline(always)]
    pub fn vmonrdy(&self) -> VMONRDY_R {
        VMONRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VMON AVDD Channel"]
    #[inline(always)]
    pub fn vmonavdd(&self) -> VMONAVDD_R {
        VMONAVDD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Alternate VMON AVDD Channel"]
    #[inline(always)]
    pub fn vmonaltavdd(&self) -> VMONALTAVDD_R {
        VMONALTAVDD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VMON DVDD Channel"]
    #[inline(always)]
    pub fn vmondvdd(&self) -> VMONDVDD_R {
        VMONDVDD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VMON IOVDD0 Channel"]
    #[inline(always)]
    pub fn vmonio0(&self) -> VMONIO0_R {
        VMONIO0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VMON VDDFLASH Channel"]
    #[inline(always)]
    pub fn vmonfvdd(&self) -> VMONFVDD_R {
        VMONFVDD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IO Retention Status"]
    #[inline(always)]
    pub fn em4ioret(&self) -> EM4IORET_R {
        EM4IORET_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
