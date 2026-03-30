use std::fmt::{Debug, Display};

#[derive(PartialEq, Copy, Clone)]
pub enum Magic {
    Fat,
    FatReverse,
    Bin32,
    Bin32Reverse,
    Bin64,
    Bin64Reverse,
}

impl Magic {
    #[must_use]
    pub fn raw_value(self) -> u32 {
        match self {
            Magic::Fat => 0xcafe_babe,
            Magic::FatReverse => 0xbeba_feca,
            Magic::Bin32 => 0xfeed_face,
            Magic::Bin32Reverse => 0xcefa_edfe,
            Magic::Bin64 => 0xfeed_facf,
            Magic::Bin64Reverse => 0xcffa_edfe,
        }
    }

    #[must_use]
    pub fn is_fat(self) -> bool {
        matches!(self, Self::Fat | Self::FatReverse)
    }

    #[must_use]
    pub fn is_reverse(self) -> bool {
        matches!(
            self,
            Magic::FatReverse | Magic::Bin32Reverse | Magic::Bin64Reverse
        )
    }

    #[must_use]
    pub fn is_64(self) -> bool {
        matches!(self, Self::Bin64 | Self::Bin64Reverse)
    }
}

impl TryInto<Magic> for u32 {
    type Error = crate::result::Error;

    fn try_into(self) -> Result<Magic, Self::Error> {
        match self {
            0xcafe_babe => Ok(Magic::Fat),
            0xbeba_feca => Ok(Magic::FatReverse),
            0xfeed_face => Ok(Magic::Bin32),
            0xcefa_edfe => Ok(Magic::Bin32Reverse),
            0xfeed_facf => Ok(Magic::Bin64),
            0xcffa_edfe => Ok(Magic::Bin64Reverse),
            _ => Err(Self::Error::BadMagic(self)),
        }
    }
}

impl Magic {
    pub(super) fn endian(self) -> scroll::Endian {
        if self.is_fat() || !self.is_reverse() {
            scroll::BE
        } else {
            scroll::LE
        }
    }
}

impl Display for Magic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.raw_value())
    }
}

impl Debug for Magic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.raw_value())
    }
}
