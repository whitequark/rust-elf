use crate::gabi;
use crate::parse::{Class, Endian, EndianParseExt, ParseAt, ParseError, ParsingIterator};

pub type SectionHeaderIterator<'data> = ParsingIterator<'data, SectionHeader>;

/// Encapsulates the contents of an ELF Section Header
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SectionHeader {
    /// Section Name
    pub sh_name: u32,
    /// Section Type
    pub sh_type: SectionType,
    /// Section Flags
    pub sh_flags: SectionFlag,
    /// in-memory address where this section is loaded
    pub sh_addr: u64,
    /// Byte-offset into the file where this section starts
    pub sh_offset: u64,
    /// Section size in bytes
    pub sh_size: u64,
    /// Defined by section type
    pub sh_link: u32,
    /// Defined by section type
    pub sh_info: u32,
    /// address alignment
    pub sh_addralign: u64,
    /// size of an entry if section data is an array of entries
    pub sh_entsize: u64,
}

impl ParseAt for SectionHeader {
    fn parse_at<P: EndianParseExt>(
        endian: Endian,
        class: Class,
        offset: &mut usize,
        parser: &P,
    ) -> Result<Self, ParseError> {
        match class {
            Class::ELF32 => Ok(SectionHeader {
                sh_name: parser.parse_u32_at(endian, offset)?,
                sh_type: SectionType(parser.parse_u32_at(endian, offset)?),
                sh_flags: SectionFlag(parser.parse_u32_at(endian, offset)? as u64),
                sh_addr: parser.parse_u32_at(endian, offset)? as u64,
                sh_offset: parser.parse_u32_at(endian, offset)? as u64,
                sh_size: parser.parse_u32_at(endian, offset)? as u64,
                sh_link: parser.parse_u32_at(endian, offset)?,
                sh_info: parser.parse_u32_at(endian, offset)?,
                sh_addralign: parser.parse_u32_at(endian, offset)? as u64,
                sh_entsize: parser.parse_u32_at(endian, offset)? as u64,
            }),
            Class::ELF64 => Ok(SectionHeader {
                sh_name: parser.parse_u32_at(endian, offset)?,
                sh_type: SectionType(parser.parse_u32_at(endian, offset)?),
                sh_flags: SectionFlag(parser.parse_u64_at(endian, offset)?),
                sh_addr: parser.parse_u64_at(endian, offset)?,
                sh_offset: parser.parse_u64_at(endian, offset)?,
                sh_size: parser.parse_u64_at(endian, offset)?,
                sh_link: parser.parse_u32_at(endian, offset)?,
                sh_info: parser.parse_u32_at(endian, offset)?,
                sh_addralign: parser.parse_u64_at(endian, offset)?,
                sh_entsize: parser.parse_u64_at(endian, offset)?,
            }),
        }
    }
}

impl core::fmt::Display for SectionHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Section Header: Name: {} Type: {} Flags: {} Addr: {:#010x} Offset: {:#06x} Size: {:#06x} Link: {} Info: {:#x} AddrAlign: {} EntSize: {}",
            self.sh_name, self.sh_type, self.sh_flags, self.sh_addr, self.sh_offset,
            self.sh_size, self.sh_link, self.sh_info, self.sh_addralign, self.sh_entsize)
    }
}

/// Represens ELF Section type
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SectionType(pub u32);

impl PartialEq<u32> for SectionType {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl core::fmt::Debug for SectionType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl core::fmt::Display for SectionType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let str = match self.0 {
            gabi::SHT_NULL => "SHT_NULL",
            gabi::SHT_PROGBITS => "SHT_PROGBITS",
            gabi::SHT_SYMTAB => "SHT_SYMTAB",
            gabi::SHT_STRTAB => "SHT_STRTAB",
            gabi::SHT_RELA => "SHT_RELA",
            gabi::SHT_HASH => "SHT_HASH",
            gabi::SHT_DYNAMIC => "SHT_DYNAMIC",
            gabi::SHT_NOTE => "SHT_NOTE",
            gabi::SHT_NOBITS => "SHT_NOBITS",
            gabi::SHT_REL => "SHT_REL",
            gabi::SHT_SHLIB => "SHT_SHLIB",
            gabi::SHT_DYNSYM => "SHT_DYNSYM",
            gabi::SHT_INIT_ARRAY => "SHT_INIT_ARRAY",
            gabi::SHT_FINI_ARRAY => "SHT_FINI_ARRAY",
            gabi::SHT_PREINIT_ARRAY => "SHT_PREINIT_ARRAY",
            gabi::SHT_GROUP => "SHT_GROUP",
            gabi::SHT_SYMTAB_SHNDX => "SHT_SYMTAB_SHNDX",
            gabi::SHT_NUM => "SHT_NUM",
            gabi::SHT_GNU_ATTRIBUTES => "SHT_GNU_ATTRIBUTES",
            gabi::SHT_GNU_HASH => "SHT_GNU_HASH",
            gabi::SHT_GNU_LIBLIST => "SHT_GNU_LIBLIST",
            gabi::SHT_GNU_VERDEF => "SHT_GNU_VERDEF",
            gabi::SHT_GNU_VERNEED => "SHT_GNU_VERNEED",
            gabi::SHT_GNU_VERSYM => "SHT_GNU_VERSYM",
            _ => "Unknown",
        };
        write!(f, "{}", str)
    }
}

///
/// Wrapper type for SectionFlag
///
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SectionFlag(pub u64);

impl core::fmt::Debug for SectionFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl core::fmt::Display for SectionFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

#[cfg(test)]
mod iter_tests {
    use super::*;

    const ELF32SHDRSIZE: u16 = 40;
    const ELF64SHDRSIZE: u16 = 64;

    #[test]
    fn get_32_lsb() {
        // init data buf with two header's worth of increasing byte values
        let mut data = [0u8; 2 * ELF32SHDRSIZE as usize];
        for n in 0..(2 * ELF32SHDRSIZE) {
            data[n as usize] = n as u8;
        }
        let mut iter = SectionHeaderIterator::new(Endian::Little, Class::ELF32, &data);

        assert_eq!(
            iter.next().unwrap(),
            SectionHeader {
                sh_name: 0x03020100,
                sh_type: SectionType(0x07060504),
                sh_flags: SectionFlag(0xB0A0908),
                sh_addr: 0x0F0E0D0C,
                sh_offset: 0x13121110,
                sh_size: 0x17161514,
                sh_link: 0x1B1A1918,
                sh_info: 0x1F1E1D1C,
                sh_addralign: 0x23222120,
                sh_entsize: 0x27262524,
            }
        );
        assert_eq!(
            iter.next().unwrap(),
            SectionHeader {
                sh_name: 0x2B2A2928,
                sh_type: SectionType(0x2F2E2D2C),
                sh_flags: SectionFlag(0x33323130),
                sh_addr: 0x37363534,
                sh_offset: 0x3B3A3938,
                sh_size: 0x3F3E3D3C,
                sh_link: 0x43424140,
                sh_info: 0x47464544,
                sh_addralign: 0x4B4A4948,
                sh_entsize: 0x4F4E4D4C,
            }
        );
        let next = iter.next();
        assert!(next.is_none());
    }

    #[test]
    fn get_64_msb() {
        // init data buf with two header's worth of increasing byte values
        let mut data = [0u8; 2 * ELF64SHDRSIZE as usize];
        for n in 0..(2 * ELF64SHDRSIZE) {
            data[n as usize] = n as u8;
        }
        let mut iter = SectionHeaderIterator::new(Endian::Big, Class::ELF64, &data);

        assert_eq!(
            iter.next().unwrap(),
            SectionHeader {
                sh_name: 0x00010203,
                sh_type: SectionType(0x04050607),
                sh_flags: SectionFlag(0x08090A0B0C0D0E0F),
                sh_addr: 0x1011121314151617,
                sh_offset: 0x18191A1B1C1D1E1F,
                sh_size: 0x2021222324252627,
                sh_link: 0x28292A2B,
                sh_info: 0x2C2D2E2F,
                sh_addralign: 0x3031323334353637,
                sh_entsize: 0x38393A3B3C3D3E3F,
            }
        );
        assert_eq!(
            iter.next().unwrap(),
            SectionHeader {
                sh_name: 0x40414243,
                sh_type: SectionType(0x44454647),
                sh_flags: SectionFlag(0x48494A4B4C4D4E4F),
                sh_addr: 0x5051525354555657,
                sh_offset: 0x58595A5B5C5D5E5F,
                sh_size: 0x6061626364656667,
                sh_link: 0x68696A6B,
                sh_info: 0x6C6D6E6F,
                sh_addralign: 0x7071727374757677,
                sh_entsize: 0x78797A7B7C7D7E7F,
            }
        );
        let next = iter.next();
        assert!(next.is_none());
    }
}

#[cfg(test)]
mod shdr_tests {
    use super::*;

    const ELF32SHDRSIZE: u16 = 40;
    const ELF64SHDRSIZE: u16 = 64;

    #[test]
    fn parse_shdr32_fuzz_too_short() {
        let data = [0u8; ELF32SHDRSIZE as usize];
        for n in 0..ELF32SHDRSIZE as usize {
            let buf = data.split_at(n).0.as_ref();
            let mut offset = 0;
            let result = SectionHeader::parse_at(Endian::Little, Class::ELF32, &mut offset, &buf);
            assert!(
                matches!(result, Err(ParseError::BadOffset(_))),
                "Unexpected Error type found: {result:?}"
            );
        }
    }

    #[test]
    fn parse_shdr32_works() {
        let mut data = [0u8; ELF32SHDRSIZE as usize];
        for n in 0..ELF32SHDRSIZE as u8 {
            data[n as usize] = n;
        }

        let mut offset = 0;
        assert_eq!(
            SectionHeader::parse_at(Endian::Little, Class::ELF32, &mut offset, &data.as_ref())
                .unwrap(),
            SectionHeader {
                sh_name: 0x03020100,
                sh_type: SectionType(0x07060504),
                sh_flags: SectionFlag(0xB0A0908),
                sh_addr: 0x0F0E0D0C,
                sh_offset: 0x13121110,
                sh_size: 0x17161514,
                sh_link: 0x1B1A1918,
                sh_info: 0x1F1E1D1C,
                sh_addralign: 0x23222120,
                sh_entsize: 0x27262524,
            }
        );
    }

    #[test]
    fn parse_shdr64_fuzz_too_short() {
        let data = [0u8; ELF64SHDRSIZE as usize];
        for n in 0..ELF64SHDRSIZE as usize {
            let buf = data.split_at(n).0.as_ref();
            let mut offset = 0;
            let result = SectionHeader::parse_at(Endian::Big, Class::ELF64, &mut offset, &buf);
            assert!(
                matches!(result, Err(ParseError::BadOffset(_))),
                "Unexpected Error type found: {result:?}"
            );
        }
    }

    #[test]
    fn parse_shdr64_works() {
        let mut data = [0u8; ELF64SHDRSIZE as usize];
        for n in 0..ELF64SHDRSIZE as u8 {
            data[n as usize] = n;
        }

        let mut offset = 0;
        assert_eq!(
            SectionHeader::parse_at(Endian::Big, Class::ELF64, &mut offset, &data.as_ref())
                .unwrap(),
            SectionHeader {
                sh_name: 0x00010203,
                sh_type: SectionType(0x04050607),
                sh_flags: SectionFlag(0x08090A0B0C0D0E0F),
                sh_addr: 0x1011121314151617,
                sh_offset: 0x18191A1B1C1D1E1F,
                sh_size: 0x2021222324252627,
                sh_link: 0x28292A2B,
                sh_info: 0x2C2D2E2F,
                sh_addralign: 0x3031323334353637,
                sh_entsize: 0x38393A3B3C3D3E3F,
            }
        );
    }
}
