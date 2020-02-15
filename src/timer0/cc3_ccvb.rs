#[doc = "Reader of register CC3_CCVB"]
pub type R = crate::R<u32, super::CC3_CCVB>;
#[doc = "Writer for register CC3_CCVB"]
pub type W = crate::W<u32, super::CC3_CCVB>;
#[doc = "Register CC3_CCVB `reset()`'s with value 0"]
impl crate::ResetValue for super::CC3_CCVB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCVB`"]
pub type CCVB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCVB`"]
pub struct CCVB_W<'a> {
    w: &'a mut W,
}
impl<'a> CCVB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CC Channel Value Buffer"]
    #[inline(always)]
    pub fn ccvb(&self) -> CCVB_R {
        CCVB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CC Channel Value Buffer"]
    #[inline(always)]
    pub fn ccvb(&mut self) -> CCVB_W {
        CCVB_W { w: self }
    }
}
