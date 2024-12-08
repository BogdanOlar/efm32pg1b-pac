///Register `PEEK` reader
pub type R = crate::R<PEEKrs>;
///Field `CH0VAL` reader - Channel 0 Current Value
pub type Ch0valR = crate::BitReader;
///Field `CH1VAL` reader - Channel 1 Current Value
pub type Ch1valR = crate::BitReader;
///Field `CH2VAL` reader - Channel 2 Current Value
pub type Ch2valR = crate::BitReader;
///Field `CH3VAL` reader - Channel 3 Current Value
pub type Ch3valR = crate::BitReader;
///Field `CH4VAL` reader - Channel 4 Current Value
pub type Ch4valR = crate::BitReader;
///Field `CH5VAL` reader - Channel 5 Current Value
pub type Ch5valR = crate::BitReader;
///Field `CH6VAL` reader - Channel 6 Current Value
pub type Ch6valR = crate::BitReader;
///Field `CH7VAL` reader - Channel 7 Current Value
pub type Ch7valR = crate::BitReader;
///Field `CH8VAL` reader - Channel 8 Current Value
pub type Ch8valR = crate::BitReader;
///Field `CH9VAL` reader - Channel 9 Current Value
pub type Ch9valR = crate::BitReader;
///Field `CH10VAL` reader - Channel 10 Current Value
pub type Ch10valR = crate::BitReader;
///Field `CH11VAL` reader - Channel 11 Current Value
pub type Ch11valR = crate::BitReader;
impl R {
    ///Bit 0 - Channel 0 Current Value
    #[inline(always)]
    pub fn ch0val(&self) -> Ch0valR {
        Ch0valR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel 1 Current Value
    #[inline(always)]
    pub fn ch1val(&self) -> Ch1valR {
        Ch1valR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel 2 Current Value
    #[inline(always)]
    pub fn ch2val(&self) -> Ch2valR {
        Ch2valR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel 3 Current Value
    #[inline(always)]
    pub fn ch3val(&self) -> Ch3valR {
        Ch3valR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel 4 Current Value
    #[inline(always)]
    pub fn ch4val(&self) -> Ch4valR {
        Ch4valR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel 5 Current Value
    #[inline(always)]
    pub fn ch5val(&self) -> Ch5valR {
        Ch5valR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel 6 Current Value
    #[inline(always)]
    pub fn ch6val(&self) -> Ch6valR {
        Ch6valR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel 7 Current Value
    #[inline(always)]
    pub fn ch7val(&self) -> Ch7valR {
        Ch7valR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel 8 Current Value
    #[inline(always)]
    pub fn ch8val(&self) -> Ch8valR {
        Ch8valR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel 9 Current Value
    #[inline(always)]
    pub fn ch9val(&self) -> Ch9valR {
        Ch9valR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel 10 Current Value
    #[inline(always)]
    pub fn ch10val(&self) -> Ch10valR {
        Ch10valR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel 11 Current Value
    #[inline(always)]
    pub fn ch11val(&self) -> Ch11valR {
        Ch11valR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK")
            .field("ch0val", &self.ch0val())
            .field("ch1val", &self.ch1val())
            .field("ch2val", &self.ch2val())
            .field("ch3val", &self.ch3val())
            .field("ch4val", &self.ch4val())
            .field("ch5val", &self.ch5val())
            .field("ch6val", &self.ch6val())
            .field("ch7val", &self.ch7val())
            .field("ch8val", &self.ch8val())
            .field("ch9val", &self.ch9val())
            .field("ch10val", &self.ch10val())
            .field("ch11val", &self.ch11val())
            .finish()
    }
}
///PRS Channel Values
///
///You can [`read`](crate::Reg::read) this register and get [`peek::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PEEKrs;
impl crate::RegisterSpec for PEEKrs {
    type Ux = u32;
}
///`read()` method returns [`peek::R`](R) reader structure
impl crate::Readable for PEEKrs {}
///`reset()` method sets PEEK to value 0
impl crate::Resettable for PEEKrs {
    const RESET_VALUE: u32 = 0;
}
