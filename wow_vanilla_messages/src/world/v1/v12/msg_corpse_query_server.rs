use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{CorpseQueryResult, CorpseQueryResultError};
use crate::world::v1::v12::{Map, MapError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_CORPSE_QUERY_Server {
    pub result: MSG_CORPSE_QUERY_ServerCorpseQueryResult,
}

impl ServerMessageWrite for MSG_CORPSE_QUERY_Server {}

impl MessageBody for MSG_CORPSE_QUERY_Server {
    const OPCODE: u16 = 0x0216;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = MSG_CORPSE_QUERY_ServerError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: CorpseQueryResult
        let result = CorpseQueryResult::read(r)?;

        let result_if = match result {
            CorpseQueryResult::NOT_FOUND => MSG_CORPSE_QUERY_ServerCorpseQueryResult::NOT_FOUND,
            CorpseQueryResult::FOUND => {
                // map: Map
                let map = Map::read(r)?;

                // position_x: f32
                let position_x = crate::util::read_f32_le(r)?;
                // position_y: f32
                let position_y = crate::util::read_f32_le(r)?;
                // position_z: f32
                let position_z = crate::util::read_f32_le(r)?;
                // corpse_map: Map
                let corpse_map = Map::read(r)?;

                MSG_CORPSE_QUERY_ServerCorpseQueryResult::FOUND {
                    corpse_map,
                    map,
                    position_x,
                    position_y,
                    position_z,
                }
            }
        };

        Ok(Self {
            result: result_if,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: CorpseQueryResult
        self.result.write(w)?;

        match &self.result {
            MSG_CORPSE_QUERY_ServerCorpseQueryResult::NOT_FOUND => {}
            MSG_CORPSE_QUERY_ServerCorpseQueryResult::FOUND {
                corpse_map,
                map,
                position_x,
                position_y,
                position_z,
            } => {
                // map: Map
                map.write(w)?;

                // position_x: f32
                w.write_all(&position_x.to_le_bytes())?;

                // position_y: f32
                w.write_all(&position_y.to_le_bytes())?;

                // position_z: f32
                w.write_all(&position_z.to_le_bytes())?;

                // corpse_map: Map
                corpse_map.write(w)?;

            }
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // result: CorpseQueryResult
            let result = CorpseQueryResult::tokio_read(r).await?;

            let result_if = match result {
                CorpseQueryResult::NOT_FOUND => MSG_CORPSE_QUERY_ServerCorpseQueryResult::NOT_FOUND,
                CorpseQueryResult::FOUND => {
                    // map: Map
                    let map = Map::tokio_read(r).await?;

                    // position_x: f32
                    let position_x = crate::util::tokio_read_f32_le(r).await?;
                    // position_y: f32
                    let position_y = crate::util::tokio_read_f32_le(r).await?;
                    // position_z: f32
                    let position_z = crate::util::tokio_read_f32_le(r).await?;
                    // corpse_map: Map
                    let corpse_map = Map::tokio_read(r).await?;

                    MSG_CORPSE_QUERY_ServerCorpseQueryResult::FOUND {
                        corpse_map,
                        map,
                        position_x,
                        position_y,
                        position_z,
                    }
                }
            };

            Ok(Self {
                result: result_if,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // result: CorpseQueryResult
            self.result.tokio_write(w).await?;

            match &self.result {
                MSG_CORPSE_QUERY_ServerCorpseQueryResult::NOT_FOUND => {}
                MSG_CORPSE_QUERY_ServerCorpseQueryResult::FOUND {
                    corpse_map,
                    map,
                    position_x,
                    position_y,
                    position_z,
                } => {
                    // map: Map
                    map.tokio_write(w).await?;

                    // position_x: f32
                    w.write_all(&position_x.to_le_bytes()).await?;

                    // position_y: f32
                    w.write_all(&position_y.to_le_bytes()).await?;

                    // position_z: f32
                    w.write_all(&position_z.to_le_bytes()).await?;

                    // corpse_map: Map
                    corpse_map.tokio_write(w).await?;

                }
            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // result: CorpseQueryResult
            let result = CorpseQueryResult::astd_read(r).await?;

            let result_if = match result {
                CorpseQueryResult::NOT_FOUND => MSG_CORPSE_QUERY_ServerCorpseQueryResult::NOT_FOUND,
                CorpseQueryResult::FOUND => {
                    // map: Map
                    let map = Map::astd_read(r).await?;

                    // position_x: f32
                    let position_x = crate::util::astd_read_f32_le(r).await?;
                    // position_y: f32
                    let position_y = crate::util::astd_read_f32_le(r).await?;
                    // position_z: f32
                    let position_z = crate::util::astd_read_f32_le(r).await?;
                    // corpse_map: Map
                    let corpse_map = Map::astd_read(r).await?;

                    MSG_CORPSE_QUERY_ServerCorpseQueryResult::FOUND {
                        corpse_map,
                        map,
                        position_x,
                        position_y,
                        position_z,
                    }
                }
            };

            Ok(Self {
                result: result_if,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // result: CorpseQueryResult
            self.result.astd_write(w).await?;

            match &self.result {
                MSG_CORPSE_QUERY_ServerCorpseQueryResult::NOT_FOUND => {}
                MSG_CORPSE_QUERY_ServerCorpseQueryResult::FOUND {
                    corpse_map,
                    map,
                    position_x,
                    position_y,
                    position_z,
                } => {
                    // map: Map
                    map.astd_write(w).await?;

                    // position_x: f32
                    w.write_all(&position_x.to_le_bytes()).await?;

                    // position_y: f32
                    w.write_all(&position_y.to_le_bytes()).await?;

                    // position_z: f32
                    w.write_all(&position_z.to_le_bytes()).await?;

                    // corpse_map: Map
                    corpse_map.astd_write(w).await?;

                }
            }

            Ok(())
        })
    }

}

impl VariableSized for MSG_CORPSE_QUERY_Server {
    fn size(&self) -> usize {
        0
        + self.result.size() // result: MSG_CORPSE_QUERY_ServerCorpseQueryResult
    }
}

impl MaximumPossibleSized for MSG_CORPSE_QUERY_Server {
    fn maximum_possible_size() -> usize {
        0
        + 21 // result: MSG_CORPSE_QUERY_ServerCorpseQueryResult
    }
}

#[derive(Debug)]
pub enum MSG_CORPSE_QUERY_ServerError {
    Io(std::io::Error),
    CorpseQueryResult(CorpseQueryResultError),
    Map(MapError),
}

impl std::error::Error for MSG_CORPSE_QUERY_ServerError {}
impl std::fmt::Display for MSG_CORPSE_QUERY_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::CorpseQueryResult(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MSG_CORPSE_QUERY_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<CorpseQueryResultError> for MSG_CORPSE_QUERY_ServerError {
    fn from(e: CorpseQueryResultError) -> Self {
        Self::CorpseQueryResult(e)
    }
}

impl From<MapError> for MSG_CORPSE_QUERY_ServerError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    NOT_FOUND,
    FOUND {
        corpse_map: Map,
        map: Map,
        position_x: f32,
        position_y: f32,
        position_z: f32,
    },
}

impl From<&CorpseQueryResult> for MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    fn from(e: &CorpseQueryResult) -> Self {
        match &e {
            CorpseQueryResult::NOT_FOUND => Self::NOT_FOUND,
            CorpseQueryResult::FOUND => Self::FOUND {
                corpse_map: Default::default(),
                map: Default::default(),
                position_x: Default::default(),
                position_y: Default::default(),
                position_z: Default::default(),
            },
        }
    }
}

impl From<&MSG_CORPSE_QUERY_ServerCorpseQueryResult> for CorpseQueryResult {
    fn from(v: &MSG_CORPSE_QUERY_ServerCorpseQueryResult) -> Self {
        match &v {
            MSG_CORPSE_QUERY_ServerCorpseQueryResult::NOT_FOUND => Self::NOT_FOUND,
            MSG_CORPSE_QUERY_ServerCorpseQueryResult::FOUND { .. } => Self::FOUND,
        }
    }
}

impl Default for MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NOT_FOUND
    }
}

impl MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub(crate) fn as_int(&self) -> u8 {
        let a: CorpseQueryResult = self.into();
        a.as_int() as u8
    }

}

impl VariableSized for MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    fn size(&self) -> usize {
        match self {
            Self::NOT_FOUND => {
                1
            }
            Self::FOUND {
                corpse_map,
                map,
                position_x,
                position_y,
                position_z,
            } => {
                1
                + 4 // corpse_map: Map
                + 4 // map: Map
                + 4 // position_x: f32
                + 4 // position_y: f32
                + 4 // position_z: f32
            }
        }
    }
}

impl MaximumPossibleSized for MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    fn maximum_possible_size() -> usize {
        21
    }
}

