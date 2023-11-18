#[doc = "Register `APORTCONFLICT` reader"]
pub type R = crate::R<APORTCONFLICT_SPEC>;
#[doc = "Field `APORT0XCONFLICT` reader - 1 If the Bus Connected to APORT0X is in Conflict With Another Peripheral"]
pub type APORT0XCONFLICT_R = crate::BitReader;
#[doc = "Field `APORT0YCONFLICT` reader - 1 If the Bus Connected to APORT0Y is in Conflict With Another Peripheral"]
pub type APORT0YCONFLICT_R = crate::BitReader;
#[doc = "Field `APORT1XCONFLICT` reader - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
pub type APORT1XCONFLICT_R = crate::BitReader;
#[doc = "Field `APORT1YCONFLICT` reader - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
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
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APORTCONFLICT")
            .field(
                "aport0xconflict",
                &format_args!("{}", self.aport0xconflict().bit()),
            )
            .field(
                "aport0yconflict",
                &format_args!("{}", self.aport0yconflict().bit()),
            )
            .field(
                "aport1xconflict",
                &format_args!("{}", self.aport1xconflict().bit()),
            )
            .field(
                "aport1yconflict",
                &format_args!("{}", self.aport1yconflict().bit()),
            )
            .field(
                "aport2xconflict",
                &format_args!("{}", self.aport2xconflict().bit()),
            )
            .field(
                "aport2yconflict",
                &format_args!("{}", self.aport2yconflict().bit()),
            )
            .field(
                "aport3xconflict",
                &format_args!("{}", self.aport3xconflict().bit()),
            )
            .field(
                "aport3yconflict",
                &format_args!("{}", self.aport3yconflict().bit()),
            )
            .field(
                "aport4xconflict",
                &format_args!("{}", self.aport4xconflict().bit()),
            )
            .field(
                "aport4yconflict",
                &format_args!("{}", self.aport4yconflict().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<APORTCONFLICT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "APORT Conflict Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aportconflict::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APORTCONFLICT_SPEC;
impl crate::RegisterSpec for APORTCONFLICT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aportconflict::R`](R) reader structure"]
impl crate::Readable for APORTCONFLICT_SPEC {}
#[doc = "`reset()` method sets APORTCONFLICT to value 0"]
impl crate::Resettable for APORTCONFLICT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
