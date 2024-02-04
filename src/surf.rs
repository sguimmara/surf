use std::fmt::Display;

use byteorder::ByteOrder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WavInfo {
    format: AudioFormat,
    channels: u16,
    frequency: u16,
    bits_per_sample: u16,
}

impl Display for WavInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "format            {}", self.format)?;
        writeln!(f, "channels          {}", self.channels)?;
        writeln!(f, "frequency         {} Hz", self.frequency)?;
        writeln!(f, "bits per sample   {}", self.bits_per_sample)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WavError {
    InvalidRIFFHeader,
    InvalidFileFormatId,
    InvalidDataBloc,
    InvalidFormatBlocId,
    InvalidFileSize,
    InvalidAudioFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    PCM,
    PCMFloat,
    WaveFormatExtensible,
}

impl Display for AudioFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioFormat::PCM => write!(f, "PCM"),
            AudioFormat::PCMFloat => write!(f, "PCM float"),
            AudioFormat::WaveFormatExtensible => write!(f, "WAVE_FORMAT_EXTENSIBLE"),
        }
    }
}

impl Display for WavError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WavError::InvalidRIFFHeader => write!(f, "Invalid RIFF header"),
            WavError::InvalidFileSize => write!(f, "Invalid file size"),
            WavError::InvalidFileFormatId => write!(f, "Invalid file format ID"),
            WavError::InvalidFormatBlocId => write!(f, "Invalid file format bloc ID"),
            WavError::InvalidAudioFormat => write!(f, "Invalid audio format"),
            WavError::InvalidDataBloc => write!(f, "Invalid data bloc"),
        }
    }
}

pub fn get_info(data: &[u8]) -> Result<WavInfo, WavError> {
    if &data[0..4] != b"RIFF" {
        return Err(WavError::InvalidRIFFHeader);
    }

    let declared_size = byteorder::LittleEndian::read_u32(&data[4..8]);
    let actual_size = declared_size + 8;

    if actual_size != data.len() as u32 {
        return Err(WavError::InvalidFileSize);
    }

    if &data[8..12] != b"WAVE" {
        return Err(WavError::InvalidFileFormatId);
    }

    if &data[12..16] != [0x66, 0x6D, 0x74, 0x20] {
        return Err(WavError::InvalidFormatBlocId);
    }

    let bloc_size = byteorder::LittleEndian::read_u32(&data[16..20]);
    let _actual_bloc_size = bloc_size + 16;

    // TODO check bloc size

    let audio_format_num = byteorder::LittleEndian::read_u16(&data[20..22]);

    let format = match audio_format_num {
        1 => AudioFormat::PCM,
        3 => AudioFormat::PCMFloat,
        65534 => AudioFormat::WaveFormatExtensible,
        _ => return Err(WavError::InvalidAudioFormat),
    };

    let channels = byteorder::LittleEndian::read_u16(&data[22..24]);
    let frequency = byteorder::LittleEndian::read_u16(&data[24..28]);
    let _bytes_per_sec = byteorder::LittleEndian::read_u16(&data[28..32]);
    let _bytes_per_bloc = byteorder::LittleEndian::read_u16(&data[32..34]);
    let bits_per_sample = byteorder::LittleEndian::read_u16(&data[34..36]);

    if &data[36..40] != b"data" {
        return Err(WavError::InvalidDataBloc);
    }

    let info = WavInfo {
        format,
        channels,
        frequency,
        bits_per_sample,
    };

    Ok(info)
}

#[cfg(test)]
mod test {
    use crate::surf::WavError;

    #[test]
    fn get_info_invalid_riff() {
        let result = super::get_info(b"RAFF");

        assert_eq!(Err(WavError::InvalidRIFFHeader), result);
    }

    #[test]
    fn get_info_invalid_size() {
        let result = super::get_info(b"RIFF\0\0\0\0blablabla");

        assert_eq!(Err(WavError::InvalidFileSize), result);
    }

    #[test]
    fn get_info_invalid_file_format_id() {
        let result = super::get_info(b"RIFF\x04\0\0\0WOVE");

        assert_eq!(Err(WavError::InvalidFileFormatId), result);
    }

    #[test]
    fn get_info_invalid_file_format_bloc_id() {
        let result = super::get_info(b"RIFF\x08\0\0\0WAVE....");

        assert_eq!(Err(WavError::InvalidFormatBlocId), result);
    }

    #[test]
    fn get_info_invalid_audio_format() {
        let result = super::get_info(b"RIFF\x0e\0\0\0WAVE\x66\x6D\x74\x20\0\0\0\0\x04\x04");

        assert_eq!(Err(WavError::InvalidAudioFormat), result);
    }
}
