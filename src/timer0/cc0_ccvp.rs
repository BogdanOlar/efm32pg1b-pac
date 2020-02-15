#[doc = "Reader of register CC0_CCVP"]
pub type R = crate::R<u32, super::CC0_CCVP>;
#[doc = "Reader of field `CCVP`"]
pub type CCVP_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value Peek"]
    #[inline(always)]
    pub fn ccvp(&self) -> CCVP_R {
        CCVP_R::new((self.bits & 0xffff) as u16)
    }
}
