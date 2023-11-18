#[doc = "Register `APORTMASTERDIS` reader"]
pub type R = crate::R<APORTMASTERDIS_SPEC>;
#[doc = "Register `APORTMASTERDIS` writer"]
pub type W = crate::W<APORTMASTERDIS_SPEC>;
#[doc = "Field `APORT1XMASTERDIS` reader - APORT1X Master Disable"]
pub type APORT1XMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT1XMASTERDIS` writer - APORT1X Master Disable"]
pub type APORT1XMASTERDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APORT1YMASTERDIS` reader - APORT1Y Master Disable"]
pub type APORT1YMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT1YMASTERDIS` writer - APORT1Y Master Disable"]
pub type APORT1YMASTERDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APORT2XMASTERDIS` reader - APORT2X Master Disable"]
pub type APORT2XMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT2XMASTERDIS` writer - APORT2X Master Disable"]
pub type APORT2XMASTERDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APORT2YMASTERDIS` reader - APORT2Y Master Disable"]
pub type APORT2YMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT2YMASTERDIS` writer - APORT2Y Master Disable"]
pub type APORT2YMASTERDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APORT3XMASTERDIS` reader - APORT3X Master Disable"]
pub type APORT3XMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT3XMASTERDIS` writer - APORT3X Master Disable"]
pub type APORT3XMASTERDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APORT3YMASTERDIS` reader - APORT3Y Master Disable"]
pub type APORT3YMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT3YMASTERDIS` writer - APORT3Y Master Disable"]
pub type APORT3YMASTERDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APORT4XMASTERDIS` reader - APORT4X Master Disable"]
pub type APORT4XMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT4XMASTERDIS` writer - APORT4X Master Disable"]
pub type APORT4XMASTERDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APORT4YMASTERDIS` reader - APORT4Y Master Disable"]
pub type APORT4YMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT4YMASTERDIS` writer - APORT4Y Master Disable"]
pub type APORT4YMASTERDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - APORT1X Master Disable"]
    #[inline(always)]
    pub fn aport1xmasterdis(&self) -> APORT1XMASTERDIS_R {
        APORT1XMASTERDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - APORT1Y Master Disable"]
    #[inline(always)]
    pub fn aport1ymasterdis(&self) -> APORT1YMASTERDIS_R {
        APORT1YMASTERDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - APORT2X Master Disable"]
    #[inline(always)]
    pub fn aport2xmasterdis(&self) -> APORT2XMASTERDIS_R {
        APORT2XMASTERDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - APORT2Y Master Disable"]
    #[inline(always)]
    pub fn aport2ymasterdis(&self) -> APORT2YMASTERDIS_R {
        APORT2YMASTERDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - APORT3X Master Disable"]
    #[inline(always)]
    pub fn aport3xmasterdis(&self) -> APORT3XMASTERDIS_R {
        APORT3XMASTERDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - APORT3Y Master Disable"]
    #[inline(always)]
    pub fn aport3ymasterdis(&self) -> APORT3YMASTERDIS_R {
        APORT3YMASTERDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - APORT4X Master Disable"]
    #[inline(always)]
    pub fn aport4xmasterdis(&self) -> APORT4XMASTERDIS_R {
        APORT4XMASTERDIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - APORT4Y Master Disable"]
    #[inline(always)]
    pub fn aport4ymasterdis(&self) -> APORT4YMASTERDIS_R {
        APORT4YMASTERDIS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APORTMASTERDIS")
            .field(
                "aport1xmasterdis",
                &format_args!("{}", self.aport1xmasterdis().bit()),
            )
            .field(
                "aport1ymasterdis",
                &format_args!("{}", self.aport1ymasterdis().bit()),
            )
            .field(
                "aport2xmasterdis",
                &format_args!("{}", self.aport2xmasterdis().bit()),
            )
            .field(
                "aport2ymasterdis",
                &format_args!("{}", self.aport2ymasterdis().bit()),
            )
            .field(
                "aport3xmasterdis",
                &format_args!("{}", self.aport3xmasterdis().bit()),
            )
            .field(
                "aport3ymasterdis",
                &format_args!("{}", self.aport3ymasterdis().bit()),
            )
            .field(
                "aport4xmasterdis",
                &format_args!("{}", self.aport4xmasterdis().bit()),
            )
            .field(
                "aport4ymasterdis",
                &format_args!("{}", self.aport4ymasterdis().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<APORTMASTERDIS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2 - APORT1X Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport1xmasterdis(&mut self) -> APORT1XMASTERDIS_W<APORTMASTERDIS_SPEC, 2> {
        APORT1XMASTERDIS_W::new(self)
    }
    #[doc = "Bit 3 - APORT1Y Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport1ymasterdis(&mut self) -> APORT1YMASTERDIS_W<APORTMASTERDIS_SPEC, 3> {
        APORT1YMASTERDIS_W::new(self)
    }
    #[doc = "Bit 4 - APORT2X Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport2xmasterdis(&mut self) -> APORT2XMASTERDIS_W<APORTMASTERDIS_SPEC, 4> {
        APORT2XMASTERDIS_W::new(self)
    }
    #[doc = "Bit 5 - APORT2Y Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport2ymasterdis(&mut self) -> APORT2YMASTERDIS_W<APORTMASTERDIS_SPEC, 5> {
        APORT2YMASTERDIS_W::new(self)
    }
    #[doc = "Bit 6 - APORT3X Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport3xmasterdis(&mut self) -> APORT3XMASTERDIS_W<APORTMASTERDIS_SPEC, 6> {
        APORT3XMASTERDIS_W::new(self)
    }
    #[doc = "Bit 7 - APORT3Y Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport3ymasterdis(&mut self) -> APORT3YMASTERDIS_W<APORTMASTERDIS_SPEC, 7> {
        APORT3YMASTERDIS_W::new(self)
    }
    #[doc = "Bit 8 - APORT4X Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport4xmasterdis(&mut self) -> APORT4XMASTERDIS_W<APORTMASTERDIS_SPEC, 8> {
        APORT4XMASTERDIS_W::new(self)
    }
    #[doc = "Bit 9 - APORT4Y Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport4ymasterdis(&mut self) -> APORT4YMASTERDIS_W<APORTMASTERDIS_SPEC, 9> {
        APORT4YMASTERDIS_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APORT Bus Master Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aportmasterdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aportmasterdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APORTMASTERDIS_SPEC;
impl crate::RegisterSpec for APORTMASTERDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aportmasterdis::R`](R) reader structure"]
impl crate::Readable for APORTMASTERDIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aportmasterdis::W`](W) writer structure"]
impl crate::Writable for APORTMASTERDIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APORTMASTERDIS to value 0"]
impl crate::Resettable for APORTMASTERDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
