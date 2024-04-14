#[doc = "Register `OVT_DIS` reader"]
pub type R = crate::R<OVT_DISrs>;
#[doc = "Register `OVT_DIS` writer"]
pub type W = crate::W<OVT_DISrs>;
#[doc = "Field `OVTDIS0` reader - Pin 0 Disable Over Voltage Capability"]
pub type Ovtdis0R = crate::BitReader;
#[doc = "Field `OVTDIS0` writer - Pin 0 Disable Over Voltage Capability"]
pub type Ovtdis0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS1` reader - Pin 1 Disable Over Voltage Capability"]
pub type Ovtdis1R = crate::BitReader;
#[doc = "Field `OVTDIS1` writer - Pin 1 Disable Over Voltage Capability"]
pub type Ovtdis1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS2` reader - Pin 2 Disable Over Voltage Capability"]
pub type Ovtdis2R = crate::BitReader;
#[doc = "Field `OVTDIS2` writer - Pin 2 Disable Over Voltage Capability"]
pub type Ovtdis2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS3` reader - Pin 3 Disable Over Voltage Capability"]
pub type Ovtdis3R = crate::BitReader;
#[doc = "Field `OVTDIS3` writer - Pin 3 Disable Over Voltage Capability"]
pub type Ovtdis3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS4` reader - Pin 4 Disable Over Voltage Capability"]
pub type Ovtdis4R = crate::BitReader;
#[doc = "Field `OVTDIS4` writer - Pin 4 Disable Over Voltage Capability"]
pub type Ovtdis4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS5` reader - Pin 5 Disable Over Voltage Capability"]
pub type Ovtdis5R = crate::BitReader;
#[doc = "Field `OVTDIS5` writer - Pin 5 Disable Over Voltage Capability"]
pub type Ovtdis5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS6` reader - Pin 6 Disable Over Voltage Capability"]
pub type Ovtdis6R = crate::BitReader;
#[doc = "Field `OVTDIS6` writer - Pin 6 Disable Over Voltage Capability"]
pub type Ovtdis6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS7` reader - Pin 7 Disable Over Voltage Capability"]
pub type Ovtdis7R = crate::BitReader;
#[doc = "Field `OVTDIS7` writer - Pin 7 Disable Over Voltage Capability"]
pub type Ovtdis7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS8` reader - Pin 8 Disable Over Voltage Capability"]
pub type Ovtdis8R = crate::BitReader;
#[doc = "Field `OVTDIS8` writer - Pin 8 Disable Over Voltage Capability"]
pub type Ovtdis8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS9` reader - Pin 9 Disable Over Voltage Capability"]
pub type Ovtdis9R = crate::BitReader;
#[doc = "Field `OVTDIS9` writer - Pin 9 Disable Over Voltage Capability"]
pub type Ovtdis9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS10` reader - Pin 10 Disable Over Voltage Capability"]
pub type Ovtdis10R = crate::BitReader;
#[doc = "Field `OVTDIS10` writer - Pin 10 Disable Over Voltage Capability"]
pub type Ovtdis10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS11` reader - Pin 11 Disable Over Voltage Capability"]
pub type Ovtdis11R = crate::BitReader;
#[doc = "Field `OVTDIS11` writer - Pin 11 Disable Over Voltage Capability"]
pub type Ovtdis11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS12` reader - Pin 12 Disable Over Voltage Capability"]
pub type Ovtdis12R = crate::BitReader;
#[doc = "Field `OVTDIS12` writer - Pin 12 Disable Over Voltage Capability"]
pub type Ovtdis12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS13` reader - Pin 13 Disable Over Voltage Capability"]
pub type Ovtdis13R = crate::BitReader;
#[doc = "Field `OVTDIS13` writer - Pin 13 Disable Over Voltage Capability"]
pub type Ovtdis13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS14` reader - Pin 14 Disable Over Voltage Capability"]
pub type Ovtdis14R = crate::BitReader;
#[doc = "Field `OVTDIS14` writer - Pin 14 Disable Over Voltage Capability"]
pub type Ovtdis14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVTDIS15` reader - Pin 15 Disable Over Voltage Capability"]
pub type Ovtdis15R = crate::BitReader;
#[doc = "Field `OVTDIS15` writer - Pin 15 Disable Over Voltage Capability"]
pub type Ovtdis15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin 0 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis0(&self) -> Ovtdis0R {
        Ovtdis0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin 1 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis1(&self) -> Ovtdis1R {
        Ovtdis1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin 2 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis2(&self) -> Ovtdis2R {
        Ovtdis2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin 3 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis3(&self) -> Ovtdis3R {
        Ovtdis3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin 4 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis4(&self) -> Ovtdis4R {
        Ovtdis4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin 5 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis5(&self) -> Ovtdis5R {
        Ovtdis5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin 6 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis6(&self) -> Ovtdis6R {
        Ovtdis6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin 7 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis7(&self) -> Ovtdis7R {
        Ovtdis7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin 8 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis8(&self) -> Ovtdis8R {
        Ovtdis8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin 9 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis9(&self) -> Ovtdis9R {
        Ovtdis9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin 10 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis10(&self) -> Ovtdis10R {
        Ovtdis10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin 11 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis11(&self) -> Ovtdis11R {
        Ovtdis11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin 12 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis12(&self) -> Ovtdis12R {
        Ovtdis12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin 13 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis13(&self) -> Ovtdis13R {
        Ovtdis13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin 14 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis14(&self) -> Ovtdis14R {
        Ovtdis14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pin 15 Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis15(&self) -> Ovtdis15R {
        Ovtdis15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin 0 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis0(&mut self) -> Ovtdis0W<OVT_DISrs> {
        Ovtdis0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin 1 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis1(&mut self) -> Ovtdis1W<OVT_DISrs> {
        Ovtdis1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin 2 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis2(&mut self) -> Ovtdis2W<OVT_DISrs> {
        Ovtdis2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin 3 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis3(&mut self) -> Ovtdis3W<OVT_DISrs> {
        Ovtdis3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin 4 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis4(&mut self) -> Ovtdis4W<OVT_DISrs> {
        Ovtdis4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin 5 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis5(&mut self) -> Ovtdis5W<OVT_DISrs> {
        Ovtdis5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin 6 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis6(&mut self) -> Ovtdis6W<OVT_DISrs> {
        Ovtdis6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin 7 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis7(&mut self) -> Ovtdis7W<OVT_DISrs> {
        Ovtdis7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pin 8 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis8(&mut self) -> Ovtdis8W<OVT_DISrs> {
        Ovtdis8W::new(self, 8)
    }
    #[doc = "Bit 9 - Pin 9 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis9(&mut self) -> Ovtdis9W<OVT_DISrs> {
        Ovtdis9W::new(self, 9)
    }
    #[doc = "Bit 10 - Pin 10 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis10(&mut self) -> Ovtdis10W<OVT_DISrs> {
        Ovtdis10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pin 11 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis11(&mut self) -> Ovtdis11W<OVT_DISrs> {
        Ovtdis11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pin 12 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis12(&mut self) -> Ovtdis12W<OVT_DISrs> {
        Ovtdis12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pin 13 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis13(&mut self) -> Ovtdis13W<OVT_DISrs> {
        Ovtdis13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pin 14 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis14(&mut self) -> Ovtdis14W<OVT_DISrs> {
        Ovtdis14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pin 15 Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis15(&mut self) -> Ovtdis15W<OVT_DISrs> {
        Ovtdis15W::new(self, 15)
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
