#[doc = "Register `EXTILEVEL` reader"]
pub type R = crate::R<EXTILEVELrs>;
#[doc = "Register `EXTILEVEL` writer"]
pub type W = crate::W<EXTILEVELrs>;
#[doc = "Field `EM4WU0` reader - EM4 Wake Up Level for EM4WU0 Pin"]
pub type EM4WU0_R = crate::BitReader;
#[doc = "Field `EM4WU0` writer - EM4 Wake Up Level for EM4WU0 Pin"]
pub type EM4WU0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU1` reader - EM4 Wake Up Level for EM4WU1 Pin"]
pub type EM4WU1_R = crate::BitReader;
#[doc = "Field `EM4WU1` writer - EM4 Wake Up Level for EM4WU1 Pin"]
pub type EM4WU1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU4` reader - EM4 Wake Up Level for EM4WU4 Pin"]
pub type EM4WU4_R = crate::BitReader;
#[doc = "Field `EM4WU4` writer - EM4 Wake Up Level for EM4WU4 Pin"]
pub type EM4WU4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU8` reader - EM4 Wake Up Level for EM4WU8 Pin"]
pub type EM4WU8_R = crate::BitReader;
#[doc = "Field `EM4WU8` writer - EM4 Wake Up Level for EM4WU8 Pin"]
pub type EM4WU8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU9` reader - EM4 Wake Up Level for EM4WU9 Pin"]
pub type EM4WU9_R = crate::BitReader;
#[doc = "Field `EM4WU9` writer - EM4 Wake Up Level for EM4WU9 Pin"]
pub type EM4WU9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU12` reader - EM4 Wake Up Level for EM4WU12 Pin"]
pub type EM4WU12_R = crate::BitReader;
#[doc = "Field `EM4WU12` writer - EM4 Wake Up Level for EM4WU12 Pin"]
pub type EM4WU12_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - EM4 Wake Up Level for EM4WU0 Pin"]
    #[inline(always)]
    pub fn em4wu0(&self) -> EM4WU0_R {
        EM4WU0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - EM4 Wake Up Level for EM4WU1 Pin"]
    #[inline(always)]
    pub fn em4wu1(&self) -> EM4WU1_R {
        EM4WU1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - EM4 Wake Up Level for EM4WU4 Pin"]
    #[inline(always)]
    pub fn em4wu4(&self) -> EM4WU4_R {
        EM4WU4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - EM4 Wake Up Level for EM4WU8 Pin"]
    #[inline(always)]
    pub fn em4wu8(&self) -> EM4WU8_R {
        EM4WU8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EM4 Wake Up Level for EM4WU9 Pin"]
    #[inline(always)]
    pub fn em4wu9(&self) -> EM4WU9_R {
        EM4WU9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - EM4 Wake Up Level for EM4WU12 Pin"]
    #[inline(always)]
    pub fn em4wu12(&self) -> EM4WU12_R {
        EM4WU12_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - EM4 Wake Up Level for EM4WU0 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu0(&mut self) -> EM4WU0_W<EXTILEVELrs> {
        EM4WU0_W::new(self, 16)
    }
    #[doc = "Bit 17 - EM4 Wake Up Level for EM4WU1 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu1(&mut self) -> EM4WU1_W<EXTILEVELrs> {
        EM4WU1_W::new(self, 17)
    }
    #[doc = "Bit 20 - EM4 Wake Up Level for EM4WU4 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu4(&mut self) -> EM4WU4_W<EXTILEVELrs> {
        EM4WU4_W::new(self, 20)
    }
    #[doc = "Bit 24 - EM4 Wake Up Level for EM4WU8 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu8(&mut self) -> EM4WU8_W<EXTILEVELrs> {
        EM4WU8_W::new(self, 24)
    }
    #[doc = "Bit 25 - EM4 Wake Up Level for EM4WU9 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu9(&mut self) -> EM4WU9_W<EXTILEVELrs> {
        EM4WU9_W::new(self, 25)
    }
    #[doc = "Bit 28 - EM4 Wake Up Level for EM4WU12 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu12(&mut self) -> EM4WU12_W<EXTILEVELrs> {
        EM4WU12_W::new(self, 28)
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
#[doc = "External Interrupt Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extilevel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extilevel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTILEVELrs;
impl crate::RegisterSpec for EXTILEVELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extilevel::R`](R) reader structure"]
impl crate::Readable for EXTILEVELrs {}
#[doc = "`write(|w| ..)` method takes [`extilevel::W`](W) writer structure"]
impl crate::Writable for EXTILEVELrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTILEVEL to value 0"]
impl crate::Resettable for EXTILEVELrs {
    const RESET_VALUE: u32 = 0;
}
