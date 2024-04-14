#[doc = "Register `PINLOCKN` reader"]
pub type R = crate::R<PINLOCKNrs>;
#[doc = "Register `PINLOCKN` writer"]
pub type W = crate::W<PINLOCKNrs>;
#[doc = "Field `PINLOCKN0` reader - Pin 0 Unlocked Status"]
pub type Pinlockn0R = crate::BitReader;
#[doc = "Field `PINLOCKN0` writer - Pin 0 Unlocked Status"]
pub type Pinlockn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN1` reader - Pin 1 Unlocked Status"]
pub type Pinlockn1R = crate::BitReader;
#[doc = "Field `PINLOCKN1` writer - Pin 1 Unlocked Status"]
pub type Pinlockn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN2` reader - Pin 2 Unlocked Status"]
pub type Pinlockn2R = crate::BitReader;
#[doc = "Field `PINLOCKN2` writer - Pin 2 Unlocked Status"]
pub type Pinlockn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN3` reader - Pin 3 Unlocked Status"]
pub type Pinlockn3R = crate::BitReader;
#[doc = "Field `PINLOCKN3` writer - Pin 3 Unlocked Status"]
pub type Pinlockn3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN4` reader - Pin 4 Unlocked Status"]
pub type Pinlockn4R = crate::BitReader;
#[doc = "Field `PINLOCKN4` writer - Pin 4 Unlocked Status"]
pub type Pinlockn4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN5` reader - Pin 5 Unlocked Status"]
pub type Pinlockn5R = crate::BitReader;
#[doc = "Field `PINLOCKN5` writer - Pin 5 Unlocked Status"]
pub type Pinlockn5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN6` reader - Pin 6 Unlocked Status"]
pub type Pinlockn6R = crate::BitReader;
#[doc = "Field `PINLOCKN6` writer - Pin 6 Unlocked Status"]
pub type Pinlockn6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN7` reader - Pin 7 Unlocked Status"]
pub type Pinlockn7R = crate::BitReader;
#[doc = "Field `PINLOCKN7` writer - Pin 7 Unlocked Status"]
pub type Pinlockn7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN8` reader - Pin 8 Unlocked Status"]
pub type Pinlockn8R = crate::BitReader;
#[doc = "Field `PINLOCKN8` writer - Pin 8 Unlocked Status"]
pub type Pinlockn8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN9` reader - Pin 9 Unlocked Status"]
pub type Pinlockn9R = crate::BitReader;
#[doc = "Field `PINLOCKN9` writer - Pin 9 Unlocked Status"]
pub type Pinlockn9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN10` reader - Pin 10 Unlocked Status"]
pub type Pinlockn10R = crate::BitReader;
#[doc = "Field `PINLOCKN10` writer - Pin 10 Unlocked Status"]
pub type Pinlockn10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN11` reader - Pin 11 Unlocked Status"]
pub type Pinlockn11R = crate::BitReader;
#[doc = "Field `PINLOCKN11` writer - Pin 11 Unlocked Status"]
pub type Pinlockn11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN12` reader - Pin 12 Unlocked Status"]
pub type Pinlockn12R = crate::BitReader;
#[doc = "Field `PINLOCKN12` writer - Pin 12 Unlocked Status"]
pub type Pinlockn12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN13` reader - Pin 13 Unlocked Status"]
pub type Pinlockn13R = crate::BitReader;
#[doc = "Field `PINLOCKN13` writer - Pin 13 Unlocked Status"]
pub type Pinlockn13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN14` reader - Pin 14 Unlocked Status"]
pub type Pinlockn14R = crate::BitReader;
#[doc = "Field `PINLOCKN14` writer - Pin 14 Unlocked Status"]
pub type Pinlockn14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOCKN15` reader - Pin 15 Unlocked Status"]
pub type Pinlockn15R = crate::BitReader;
#[doc = "Field `PINLOCKN15` writer - Pin 15 Unlocked Status"]
pub type Pinlockn15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin 0 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn0(&self) -> Pinlockn0R {
        Pinlockn0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin 1 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn1(&self) -> Pinlockn1R {
        Pinlockn1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin 2 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn2(&self) -> Pinlockn2R {
        Pinlockn2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin 3 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn3(&self) -> Pinlockn3R {
        Pinlockn3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin 4 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn4(&self) -> Pinlockn4R {
        Pinlockn4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin 5 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn5(&self) -> Pinlockn5R {
        Pinlockn5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin 6 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn6(&self) -> Pinlockn6R {
        Pinlockn6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin 7 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn7(&self) -> Pinlockn7R {
        Pinlockn7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin 8 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn8(&self) -> Pinlockn8R {
        Pinlockn8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin 9 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn9(&self) -> Pinlockn9R {
        Pinlockn9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin 10 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn10(&self) -> Pinlockn10R {
        Pinlockn10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin 11 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn11(&self) -> Pinlockn11R {
        Pinlockn11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin 12 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn12(&self) -> Pinlockn12R {
        Pinlockn12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin 13 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn13(&self) -> Pinlockn13R {
        Pinlockn13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin 14 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn14(&self) -> Pinlockn14R {
        Pinlockn14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pin 15 Unlocked Status"]
    #[inline(always)]
    pub fn pinlockn15(&self) -> Pinlockn15R {
        Pinlockn15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin 0 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn0(&mut self) -> Pinlockn0W<PINLOCKNrs> {
        Pinlockn0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin 1 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn1(&mut self) -> Pinlockn1W<PINLOCKNrs> {
        Pinlockn1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin 2 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn2(&mut self) -> Pinlockn2W<PINLOCKNrs> {
        Pinlockn2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin 3 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn3(&mut self) -> Pinlockn3W<PINLOCKNrs> {
        Pinlockn3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin 4 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn4(&mut self) -> Pinlockn4W<PINLOCKNrs> {
        Pinlockn4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin 5 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn5(&mut self) -> Pinlockn5W<PINLOCKNrs> {
        Pinlockn5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin 6 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn6(&mut self) -> Pinlockn6W<PINLOCKNrs> {
        Pinlockn6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin 7 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn7(&mut self) -> Pinlockn7W<PINLOCKNrs> {
        Pinlockn7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pin 8 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn8(&mut self) -> Pinlockn8W<PINLOCKNrs> {
        Pinlockn8W::new(self, 8)
    }
    #[doc = "Bit 9 - Pin 9 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn9(&mut self) -> Pinlockn9W<PINLOCKNrs> {
        Pinlockn9W::new(self, 9)
    }
    #[doc = "Bit 10 - Pin 10 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn10(&mut self) -> Pinlockn10W<PINLOCKNrs> {
        Pinlockn10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pin 11 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn11(&mut self) -> Pinlockn11W<PINLOCKNrs> {
        Pinlockn11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pin 12 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn12(&mut self) -> Pinlockn12W<PINLOCKNrs> {
        Pinlockn12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pin 13 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn13(&mut self) -> Pinlockn13W<PINLOCKNrs> {
        Pinlockn13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pin 14 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn14(&mut self) -> Pinlockn14W<PINLOCKNrs> {
        Pinlockn14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pin 15 Unlocked Status"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn15(&mut self) -> Pinlockn15W<PINLOCKNrs> {
        Pinlockn15W::new(self, 15)
    }
}
#[doc = "Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinlockn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinlockn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINLOCKNrs;
impl crate::RegisterSpec for PINLOCKNrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinlockn::R`](R) reader structure"]
impl crate::Readable for PINLOCKNrs {}
#[doc = "`write(|w| ..)` method takes [`pinlockn::W`](W) writer structure"]
impl crate::Writable for PINLOCKNrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINLOCKN to value 0xffff"]
impl crate::Resettable for PINLOCKNrs {
    const RESET_VALUE: u32 = 0xffff;
}
