#[doc = "Register `DOUT` reader"]
pub type R = crate::R<DOUTrs>;
#[doc = "Register `DOUT` writer"]
pub type W = crate::W<DOUTrs>;
#[doc = "Field `DOUT0` reader - Pin 0 Data Out"]
pub type Dout0R = crate::BitReader;
#[doc = "Field `DOUT0` writer - Pin 0 Data Out"]
pub type Dout0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT1` reader - Pin 1 Data Out"]
pub type Dout1R = crate::BitReader;
#[doc = "Field `DOUT1` writer - Pin 1 Data Out"]
pub type Dout1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT2` reader - Pin 2 Data Out"]
pub type Dout2R = crate::BitReader;
#[doc = "Field `DOUT2` writer - Pin 2 Data Out"]
pub type Dout2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT3` reader - Pin 3 Data Out"]
pub type Dout3R = crate::BitReader;
#[doc = "Field `DOUT3` writer - Pin 3 Data Out"]
pub type Dout3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT4` reader - Pin 4 Data Out"]
pub type Dout4R = crate::BitReader;
#[doc = "Field `DOUT4` writer - Pin 4 Data Out"]
pub type Dout4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT5` reader - Pin 5 Data Out"]
pub type Dout5R = crate::BitReader;
#[doc = "Field `DOUT5` writer - Pin 5 Data Out"]
pub type Dout5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT6` reader - Pin 6 Data Out"]
pub type Dout6R = crate::BitReader;
#[doc = "Field `DOUT6` writer - Pin 6 Data Out"]
pub type Dout6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT7` reader - Pin 7 Data Out"]
pub type Dout7R = crate::BitReader;
#[doc = "Field `DOUT7` writer - Pin 7 Data Out"]
pub type Dout7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT8` reader - Pin 8 Data Out"]
pub type Dout8R = crate::BitReader;
#[doc = "Field `DOUT8` writer - Pin 8 Data Out"]
pub type Dout8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT9` reader - Pin 9 Data Out"]
pub type Dout9R = crate::BitReader;
#[doc = "Field `DOUT9` writer - Pin 9 Data Out"]
pub type Dout9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT10` reader - Pin 10 Data Out"]
pub type Dout10R = crate::BitReader;
#[doc = "Field `DOUT10` writer - Pin 10 Data Out"]
pub type Dout10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT11` reader - Pin 11 Data Out"]
pub type Dout11R = crate::BitReader;
#[doc = "Field `DOUT11` writer - Pin 11 Data Out"]
pub type Dout11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT12` reader - Pin 12 Data Out"]
pub type Dout12R = crate::BitReader;
#[doc = "Field `DOUT12` writer - Pin 12 Data Out"]
pub type Dout12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT13` reader - Pin 13 Data Out"]
pub type Dout13R = crate::BitReader;
#[doc = "Field `DOUT13` writer - Pin 13 Data Out"]
pub type Dout13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT14` reader - Pin 14 Data Out"]
pub type Dout14R = crate::BitReader;
#[doc = "Field `DOUT14` writer - Pin 14 Data Out"]
pub type Dout14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT15` reader - Pin 15 Data Out"]
pub type Dout15R = crate::BitReader;
#[doc = "Field `DOUT15` writer - Pin 15 Data Out"]
pub type Dout15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin 0 Data Out"]
    #[inline(always)]
    pub fn dout0(&self) -> Dout0R {
        Dout0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin 1 Data Out"]
    #[inline(always)]
    pub fn dout1(&self) -> Dout1R {
        Dout1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin 2 Data Out"]
    #[inline(always)]
    pub fn dout2(&self) -> Dout2R {
        Dout2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin 3 Data Out"]
    #[inline(always)]
    pub fn dout3(&self) -> Dout3R {
        Dout3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin 4 Data Out"]
    #[inline(always)]
    pub fn dout4(&self) -> Dout4R {
        Dout4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin 5 Data Out"]
    #[inline(always)]
    pub fn dout5(&self) -> Dout5R {
        Dout5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin 6 Data Out"]
    #[inline(always)]
    pub fn dout6(&self) -> Dout6R {
        Dout6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin 7 Data Out"]
    #[inline(always)]
    pub fn dout7(&self) -> Dout7R {
        Dout7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin 8 Data Out"]
    #[inline(always)]
    pub fn dout8(&self) -> Dout8R {
        Dout8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin 9 Data Out"]
    #[inline(always)]
    pub fn dout9(&self) -> Dout9R {
        Dout9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin 10 Data Out"]
    #[inline(always)]
    pub fn dout10(&self) -> Dout10R {
        Dout10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin 11 Data Out"]
    #[inline(always)]
    pub fn dout11(&self) -> Dout11R {
        Dout11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin 12 Data Out"]
    #[inline(always)]
    pub fn dout12(&self) -> Dout12R {
        Dout12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin 13 Data Out"]
    #[inline(always)]
    pub fn dout13(&self) -> Dout13R {
        Dout13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin 14 Data Out"]
    #[inline(always)]
    pub fn dout14(&self) -> Dout14R {
        Dout14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pin 15 Data Out"]
    #[inline(always)]
    pub fn dout15(&self) -> Dout15R {
        Dout15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin 0 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout0(&mut self) -> Dout0W<DOUTrs> {
        Dout0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin 1 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout1(&mut self) -> Dout1W<DOUTrs> {
        Dout1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin 2 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout2(&mut self) -> Dout2W<DOUTrs> {
        Dout2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin 3 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout3(&mut self) -> Dout3W<DOUTrs> {
        Dout3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin 4 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout4(&mut self) -> Dout4W<DOUTrs> {
        Dout4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin 5 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout5(&mut self) -> Dout5W<DOUTrs> {
        Dout5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin 6 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout6(&mut self) -> Dout6W<DOUTrs> {
        Dout6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin 7 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout7(&mut self) -> Dout7W<DOUTrs> {
        Dout7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pin 8 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout8(&mut self) -> Dout8W<DOUTrs> {
        Dout8W::new(self, 8)
    }
    #[doc = "Bit 9 - Pin 9 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout9(&mut self) -> Dout9W<DOUTrs> {
        Dout9W::new(self, 9)
    }
    #[doc = "Bit 10 - Pin 10 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout10(&mut self) -> Dout10W<DOUTrs> {
        Dout10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pin 11 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout11(&mut self) -> Dout11W<DOUTrs> {
        Dout11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pin 12 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout12(&mut self) -> Dout12W<DOUTrs> {
        Dout12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pin 13 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout13(&mut self) -> Dout13W<DOUTrs> {
        Dout13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pin 14 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout14(&mut self) -> Dout14W<DOUTrs> {
        Dout14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pin 15 Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout15(&mut self) -> Dout15W<DOUTrs> {
        Dout15W::new(self, 15)
    }
}
#[doc = "Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTrs;
impl crate::RegisterSpec for DOUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout::R`](R) reader structure"]
impl crate::Readable for DOUTrs {}
#[doc = "`write(|w| ..)` method takes [`dout::W`](W) writer structure"]
impl crate::Writable for DOUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT to value 0"]
impl crate::Resettable for DOUTrs {
    const RESET_VALUE: u32 = 0;
}
