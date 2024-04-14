#[doc = "Register `DOUTTGL` writer"]
pub type W = crate::W<DOUTTGLrs>;
#[doc = "Field `DOUTTGL0` writer - Pin 0 Data Out Toggle"]
pub type Douttgl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL1` writer - Pin 1 Data Out Toggle"]
pub type Douttgl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL2` writer - Pin 2 Data Out Toggle"]
pub type Douttgl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL3` writer - Pin 3 Data Out Toggle"]
pub type Douttgl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL4` writer - Pin 4 Data Out Toggle"]
pub type Douttgl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL5` writer - Pin 5 Data Out Toggle"]
pub type Douttgl5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL6` writer - Pin 6 Data Out Toggle"]
pub type Douttgl6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL7` writer - Pin 7 Data Out Toggle"]
pub type Douttgl7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL8` writer - Pin 8 Data Out Toggle"]
pub type Douttgl8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL9` writer - Pin 9 Data Out Toggle"]
pub type Douttgl9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL10` writer - Pin 10 Data Out Toggle"]
pub type Douttgl10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL11` writer - Pin 11 Data Out Toggle"]
pub type Douttgl11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL12` writer - Pin 12 Data Out Toggle"]
pub type Douttgl12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL13` writer - Pin 13 Data Out Toggle"]
pub type Douttgl13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL14` writer - Pin 14 Data Out Toggle"]
pub type Douttgl14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTGL15` writer - Pin 15 Data Out Toggle"]
pub type Douttgl15W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Pin 0 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl0(&mut self) -> Douttgl0W<DOUTTGLrs> {
        Douttgl0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin 1 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl1(&mut self) -> Douttgl1W<DOUTTGLrs> {
        Douttgl1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin 2 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl2(&mut self) -> Douttgl2W<DOUTTGLrs> {
        Douttgl2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin 3 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl3(&mut self) -> Douttgl3W<DOUTTGLrs> {
        Douttgl3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin 4 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl4(&mut self) -> Douttgl4W<DOUTTGLrs> {
        Douttgl4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin 5 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl5(&mut self) -> Douttgl5W<DOUTTGLrs> {
        Douttgl5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin 6 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl6(&mut self) -> Douttgl6W<DOUTTGLrs> {
        Douttgl6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin 7 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl7(&mut self) -> Douttgl7W<DOUTTGLrs> {
        Douttgl7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pin 8 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl8(&mut self) -> Douttgl8W<DOUTTGLrs> {
        Douttgl8W::new(self, 8)
    }
    #[doc = "Bit 9 - Pin 9 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl9(&mut self) -> Douttgl9W<DOUTTGLrs> {
        Douttgl9W::new(self, 9)
    }
    #[doc = "Bit 10 - Pin 10 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl10(&mut self) -> Douttgl10W<DOUTTGLrs> {
        Douttgl10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pin 11 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl11(&mut self) -> Douttgl11W<DOUTTGLrs> {
        Douttgl11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pin 12 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl12(&mut self) -> Douttgl12W<DOUTTGLrs> {
        Douttgl12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pin 13 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl13(&mut self) -> Douttgl13W<DOUTTGLrs> {
        Douttgl13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pin 14 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl14(&mut self) -> Douttgl14W<DOUTTGLrs> {
        Douttgl14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pin 15 Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl15(&mut self) -> Douttgl15W<DOUTTGLrs> {
        Douttgl15W::new(self, 15)
    }
}
#[doc = "Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`douttgl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTTGLrs;
impl crate::RegisterSpec for DOUTTGLrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`douttgl::W`](W) writer structure"]
impl crate::Writable for DOUTTGLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTTGL to value 0"]
impl crate::Resettable for DOUTTGLrs {
    const RESET_VALUE: u32 = 0;
}
