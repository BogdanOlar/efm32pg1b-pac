#[doc = "Register `EXTIRISE` reader"]
pub type R = crate::R<EXTIRISErs>;
#[doc = "Register `EXTIRISE` writer"]
pub type W = crate::W<EXTIRISErs>;
#[doc = "Field `EXTIRISE0` reader - Pin 0 Rising Edge Enable"]
pub type Extirise0R = crate::BitReader;
#[doc = "Field `EXTIRISE0` writer - Pin 0 Rising Edge Enable"]
pub type Extirise0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE1` reader - Pin 1 Rising Edge Enable"]
pub type Extirise1R = crate::BitReader;
#[doc = "Field `EXTIRISE1` writer - Pin 1 Rising Edge Enable"]
pub type Extirise1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE2` reader - Pin 2 Rising Edge Enable"]
pub type Extirise2R = crate::BitReader;
#[doc = "Field `EXTIRISE2` writer - Pin 2 Rising Edge Enable"]
pub type Extirise2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE3` reader - Pin 3 Rising Edge Enable"]
pub type Extirise3R = crate::BitReader;
#[doc = "Field `EXTIRISE3` writer - Pin 3 Rising Edge Enable"]
pub type Extirise3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE4` reader - Pin 4 Rising Edge Enable"]
pub type Extirise4R = crate::BitReader;
#[doc = "Field `EXTIRISE4` writer - Pin 4 Rising Edge Enable"]
pub type Extirise4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE5` reader - Pin 5 Rising Edge Enable"]
pub type Extirise5R = crate::BitReader;
#[doc = "Field `EXTIRISE5` writer - Pin 5 Rising Edge Enable"]
pub type Extirise5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE6` reader - Pin 6 Rising Edge Enable"]
pub type Extirise6R = crate::BitReader;
#[doc = "Field `EXTIRISE6` writer - Pin 6 Rising Edge Enable"]
pub type Extirise6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE7` reader - Pin 7 Rising Edge Enable"]
pub type Extirise7R = crate::BitReader;
#[doc = "Field `EXTIRISE7` writer - Pin 7 Rising Edge Enable"]
pub type Extirise7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE8` reader - Pin 8 Rising Edge Enable"]
pub type Extirise8R = crate::BitReader;
#[doc = "Field `EXTIRISE8` writer - Pin 8 Rising Edge Enable"]
pub type Extirise8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE9` reader - Pin 9 Rising Edge Enable"]
pub type Extirise9R = crate::BitReader;
#[doc = "Field `EXTIRISE9` writer - Pin 9 Rising Edge Enable"]
pub type Extirise9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE10` reader - Pin 10 Rising Edge Enable"]
pub type Extirise10R = crate::BitReader;
#[doc = "Field `EXTIRISE10` writer - Pin 10 Rising Edge Enable"]
pub type Extirise10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE11` reader - Pin 11 Rising Edge Enable"]
pub type Extirise11R = crate::BitReader;
#[doc = "Field `EXTIRISE11` writer - Pin 11 Rising Edge Enable"]
pub type Extirise11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE12` reader - Pin 12 Rising Edge Enable"]
pub type Extirise12R = crate::BitReader;
#[doc = "Field `EXTIRISE12` writer - Pin 12 Rising Edge Enable"]
pub type Extirise12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE13` reader - Pin 13 Rising Edge Enable"]
pub type Extirise13R = crate::BitReader;
#[doc = "Field `EXTIRISE13` writer - Pin 13 Rising Edge Enable"]
pub type Extirise13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE14` reader - Pin 14 Rising Edge Enable"]
pub type Extirise14R = crate::BitReader;
#[doc = "Field `EXTIRISE14` writer - Pin 14 Rising Edge Enable"]
pub type Extirise14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRISE15` reader - Pin 15 Rising Edge Enable"]
pub type Extirise15R = crate::BitReader;
#[doc = "Field `EXTIRISE15` writer - Pin 15 Rising Edge Enable"]
pub type Extirise15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin 0 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise0(&self) -> Extirise0R {
        Extirise0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin 1 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise1(&self) -> Extirise1R {
        Extirise1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin 2 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise2(&self) -> Extirise2R {
        Extirise2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin 3 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise3(&self) -> Extirise3R {
        Extirise3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin 4 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise4(&self) -> Extirise4R {
        Extirise4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin 5 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise5(&self) -> Extirise5R {
        Extirise5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin 6 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise6(&self) -> Extirise6R {
        Extirise6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin 7 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise7(&self) -> Extirise7R {
        Extirise7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin 8 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise8(&self) -> Extirise8R {
        Extirise8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin 9 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise9(&self) -> Extirise9R {
        Extirise9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin 10 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise10(&self) -> Extirise10R {
        Extirise10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin 11 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise11(&self) -> Extirise11R {
        Extirise11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin 12 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise12(&self) -> Extirise12R {
        Extirise12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin 13 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise13(&self) -> Extirise13R {
        Extirise13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin 14 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise14(&self) -> Extirise14R {
        Extirise14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pin 15 Rising Edge Enable"]
    #[inline(always)]
    pub fn extirise15(&self) -> Extirise15R {
        Extirise15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTIRISE")
            .field("extirise0", &self.extirise0())
            .field("extirise1", &self.extirise1())
            .field("extirise2", &self.extirise2())
            .field("extirise3", &self.extirise3())
            .field("extirise4", &self.extirise4())
            .field("extirise5", &self.extirise5())
            .field("extirise6", &self.extirise6())
            .field("extirise7", &self.extirise7())
            .field("extirise8", &self.extirise8())
            .field("extirise9", &self.extirise9())
            .field("extirise10", &self.extirise10())
            .field("extirise11", &self.extirise11())
            .field("extirise12", &self.extirise12())
            .field("extirise13", &self.extirise13())
            .field("extirise14", &self.extirise14())
            .field("extirise15", &self.extirise15())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Pin 0 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise0(&mut self) -> Extirise0W<EXTIRISErs> {
        Extirise0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin 1 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise1(&mut self) -> Extirise1W<EXTIRISErs> {
        Extirise1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin 2 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise2(&mut self) -> Extirise2W<EXTIRISErs> {
        Extirise2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin 3 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise3(&mut self) -> Extirise3W<EXTIRISErs> {
        Extirise3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin 4 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise4(&mut self) -> Extirise4W<EXTIRISErs> {
        Extirise4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin 5 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise5(&mut self) -> Extirise5W<EXTIRISErs> {
        Extirise5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin 6 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise6(&mut self) -> Extirise6W<EXTIRISErs> {
        Extirise6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin 7 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise7(&mut self) -> Extirise7W<EXTIRISErs> {
        Extirise7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pin 8 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise8(&mut self) -> Extirise8W<EXTIRISErs> {
        Extirise8W::new(self, 8)
    }
    #[doc = "Bit 9 - Pin 9 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise9(&mut self) -> Extirise9W<EXTIRISErs> {
        Extirise9W::new(self, 9)
    }
    #[doc = "Bit 10 - Pin 10 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise10(&mut self) -> Extirise10W<EXTIRISErs> {
        Extirise10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pin 11 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise11(&mut self) -> Extirise11W<EXTIRISErs> {
        Extirise11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pin 12 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise12(&mut self) -> Extirise12W<EXTIRISErs> {
        Extirise12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pin 13 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise13(&mut self) -> Extirise13W<EXTIRISErs> {
        Extirise13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pin 14 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise14(&mut self) -> Extirise14W<EXTIRISErs> {
        Extirise14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pin 15 Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise15(&mut self) -> Extirise15W<EXTIRISErs> {
        Extirise15W::new(self, 15)
    }
}
#[doc = "External Interrupt Rising Edge Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extirise::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extirise::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIRISErs;
impl crate::RegisterSpec for EXTIRISErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extirise::R`](R) reader structure"]
impl crate::Readable for EXTIRISErs {}
#[doc = "`write(|w| ..)` method takes [`extirise::W`](W) writer structure"]
impl crate::Writable for EXTIRISErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTIRISE to value 0"]
impl crate::Resettable for EXTIRISErs {
    const RESET_VALUE: u32 = 0;
}
