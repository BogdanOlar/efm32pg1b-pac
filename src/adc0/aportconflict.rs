#[doc = "Register `APORTCONFLICT` reader"]
pub type R = crate::R<APORTCONFLICTrs>;
#[doc = "Field `APORT0XCONFLICT` reader - 1 If the Bus Connected to APORT0X is in Conflict With Another Peripheral"]
pub type APORT0XCONFLICT_R = crate::BitReader;
#[doc = "Field `APORT0YCONFLICT` reader - 1 If the Bus Connected to APORT0Y is in Conflict With Another Peripheral"]
pub type APORT0YCONFLICT_R = crate::BitReader;
#[doc = "Field `APORT1XCONFLICT` reader - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
pub type APORT1XCONFLICT_R = crate::BitReader;
#[doc = "Field `APORT1YCONFLICT` reader - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral"]
pub type APORT1YCONFLICT_R = crate::BitReader;
#[doc = "Field `APORT2XCONFLICT` reader - 1 If the Bus Connected to APORT2X is in Conflict With Another Peripheral"]
pub type APORT2XCONFLICT_R = crate::BitReader;
#[doc = "Field `APORT2YCONFLICT` reader - 1 If the Bus Connected to APORT2Y is in Conflict With Another Peripheral"]
pub type APORT2YCONFLICT_R = crate::BitReader;
#[doc = "Field `APORT3XCONFLICT` reader - 1 If the Bus Connected to APORT3X is in Conflict With Another Peripheral"]
pub type APORT3XCONFLICT_R = crate::BitReader;
#[doc = "Field `APORT3YCONFLICT` reader - 1 If the Bus Connected to APORT3Y is in Conflict With Another Peripheral"]
pub type APORT3YCONFLICT_R = crate::BitReader;
#[doc = "Field `APORT4XCONFLICT` reader - 1 If the Bus Connected to APORT4X is in Conflict With Another Peripheral"]
pub type APORT4XCONFLICT_R = crate::BitReader;
#[doc = "Field `APORT4YCONFLICT` reader - 1 If the Bus Connected to APORT4Y is in Conflict With Another Peripheral"]
pub type APORT4YCONFLICT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1 If the Bus Connected to APORT0X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport0xconflict(&self) -> APORT0XCONFLICT_R {
        APORT0XCONFLICT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 If the Bus Connected to APORT0Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport0yconflict(&self) -> APORT0YCONFLICT_R {
        APORT0YCONFLICT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1xconflict(&self) -> APORT1XCONFLICT_R {
        APORT1XCONFLICT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1yconflict(&self) -> APORT1YCONFLICT_R {
        APORT1YCONFLICT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1 If the Bus Connected to APORT2X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport2xconflict(&self) -> APORT2XCONFLICT_R {
        APORT2XCONFLICT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1 If the Bus Connected to APORT2Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport2yconflict(&self) -> APORT2YCONFLICT_R {
        APORT2YCONFLICT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1 If the Bus Connected to APORT3X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport3xconflict(&self) -> APORT3XCONFLICT_R {
        APORT3XCONFLICT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1 If the Bus Connected to APORT3Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport3yconflict(&self) -> APORT3YCONFLICT_R {
        APORT3YCONFLICT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1 If the Bus Connected to APORT4X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport4xconflict(&self) -> APORT4XCONFLICT_R {
        APORT4XCONFLICT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1 If the Bus Connected to APORT4Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport4yconflict(&self) -> APORT4YCONFLICT_R {
        APORT4YCONFLICT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "APORT Conflict Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aportconflict::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APORTCONFLICTrs;
impl crate::RegisterSpec for APORTCONFLICTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aportconflict::R`](R) reader structure"]
impl crate::Readable for APORTCONFLICTrs {}
#[doc = "`reset()` method sets APORTCONFLICT to value 0"]
impl crate::Resettable for APORTCONFLICTrs {
    const RESET_VALUE: u32 = 0;
}
