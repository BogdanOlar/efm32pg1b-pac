#[doc = "Register `OVT_DIS` reader"]
pub type R = crate::R<OVT_DISrs>;
#[doc = "Register `OVT_DIS` writer"]
pub type W = crate::W<OVT_DISrs>;
#[doc = "Field `OVT_DIS0` reader - Pin 0 Disable Over Voltage Capability"]
pub type OvtDis0R = crate::BitReader;
#[doc = "Field `OVT_DIS0` writer - Pin 0 Disable Over Voltage Capability"]
pub type OvtDis0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS1` reader - Pin 1 Disable Over Voltage Capability"]
pub type OvtDis1R = crate::BitReader;
#[doc = "Field `OVT_DIS1` writer - Pin 1 Disable Over Voltage Capability"]
pub type OvtDis1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS2` reader - Pin 2 Disable Over Voltage Capability"]
pub type OvtDis2R = crate::BitReader;
#[doc = "Field `OVT_DIS2` writer - Pin 2 Disable Over Voltage Capability"]
pub type OvtDis2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS3` reader - Pin 3 Disable Over Voltage Capability"]
pub type OvtDis3R = crate::BitReader;
#[doc = "Field `OVT_DIS3` writer - Pin 3 Disable Over Voltage Capability"]
pub type OvtDis3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS4` reader - Pin 4 Disable Over Voltage Capability"]
pub type OvtDis4R = crate::BitReader;
#[doc = "Field `OVT_DIS4` writer - Pin 4 Disable Over Voltage Capability"]
pub type OvtDis4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS5` reader - Pin 5 Disable Over Voltage Capability"]
pub type OvtDis5R = crate::BitReader;
#[doc = "Field `OVT_DIS5` writer - Pin 5 Disable Over Voltage Capability"]
pub type OvtDis5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS6` reader - Pin 6 Disable Over Voltage Capability"]
pub type OvtDis6R = crate::BitReader;
#[doc = "Field `OVT_DIS6` writer - Pin 6 Disable Over Voltage Capability"]
pub type OvtDis6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS7` reader - Pin 7 Disable Over Voltage Capability"]
pub type OvtDis7R = crate::BitReader;
#[doc = "Field `OVT_DIS7` writer - Pin 7 Disable Over Voltage Capability"]
pub type OvtDis7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS8` reader - Pin 8 Disable Over Voltage Capability"]
pub type OvtDis8R = crate::BitReader;
#[doc = "Field `OVT_DIS8` writer - Pin 8 Disable Over Voltage Capability"]
pub type OvtDis8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS9` reader - Pin 9 Disable Over Voltage Capability"]
pub type OvtDis9R = crate::BitReader;
#[doc = "Field `OVT_DIS9` writer - Pin 9 Disable Over Voltage Capability"]
pub type OvtDis9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS10` reader - Pin 10 Disable Over Voltage Capability"]
pub type OvtDis10R = crate::BitReader;
#[doc = "Field `OVT_DIS10` writer - Pin 10 Disable Over Voltage Capability"]
pub type OvtDis10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS11` reader - Pin 11 Disable Over Voltage Capability"]
pub type OvtDis11R = crate::BitReader;
#[doc = "Field `OVT_DIS11` writer - Pin 11 Disable Over Voltage Capability"]
pub type OvtDis11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS12` reader - Pin 12 Disable Over Voltage Capability"]
pub type OvtDis12R = crate::BitReader;
#[doc = "Field `OVT_DIS12` writer - Pin 12 Disable Over Voltage Capability"]
pub type OvtDis12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS13` reader - Pin 13 Disable Over Voltage Capability"]
pub type OvtDis13R = crate::BitReader;
#[doc = "Field `OVT_DIS13` writer - Pin 13 Disable Over Voltage Capability"]
pub type OvtDis13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS14` reader - Pin 14 Disable Over Voltage Capability"]
pub type OvtDis14R = crate::BitReader;
#[doc = "Field `OVT_DIS14` writer - Pin 14 Disable Over Voltage Capability"]
pub type OvtDis14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVT_DIS15` reader - Pin 15 Disable Over Voltage Capability"]
pub type OvtDis15R = crate::BitReader;
#[doc = "Field `OVT_DIS15` writer - Pin 15 Disable Over Voltage Capability"]
pub type OvtDis15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin 0 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis0(&self) -> OvtDis0R {
        OvtDis0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin 1 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis1(&self) -> OvtDis1R {
        OvtDis1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin 2 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis2(&self) -> OvtDis2R {
        OvtDis2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin 3 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis3(&self) -> OvtDis3R {
        OvtDis3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin 4 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis4(&self) -> OvtDis4R {
        OvtDis4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin 5 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis5(&self) -> OvtDis5R {
        OvtDis5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin 6 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis6(&self) -> OvtDis6R {
        OvtDis6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin 7 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis7(&self) -> OvtDis7R {
        OvtDis7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin 8 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis8(&self) -> OvtDis8R {
        OvtDis8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin 9 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis9(&self) -> OvtDis9R {
        OvtDis9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin 10 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis10(&self) -> OvtDis10R {
        OvtDis10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin 11 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis11(&self) -> OvtDis11R {
        OvtDis11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin 12 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis12(&self) -> OvtDis12R {
        OvtDis12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin 13 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis13(&self) -> OvtDis13R {
        OvtDis13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin 14 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis14(&self) -> OvtDis14R {
        OvtDis14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pin 15 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovt_dis15(&self) -> OvtDis15R {
        OvtDis15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin 0 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis0(&mut self) -> OvtDis0W<OVT_DISrs> {
        OvtDis0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin 1 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis1(&mut self) -> OvtDis1W<OVT_DISrs> {
        OvtDis1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin 2 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis2(&mut self) -> OvtDis2W<OVT_DISrs> {
        OvtDis2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin 3 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis3(&mut self) -> OvtDis3W<OVT_DISrs> {
        OvtDis3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin 4 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis4(&mut self) -> OvtDis4W<OVT_DISrs> {
        OvtDis4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin 5 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis5(&mut self) -> OvtDis5W<OVT_DISrs> {
        OvtDis5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin 6 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis6(&mut self) -> OvtDis6W<OVT_DISrs> {
        OvtDis6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin 7 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis7(&mut self) -> OvtDis7W<OVT_DISrs> {
        OvtDis7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pin 8 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis8(&mut self) -> OvtDis8W<OVT_DISrs> {
        OvtDis8W::new(self, 8)
    }
    #[doc = "Bit 9 - Pin 9 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis9(&mut self) -> OvtDis9W<OVT_DISrs> {
        OvtDis9W::new(self, 9)
    }
    #[doc = "Bit 10 - Pin 10 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis10(&mut self) -> OvtDis10W<OVT_DISrs> {
        OvtDis10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pin 11 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis11(&mut self) -> OvtDis11W<OVT_DISrs> {
        OvtDis11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pin 12 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis12(&mut self) -> OvtDis12W<OVT_DISrs> {
        OvtDis12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pin 13 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis13(&mut self) -> OvtDis13W<OVT_DISrs> {
        OvtDis13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pin 14 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis14(&mut self) -> OvtDis14W<OVT_DISrs> {
        OvtDis14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pin 15 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovt_dis15(&mut self) -> OvtDis15W<OVT_DISrs> {
        OvtDis15W::new(self, 15)
    }
}
#[doc = "Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ovt_dis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ovt_dis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OVT_DISrs;
impl crate::RegisterSpec for OVT_DISrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovt_dis::R`](R) reader structure"]
impl crate::Readable for OVT_DISrs {}
#[doc = "`write(|w| ..)` method takes [`ovt_dis::W`](W) writer structure"]
impl crate::Writable for OVT_DISrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OVT_DIS to value 0"]
impl crate::Resettable for OVT_DISrs {
    const RESET_VALUE: u32 = 0;
}
