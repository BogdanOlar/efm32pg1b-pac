#[doc = "Register `EXTIFALL` reader"]
pub type R = crate::R<EXTIFALLrs>;
#[doc = "Register `EXTIFALL` writer"]
pub type W = crate::W<EXTIFALLrs>;
#[doc = "Field `EXTIFALL0` reader - Pin 0 Falling Edge"]
pub type Extifall0R = crate::BitReader;
#[doc = "Field `EXTIFALL0` writer - Pin 0 Falling Edge"]
pub type Extifall0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL1` reader - Pin 1 Falling Edge"]
pub type Extifall1R = crate::BitReader;
#[doc = "Field `EXTIFALL1` writer - Pin 1 Falling Edge"]
pub type Extifall1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL2` reader - Pin 2 Falling Edge"]
pub type Extifall2R = crate::BitReader;
#[doc = "Field `EXTIFALL2` writer - Pin 2 Falling Edge"]
pub type Extifall2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL3` reader - Pin 3 Falling Edge"]
pub type Extifall3R = crate::BitReader;
#[doc = "Field `EXTIFALL3` writer - Pin 3 Falling Edge"]
pub type Extifall3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL4` reader - Pin 4 Falling Edge"]
pub type Extifall4R = crate::BitReader;
#[doc = "Field `EXTIFALL4` writer - Pin 4 Falling Edge"]
pub type Extifall4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL5` reader - Pin 5 Falling Edge"]
pub type Extifall5R = crate::BitReader;
#[doc = "Field `EXTIFALL5` writer - Pin 5 Falling Edge"]
pub type Extifall5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL6` reader - Pin 6 Falling Edge"]
pub type Extifall6R = crate::BitReader;
#[doc = "Field `EXTIFALL6` writer - Pin 6 Falling Edge"]
pub type Extifall6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL7` reader - Pin 7 Falling Edge"]
pub type Extifall7R = crate::BitReader;
#[doc = "Field `EXTIFALL7` writer - Pin 7 Falling Edge"]
pub type Extifall7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL8` reader - Pin 8 Falling Edge"]
pub type Extifall8R = crate::BitReader;
#[doc = "Field `EXTIFALL8` writer - Pin 8 Falling Edge"]
pub type Extifall8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL9` reader - Pin 9 Falling Edge"]
pub type Extifall9R = crate::BitReader;
#[doc = "Field `EXTIFALL9` writer - Pin 9 Falling Edge"]
pub type Extifall9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL10` reader - Pin 10 Falling Edge"]
pub type Extifall10R = crate::BitReader;
#[doc = "Field `EXTIFALL10` writer - Pin 10 Falling Edge"]
pub type Extifall10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL11` reader - Pin 11 Falling Edge"]
pub type Extifall11R = crate::BitReader;
#[doc = "Field `EXTIFALL11` writer - Pin 11 Falling Edge"]
pub type Extifall11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL12` reader - Pin 12 Falling Edge"]
pub type Extifall12R = crate::BitReader;
#[doc = "Field `EXTIFALL12` writer - Pin 12 Falling Edge"]
pub type Extifall12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL13` reader - Pin 13 Falling Edge"]
pub type Extifall13R = crate::BitReader;
#[doc = "Field `EXTIFALL13` writer - Pin 13 Falling Edge"]
pub type Extifall13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL14` reader - Pin 14 Falling Edge"]
pub type Extifall14R = crate::BitReader;
#[doc = "Field `EXTIFALL14` writer - Pin 14 Falling Edge"]
pub type Extifall14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIFALL15` reader - Pin 15 Falling Edge"]
pub type Extifall15R = crate::BitReader;
#[doc = "Field `EXTIFALL15` writer - Pin 15 Falling Edge"]
pub type Extifall15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin 0 Falling Edge"]
    #[inline(always)]
    pub fn extifall0(&self) -> Extifall0R {
        Extifall0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin 1 Falling Edge"]
    #[inline(always)]
    pub fn extifall1(&self) -> Extifall1R {
        Extifall1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin 2 Falling Edge"]
    #[inline(always)]
    pub fn extifall2(&self) -> Extifall2R {
        Extifall2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin 3 Falling Edge"]
    #[inline(always)]
    pub fn extifall3(&self) -> Extifall3R {
        Extifall3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin 4 Falling Edge"]
    #[inline(always)]
    pub fn extifall4(&self) -> Extifall4R {
        Extifall4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin 5 Falling Edge"]
    #[inline(always)]
    pub fn extifall5(&self) -> Extifall5R {
        Extifall5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin 6 Falling Edge"]
    #[inline(always)]
    pub fn extifall6(&self) -> Extifall6R {
        Extifall6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin 7 Falling Edge"]
    #[inline(always)]
    pub fn extifall7(&self) -> Extifall7R {
        Extifall7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin 8 Falling Edge"]
    #[inline(always)]
    pub fn extifall8(&self) -> Extifall8R {
        Extifall8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin 9 Falling Edge"]
    #[inline(always)]
    pub fn extifall9(&self) -> Extifall9R {
        Extifall9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin 10 Falling Edge"]
    #[inline(always)]
    pub fn extifall10(&self) -> Extifall10R {
        Extifall10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin 11 Falling Edge"]
    #[inline(always)]
    pub fn extifall11(&self) -> Extifall11R {
        Extifall11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin 12 Falling Edge"]
    #[inline(always)]
    pub fn extifall12(&self) -> Extifall12R {
        Extifall12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin 13 Falling Edge"]
    #[inline(always)]
    pub fn extifall13(&self) -> Extifall13R {
        Extifall13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin 14 Falling Edge"]
    #[inline(always)]
    pub fn extifall14(&self) -> Extifall14R {
        Extifall14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pin 15 Falling Edge"]
    #[inline(always)]
    pub fn extifall15(&self) -> Extifall15R {
        Extifall15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTIFALL")
            .field("extifall0", &self.extifall0())
            .field("extifall1", &self.extifall1())
            .field("extifall2", &self.extifall2())
            .field("extifall3", &self.extifall3())
            .field("extifall4", &self.extifall4())
            .field("extifall5", &self.extifall5())
            .field("extifall6", &self.extifall6())
            .field("extifall7", &self.extifall7())
            .field("extifall8", &self.extifall8())
            .field("extifall9", &self.extifall9())
            .field("extifall10", &self.extifall10())
            .field("extifall11", &self.extifall11())
            .field("extifall12", &self.extifall12())
            .field("extifall13", &self.extifall13())
            .field("extifall14", &self.extifall14())
            .field("extifall15", &self.extifall15())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Pin 0 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall0(&mut self) -> Extifall0W<EXTIFALLrs> {
        Extifall0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin 1 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall1(&mut self) -> Extifall1W<EXTIFALLrs> {
        Extifall1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin 2 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall2(&mut self) -> Extifall2W<EXTIFALLrs> {
        Extifall2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin 3 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall3(&mut self) -> Extifall3W<EXTIFALLrs> {
        Extifall3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin 4 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall4(&mut self) -> Extifall4W<EXTIFALLrs> {
        Extifall4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin 5 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall5(&mut self) -> Extifall5W<EXTIFALLrs> {
        Extifall5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin 6 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall6(&mut self) -> Extifall6W<EXTIFALLrs> {
        Extifall6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin 7 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall7(&mut self) -> Extifall7W<EXTIFALLrs> {
        Extifall7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pin 8 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall8(&mut self) -> Extifall8W<EXTIFALLrs> {
        Extifall8W::new(self, 8)
    }
    #[doc = "Bit 9 - Pin 9 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall9(&mut self) -> Extifall9W<EXTIFALLrs> {
        Extifall9W::new(self, 9)
    }
    #[doc = "Bit 10 - Pin 10 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall10(&mut self) -> Extifall10W<EXTIFALLrs> {
        Extifall10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pin 11 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall11(&mut self) -> Extifall11W<EXTIFALLrs> {
        Extifall11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pin 12 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall12(&mut self) -> Extifall12W<EXTIFALLrs> {
        Extifall12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pin 13 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall13(&mut self) -> Extifall13W<EXTIFALLrs> {
        Extifall13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pin 14 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall14(&mut self) -> Extifall14W<EXTIFALLrs> {
        Extifall14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pin 15 Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn extifall15(&mut self) -> Extifall15W<EXTIFALLrs> {
        Extifall15W::new(self, 15)
    }
}
#[doc = "External Interrupt Falling Edge Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extifall::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extifall::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIFALLrs;
impl crate::RegisterSpec for EXTIFALLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extifall::R`](R) reader structure"]
impl crate::Readable for EXTIFALLrs {}
#[doc = "`write(|w| ..)` method takes [`extifall::W`](W) writer structure"]
impl crate::Writable for EXTIFALLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTIFALL to value 0"]
impl crate::Resettable for EXTIFALLrs {
    const RESET_VALUE: u32 = 0;
}
