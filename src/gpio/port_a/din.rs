#[doc = "Register `DIN` reader"]
pub type R = crate::R<DINrs>;
#[doc = "Field `DIN0` reader - Pin 0 Data In"]
pub type Din0R = crate::BitReader;
#[doc = "Field `DIN1` reader - Pin 1 Data In"]
pub type Din1R = crate::BitReader;
#[doc = "Field `DIN2` reader - Pin 2 Data In"]
pub type Din2R = crate::BitReader;
#[doc = "Field `DIN3` reader - Pin 3 Data In"]
pub type Din3R = crate::BitReader;
#[doc = "Field `DIN4` reader - Pin 4 Data In"]
pub type Din4R = crate::BitReader;
#[doc = "Field `DIN5` reader - Pin 5 Data In"]
pub type Din5R = crate::BitReader;
#[doc = "Field `DIN6` reader - Pin 6 Data In"]
pub type Din6R = crate::BitReader;
#[doc = "Field `DIN7` reader - Pin 7 Data In"]
pub type Din7R = crate::BitReader;
#[doc = "Field `DIN8` reader - Pin 8 Data In"]
pub type Din8R = crate::BitReader;
#[doc = "Field `DIN9` reader - Pin 9 Data In"]
pub type Din9R = crate::BitReader;
#[doc = "Field `DIN10` reader - Pin 10 Data In"]
pub type Din10R = crate::BitReader;
#[doc = "Field `DIN11` reader - Pin 11 Data In"]
pub type Din11R = crate::BitReader;
#[doc = "Field `DIN12` reader - Pin 12 Data In"]
pub type Din12R = crate::BitReader;
#[doc = "Field `DIN13` reader - Pin 13 Data In"]
pub type Din13R = crate::BitReader;
#[doc = "Field `DIN14` reader - Pin 14 Data In"]
pub type Din14R = crate::BitReader;
#[doc = "Field `DIN15` reader - Pin 15 Data In"]
pub type Din15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Pin 0 Data In"]
    #[inline(always)]
    pub fn din0(&self) -> Din0R {
        Din0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin 1 Data In"]
    #[inline(always)]
    pub fn din1(&self) -> Din1R {
        Din1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin 2 Data In"]
    #[inline(always)]
    pub fn din2(&self) -> Din2R {
        Din2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin 3 Data In"]
    #[inline(always)]
    pub fn din3(&self) -> Din3R {
        Din3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin 4 Data In"]
    #[inline(always)]
    pub fn din4(&self) -> Din4R {
        Din4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin 5 Data In"]
    #[inline(always)]
    pub fn din5(&self) -> Din5R {
        Din5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin 6 Data In"]
    #[inline(always)]
    pub fn din6(&self) -> Din6R {
        Din6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin 7 Data In"]
    #[inline(always)]
    pub fn din7(&self) -> Din7R {
        Din7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin 8 Data In"]
    #[inline(always)]
    pub fn din8(&self) -> Din8R {
        Din8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin 9 Data In"]
    #[inline(always)]
    pub fn din9(&self) -> Din9R {
        Din9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin 10 Data In"]
    #[inline(always)]
    pub fn din10(&self) -> Din10R {
        Din10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin 11 Data In"]
    #[inline(always)]
    pub fn din11(&self) -> Din11R {
        Din11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin 12 Data In"]
    #[inline(always)]
    pub fn din12(&self) -> Din12R {
        Din12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin 13 Data In"]
    #[inline(always)]
    pub fn din13(&self) -> Din13R {
        Din13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin 14 Data In"]
    #[inline(always)]
    pub fn din14(&self) -> Din14R {
        Din14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pin 15 Data In"]
    #[inline(always)]
    pub fn din15(&self) -> Din15R {
        Din15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINrs;
impl crate::RegisterSpec for DINrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din::R`](R) reader structure"]
impl crate::Readable for DINrs {}
#[doc = "`reset()` method sets DIN to value 0"]
impl crate::Resettable for DINrs {
    const RESET_VALUE: u32 = 0;
}
