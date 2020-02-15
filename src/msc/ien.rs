#[doc = "Reader of register IEN"]
pub type R = crate::R<u32, super::IEN>;
#[doc = "Writer for register IEN"]
pub type W = crate::W<u32, super::IEN>;
#[doc = "Register IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERASE`"]
pub type ERASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE`"]
pub struct ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_W<'a> {
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
#[doc = "Reader of field `WRITE`"]
pub type WRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRITE`"]
pub struct WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_W<'a> {
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
#[doc = "Reader of field `CHOF`"]
pub type CHOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHOF`"]
pub struct CHOF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOF_W<'a> {
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
#[doc = "Reader of field `CMOF`"]
pub type CMOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMOF`"]
pub struct CMOF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMOF_W<'a> {
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
#[doc = "Reader of field `PWRUPF`"]
pub type PWRUPF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRUPF`"]
pub struct PWRUPF_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUPF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ICACHERR`"]
pub type ICACHERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICACHERR`"]
pub struct ICACHERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ERASE Interrupt Enable"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WRITE Interrupt Enable"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CHOF Interrupt Enable"]
    #[inline(always)]
    pub fn chof(&self) -> CHOF_R {
        CHOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CMOF Interrupt Enable"]
    #[inline(always)]
    pub fn cmof(&self) -> CMOF_R {
        CMOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWRUPF Interrupt Enable"]
    #[inline(always)]
    pub fn pwrupf(&self) -> PWRUPF_R {
        PWRUPF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ICACHERR Interrupt Enable"]
    #[inline(always)]
    pub fn icacherr(&self) -> ICACHERR_R {
        ICACHERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ERASE Interrupt Enable"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W { w: self }
    }
    #[doc = "Bit 1 - WRITE Interrupt Enable"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W { w: self }
    }
    #[doc = "Bit 2 - CHOF Interrupt Enable"]
    #[inline(always)]
    pub fn chof(&mut self) -> CHOF_W {
        CHOF_W { w: self }
    }
    #[doc = "Bit 3 - CMOF Interrupt Enable"]
    #[inline(always)]
    pub fn cmof(&mut self) -> CMOF_W {
        CMOF_W { w: self }
    }
    #[doc = "Bit 4 - PWRUPF Interrupt Enable"]
    #[inline(always)]
    pub fn pwrupf(&mut self) -> PWRUPF_W {
        PWRUPF_W { w: self }
    }
    #[doc = "Bit 5 - ICACHERR Interrupt Enable"]
    #[inline(always)]
    pub fn icacherr(&mut self) -> ICACHERR_W {
        ICACHERR_W { w: self }
    }
}
