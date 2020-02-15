#[doc = "Reader of register DBGCLKSEL"]
pub type R = crate::R<u32, super::DBGCLKSEL>;
#[doc = "Writer for register DBGCLKSEL"]
pub type W = crate::W<u32, super::DBGCLKSEL>;
#[doc = "Register DBGCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::DBGCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Debug Trace Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_A {
    #[doc = "0: AUXHFRCO is the debug trace clock"]
    AUXHFRCO = 0,
    #[doc = "1: HFCLK is the debug trace clock"]
    HFCLK = 1,
}
impl From<DBG_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG`"]
pub type DBG_R = crate::R<bool, DBG_A>;
impl DBG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_A {
        match self.bits {
            false => DBG_A::AUXHFRCO,
            true => DBG_A::HFCLK,
        }
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DBG_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == DBG_A::HFCLK
    }
}
#[doc = "Write proxy for field `DBG`"]
pub struct DBG_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AUXHFRCO is the debug trace clock"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(DBG_A::AUXHFRCO)
    }
    #[doc = "HFCLK is the debug trace clock"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(DBG_A::HFCLK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Debug Trace Clock"]
    #[inline(always)]
    pub fn dbg(&self) -> DBG_R {
        DBG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Trace Clock"]
    #[inline(always)]
    pub fn dbg(&mut self) -> DBG_W {
        DBG_W { w: self }
    }
}
