#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `ADDRFAULTEN`"]
pub type ADDRFAULTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDRFAULTEN`"]
pub struct ADDRFAULTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRFAULTEN_W<'a> {
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
#[doc = "Reader of field `CLKDISFAULTEN`"]
pub type CLKDISFAULTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKDISFAULTEN`"]
pub struct CLKDISFAULTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDISFAULTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PWRUPONDEMAND`"]
pub type PWRUPONDEMAND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRUPONDEMAND`"]
pub struct PWRUPONDEMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUPONDEMAND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `IFCREADCLEAR`"]
pub type IFCREADCLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IFCREADCLEAR`"]
pub struct IFCREADCLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> IFCREADCLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enable"]
    #[inline(always)]
    pub fn addrfaulten(&self) -> ADDRFAULTEN_R {
        ADDRFAULTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock-disabled Bus Fault Response Enable"]
    #[inline(always)]
    pub fn clkdisfaulten(&self) -> CLKDISFAULTEN_R {
        CLKDISFAULTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Power Up on Demand During Wake Up"]
    #[inline(always)]
    pub fn pwrupondemand(&self) -> PWRUPONDEMAND_R {
        PWRUPONDEMAND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IFC Read Clears IF"]
    #[inline(always)]
    pub fn ifcreadclear(&self) -> IFCREADCLEAR_R {
        IFCREADCLEAR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enable"]
    #[inline(always)]
    pub fn addrfaulten(&mut self) -> ADDRFAULTEN_W {
        ADDRFAULTEN_W { w: self }
    }
    #[doc = "Bit 1 - Clock-disabled Bus Fault Response Enable"]
    #[inline(always)]
    pub fn clkdisfaulten(&mut self) -> CLKDISFAULTEN_W {
        CLKDISFAULTEN_W { w: self }
    }
    #[doc = "Bit 2 - Power Up on Demand During Wake Up"]
    #[inline(always)]
    pub fn pwrupondemand(&mut self) -> PWRUPONDEMAND_W {
        PWRUPONDEMAND_W { w: self }
    }
    #[doc = "Bit 3 - IFC Read Clears IF"]
    #[inline(always)]
    pub fn ifcreadclear(&mut self) -> IFCREADCLEAR_W {
        IFCREADCLEAR_W { w: self }
    }
}
