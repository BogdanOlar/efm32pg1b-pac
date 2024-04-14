#[doc = "Register `MODEL` reader"]
pub type R = crate::R<MODELrs>;
#[doc = "Register `MODEL` writer"]
pub type W = crate::W<MODELrs>;
#[doc = "Field `MODE0` reader - Pin 0 Mode"]
pub type Mode0R = crate::FieldReader;
#[doc = "Field `MODE0` writer - Pin 0 Mode"]
pub type Mode0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MODE1` reader - Pin 1 Mode"]
pub type Mode1R = crate::FieldReader;
#[doc = "Field `MODE1` writer - Pin 1 Mode"]
pub type Mode1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MODE2` reader - Pin 2 Mode"]
pub type Mode2R = crate::FieldReader;
#[doc = "Field `MODE2` writer - Pin 2 Mode"]
pub type Mode2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MODE3` reader - Pin 3 Mode"]
pub type Mode3R = crate::FieldReader;
#[doc = "Field `MODE3` writer - Pin 3 Mode"]
pub type Mode3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MODE4` reader - Pin 4 Mode"]
pub type Mode4R = crate::FieldReader;
#[doc = "Field `MODE4` writer - Pin 4 Mode"]
pub type Mode4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MODE5` reader - Pin 5 Mode"]
pub type Mode5R = crate::FieldReader;
#[doc = "Field `MODE5` writer - Pin 5 Mode"]
pub type Mode5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MODE6` reader - Pin 6 Mode"]
pub type Mode6R = crate::FieldReader;
#[doc = "Field `MODE6` writer - Pin 6 Mode"]
pub type Mode6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MODE7` reader - Pin 7 Mode"]
pub type Mode7R = crate::FieldReader;
#[doc = "Field `MODE7` writer - Pin 7 Mode"]
pub type Mode7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Pin 0 Mode"]
    #[inline(always)]
    pub fn mode0(&self) -> Mode0R {
        Mode0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pin 1 Mode"]
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pin 2 Mode"]
    #[inline(always)]
    pub fn mode2(&self) -> Mode2R {
        Mode2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pin 3 Mode"]
    #[inline(always)]
    pub fn mode3(&self) -> Mode3R {
        Mode3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Pin 4 Mode"]
    #[inline(always)]
    pub fn mode4(&self) -> Mode4R {
        Mode4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Pin 5 Mode"]
    #[inline(always)]
    pub fn mode5(&self) -> Mode5R {
        Mode5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Pin 6 Mode"]
    #[inline(always)]
    pub fn mode6(&self) -> Mode6R {
        Mode6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Pin 7 Mode"]
    #[inline(always)]
    pub fn mode7(&self) -> Mode7R {
        Mode7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin 0 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> Mode0W<MODELrs> {
        Mode0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Pin 1 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> Mode1W<MODELrs> {
        Mode1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Pin 2 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> Mode2W<MODELrs> {
        Mode2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Pin 3 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> Mode3W<MODELrs> {
        Mode3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Pin 4 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode4(&mut self) -> Mode4W<MODELrs> {
        Mode4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Pin 5 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode5(&mut self) -> Mode5W<MODELrs> {
        Mode5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Pin 6 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode6(&mut self) -> Mode6W<MODELrs> {
        Mode6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Pin 7 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode7(&mut self) -> Mode7W<MODELrs> {
        Mode7W::new(self, 28)
    }
}
#[doc = "Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`model::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`model::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODELrs;
impl crate::RegisterSpec for MODELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`model::R`](R) reader structure"]
impl crate::Readable for MODELrs {}
#[doc = "`write(|w| ..)` method takes [`model::W`](W) writer structure"]
impl crate::Writable for MODELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODEL to value 0"]
impl crate::Resettable for MODELrs {
    const RESET_VALUE: u32 = 0;
}
