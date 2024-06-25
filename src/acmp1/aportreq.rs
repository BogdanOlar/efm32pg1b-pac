#[doc = "Register `APORTREQ` reader"]
pub type R = crate::R<APORTREQrs>;
#[doc = "Field `APORT0XREQ` reader - 1 If the Bus Connected to APORT0X is Requested"]
pub type Aport0xreqR = crate::BitReader;
#[doc = "Field `APORT0YREQ` reader - 1 If the Bus Connected to APORT0Y is Requested"]
pub type Aport0yreqR = crate::BitReader;
#[doc = "Field `APORT1XREQ` reader - 1 If the Bus Connected to APORT2X is Requested"]
pub type Aport1xreqR = crate::BitReader;
#[doc = "Field `APORT1YREQ` reader - 1 If the Bus Connected to APORT1X is Requested"]
pub type Aport1yreqR = crate::BitReader;
#[doc = "Field `APORT2XREQ` reader - 1 If the Bus Connected to APORT2X is Requested"]
pub type Aport2xreqR = crate::BitReader;
#[doc = "Field `APORT2YREQ` reader - 1 If the Bus Connected to APORT2Y is Requested"]
pub type Aport2yreqR = crate::BitReader;
#[doc = "Field `APORT3XREQ` reader - 1 If the Bus Connected to APORT3X is Requested"]
pub type Aport3xreqR = crate::BitReader;
#[doc = "Field `APORT3YREQ` reader - 1 If the Bus Connected to APORT3Y is Requested"]
pub type Aport3yreqR = crate::BitReader;
#[doc = "Field `APORT4XREQ` reader - 1 If the Bus Connected to APORT4X is Requested"]
pub type Aport4xreqR = crate::BitReader;
#[doc = "Field `APORT4YREQ` reader - 1 If the Bus Connected to APORT4Y is Requested"]
pub type Aport4yreqR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1 If the Bus Connected to APORT0X is Requested"]
    #[inline(always)]
    pub fn aport0xreq(&self) -> Aport0xreqR {
        Aport0xreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 If the Bus Connected to APORT0Y is Requested"]
    #[inline(always)]
    pub fn aport0yreq(&self) -> Aport0yreqR {
        Aport0yreqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1 If the Bus Connected to APORT2X is Requested"]
    #[inline(always)]
    pub fn aport1xreq(&self) -> Aport1xreqR {
        Aport1xreqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1X is Requested"]
    #[inline(always)]
    pub fn aport1yreq(&self) -> Aport1yreqR {
        Aport1yreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1 If the Bus Connected to APORT2X is Requested"]
    #[inline(always)]
    pub fn aport2xreq(&self) -> Aport2xreqR {
        Aport2xreqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1 If the Bus Connected to APORT2Y is Requested"]
    #[inline(always)]
    pub fn aport2yreq(&self) -> Aport2yreqR {
        Aport2yreqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1 If the Bus Connected to APORT3X is Requested"]
    #[inline(always)]
    pub fn aport3xreq(&self) -> Aport3xreqR {
        Aport3xreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1 If the Bus Connected to APORT3Y is Requested"]
    #[inline(always)]
    pub fn aport3yreq(&self) -> Aport3yreqR {
        Aport3yreqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1 If the Bus Connected to APORT4X is Requested"]
    #[inline(always)]
    pub fn aport4xreq(&self) -> Aport4xreqR {
        Aport4xreqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1 If the Bus Connected to APORT4Y is Requested"]
    #[inline(always)]
    pub fn aport4yreq(&self) -> Aport4yreqR {
        Aport4yreqR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APORTREQ")
            .field("aport0xreq", &self.aport0xreq())
            .field("aport0yreq", &self.aport0yreq())
            .field("aport1xreq", &self.aport1xreq())
            .field("aport1yreq", &self.aport1yreq())
            .field("aport2xreq", &self.aport2xreq())
            .field("aport2yreq", &self.aport2yreq())
            .field("aport3xreq", &self.aport3xreq())
            .field("aport3yreq", &self.aport3yreq())
            .field("aport4xreq", &self.aport4xreq())
            .field("aport4yreq", &self.aport4yreq())
            .finish()
    }
}
#[doc = "APORT Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aportreq::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APORTREQrs;
impl crate::RegisterSpec for APORTREQrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aportreq::R`](R) reader structure"]
impl crate::Readable for APORTREQrs {}
#[doc = "`reset()` method sets APORTREQ to value 0"]
impl crate::Resettable for APORTREQrs {
    const RESET_VALUE: u32 = 0;
}
