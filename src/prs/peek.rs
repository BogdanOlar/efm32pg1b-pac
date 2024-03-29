#[doc = "Register `PEEK` reader"]
pub type R = crate::R<PEEKrs>;
#[doc = "Field `CH0VAL` reader - Channel 0 Current Value"]
pub type CH0VAL_R = crate::BitReader;
#[doc = "Field `CH1VAL` reader - Channel 1 Current Value"]
pub type CH1VAL_R = crate::BitReader;
#[doc = "Field `CH2VAL` reader - Channel 2 Current Value"]
pub type CH2VAL_R = crate::BitReader;
#[doc = "Field `CH3VAL` reader - Channel 3 Current Value"]
pub type CH3VAL_R = crate::BitReader;
#[doc = "Field `CH4VAL` reader - Channel 4 Current Value"]
pub type CH4VAL_R = crate::BitReader;
#[doc = "Field `CH5VAL` reader - Channel 5 Current Value"]
pub type CH5VAL_R = crate::BitReader;
#[doc = "Field `CH6VAL` reader - Channel 6 Current Value"]
pub type CH6VAL_R = crate::BitReader;
#[doc = "Field `CH7VAL` reader - Channel 7 Current Value"]
pub type CH7VAL_R = crate::BitReader;
#[doc = "Field `CH8VAL` reader - Channel 8 Current Value"]
pub type CH8VAL_R = crate::BitReader;
#[doc = "Field `CH9VAL` reader - Channel 9 Current Value"]
pub type CH9VAL_R = crate::BitReader;
#[doc = "Field `CH10VAL` reader - Channel 10 Current Value"]
pub type CH10VAL_R = crate::BitReader;
#[doc = "Field `CH11VAL` reader - Channel 11 Current Value"]
pub type CH11VAL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Current Value"]
    #[inline(always)]
    pub fn ch0val(&self) -> CH0VAL_R {
        CH0VAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Current Value"]
    #[inline(always)]
    pub fn ch1val(&self) -> CH1VAL_R {
        CH1VAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Current Value"]
    #[inline(always)]
    pub fn ch2val(&self) -> CH2VAL_R {
        CH2VAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Current Value"]
    #[inline(always)]
    pub fn ch3val(&self) -> CH3VAL_R {
        CH3VAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Current Value"]
    #[inline(always)]
    pub fn ch4val(&self) -> CH4VAL_R {
        CH4VAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Current Value"]
    #[inline(always)]
    pub fn ch5val(&self) -> CH5VAL_R {
        CH5VAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Current Value"]
    #[inline(always)]
    pub fn ch6val(&self) -> CH6VAL_R {
        CH6VAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Current Value"]
    #[inline(always)]
    pub fn ch7val(&self) -> CH7VAL_R {
        CH7VAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Current Value"]
    #[inline(always)]
    pub fn ch8val(&self) -> CH8VAL_R {
        CH8VAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Current Value"]
    #[inline(always)]
    pub fn ch9val(&self) -> CH9VAL_R {
        CH9VAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Current Value"]
    #[inline(always)]
    pub fn ch10val(&self) -> CH10VAL_R {
        CH10VAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Current Value"]
    #[inline(always)]
    pub fn ch11val(&self) -> CH11VAL_R {
        CH11VAL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "PRS Channel Values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PEEKrs;
impl crate::RegisterSpec for PEEKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peek::R`](R) reader structure"]
impl crate::Readable for PEEKrs {}
#[doc = "`reset()` method sets PEEK to value 0"]
impl crate::Resettable for PEEKrs {
    const RESET_VALUE: u32 = 0;
}
