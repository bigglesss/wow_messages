use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::GmSurveyQuestion;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GMSURVEY_SUBMIT {
    pub survey_id: u32,
    pub questions: [GmSurveyQuestion; 10],
    pub answer_comment: String,
}

impl ClientMessageWrite for CMSG_GMSURVEY_SUBMIT {}

impl MessageBody for CMSG_GMSURVEY_SUBMIT {
    const OPCODE: u16 = 0x032a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_GMSURVEY_SUBMITError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // survey_id: u32
        let survey_id = crate::util::read_u32_le(r)?;

        // questions: GmSurveyQuestion[10]
        let mut questions = Vec::with_capacity(10 as usize);
        for i in 0..10 {
            questions.push(GmSurveyQuestion::read(r)?);
        }
        let questions = questions.try_into().unwrap();

        // answer_comment: CString
        let answer_comment = crate::util::read_c_string_to_vec(r)?;
        let answer_comment = String::from_utf8(answer_comment)?;

        Ok(Self {
            survey_id,
            questions,
            answer_comment,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // survey_id: u32
        w.write_all(&self.survey_id.to_le_bytes())?;

        // questions: GmSurveyQuestion[10]
        for i in self.questions.iter() {
            i.write(w)?;
        }

        // answer_comment: CString
        w.write_all(self.answer_comment.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for CMSG_GMSURVEY_SUBMIT {
    fn size(&self) -> usize {
        4 // survey_id: u32
        + 10 * GmSurveyQuestion::size() // questions: GmSurveyQuestion[10]
        + self.answer_comment.len() + 1 // answer_comment: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_GMSURVEY_SUBMIT {
    fn maximum_possible_size() -> usize {
        4 // survey_id: u32
        + 10 * GmSurveyQuestion::maximum_possible_size() // questions: GmSurveyQuestion[10]
        + 256 // answer_comment: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GMSURVEY_SUBMITError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GMSURVEY_SUBMITError {}
impl std::fmt::Display for CMSG_GMSURVEY_SUBMITError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GMSURVEY_SUBMITError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GMSURVEY_SUBMITError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

