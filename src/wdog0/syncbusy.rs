#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSYrs>;
#[doc = "Field `CTRL` reader - CTRL Register Busy"]
pub type CTRL_R = crate::BitReader;
#[doc = "Field `CMD` reader - CMD Register Busy"]
pub type CMD_R = crate::BitReader;
#[doc = "Field `PCH0_PRSCTRL` reader - PCH0_PRSCTRL Register Busy"]
pub type PCH0_PRSCTRL_R = crate::BitReader;
#[doc = "Field `PCH1_PRSCTRL` reader - PCH1_PRSCTRL Register Busy"]
pub type PCH1_PRSCTRL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CTRL Register Busy"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMD Register Busy"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCH0_PRSCTRL Register Busy"]
    #[inline(always)]
    pub fn pch0_prsctrl(&self) -> PCH0_PRSCTRL_R {
        PCH0_PRSCTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCH1_PRSCTRL Register Busy"]
    #[inline(always)]
    pub fn pch1_prsctrl(&self) -> PCH1_PRSCTRL_R {
        PCH1_PRSCTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Synchronization Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCBUSYrs;
impl crate::RegisterSpec for SYNCBUSYrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SYNCBUSYrs {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSYrs {
    const RESET_VALUE: u32 = 0;
}
