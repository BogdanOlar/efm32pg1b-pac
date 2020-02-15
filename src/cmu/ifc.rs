#[doc = "Writer for register IFC"]
pub type W = crate::W<u32, super::IFC>;
#[doc = "Register IFC `reset()`'s with value 0"]
impl crate::ResetValue for super::IFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `HFRCORDY`"]
pub struct HFRCORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HFRCORDY_W<'a> {
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
#[doc = "Write proxy for field `HFXORDY`"]
pub struct HFXORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXORDY_W<'a> {
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
#[doc = "Write proxy for field `LFRCORDY`"]
pub struct LFRCORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LFRCORDY_W<'a> {
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
#[doc = "Write proxy for field `LFXORDY`"]
pub struct LFXORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXORDY_W<'a> {
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
#[doc = "Write proxy for field `AUXHFRCORDY`"]
pub struct AUXHFRCORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXHFRCORDY_W<'a> {
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
#[doc = "Write proxy for field `CALRDY`"]
pub struct CALRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRDY_W<'a> {
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
#[doc = "Write proxy for field `CALOF`"]
pub struct CALOF_W<'a> {
    w: &'a mut W,
}
impl<'a> CALOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `HFXODISERR`"]
pub struct HFXODISERR_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXODISERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `HFXOAUTOSW`"]
pub struct HFXOAUTOSW_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOAUTOSW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `HFXOPEAKDETERR`"]
pub struct HFXOPEAKDETERR_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOPEAKDETERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `HFXOPEAKDETRDY`"]
pub struct HFXOPEAKDETRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOPEAKDETRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `HFXOSHUNTOPTRDY`"]
pub struct HFXOSHUNTOPTRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOSHUNTOPTRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `HFRCODIS`"]
pub struct HFRCODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HFRCODIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `LFTIMEOUTERR`"]
pub struct LFTIMEOUTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LFTIMEOUTERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `CMUERR`"]
pub struct CMUERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMUERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clear HFRCORDY Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcordy(&mut self) -> HFRCORDY_W {
        HFRCORDY_W { w: self }
    }
    #[doc = "Bit 1 - Clear HFXORDY Interrupt Flag"]
    #[inline(always)]
    pub fn hfxordy(&mut self) -> HFXORDY_W {
        HFXORDY_W { w: self }
    }
    #[doc = "Bit 2 - Clear LFRCORDY Interrupt Flag"]
    #[inline(always)]
    pub fn lfrcordy(&mut self) -> LFRCORDY_W {
        LFRCORDY_W { w: self }
    }
    #[doc = "Bit 3 - Clear LFXORDY Interrupt Flag"]
    #[inline(always)]
    pub fn lfxordy(&mut self) -> LFXORDY_W {
        LFXORDY_W { w: self }
    }
    #[doc = "Bit 4 - Clear AUXHFRCORDY Interrupt Flag"]
    #[inline(always)]
    pub fn auxhfrcordy(&mut self) -> AUXHFRCORDY_W {
        AUXHFRCORDY_W { w: self }
    }
    #[doc = "Bit 5 - Clear CALRDY Interrupt Flag"]
    #[inline(always)]
    pub fn calrdy(&mut self) -> CALRDY_W {
        CALRDY_W { w: self }
    }
    #[doc = "Bit 6 - Clear CALOF Interrupt Flag"]
    #[inline(always)]
    pub fn calof(&mut self) -> CALOF_W {
        CALOF_W { w: self }
    }
    #[doc = "Bit 8 - Clear HFXODISERR Interrupt Flag"]
    #[inline(always)]
    pub fn hfxodiserr(&mut self) -> HFXODISERR_W {
        HFXODISERR_W { w: self }
    }
    #[doc = "Bit 9 - Clear HFXOAUTOSW Interrupt Flag"]
    #[inline(always)]
    pub fn hfxoautosw(&mut self) -> HFXOAUTOSW_W {
        HFXOAUTOSW_W { w: self }
    }
    #[doc = "Bit 10 - Clear HFXOPEAKDETERR Interrupt Flag"]
    #[inline(always)]
    pub fn hfxopeakdeterr(&mut self) -> HFXOPEAKDETERR_W {
        HFXOPEAKDETERR_W { w: self }
    }
    #[doc = "Bit 11 - Clear HFXOPEAKDETRDY Interrupt Flag"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&mut self) -> HFXOPEAKDETRDY_W {
        HFXOPEAKDETRDY_W { w: self }
    }
    #[doc = "Bit 12 - Clear HFXOSHUNTOPTRDY Interrupt Flag"]
    #[inline(always)]
    pub fn hfxoshuntoptrdy(&mut self) -> HFXOSHUNTOPTRDY_W {
        HFXOSHUNTOPTRDY_W { w: self }
    }
    #[doc = "Bit 13 - Clear HFRCODIS Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W {
        HFRCODIS_W { w: self }
    }
    #[doc = "Bit 14 - Clear LFTIMEOUTERR Interrupt Flag"]
    #[inline(always)]
    pub fn lftimeouterr(&mut self) -> LFTIMEOUTERR_W {
        LFTIMEOUTERR_W { w: self }
    }
    #[doc = "Bit 31 - Clear CMUERR Interrupt Flag"]
    #[inline(always)]
    pub fn cmuerr(&mut self) -> CMUERR_W {
        CMUERR_W { w: self }
    }
}
