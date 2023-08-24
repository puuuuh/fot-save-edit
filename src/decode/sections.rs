mod zar;
mod ssg;
mod sdg;
mod entity_file;
mod esh;

use crate::decode::error::ParseError;
use crate::decode::primitive::FOTString;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Read, Seek, SeekFrom};
use crate::{assert_section, skip};

