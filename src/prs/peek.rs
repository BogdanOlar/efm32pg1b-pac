#[doc = "Reader of register PEEK"]
pub type R = crate::R<u32, super::PEEK>;
#[doc = "Reader of field `CH0VAL`"]
pub type CH0VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1VAL`"]
pub type CH1VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH2VAL`"]
pub type CH2VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH3VAL`"]
pub type CH3VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH4VAL`"]
pub type CH4VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH5VAL`"]
pub type CH5VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH6VAL`"]
pub type CH6VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH7VAL`"]
pub type CH7VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH8VAL`"]
pub type CH8VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH9VAL`"]
pub type CH9VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH10VAL`"]
pub type CH10VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH11VAL`"]
pub type CH11VAL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Current Value"]
    #[inline(always)]
    pub fn ch0val(&self) -> CH0VAL_R {
        CH0VAL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Current Value"]
    #[inline(always)]
    pub fn ch1val(&self) -> CH1VAL_R {
        CH1VAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Current Value"]
    #[inline(always)]
    pub fn ch2val(&self) -> CH2VAL_R {
        CH2VAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Current Value"]
    #[inline(always)]
    pub fn ch3val(&self) -> CH3VAL_R {
        CH3VAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Current Value"]
    #[inline(always)]
    pub fn ch4val(&self) -> CH4VAL_R {
        CH4VAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Current Value"]
    #[inline(always)]
    pub fn ch5val(&self) -> CH5VAL_R {
        CH5VAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Current Value"]
    #[inline(always)]
    pub fn ch6val(&self) -> CH6VAL_R {
        CH6VAL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Current Value"]
    #[inline(always)]
    pub fn ch7val(&self) -> CH7VAL_R {
        CH7VAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Current Value"]
    #[inline(always)]
    pub fn ch8val(&self) -> CH8VAL_R {
        CH8VAL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Current Value"]
    #[inline(always)]
    pub fn ch9val(&self) -> CH9VAL_R {
        CH9VAL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Current Value"]
    #[inline(always)]
    pub fn ch10val(&self) -> CH10VAL_R {
        CH10VAL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Current Value"]
    #[inline(always)]
    pub fn ch11val(&self) -> CH11VAL_R {
        CH11VAL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
