use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{MovementFlags};
use crate::world::v1::v12::{SplineFlag};
use crate::world::v1::v12::TransportInfo;
use crate::world::v1::v12::{UpdateFlag};
use crate::world::v1::v12::Vector3d;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MovementBlock {
    pub update_flag: MovementBlockUpdateFlag,
}

impl MovementBlock {
    pub const UNKNOWN0_VALUE: u32 = 0x00;

    pub const UNKNOWN1_VALUE: u32 = 0x01;

}

impl ReadableAndWritable for MovementBlock {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // update_flag: UpdateFlag
        let update_flag = UpdateFlag::read(r)?;

        let update_flag_LIVING = if update_flag.is_LIVING() {
            // flags: MovementFlags
            let flags = MovementFlags::read(r)?;

            // timestamp: u32
            let timestamp = crate::util::read_u32_le(r)?;

            // living_position_x: f32
            let living_position_x = crate::util::read_f32_le(r)?;
            // living_position_y: f32
            let living_position_y = crate::util::read_f32_le(r)?;
            // living_position_z: f32
            let living_position_z = crate::util::read_f32_le(r)?;
            // living_orientation: f32
            let living_orientation = crate::util::read_f32_le(r)?;
            let flags_ON_TRANSPORT = if flags.is_ON_TRANSPORT() {
                // transport: TransportInfo
                let transport = TransportInfo::read(r)?;

                Some(MovementBlockMovementFlagsON_TRANSPORT {
                    transport,
                })
            }
            else {
                None
            };

            let flags_SWIMMING = if flags.is_SWIMMING() {
                // pitch: f32
                let pitch = crate::util::read_f32_le(r)?;
                Some(MovementBlockMovementFlagsSWIMMING {
                    pitch,
                })
            }
            else {
                None
            };

            // fall_time: f32
            let fall_time = crate::util::read_f32_le(r)?;
            let flags_JUMPING = if flags.is_JUMPING() {
                // z_speed: f32
                let z_speed = crate::util::read_f32_le(r)?;
                // cos_angle: f32
                let cos_angle = crate::util::read_f32_le(r)?;
                // sin_angle: f32
                let sin_angle = crate::util::read_f32_le(r)?;
                // xy_speed: f32
                let xy_speed = crate::util::read_f32_le(r)?;
                Some(MovementBlockMovementFlagsJUMPING {
                    cos_angle,
                    sin_angle,
                    xy_speed,
                    z_speed,
                })
            }
            else {
                None
            };

            let flags_SPLINE_ELEVATION = if flags.is_SPLINE_ELEVATION() {
                // spline_elevation: f32
                let spline_elevation = crate::util::read_f32_le(r)?;
                Some(MovementBlockMovementFlagsSPLINE_ELEVATION {
                    spline_elevation,
                })
            }
            else {
                None
            };

            // walking_speed: f32
            let walking_speed = crate::util::read_f32_le(r)?;
            // running_speed: f32
            let running_speed = crate::util::read_f32_le(r)?;
            // backwards_running_speed: f32
            let backwards_running_speed = crate::util::read_f32_le(r)?;
            // swimming_speed: f32
            let swimming_speed = crate::util::read_f32_le(r)?;
            // backwards_swimming_speed: f32
            let backwards_swimming_speed = crate::util::read_f32_le(r)?;
            // turn_rate: f32
            let turn_rate = crate::util::read_f32_le(r)?;
            let flags_SPLINE_ENABLED = if flags.is_SPLINE_ENABLED() {
                // spline_flags: SplineFlag
                let spline_flags = SplineFlag::read(r)?;

                let spline_flags_FINAL_ANGLE = if spline_flags.is_FINAL_ANGLE() {
                    // angle: f32
                    let angle = crate::util::read_f32_le(r)?;
                    Some(MovementBlockSplineFlagFINAL_ANGLE::FINAL_ANGLE {
                        angle,
                    })
                }
                else if spline_flags.is_FINAL_TARGET() {
                    // target: u64
                    let target = crate::util::read_u64_le(r)?;

                    Some(MovementBlockSplineFlagFINAL_ANGLE::FINAL_TARGET {
                        target,
                    })
                }
                else if spline_flags.is_FINAL_POINT() {
                    // spline_final_point_x: f32
                    let spline_final_point_x = crate::util::read_f32_le(r)?;
                    // spline_final_point_y: f32
                    let spline_final_point_y = crate::util::read_f32_le(r)?;
                    // spline_final_point_z: f32
                    let spline_final_point_z = crate::util::read_f32_le(r)?;
                    Some(MovementBlockSplineFlagFINAL_ANGLE::FINAL_POINT {
                        spline_final_point_x,
                        spline_final_point_y,
                        spline_final_point_z,
                    })
                }
                else {
                    None
                };

                // time_passed: u32
                let time_passed = crate::util::read_u32_le(r)?;

                // duration: u32
                let duration = crate::util::read_u32_le(r)?;

                // id: u32
                let id = crate::util::read_u32_le(r)?;

                // amount_of_nodes: u32
                let amount_of_nodes = crate::util::read_u32_le(r)?;

                // nodes: Vector3d[amount_of_nodes]
                let mut nodes = Vec::with_capacity(amount_of_nodes as usize);
                for i in 0..amount_of_nodes {
                    nodes.push(Vector3d::read(r)?);
                }

                // final_node: Vector3d
                let final_node = Vector3d::read(r)?;

                let spline_flags = MovementBlockSplineFlag {
                    inner: spline_flags.as_int(),
                    final_angle: spline_flags_FINAL_ANGLE,
                };

                Some(MovementBlockMovementFlagsSPLINE_ENABLED {
                    duration,
                    final_node,
                    id,
                    nodes,
                    spline_flags,
                    time_passed,
                })
            }
            else {
                None
            };

            let flags = MovementBlockMovementFlags {
                inner: flags.as_int(),
                on_transport: flags_ON_TRANSPORT,
                jumping: flags_JUMPING,
                swimming: flags_SWIMMING,
                spline_enabled: flags_SPLINE_ENABLED,
                spline_elevation: flags_SPLINE_ELEVATION,
            };

            Some(MovementBlockUpdateFlagLIVING::LIVING {
                backwards_running_speed,
                backwards_swimming_speed,
                fall_time,
                flags,
                living_orientation,
                living_position_x,
                living_position_y,
                living_position_z,
                running_speed,
                swimming_speed,
                timestamp,
                turn_rate,
                walking_speed,
            })
        }
        else if update_flag.is_HAS_POSITION() {
            // position_x: f32
            let position_x = crate::util::read_f32_le(r)?;
            // position_y: f32
            let position_y = crate::util::read_f32_le(r)?;
            // position_z: f32
            let position_z = crate::util::read_f32_le(r)?;
            // orientation: f32
            let orientation = crate::util::read_f32_le(r)?;
            Some(MovementBlockUpdateFlagLIVING::HAS_POSITION {
                orientation,
                position_x,
                position_y,
                position_z,
            })
        }
        else {
            None
        };

        let update_flag_HIGH_GUID = if update_flag.is_HIGH_GUID() {
            // unknown0: u32
            let _unknown0 = crate::util::read_u32_le(r)?;
            // unknown0 is expected to always be 0 (0)

            Some(MovementBlockUpdateFlagHIGH_GUID {
            })
        }
        else {
            None
        };

        let update_flag_ALL = if update_flag.is_ALL() {
            // unknown1: u32
            let _unknown1 = crate::util::read_u32_le(r)?;
            // unknown1 is expected to always be 1 (1)

            Some(MovementBlockUpdateFlagALL {
            })
        }
        else {
            None
        };

        let update_flag_MELEE_ATTACKING = if update_flag.is_MELEE_ATTACKING() {
            // guid: PackedGuid
            let guid = Guid::read_packed(r)?;

            Some(MovementBlockUpdateFlagMELEE_ATTACKING {
                guid,
            })
        }
        else {
            None
        };

        let update_flag_TRANSPORT = if update_flag.is_TRANSPORT() {
            // transport_progress_in_ms: u32
            let transport_progress_in_ms = crate::util::read_u32_le(r)?;

            Some(MovementBlockUpdateFlagTRANSPORT {
                transport_progress_in_ms,
            })
        }
        else {
            None
        };

        let update_flag = MovementBlockUpdateFlag {
            inner: update_flag.as_int(),
            transport: update_flag_TRANSPORT,
            melee_attacking: update_flag_MELEE_ATTACKING,
            high_guid: update_flag_HIGH_GUID,
            all: update_flag_ALL,
            living: update_flag_LIVING,
        };

        Ok(Self {
            update_flag,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // update_flag: UpdateFlag
        self.update_flag.write(w)?;

        if let Some(if_statement) = &self.update_flag.living {
            match if_statement {
                MovementBlockUpdateFlagLIVING::LIVING {
                    backwards_running_speed,
                    backwards_swimming_speed,
                    fall_time,
                    flags,
                    living_orientation,
                    living_position_x,
                    living_position_y,
                    living_position_z,
                    running_speed,
                    swimming_speed,
                    timestamp,
                    turn_rate,
                    walking_speed,
                } => {
                    // flags: MovementFlags
                    flags.write(w)?;

                    // timestamp: u32
                    w.write_all(&timestamp.to_le_bytes())?;

                    // living_position_x: f32
                    w.write_all(&living_position_x.to_le_bytes())?;

                    // living_position_y: f32
                    w.write_all(&living_position_y.to_le_bytes())?;

                    // living_position_z: f32
                    w.write_all(&living_position_z.to_le_bytes())?;

                    // living_orientation: f32
                    w.write_all(&living_orientation.to_le_bytes())?;

                    if let Some(if_statement) = &flags.on_transport {
                        // transport: TransportInfo
                        if_statement.transport.write(w)?;

                    }

                    if let Some(if_statement) = &flags.swimming {
                        // pitch: f32
                        w.write_all(&if_statement.pitch.to_le_bytes())?;

                    }

                    // fall_time: f32
                    w.write_all(&fall_time.to_le_bytes())?;

                    if let Some(if_statement) = &flags.jumping {
                        // z_speed: f32
                        w.write_all(&if_statement.z_speed.to_le_bytes())?;

                        // cos_angle: f32
                        w.write_all(&if_statement.cos_angle.to_le_bytes())?;

                        // sin_angle: f32
                        w.write_all(&if_statement.sin_angle.to_le_bytes())?;

                        // xy_speed: f32
                        w.write_all(&if_statement.xy_speed.to_le_bytes())?;

                    }

                    if let Some(if_statement) = &flags.spline_elevation {
                        // spline_elevation: f32
                        w.write_all(&if_statement.spline_elevation.to_le_bytes())?;

                    }

                    // walking_speed: f32
                    w.write_all(&walking_speed.to_le_bytes())?;

                    // running_speed: f32
                    w.write_all(&running_speed.to_le_bytes())?;

                    // backwards_running_speed: f32
                    w.write_all(&backwards_running_speed.to_le_bytes())?;

                    // swimming_speed: f32
                    w.write_all(&swimming_speed.to_le_bytes())?;

                    // backwards_swimming_speed: f32
                    w.write_all(&backwards_swimming_speed.to_le_bytes())?;

                    // turn_rate: f32
                    w.write_all(&turn_rate.to_le_bytes())?;

                    if let Some(if_statement) = &flags.spline_enabled {
                        // spline_flags: SplineFlag
                        if_statement.spline_flags.write(w)?;

                        if let Some(if_statement) = &if_statement.spline_flags.final_angle {
                            match if_statement {
                                MovementBlockSplineFlagFINAL_ANGLE::FINAL_ANGLE {
                                    angle,
                                } => {
                                    // angle: f32
                                    w.write_all(&angle.to_le_bytes())?;

                                }
                                MovementBlockSplineFlagFINAL_ANGLE::FINAL_TARGET {
                                    target,
                                } => {
                                    // target: u64
                                    w.write_all(&target.to_le_bytes())?;

                                }
                                MovementBlockSplineFlagFINAL_ANGLE::FINAL_POINT {
                                    spline_final_point_x,
                                    spline_final_point_y,
                                    spline_final_point_z,
                                } => {
                                    // spline_final_point_x: f32
                                    w.write_all(&spline_final_point_x.to_le_bytes())?;

                                    // spline_final_point_y: f32
                                    w.write_all(&spline_final_point_y.to_le_bytes())?;

                                    // spline_final_point_z: f32
                                    w.write_all(&spline_final_point_z.to_le_bytes())?;

                                }
                            }
                        }

                        // time_passed: u32
                        w.write_all(&if_statement.time_passed.to_le_bytes())?;

                        // duration: u32
                        w.write_all(&if_statement.duration.to_le_bytes())?;

                        // id: u32
                        w.write_all(&if_statement.id.to_le_bytes())?;

                        // amount_of_nodes: u32
                        w.write_all(&(if_statement.nodes.len() as u32).to_le_bytes())?;

                        // nodes: Vector3d[amount_of_nodes]
                        for i in if_statement.nodes.iter() {
                            i.write(w)?;
                        }

                        // final_node: Vector3d
                        if_statement.final_node.write(w)?;

                    }

                }
                MovementBlockUpdateFlagLIVING::HAS_POSITION {
                    orientation,
                    position_x,
                    position_y,
                    position_z,
                } => {
                    // position_x: f32
                    w.write_all(&position_x.to_le_bytes())?;

                    // position_y: f32
                    w.write_all(&position_y.to_le_bytes())?;

                    // position_z: f32
                    w.write_all(&position_z.to_le_bytes())?;

                    // orientation: f32
                    w.write_all(&orientation.to_le_bytes())?;

                }
            }
        }

        if let Some(if_statement) = &self.update_flag.high_guid {
            // unknown0: u32
            w.write_all(&Self::UNKNOWN0_VALUE.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.update_flag.all {
            // unknown1: u32
            w.write_all(&Self::UNKNOWN1_VALUE.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.update_flag.melee_attacking {
            // guid: PackedGuid
            if_statement.guid.write_packed(w)?;

        }

        if let Some(if_statement) = &self.update_flag.transport {
            // transport_progress_in_ms: u32
            w.write_all(&if_statement.transport_progress_in_ms.to_le_bytes())?;

        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // update_flag: UpdateFlag
            let update_flag = UpdateFlag::tokio_read(r).await?;

            let update_flag_LIVING = if update_flag.is_LIVING() {
                // flags: MovementFlags
                let flags = MovementFlags::tokio_read(r).await?;

                // timestamp: u32
                let timestamp = crate::util::tokio_read_u32_le(r).await?;

                // living_position_x: f32
                let living_position_x = crate::util::tokio_read_f32_le(r).await?;
                // living_position_y: f32
                let living_position_y = crate::util::tokio_read_f32_le(r).await?;
                // living_position_z: f32
                let living_position_z = crate::util::tokio_read_f32_le(r).await?;
                // living_orientation: f32
                let living_orientation = crate::util::tokio_read_f32_le(r).await?;
                let flags_ON_TRANSPORT = if flags.is_ON_TRANSPORT() {
                    // transport: TransportInfo
                    let transport = TransportInfo::tokio_read(r).await?;

                    Some(MovementBlockMovementFlagsON_TRANSPORT {
                        transport,
                    })
                }
                else {
                    None
                };

                let flags_SWIMMING = if flags.is_SWIMMING() {
                    // pitch: f32
                    let pitch = crate::util::tokio_read_f32_le(r).await?;
                    Some(MovementBlockMovementFlagsSWIMMING {
                        pitch,
                    })
                }
                else {
                    None
                };

                // fall_time: f32
                let fall_time = crate::util::tokio_read_f32_le(r).await?;
                let flags_JUMPING = if flags.is_JUMPING() {
                    // z_speed: f32
                    let z_speed = crate::util::tokio_read_f32_le(r).await?;
                    // cos_angle: f32
                    let cos_angle = crate::util::tokio_read_f32_le(r).await?;
                    // sin_angle: f32
                    let sin_angle = crate::util::tokio_read_f32_le(r).await?;
                    // xy_speed: f32
                    let xy_speed = crate::util::tokio_read_f32_le(r).await?;
                    Some(MovementBlockMovementFlagsJUMPING {
                        cos_angle,
                        sin_angle,
                        xy_speed,
                        z_speed,
                    })
                }
                else {
                    None
                };

                let flags_SPLINE_ELEVATION = if flags.is_SPLINE_ELEVATION() {
                    // spline_elevation: f32
                    let spline_elevation = crate::util::tokio_read_f32_le(r).await?;
                    Some(MovementBlockMovementFlagsSPLINE_ELEVATION {
                        spline_elevation,
                    })
                }
                else {
                    None
                };

                // walking_speed: f32
                let walking_speed = crate::util::tokio_read_f32_le(r).await?;
                // running_speed: f32
                let running_speed = crate::util::tokio_read_f32_le(r).await?;
                // backwards_running_speed: f32
                let backwards_running_speed = crate::util::tokio_read_f32_le(r).await?;
                // swimming_speed: f32
                let swimming_speed = crate::util::tokio_read_f32_le(r).await?;
                // backwards_swimming_speed: f32
                let backwards_swimming_speed = crate::util::tokio_read_f32_le(r).await?;
                // turn_rate: f32
                let turn_rate = crate::util::tokio_read_f32_le(r).await?;
                let flags_SPLINE_ENABLED = if flags.is_SPLINE_ENABLED() {
                    // spline_flags: SplineFlag
                    let spline_flags = SplineFlag::tokio_read(r).await?;

                    let spline_flags_FINAL_ANGLE = if spline_flags.is_FINAL_ANGLE() {
                        // angle: f32
                        let angle = crate::util::tokio_read_f32_le(r).await?;
                        Some(MovementBlockSplineFlagFINAL_ANGLE::FINAL_ANGLE {
                            angle,
                        })
                    }
                    else if spline_flags.is_FINAL_TARGET() {
                        // target: u64
                        let target = crate::util::tokio_read_u64_le(r).await?;

                        Some(MovementBlockSplineFlagFINAL_ANGLE::FINAL_TARGET {
                            target,
                        })
                    }
                    else if spline_flags.is_FINAL_POINT() {
                        // spline_final_point_x: f32
                        let spline_final_point_x = crate::util::tokio_read_f32_le(r).await?;
                        // spline_final_point_y: f32
                        let spline_final_point_y = crate::util::tokio_read_f32_le(r).await?;
                        // spline_final_point_z: f32
                        let spline_final_point_z = crate::util::tokio_read_f32_le(r).await?;
                        Some(MovementBlockSplineFlagFINAL_ANGLE::FINAL_POINT {
                            spline_final_point_x,
                            spline_final_point_y,
                            spline_final_point_z,
                        })
                    }
                    else {
                        None
                    };

                    // time_passed: u32
                    let time_passed = crate::util::tokio_read_u32_le(r).await?;

                    // duration: u32
                    let duration = crate::util::tokio_read_u32_le(r).await?;

                    // id: u32
                    let id = crate::util::tokio_read_u32_le(r).await?;

                    // amount_of_nodes: u32
                    let amount_of_nodes = crate::util::tokio_read_u32_le(r).await?;

                    // nodes: Vector3d[amount_of_nodes]
                    let mut nodes = Vec::with_capacity(amount_of_nodes as usize);
                    for i in 0..amount_of_nodes {
                        nodes.push(Vector3d::tokio_read(r).await?);
                    }

                    // final_node: Vector3d
                    let final_node = Vector3d::tokio_read(r).await?;

                    let spline_flags = MovementBlockSplineFlag {
                        inner: spline_flags.as_int(),
                        final_angle: spline_flags_FINAL_ANGLE,
                    };

                    Some(MovementBlockMovementFlagsSPLINE_ENABLED {
                        duration,
                        final_node,
                        id,
                        nodes,
                        spline_flags,
                        time_passed,
                    })
                }
                else {
                    None
                };

                let flags = MovementBlockMovementFlags {
                    inner: flags.as_int(),
                    on_transport: flags_ON_TRANSPORT,
                    jumping: flags_JUMPING,
                    swimming: flags_SWIMMING,
                    spline_enabled: flags_SPLINE_ENABLED,
                    spline_elevation: flags_SPLINE_ELEVATION,
                };

                Some(MovementBlockUpdateFlagLIVING::LIVING {
                    backwards_running_speed,
                    backwards_swimming_speed,
                    fall_time,
                    flags,
                    living_orientation,
                    living_position_x,
                    living_position_y,
                    living_position_z,
                    running_speed,
                    swimming_speed,
                    timestamp,
                    turn_rate,
                    walking_speed,
                })
            }
            else if update_flag.is_HAS_POSITION() {
                // position_x: f32
                let position_x = crate::util::tokio_read_f32_le(r).await?;
                // position_y: f32
                let position_y = crate::util::tokio_read_f32_le(r).await?;
                // position_z: f32
                let position_z = crate::util::tokio_read_f32_le(r).await?;
                // orientation: f32
                let orientation = crate::util::tokio_read_f32_le(r).await?;
                Some(MovementBlockUpdateFlagLIVING::HAS_POSITION {
                    orientation,
                    position_x,
                    position_y,
                    position_z,
                })
            }
            else {
                None
            };

            let update_flag_HIGH_GUID = if update_flag.is_HIGH_GUID() {
                // unknown0: u32
                let _unknown0 = crate::util::tokio_read_u32_le(r).await?;
                // unknown0 is expected to always be 0 (0)

                Some(MovementBlockUpdateFlagHIGH_GUID {
                })
            }
            else {
                None
            };

            let update_flag_ALL = if update_flag.is_ALL() {
                // unknown1: u32
                let _unknown1 = crate::util::tokio_read_u32_le(r).await?;
                // unknown1 is expected to always be 1 (1)

                Some(MovementBlockUpdateFlagALL {
                })
            }
            else {
                None
            };

            let update_flag_MELEE_ATTACKING = if update_flag.is_MELEE_ATTACKING() {
                // guid: PackedGuid
                let guid = Guid::tokio_read_packed(r).await?;

                Some(MovementBlockUpdateFlagMELEE_ATTACKING {
                    guid,
                })
            }
            else {
                None
            };

            let update_flag_TRANSPORT = if update_flag.is_TRANSPORT() {
                // transport_progress_in_ms: u32
                let transport_progress_in_ms = crate::util::tokio_read_u32_le(r).await?;

                Some(MovementBlockUpdateFlagTRANSPORT {
                    transport_progress_in_ms,
                })
            }
            else {
                None
            };

            let update_flag = MovementBlockUpdateFlag {
                inner: update_flag.as_int(),
                transport: update_flag_TRANSPORT,
                melee_attacking: update_flag_MELEE_ATTACKING,
                high_guid: update_flag_HIGH_GUID,
                all: update_flag_ALL,
                living: update_flag_LIVING,
            };

            Ok(Self {
                update_flag,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
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
            // update_flag: UpdateFlag
            self.update_flag.tokio_write(w).await?;

            if let Some(if_statement) = &self.update_flag.living {
                match if_statement {
                    MovementBlockUpdateFlagLIVING::LIVING {
                        backwards_running_speed,
                        backwards_swimming_speed,
                        fall_time,
                        flags,
                        living_orientation,
                        living_position_x,
                        living_position_y,
                        living_position_z,
                        running_speed,
                        swimming_speed,
                        timestamp,
                        turn_rate,
                        walking_speed,
                    } => {
                        // flags: MovementFlags
                        flags.tokio_write(w).await?;

                        // timestamp: u32
                        w.write_all(&timestamp.to_le_bytes()).await?;

                        // living_position_x: f32
                        w.write_all(&living_position_x.to_le_bytes()).await?;

                        // living_position_y: f32
                        w.write_all(&living_position_y.to_le_bytes()).await?;

                        // living_position_z: f32
                        w.write_all(&living_position_z.to_le_bytes()).await?;

                        // living_orientation: f32
                        w.write_all(&living_orientation.to_le_bytes()).await?;

                        if let Some(if_statement) = &flags.on_transport {
                            // transport: TransportInfo
                            if_statement.transport.tokio_write(w).await?;

                        }

                        if let Some(if_statement) = &flags.swimming {
                            // pitch: f32
                            w.write_all(&if_statement.pitch.to_le_bytes()).await?;

                        }

                        // fall_time: f32
                        w.write_all(&fall_time.to_le_bytes()).await?;

                        if let Some(if_statement) = &flags.jumping {
                            // z_speed: f32
                            w.write_all(&if_statement.z_speed.to_le_bytes()).await?;

                            // cos_angle: f32
                            w.write_all(&if_statement.cos_angle.to_le_bytes()).await?;

                            // sin_angle: f32
                            w.write_all(&if_statement.sin_angle.to_le_bytes()).await?;

                            // xy_speed: f32
                            w.write_all(&if_statement.xy_speed.to_le_bytes()).await?;

                        }

                        if let Some(if_statement) = &flags.spline_elevation {
                            // spline_elevation: f32
                            w.write_all(&if_statement.spline_elevation.to_le_bytes()).await?;

                        }

                        // walking_speed: f32
                        w.write_all(&walking_speed.to_le_bytes()).await?;

                        // running_speed: f32
                        w.write_all(&running_speed.to_le_bytes()).await?;

                        // backwards_running_speed: f32
                        w.write_all(&backwards_running_speed.to_le_bytes()).await?;

                        // swimming_speed: f32
                        w.write_all(&swimming_speed.to_le_bytes()).await?;

                        // backwards_swimming_speed: f32
                        w.write_all(&backwards_swimming_speed.to_le_bytes()).await?;

                        // turn_rate: f32
                        w.write_all(&turn_rate.to_le_bytes()).await?;

                        if let Some(if_statement) = &flags.spline_enabled {
                            // spline_flags: SplineFlag
                            if_statement.spline_flags.tokio_write(w).await?;

                            if let Some(if_statement) = &if_statement.spline_flags.final_angle {
                                match if_statement {
                                    MovementBlockSplineFlagFINAL_ANGLE::FINAL_ANGLE {
                                        angle,
                                    } => {
                                        // angle: f32
                                        w.write_all(&angle.to_le_bytes()).await?;

                                    }
                                    MovementBlockSplineFlagFINAL_ANGLE::FINAL_TARGET {
                                        target,
                                    } => {
                                        // target: u64
                                        w.write_all(&target.to_le_bytes()).await?;

                                    }
                                    MovementBlockSplineFlagFINAL_ANGLE::FINAL_POINT {
                                        spline_final_point_x,
                                        spline_final_point_y,
                                        spline_final_point_z,
                                    } => {
                                        // spline_final_point_x: f32
                                        w.write_all(&spline_final_point_x.to_le_bytes()).await?;

                                        // spline_final_point_y: f32
                                        w.write_all(&spline_final_point_y.to_le_bytes()).await?;

                                        // spline_final_point_z: f32
                                        w.write_all(&spline_final_point_z.to_le_bytes()).await?;

                                    }
                                }
                            }

                            // time_passed: u32
                            w.write_all(&if_statement.time_passed.to_le_bytes()).await?;

                            // duration: u32
                            w.write_all(&if_statement.duration.to_le_bytes()).await?;

                            // id: u32
                            w.write_all(&if_statement.id.to_le_bytes()).await?;

                            // amount_of_nodes: u32
                            w.write_all(&(if_statement.nodes.len() as u32).to_le_bytes()).await?;

                            // nodes: Vector3d[amount_of_nodes]
                            for i in if_statement.nodes.iter() {
                                i.tokio_write(w).await?;
                            }

                            // final_node: Vector3d
                            if_statement.final_node.tokio_write(w).await?;

                        }

                    }
                    MovementBlockUpdateFlagLIVING::HAS_POSITION {
                        orientation,
                        position_x,
                        position_y,
                        position_z,
                    } => {
                        // position_x: f32
                        w.write_all(&position_x.to_le_bytes()).await?;

                        // position_y: f32
                        w.write_all(&position_y.to_le_bytes()).await?;

                        // position_z: f32
                        w.write_all(&position_z.to_le_bytes()).await?;

                        // orientation: f32
                        w.write_all(&orientation.to_le_bytes()).await?;

                    }
                }
            }

            if let Some(if_statement) = &self.update_flag.high_guid {
                // unknown0: u32
                w.write_all(&Self::UNKNOWN0_VALUE.to_le_bytes()).await?;

            }

            if let Some(if_statement) = &self.update_flag.all {
                // unknown1: u32
                w.write_all(&Self::UNKNOWN1_VALUE.to_le_bytes()).await?;

            }

            if let Some(if_statement) = &self.update_flag.melee_attacking {
                // guid: PackedGuid
                if_statement.guid.tokio_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.update_flag.transport {
                // transport_progress_in_ms: u32
                w.write_all(&if_statement.transport_progress_in_ms.to_le_bytes()).await?;

            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // update_flag: UpdateFlag
            let update_flag = UpdateFlag::astd_read(r).await?;

            let update_flag_LIVING = if update_flag.is_LIVING() {
                // flags: MovementFlags
                let flags = MovementFlags::astd_read(r).await?;

                // timestamp: u32
                let timestamp = crate::util::astd_read_u32_le(r).await?;

                // living_position_x: f32
                let living_position_x = crate::util::astd_read_f32_le(r).await?;
                // living_position_y: f32
                let living_position_y = crate::util::astd_read_f32_le(r).await?;
                // living_position_z: f32
                let living_position_z = crate::util::astd_read_f32_le(r).await?;
                // living_orientation: f32
                let living_orientation = crate::util::astd_read_f32_le(r).await?;
                let flags_ON_TRANSPORT = if flags.is_ON_TRANSPORT() {
                    // transport: TransportInfo
                    let transport = TransportInfo::astd_read(r).await?;

                    Some(MovementBlockMovementFlagsON_TRANSPORT {
                        transport,
                    })
                }
                else {
                    None
                };

                let flags_SWIMMING = if flags.is_SWIMMING() {
                    // pitch: f32
                    let pitch = crate::util::astd_read_f32_le(r).await?;
                    Some(MovementBlockMovementFlagsSWIMMING {
                        pitch,
                    })
                }
                else {
                    None
                };

                // fall_time: f32
                let fall_time = crate::util::astd_read_f32_le(r).await?;
                let flags_JUMPING = if flags.is_JUMPING() {
                    // z_speed: f32
                    let z_speed = crate::util::astd_read_f32_le(r).await?;
                    // cos_angle: f32
                    let cos_angle = crate::util::astd_read_f32_le(r).await?;
                    // sin_angle: f32
                    let sin_angle = crate::util::astd_read_f32_le(r).await?;
                    // xy_speed: f32
                    let xy_speed = crate::util::astd_read_f32_le(r).await?;
                    Some(MovementBlockMovementFlagsJUMPING {
                        cos_angle,
                        sin_angle,
                        xy_speed,
                        z_speed,
                    })
                }
                else {
                    None
                };

                let flags_SPLINE_ELEVATION = if flags.is_SPLINE_ELEVATION() {
                    // spline_elevation: f32
                    let spline_elevation = crate::util::astd_read_f32_le(r).await?;
                    Some(MovementBlockMovementFlagsSPLINE_ELEVATION {
                        spline_elevation,
                    })
                }
                else {
                    None
                };

                // walking_speed: f32
                let walking_speed = crate::util::astd_read_f32_le(r).await?;
                // running_speed: f32
                let running_speed = crate::util::astd_read_f32_le(r).await?;
                // backwards_running_speed: f32
                let backwards_running_speed = crate::util::astd_read_f32_le(r).await?;
                // swimming_speed: f32
                let swimming_speed = crate::util::astd_read_f32_le(r).await?;
                // backwards_swimming_speed: f32
                let backwards_swimming_speed = crate::util::astd_read_f32_le(r).await?;
                // turn_rate: f32
                let turn_rate = crate::util::astd_read_f32_le(r).await?;
                let flags_SPLINE_ENABLED = if flags.is_SPLINE_ENABLED() {
                    // spline_flags: SplineFlag
                    let spline_flags = SplineFlag::astd_read(r).await?;

                    let spline_flags_FINAL_ANGLE = if spline_flags.is_FINAL_ANGLE() {
                        // angle: f32
                        let angle = crate::util::astd_read_f32_le(r).await?;
                        Some(MovementBlockSplineFlagFINAL_ANGLE::FINAL_ANGLE {
                            angle,
                        })
                    }
                    else if spline_flags.is_FINAL_TARGET() {
                        // target: u64
                        let target = crate::util::astd_read_u64_le(r).await?;

                        Some(MovementBlockSplineFlagFINAL_ANGLE::FINAL_TARGET {
                            target,
                        })
                    }
                    else if spline_flags.is_FINAL_POINT() {
                        // spline_final_point_x: f32
                        let spline_final_point_x = crate::util::astd_read_f32_le(r).await?;
                        // spline_final_point_y: f32
                        let spline_final_point_y = crate::util::astd_read_f32_le(r).await?;
                        // spline_final_point_z: f32
                        let spline_final_point_z = crate::util::astd_read_f32_le(r).await?;
                        Some(MovementBlockSplineFlagFINAL_ANGLE::FINAL_POINT {
                            spline_final_point_x,
                            spline_final_point_y,
                            spline_final_point_z,
                        })
                    }
                    else {
                        None
                    };

                    // time_passed: u32
                    let time_passed = crate::util::astd_read_u32_le(r).await?;

                    // duration: u32
                    let duration = crate::util::astd_read_u32_le(r).await?;

                    // id: u32
                    let id = crate::util::astd_read_u32_le(r).await?;

                    // amount_of_nodes: u32
                    let amount_of_nodes = crate::util::astd_read_u32_le(r).await?;

                    // nodes: Vector3d[amount_of_nodes]
                    let mut nodes = Vec::with_capacity(amount_of_nodes as usize);
                    for i in 0..amount_of_nodes {
                        nodes.push(Vector3d::astd_read(r).await?);
                    }

                    // final_node: Vector3d
                    let final_node = Vector3d::astd_read(r).await?;

                    let spline_flags = MovementBlockSplineFlag {
                        inner: spline_flags.as_int(),
                        final_angle: spline_flags_FINAL_ANGLE,
                    };

                    Some(MovementBlockMovementFlagsSPLINE_ENABLED {
                        duration,
                        final_node,
                        id,
                        nodes,
                        spline_flags,
                        time_passed,
                    })
                }
                else {
                    None
                };

                let flags = MovementBlockMovementFlags {
                    inner: flags.as_int(),
                    on_transport: flags_ON_TRANSPORT,
                    jumping: flags_JUMPING,
                    swimming: flags_SWIMMING,
                    spline_enabled: flags_SPLINE_ENABLED,
                    spline_elevation: flags_SPLINE_ELEVATION,
                };

                Some(MovementBlockUpdateFlagLIVING::LIVING {
                    backwards_running_speed,
                    backwards_swimming_speed,
                    fall_time,
                    flags,
                    living_orientation,
                    living_position_x,
                    living_position_y,
                    living_position_z,
                    running_speed,
                    swimming_speed,
                    timestamp,
                    turn_rate,
                    walking_speed,
                })
            }
            else if update_flag.is_HAS_POSITION() {
                // position_x: f32
                let position_x = crate::util::astd_read_f32_le(r).await?;
                // position_y: f32
                let position_y = crate::util::astd_read_f32_le(r).await?;
                // position_z: f32
                let position_z = crate::util::astd_read_f32_le(r).await?;
                // orientation: f32
                let orientation = crate::util::astd_read_f32_le(r).await?;
                Some(MovementBlockUpdateFlagLIVING::HAS_POSITION {
                    orientation,
                    position_x,
                    position_y,
                    position_z,
                })
            }
            else {
                None
            };

            let update_flag_HIGH_GUID = if update_flag.is_HIGH_GUID() {
                // unknown0: u32
                let _unknown0 = crate::util::astd_read_u32_le(r).await?;
                // unknown0 is expected to always be 0 (0)

                Some(MovementBlockUpdateFlagHIGH_GUID {
                })
            }
            else {
                None
            };

            let update_flag_ALL = if update_flag.is_ALL() {
                // unknown1: u32
                let _unknown1 = crate::util::astd_read_u32_le(r).await?;
                // unknown1 is expected to always be 1 (1)

                Some(MovementBlockUpdateFlagALL {
                })
            }
            else {
                None
            };

            let update_flag_MELEE_ATTACKING = if update_flag.is_MELEE_ATTACKING() {
                // guid: PackedGuid
                let guid = Guid::astd_read_packed(r).await?;

                Some(MovementBlockUpdateFlagMELEE_ATTACKING {
                    guid,
                })
            }
            else {
                None
            };

            let update_flag_TRANSPORT = if update_flag.is_TRANSPORT() {
                // transport_progress_in_ms: u32
                let transport_progress_in_ms = crate::util::astd_read_u32_le(r).await?;

                Some(MovementBlockUpdateFlagTRANSPORT {
                    transport_progress_in_ms,
                })
            }
            else {
                None
            };

            let update_flag = MovementBlockUpdateFlag {
                inner: update_flag.as_int(),
                transport: update_flag_TRANSPORT,
                melee_attacking: update_flag_MELEE_ATTACKING,
                high_guid: update_flag_HIGH_GUID,
                all: update_flag_ALL,
                living: update_flag_LIVING,
            };

            Ok(Self {
                update_flag,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
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
            // update_flag: UpdateFlag
            self.update_flag.astd_write(w).await?;

            if let Some(if_statement) = &self.update_flag.living {
                match if_statement {
                    MovementBlockUpdateFlagLIVING::LIVING {
                        backwards_running_speed,
                        backwards_swimming_speed,
                        fall_time,
                        flags,
                        living_orientation,
                        living_position_x,
                        living_position_y,
                        living_position_z,
                        running_speed,
                        swimming_speed,
                        timestamp,
                        turn_rate,
                        walking_speed,
                    } => {
                        // flags: MovementFlags
                        flags.astd_write(w).await?;

                        // timestamp: u32
                        w.write_all(&timestamp.to_le_bytes()).await?;

                        // living_position_x: f32
                        w.write_all(&living_position_x.to_le_bytes()).await?;

                        // living_position_y: f32
                        w.write_all(&living_position_y.to_le_bytes()).await?;

                        // living_position_z: f32
                        w.write_all(&living_position_z.to_le_bytes()).await?;

                        // living_orientation: f32
                        w.write_all(&living_orientation.to_le_bytes()).await?;

                        if let Some(if_statement) = &flags.on_transport {
                            // transport: TransportInfo
                            if_statement.transport.astd_write(w).await?;

                        }

                        if let Some(if_statement) = &flags.swimming {
                            // pitch: f32
                            w.write_all(&if_statement.pitch.to_le_bytes()).await?;

                        }

                        // fall_time: f32
                        w.write_all(&fall_time.to_le_bytes()).await?;

                        if let Some(if_statement) = &flags.jumping {
                            // z_speed: f32
                            w.write_all(&if_statement.z_speed.to_le_bytes()).await?;

                            // cos_angle: f32
                            w.write_all(&if_statement.cos_angle.to_le_bytes()).await?;

                            // sin_angle: f32
                            w.write_all(&if_statement.sin_angle.to_le_bytes()).await?;

                            // xy_speed: f32
                            w.write_all(&if_statement.xy_speed.to_le_bytes()).await?;

                        }

                        if let Some(if_statement) = &flags.spline_elevation {
                            // spline_elevation: f32
                            w.write_all(&if_statement.spline_elevation.to_le_bytes()).await?;

                        }

                        // walking_speed: f32
                        w.write_all(&walking_speed.to_le_bytes()).await?;

                        // running_speed: f32
                        w.write_all(&running_speed.to_le_bytes()).await?;

                        // backwards_running_speed: f32
                        w.write_all(&backwards_running_speed.to_le_bytes()).await?;

                        // swimming_speed: f32
                        w.write_all(&swimming_speed.to_le_bytes()).await?;

                        // backwards_swimming_speed: f32
                        w.write_all(&backwards_swimming_speed.to_le_bytes()).await?;

                        // turn_rate: f32
                        w.write_all(&turn_rate.to_le_bytes()).await?;

                        if let Some(if_statement) = &flags.spline_enabled {
                            // spline_flags: SplineFlag
                            if_statement.spline_flags.astd_write(w).await?;

                            if let Some(if_statement) = &if_statement.spline_flags.final_angle {
                                match if_statement {
                                    MovementBlockSplineFlagFINAL_ANGLE::FINAL_ANGLE {
                                        angle,
                                    } => {
                                        // angle: f32
                                        w.write_all(&angle.to_le_bytes()).await?;

                                    }
                                    MovementBlockSplineFlagFINAL_ANGLE::FINAL_TARGET {
                                        target,
                                    } => {
                                        // target: u64
                                        w.write_all(&target.to_le_bytes()).await?;

                                    }
                                    MovementBlockSplineFlagFINAL_ANGLE::FINAL_POINT {
                                        spline_final_point_x,
                                        spline_final_point_y,
                                        spline_final_point_z,
                                    } => {
                                        // spline_final_point_x: f32
                                        w.write_all(&spline_final_point_x.to_le_bytes()).await?;

                                        // spline_final_point_y: f32
                                        w.write_all(&spline_final_point_y.to_le_bytes()).await?;

                                        // spline_final_point_z: f32
                                        w.write_all(&spline_final_point_z.to_le_bytes()).await?;

                                    }
                                }
                            }

                            // time_passed: u32
                            w.write_all(&if_statement.time_passed.to_le_bytes()).await?;

                            // duration: u32
                            w.write_all(&if_statement.duration.to_le_bytes()).await?;

                            // id: u32
                            w.write_all(&if_statement.id.to_le_bytes()).await?;

                            // amount_of_nodes: u32
                            w.write_all(&(if_statement.nodes.len() as u32).to_le_bytes()).await?;

                            // nodes: Vector3d[amount_of_nodes]
                            for i in if_statement.nodes.iter() {
                                i.astd_write(w).await?;
                            }

                            // final_node: Vector3d
                            if_statement.final_node.astd_write(w).await?;

                        }

                    }
                    MovementBlockUpdateFlagLIVING::HAS_POSITION {
                        orientation,
                        position_x,
                        position_y,
                        position_z,
                    } => {
                        // position_x: f32
                        w.write_all(&position_x.to_le_bytes()).await?;

                        // position_y: f32
                        w.write_all(&position_y.to_le_bytes()).await?;

                        // position_z: f32
                        w.write_all(&position_z.to_le_bytes()).await?;

                        // orientation: f32
                        w.write_all(&orientation.to_le_bytes()).await?;

                    }
                }
            }

            if let Some(if_statement) = &self.update_flag.high_guid {
                // unknown0: u32
                w.write_all(&Self::UNKNOWN0_VALUE.to_le_bytes()).await?;

            }

            if let Some(if_statement) = &self.update_flag.all {
                // unknown1: u32
                w.write_all(&Self::UNKNOWN1_VALUE.to_le_bytes()).await?;

            }

            if let Some(if_statement) = &self.update_flag.melee_attacking {
                // guid: PackedGuid
                if_statement.guid.astd_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.update_flag.transport {
                // transport_progress_in_ms: u32
                w.write_all(&if_statement.transport_progress_in_ms.to_le_bytes()).await?;

            }

            Ok(())
        })
    }

}

impl VariableSized for MovementBlock {
    fn size(&self) -> usize {
        0
        + self.update_flag.size() // update_flag: MovementBlockUpdateFlag
    }
}

impl MaximumPossibleSized for MovementBlock {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MovementBlockSplineFlagFINAL_ANGLE {
    FINAL_ANGLE {
        angle: f32,
    },
    FINAL_TARGET {
        target: u64,
    },
    FINAL_POINT {
        spline_final_point_x: f32,
        spline_final_point_y: f32,
        spline_final_point_z: f32,
    },
}

impl MovementBlockSplineFlagFINAL_ANGLE {
    pub(crate) const fn as_flag_value(&self) -> u32 {
        match self {
            Self::FINAL_ANGLE { .. } => 262144,
            Self::FINAL_TARGET { .. } => 131072,
            Self::FINAL_POINT { .. } => 65536,
        }
    }

}

impl VariableSized for MovementBlockSplineFlagFINAL_ANGLE {
    fn size(&self) -> usize {
        match self {
            Self::FINAL_ANGLE {
                angle,
            } => {
                0 // Not an actual enum sent over the wire
                + 4 // angle: f32
            }
            Self::FINAL_TARGET {
                target,
            } => {
                0 // Not an actual enum sent over the wire
                + 8 // target: u64
            }
            Self::FINAL_POINT {
                spline_final_point_x,
                spline_final_point_y,
                spline_final_point_z,
            } => {
                0 // Not an actual enum sent over the wire
                + 4 // spline_final_point_x: f32
                + 4 // spline_final_point_y: f32
                + 4 // spline_final_point_z: f32
            }
        }
    }
}

impl MaximumPossibleSized for MovementBlockSplineFlagFINAL_ANGLE {
    fn maximum_possible_size() -> usize {
        0
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct MovementBlockSplineFlag {
    inner: u32,
    final_angle: Option<MovementBlockSplineFlagFINAL_ANGLE>,
}

impl From<&MovementBlockSplineFlag> for SplineFlag {
    fn from(e: &MovementBlockSplineFlag) -> Self {
        Self::new(e.inner)
    }
}

impl MovementBlockSplineFlag {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SplineFlag = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SplineFlag = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SplineFlag = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            final_angle: None,
        }
    }

    pub const fn new_NONE() -> Self {
        Self {
            inner: SplineFlag::NONE,
            final_angle: None,
        }
    }

    pub fn set_NONE(&mut self) -> Self {
        self.inner |= SplineFlag::NONE;
        self.clone()
    }

    pub const fn get_NONE(&self) -> bool {
        // Underlying value is 0
        self.inner == SplineFlag::NONE
    }

    pub fn clear_NONE(&mut self) -> Self {
        self.inner &= SplineFlag::NONE.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_DONE() -> Self {
        Self {
            inner: SplineFlag::DONE,
            final_angle: None,
        }
    }

    pub fn set_DONE(&mut self) -> Self {
        self.inner |= SplineFlag::DONE;
        self.clone()
    }

    pub const fn get_DONE(&self) -> bool {
        (self.inner & SplineFlag::DONE) != 0
    }

    pub fn clear_DONE(&mut self) -> Self {
        self.inner &= SplineFlag::DONE.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_FALLING() -> Self {
        Self {
            inner: SplineFlag::FALLING,
            final_angle: None,
        }
    }

    pub fn set_FALLING(&mut self) -> Self {
        self.inner |= SplineFlag::FALLING;
        self.clone()
    }

    pub const fn get_FALLING(&self) -> bool {
        (self.inner & SplineFlag::FALLING) != 0
    }

    pub fn clear_FALLING(&mut self) -> Self {
        self.inner &= SplineFlag::FALLING.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN3() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN3,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN3(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN3;
        self.clone()
    }

    pub const fn get_UNKNOWN3(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN3) != 0
    }

    pub fn clear_UNKNOWN3(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN3.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN4() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN4,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN4(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN4;
        self.clone()
    }

    pub const fn get_UNKNOWN4(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN4) != 0
    }

    pub fn clear_UNKNOWN4(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN4.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN5() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN5,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN5(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN5;
        self.clone()
    }

    pub const fn get_UNKNOWN5(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN5) != 0
    }

    pub fn clear_UNKNOWN5(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN5.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN6() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN6,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN6(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN6;
        self.clone()
    }

    pub const fn get_UNKNOWN6(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN6) != 0
    }

    pub fn clear_UNKNOWN6(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN6.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN7() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN7,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN7(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN7;
        self.clone()
    }

    pub const fn get_UNKNOWN7(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN7) != 0
    }

    pub fn clear_UNKNOWN7(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN7.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN8() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN8,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN8(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN8;
        self.clone()
    }

    pub const fn get_UNKNOWN8(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN8) != 0
    }

    pub fn clear_UNKNOWN8(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN8.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_RUNMODE() -> Self {
        Self {
            inner: SplineFlag::RUNMODE,
            final_angle: None,
        }
    }

    pub fn set_RUNMODE(&mut self) -> Self {
        self.inner |= SplineFlag::RUNMODE;
        self.clone()
    }

    pub const fn get_RUNMODE(&self) -> bool {
        (self.inner & SplineFlag::RUNMODE) != 0
    }

    pub fn clear_RUNMODE(&mut self) -> Self {
        self.inner &= SplineFlag::RUNMODE.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_FLYING() -> Self {
        Self {
            inner: SplineFlag::FLYING,
            final_angle: None,
        }
    }

    pub fn set_FLYING(&mut self) -> Self {
        self.inner |= SplineFlag::FLYING;
        self.clone()
    }

    pub const fn get_FLYING(&self) -> bool {
        (self.inner & SplineFlag::FLYING) != 0
    }

    pub fn clear_FLYING(&mut self) -> Self {
        self.inner &= SplineFlag::FLYING.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_NO_SPLINE() -> Self {
        Self {
            inner: SplineFlag::NO_SPLINE,
            final_angle: None,
        }
    }

    pub fn set_NO_SPLINE(&mut self) -> Self {
        self.inner |= SplineFlag::NO_SPLINE;
        self.clone()
    }

    pub const fn get_NO_SPLINE(&self) -> bool {
        (self.inner & SplineFlag::NO_SPLINE) != 0
    }

    pub fn clear_NO_SPLINE(&mut self) -> Self {
        self.inner &= SplineFlag::NO_SPLINE.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN12() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN12,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN12(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN12;
        self.clone()
    }

    pub const fn get_UNKNOWN12(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN12) != 0
    }

    pub fn clear_UNKNOWN12(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN12.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN13() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN13,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN13(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN13;
        self.clone()
    }

    pub const fn get_UNKNOWN13(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN13) != 0
    }

    pub fn clear_UNKNOWN13(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN13.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN14() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN14,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN14(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN14;
        self.clone()
    }

    pub const fn get_UNKNOWN14(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN14) != 0
    }

    pub fn clear_UNKNOWN14(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN14.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN15() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN15,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN15(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN15;
        self.clone()
    }

    pub const fn get_UNKNOWN15(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN15) != 0
    }

    pub fn clear_UNKNOWN15(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN15.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN16() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN16,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN16(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN16;
        self.clone()
    }

    pub const fn get_UNKNOWN16(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN16) != 0
    }

    pub fn clear_UNKNOWN16(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN16.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_FINAL_ANGLE(final_angle: MovementBlockSplineFlagFINAL_ANGLE) -> Self {
        Self {
            inner: final_angle.as_flag_value(),
            final_angle: Some(final_angle),
        }
    }

    pub fn set_FINAL_ANGLE(&mut self, final_angle: MovementBlockSplineFlagFINAL_ANGLE) -> Self {
        self.inner |= final_angle.as_flag_value();
        self.final_angle = Some(final_angle);
        self.clone()
    }

    pub const fn get_FINAL_ANGLE(&self) -> Option<&MovementBlockSplineFlagFINAL_ANGLE> {
        self.final_angle.as_ref()
    }

    pub fn clear_FINAL_ANGLE(&mut self) -> Self {
        self.inner &= SplineFlag::FINAL_ANGLE.reverse_bits();
        self.final_angle = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN19() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN19,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN19(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN19;
        self.clone()
    }

    pub const fn get_UNKNOWN19(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN19) != 0
    }

    pub fn clear_UNKNOWN19(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN19.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_CYCLIC() -> Self {
        Self {
            inner: SplineFlag::CYCLIC,
            final_angle: None,
        }
    }

    pub fn set_CYCLIC(&mut self) -> Self {
        self.inner |= SplineFlag::CYCLIC;
        self.clone()
    }

    pub const fn get_CYCLIC(&self) -> bool {
        (self.inner & SplineFlag::CYCLIC) != 0
    }

    pub fn clear_CYCLIC(&mut self) -> Self {
        self.inner &= SplineFlag::CYCLIC.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_ENTER_CYCLE() -> Self {
        Self {
            inner: SplineFlag::ENTER_CYCLE,
            final_angle: None,
        }
    }

    pub fn set_ENTER_CYCLE(&mut self) -> Self {
        self.inner |= SplineFlag::ENTER_CYCLE;
        self.clone()
    }

    pub const fn get_ENTER_CYCLE(&self) -> bool {
        (self.inner & SplineFlag::ENTER_CYCLE) != 0
    }

    pub fn clear_ENTER_CYCLE(&mut self) -> Self {
        self.inner &= SplineFlag::ENTER_CYCLE.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_FROZEN() -> Self {
        Self {
            inner: SplineFlag::FROZEN,
            final_angle: None,
        }
    }

    pub fn set_FROZEN(&mut self) -> Self {
        self.inner |= SplineFlag::FROZEN;
        self.clone()
    }

    pub const fn get_FROZEN(&self) -> bool {
        (self.inner & SplineFlag::FROZEN) != 0
    }

    pub fn clear_FROZEN(&mut self) -> Self {
        self.inner &= SplineFlag::FROZEN.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN23() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN23,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN23(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN23;
        self.clone()
    }

    pub const fn get_UNKNOWN23(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN23) != 0
    }

    pub fn clear_UNKNOWN23(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN23.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN24() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN24,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN24(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN24;
        self.clone()
    }

    pub const fn get_UNKNOWN24(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN24) != 0
    }

    pub fn clear_UNKNOWN24(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN24.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN25() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN25,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN25(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN25;
        self.clone()
    }

    pub const fn get_UNKNOWN25(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN25) != 0
    }

    pub fn clear_UNKNOWN25(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN25.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN26() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN26,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN26(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN26;
        self.clone()
    }

    pub const fn get_UNKNOWN26(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN26) != 0
    }

    pub fn clear_UNKNOWN26(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN26.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN27() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN27,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN27(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN27;
        self.clone()
    }

    pub const fn get_UNKNOWN27(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN27) != 0
    }

    pub fn clear_UNKNOWN27(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN27.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN28() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN28,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN28(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN28;
        self.clone()
    }

    pub const fn get_UNKNOWN28(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN28) != 0
    }

    pub fn clear_UNKNOWN28(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN28.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN29() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN29,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN29(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN29;
        self.clone()
    }

    pub const fn get_UNKNOWN29(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN29) != 0
    }

    pub fn clear_UNKNOWN29(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN29.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN30() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN30,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN30(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN30;
        self.clone()
    }

    pub const fn get_UNKNOWN30(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN30) != 0
    }

    pub fn clear_UNKNOWN30(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN30.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN31() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN31,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN31(&mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN31;
        self.clone()
    }

    pub const fn get_UNKNOWN31(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN31) != 0
    }

    pub fn clear_UNKNOWN31(&mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN31.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

}
impl VariableSized for MovementBlockSplineFlag {
    fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.final_angle {
                s.size()
            } else {
                0
            }
        }
    }
}

impl MaximumPossibleSized for MovementBlockSplineFlag {
    fn maximum_possible_size() -> usize {
        4 // inner
        + MovementBlockSplineFlagFINAL_ANGLE::maximum_possible_size() // FINAL_ANGLE enumerator
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct MovementBlockMovementFlags {
    inner: u32,
    on_transport: Option<MovementBlockMovementFlagsON_TRANSPORT>,
    jumping: Option<MovementBlockMovementFlagsJUMPING>,
    swimming: Option<MovementBlockMovementFlagsSWIMMING>,
    spline_enabled: Option<MovementBlockMovementFlagsSPLINE_ENABLED>,
    spline_elevation: Option<MovementBlockMovementFlagsSPLINE_ELEVATION>,
}

impl From<&MovementBlockMovementFlags> for MovementFlags {
    fn from(e: &MovementBlockMovementFlags) -> Self {
        Self::new(e.inner)
    }
}

impl MovementBlockMovementFlags {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: MovementFlags = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: MovementFlags = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: MovementFlags = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub const fn new_NONE() -> Self {
        Self {
            inner: MovementFlags::NONE,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_NONE(&mut self) -> Self {
        self.inner |= MovementFlags::NONE;
        self.clone()
    }

    pub const fn get_NONE(&self) -> bool {
        // Underlying value is 0
        self.inner == MovementFlags::NONE
    }

    pub fn clear_NONE(&mut self) -> Self {
        self.inner &= MovementFlags::NONE.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_FORWARD() -> Self {
        Self {
            inner: MovementFlags::FORWARD,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_FORWARD(&mut self) -> Self {
        self.inner |= MovementFlags::FORWARD;
        self.clone()
    }

    pub const fn get_FORWARD(&self) -> bool {
        (self.inner & MovementFlags::FORWARD) != 0
    }

    pub fn clear_FORWARD(&mut self) -> Self {
        self.inner &= MovementFlags::FORWARD.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_BACKWARD() -> Self {
        Self {
            inner: MovementFlags::BACKWARD,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_BACKWARD(&mut self) -> Self {
        self.inner |= MovementFlags::BACKWARD;
        self.clone()
    }

    pub const fn get_BACKWARD(&self) -> bool {
        (self.inner & MovementFlags::BACKWARD) != 0
    }

    pub fn clear_BACKWARD(&mut self) -> Self {
        self.inner &= MovementFlags::BACKWARD.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_STRAFE_LEFT() -> Self {
        Self {
            inner: MovementFlags::STRAFE_LEFT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_STRAFE_LEFT(&mut self) -> Self {
        self.inner |= MovementFlags::STRAFE_LEFT;
        self.clone()
    }

    pub const fn get_STRAFE_LEFT(&self) -> bool {
        (self.inner & MovementFlags::STRAFE_LEFT) != 0
    }

    pub fn clear_STRAFE_LEFT(&mut self) -> Self {
        self.inner &= MovementFlags::STRAFE_LEFT.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_STRAFE_RIGHT() -> Self {
        Self {
            inner: MovementFlags::STRAFE_RIGHT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_STRAFE_RIGHT(&mut self) -> Self {
        self.inner |= MovementFlags::STRAFE_RIGHT;
        self.clone()
    }

    pub const fn get_STRAFE_RIGHT(&self) -> bool {
        (self.inner & MovementFlags::STRAFE_RIGHT) != 0
    }

    pub fn clear_STRAFE_RIGHT(&mut self) -> Self {
        self.inner &= MovementFlags::STRAFE_RIGHT.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_TURN_LEFT() -> Self {
        Self {
            inner: MovementFlags::TURN_LEFT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_TURN_LEFT(&mut self) -> Self {
        self.inner |= MovementFlags::TURN_LEFT;
        self.clone()
    }

    pub const fn get_TURN_LEFT(&self) -> bool {
        (self.inner & MovementFlags::TURN_LEFT) != 0
    }

    pub fn clear_TURN_LEFT(&mut self) -> Self {
        self.inner &= MovementFlags::TURN_LEFT.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_TURN_RIGHT() -> Self {
        Self {
            inner: MovementFlags::TURN_RIGHT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_TURN_RIGHT(&mut self) -> Self {
        self.inner |= MovementFlags::TURN_RIGHT;
        self.clone()
    }

    pub const fn get_TURN_RIGHT(&self) -> bool {
        (self.inner & MovementFlags::TURN_RIGHT) != 0
    }

    pub fn clear_TURN_RIGHT(&mut self) -> Self {
        self.inner &= MovementFlags::TURN_RIGHT.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_PITCH_UP() -> Self {
        Self {
            inner: MovementFlags::PITCH_UP,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_PITCH_UP(&mut self) -> Self {
        self.inner |= MovementFlags::PITCH_UP;
        self.clone()
    }

    pub const fn get_PITCH_UP(&self) -> bool {
        (self.inner & MovementFlags::PITCH_UP) != 0
    }

    pub fn clear_PITCH_UP(&mut self) -> Self {
        self.inner &= MovementFlags::PITCH_UP.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_PITCH_DOWN() -> Self {
        Self {
            inner: MovementFlags::PITCH_DOWN,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_PITCH_DOWN(&mut self) -> Self {
        self.inner |= MovementFlags::PITCH_DOWN;
        self.clone()
    }

    pub const fn get_PITCH_DOWN(&self) -> bool {
        (self.inner & MovementFlags::PITCH_DOWN) != 0
    }

    pub fn clear_PITCH_DOWN(&mut self) -> Self {
        self.inner &= MovementFlags::PITCH_DOWN.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_WALK_MODE() -> Self {
        Self {
            inner: MovementFlags::WALK_MODE,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_WALK_MODE(&mut self) -> Self {
        self.inner |= MovementFlags::WALK_MODE;
        self.clone()
    }

    pub const fn get_WALK_MODE(&self) -> bool {
        (self.inner & MovementFlags::WALK_MODE) != 0
    }

    pub fn clear_WALK_MODE(&mut self) -> Self {
        self.inner &= MovementFlags::WALK_MODE.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_ON_TRANSPORT(on_transport: MovementBlockMovementFlagsON_TRANSPORT) -> Self {
        Self {
            inner: MovementFlags::ON_TRANSPORT,
            on_transport: Some(on_transport),
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_ON_TRANSPORT(&mut self, on_transport: MovementBlockMovementFlagsON_TRANSPORT) -> Self {
        self.inner |= MovementFlags::ON_TRANSPORT;
        self.on_transport = Some(on_transport);
        self.clone()
    }

    pub const fn get_ON_TRANSPORT(&self) -> Option<&MovementBlockMovementFlagsON_TRANSPORT> {
        self.on_transport.as_ref()
    }

    pub fn clear_ON_TRANSPORT(&mut self) -> Self {
        self.inner &= MovementFlags::ON_TRANSPORT.reverse_bits();
        self.on_transport = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_LEVITATING() -> Self {
        Self {
            inner: MovementFlags::LEVITATING,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_LEVITATING(&mut self) -> Self {
        self.inner |= MovementFlags::LEVITATING;
        self.clone()
    }

    pub const fn get_LEVITATING(&self) -> bool {
        (self.inner & MovementFlags::LEVITATING) != 0
    }

    pub fn clear_LEVITATING(&mut self) -> Self {
        self.inner &= MovementFlags::LEVITATING.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_FIXED_Z() -> Self {
        Self {
            inner: MovementFlags::FIXED_Z,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_FIXED_Z(&mut self) -> Self {
        self.inner |= MovementFlags::FIXED_Z;
        self.clone()
    }

    pub const fn get_FIXED_Z(&self) -> bool {
        (self.inner & MovementFlags::FIXED_Z) != 0
    }

    pub fn clear_FIXED_Z(&mut self) -> Self {
        self.inner &= MovementFlags::FIXED_Z.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_ROOT() -> Self {
        Self {
            inner: MovementFlags::ROOT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_ROOT(&mut self) -> Self {
        self.inner |= MovementFlags::ROOT;
        self.clone()
    }

    pub const fn get_ROOT(&self) -> bool {
        (self.inner & MovementFlags::ROOT) != 0
    }

    pub fn clear_ROOT(&mut self) -> Self {
        self.inner &= MovementFlags::ROOT.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_JUMPING(jumping: MovementBlockMovementFlagsJUMPING) -> Self {
        Self {
            inner: MovementFlags::JUMPING,
            on_transport: None,
            jumping: Some(jumping),
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_JUMPING(&mut self, jumping: MovementBlockMovementFlagsJUMPING) -> Self {
        self.inner |= MovementFlags::JUMPING;
        self.jumping = Some(jumping);
        self.clone()
    }

    pub const fn get_JUMPING(&self) -> Option<&MovementBlockMovementFlagsJUMPING> {
        self.jumping.as_ref()
    }

    pub fn clear_JUMPING(&mut self) -> Self {
        self.inner &= MovementFlags::JUMPING.reverse_bits();
        self.jumping = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_FALLINGFAR() -> Self {
        Self {
            inner: MovementFlags::FALLINGFAR,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_FALLINGFAR(&mut self) -> Self {
        self.inner |= MovementFlags::FALLINGFAR;
        self.clone()
    }

    pub const fn get_FALLINGFAR(&self) -> bool {
        (self.inner & MovementFlags::FALLINGFAR) != 0
    }

    pub fn clear_FALLINGFAR(&mut self) -> Self {
        self.inner &= MovementFlags::FALLINGFAR.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_SWIMMING(swimming: MovementBlockMovementFlagsSWIMMING) -> Self {
        Self {
            inner: MovementFlags::SWIMMING,
            on_transport: None,
            jumping: None,
            swimming: Some(swimming),
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_SWIMMING(&mut self, swimming: MovementBlockMovementFlagsSWIMMING) -> Self {
        self.inner |= MovementFlags::SWIMMING;
        self.swimming = Some(swimming);
        self.clone()
    }

    pub const fn get_SWIMMING(&self) -> Option<&MovementBlockMovementFlagsSWIMMING> {
        self.swimming.as_ref()
    }

    pub fn clear_SWIMMING(&mut self) -> Self {
        self.inner &= MovementFlags::SWIMMING.reverse_bits();
        self.swimming = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_SPLINE_ENABLED(spline_enabled: MovementBlockMovementFlagsSPLINE_ENABLED) -> Self {
        Self {
            inner: MovementFlags::SPLINE_ENABLED,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: Some(spline_enabled),
            spline_elevation: None,
        }
    }

    pub fn set_SPLINE_ENABLED(&mut self, spline_enabled: MovementBlockMovementFlagsSPLINE_ENABLED) -> Self {
        self.inner |= MovementFlags::SPLINE_ENABLED;
        self.spline_enabled = Some(spline_enabled);
        self.clone()
    }

    pub const fn get_SPLINE_ENABLED(&self) -> Option<&MovementBlockMovementFlagsSPLINE_ENABLED> {
        self.spline_enabled.as_ref()
    }

    pub fn clear_SPLINE_ENABLED(&mut self) -> Self {
        self.inner &= MovementFlags::SPLINE_ENABLED.reverse_bits();
        self.spline_enabled = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_CAN_FLY() -> Self {
        Self {
            inner: MovementFlags::CAN_FLY,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_CAN_FLY(&mut self) -> Self {
        self.inner |= MovementFlags::CAN_FLY;
        self.clone()
    }

    pub const fn get_CAN_FLY(&self) -> bool {
        (self.inner & MovementFlags::CAN_FLY) != 0
    }

    pub fn clear_CAN_FLY(&mut self) -> Self {
        self.inner &= MovementFlags::CAN_FLY.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_FLYING() -> Self {
        Self {
            inner: MovementFlags::FLYING,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_FLYING(&mut self) -> Self {
        self.inner |= MovementFlags::FLYING;
        self.clone()
    }

    pub const fn get_FLYING(&self) -> bool {
        (self.inner & MovementFlags::FLYING) != 0
    }

    pub fn clear_FLYING(&mut self) -> Self {
        self.inner &= MovementFlags::FLYING.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_ONTRANSPORT() -> Self {
        Self {
            inner: MovementFlags::ONTRANSPORT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_ONTRANSPORT(&mut self) -> Self {
        self.inner |= MovementFlags::ONTRANSPORT;
        self.clone()
    }

    pub const fn get_ONTRANSPORT(&self) -> bool {
        (self.inner & MovementFlags::ONTRANSPORT) != 0
    }

    pub fn clear_ONTRANSPORT(&mut self) -> Self {
        self.inner &= MovementFlags::ONTRANSPORT.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_SPLINE_ELEVATION(spline_elevation: MovementBlockMovementFlagsSPLINE_ELEVATION) -> Self {
        Self {
            inner: MovementFlags::SPLINE_ELEVATION,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: Some(spline_elevation),
        }
    }

    pub fn set_SPLINE_ELEVATION(&mut self, spline_elevation: MovementBlockMovementFlagsSPLINE_ELEVATION) -> Self {
        self.inner |= MovementFlags::SPLINE_ELEVATION;
        self.spline_elevation = Some(spline_elevation);
        self.clone()
    }

    pub const fn get_SPLINE_ELEVATION(&self) -> Option<&MovementBlockMovementFlagsSPLINE_ELEVATION> {
        self.spline_elevation.as_ref()
    }

    pub fn clear_SPLINE_ELEVATION(&mut self) -> Self {
        self.inner &= MovementFlags::SPLINE_ELEVATION.reverse_bits();
        self.spline_elevation = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_WATERWALKING() -> Self {
        Self {
            inner: MovementFlags::WATERWALKING,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_WATERWALKING(&mut self) -> Self {
        self.inner |= MovementFlags::WATERWALKING;
        self.clone()
    }

    pub const fn get_WATERWALKING(&self) -> bool {
        (self.inner & MovementFlags::WATERWALKING) != 0
    }

    pub fn clear_WATERWALKING(&mut self) -> Self {
        self.inner &= MovementFlags::WATERWALKING.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_SAFE_FALL() -> Self {
        Self {
            inner: MovementFlags::SAFE_FALL,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_SAFE_FALL(&mut self) -> Self {
        self.inner |= MovementFlags::SAFE_FALL;
        self.clone()
    }

    pub const fn get_SAFE_FALL(&self) -> bool {
        (self.inner & MovementFlags::SAFE_FALL) != 0
    }

    pub fn clear_SAFE_FALL(&mut self) -> Self {
        self.inner &= MovementFlags::SAFE_FALL.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_HOVER() -> Self {
        Self {
            inner: MovementFlags::HOVER,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_enabled: None,
            spline_elevation: None,
        }
    }

    pub fn set_HOVER(&mut self) -> Self {
        self.inner |= MovementFlags::HOVER;
        self.clone()
    }

    pub const fn get_HOVER(&self) -> bool {
        (self.inner & MovementFlags::HOVER) != 0
    }

    pub fn clear_HOVER(&mut self) -> Self {
        self.inner &= MovementFlags::HOVER.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

}
impl VariableSized for MovementBlockMovementFlags {
    fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.on_transport {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.jumping {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.swimming {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.spline_enabled {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.spline_elevation {
                s.size()
            } else {
                0
            }
        }
    }
}

impl MaximumPossibleSized for MovementBlockMovementFlags {
    fn maximum_possible_size() -> usize {
        4 // inner
        + MovementBlockMovementFlagsON_TRANSPORT::maximum_possible_size() // ON_TRANSPORT enumerator
        + MovementBlockMovementFlagsJUMPING::maximum_possible_size() // JUMPING enumerator
        + MovementBlockMovementFlagsSWIMMING::maximum_possible_size() // SWIMMING enumerator
        + MovementBlockMovementFlagsSPLINE_ENABLED::maximum_possible_size() // SPLINE_ENABLED enumerator
        + MovementBlockMovementFlagsSPLINE_ELEVATION::maximum_possible_size() // SPLINE_ELEVATION enumerator
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MovementBlockMovementFlagsON_TRANSPORT {
    pub transport: TransportInfo,
}

impl VariableSized for MovementBlockMovementFlagsON_TRANSPORT {
    fn size(&self) -> usize {
        self.transport.size() // transport: TransportInfo
    }
}

impl MaximumPossibleSized for MovementBlockMovementFlagsON_TRANSPORT {
    fn maximum_possible_size() -> usize {
        29 // transport: TransportInfo
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MovementBlockMovementFlagsJUMPING {
    pub cos_angle: f32,
    pub sin_angle: f32,
    pub xy_speed: f32,
    pub z_speed: f32,
}

impl VariableSized for MovementBlockMovementFlagsJUMPING {
    fn size(&self) -> usize {
        4 // cos_angle: f32
        + 4 // sin_angle: f32
        + 4 // xy_speed: f32
        + 4 // z_speed: f32
    }
}

impl MaximumPossibleSized for MovementBlockMovementFlagsJUMPING {
    fn maximum_possible_size() -> usize {
        4 // cos_angle: f32
        + 4 // sin_angle: f32
        + 4 // xy_speed: f32
        + 4 // z_speed: f32
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MovementBlockMovementFlagsSWIMMING {
    pub pitch: f32,
}

impl VariableSized for MovementBlockMovementFlagsSWIMMING {
    fn size(&self) -> usize {
        4 // pitch: f32
    }
}

impl MaximumPossibleSized for MovementBlockMovementFlagsSWIMMING {
    fn maximum_possible_size() -> usize {
        4 // pitch: f32
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MovementBlockMovementFlagsSPLINE_ENABLED {
    pub duration: u32,
    pub final_node: Vector3d,
    pub id: u32,
    pub nodes: Vec<Vector3d>,
    pub spline_flags: MovementBlockSplineFlag,
    pub time_passed: u32,
}

impl VariableSized for MovementBlockMovementFlagsSPLINE_ENABLED {
    fn size(&self) -> usize {
        4 // amount_of_nodes: u32
        + 4 // duration: u32
        + Vector3d::size() // final_node: Vector3d
        + 4 // id: u32
        + self.nodes.iter().fold(0, |acc, x| acc + Vector3d::size()) // nodes: Vector3d[amount_of_nodes]
        + self.spline_flags.size() // spline_flags: MovementBlockSplineFlag
        + 4 // time_passed: u32
    }
}

impl MaximumPossibleSized for MovementBlockMovementFlagsSPLINE_ENABLED {
    fn maximum_possible_size() -> usize {
        4 // amount_of_nodes: u32
        + 4 // duration: u32
        + 12 // final_node: Vector3d
        + 4 // id: u32
        + 51539607552 // nodes: Vector3d[amount_of_nodes]
        + 4 // spline_flags: MovementBlockSplineFlag
        + 4 // time_passed: u32
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MovementBlockMovementFlagsSPLINE_ELEVATION {
    pub spline_elevation: f32,
}

impl VariableSized for MovementBlockMovementFlagsSPLINE_ELEVATION {
    fn size(&self) -> usize {
        4 // spline_elevation: f32
    }
}

impl MaximumPossibleSized for MovementBlockMovementFlagsSPLINE_ELEVATION {
    fn maximum_possible_size() -> usize {
        4 // spline_elevation: f32
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MovementBlockUpdateFlagLIVING {
    LIVING {
        backwards_running_speed: f32,
        backwards_swimming_speed: f32,
        fall_time: f32,
        flags: MovementBlockMovementFlags,
        living_orientation: f32,
        living_position_x: f32,
        living_position_y: f32,
        living_position_z: f32,
        running_speed: f32,
        swimming_speed: f32,
        timestamp: u32,
        turn_rate: f32,
        walking_speed: f32,
    },
    HAS_POSITION {
        orientation: f32,
        position_x: f32,
        position_y: f32,
        position_z: f32,
    },
}

impl MovementBlockUpdateFlagLIVING {
    pub(crate) const fn as_flag_value(&self) -> u8 {
        match self {
            Self::LIVING { .. } => 32,
            Self::HAS_POSITION { .. } => 64,
        }
    }

}

impl VariableSized for MovementBlockUpdateFlagLIVING {
    fn size(&self) -> usize {
        match self {
            Self::LIVING {
                backwards_running_speed,
                backwards_swimming_speed,
                fall_time,
                flags,
                living_orientation,
                living_position_x,
                living_position_y,
                living_position_z,
                running_speed,
                swimming_speed,
                timestamp,
                turn_rate,
                walking_speed,
            } => {
                0 // Not an actual enum sent over the wire
                + 4 // backwards_running_speed: f32
                + 4 // backwards_swimming_speed: f32
                + 4 // fall_time: f32
                + flags.size() // flags: MovementBlockMovementFlags
                + 4 // living_orientation: f32
                + 4 // living_position_x: f32
                + 4 // living_position_y: f32
                + 4 // living_position_z: f32
                + 4 // running_speed: f32
                + 4 // swimming_speed: f32
                + 4 // timestamp: u32
                + 4 // turn_rate: f32
                + 4 // walking_speed: f32
            }
            Self::HAS_POSITION {
                orientation,
                position_x,
                position_y,
                position_z,
            } => {
                0 // Not an actual enum sent over the wire
                + 4 // orientation: f32
                + 4 // position_x: f32
                + 4 // position_y: f32
                + 4 // position_z: f32
            }
        }
    }
}

impl MaximumPossibleSized for MovementBlockUpdateFlagLIVING {
    fn maximum_possible_size() -> usize {
        0
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct MovementBlockUpdateFlag {
    inner: u8,
    transport: Option<MovementBlockUpdateFlagTRANSPORT>,
    melee_attacking: Option<MovementBlockUpdateFlagMELEE_ATTACKING>,
    high_guid: Option<MovementBlockUpdateFlagHIGH_GUID>,
    all: Option<MovementBlockUpdateFlagALL>,
    living: Option<MovementBlockUpdateFlagLIVING>,
}

impl From<&MovementBlockUpdateFlag> for UpdateFlag {
    fn from(e: &MovementBlockUpdateFlag) -> Self {
        Self::new(e.inner)
    }
}

impl MovementBlockUpdateFlag {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: UpdateFlag = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: UpdateFlag = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: UpdateFlag = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            transport: None,
            melee_attacking: None,
            high_guid: None,
            all: None,
            living: None,
        }
    }

    pub const fn new_NONE() -> Self {
        Self {
            inner: UpdateFlag::NONE,
            transport: None,
            melee_attacking: None,
            high_guid: None,
            all: None,
            living: None,
        }
    }

    pub fn set_NONE(&mut self) -> Self {
        self.inner |= UpdateFlag::NONE;
        self.clone()
    }

    pub const fn get_NONE(&self) -> bool {
        // Underlying value is 0
        self.inner == UpdateFlag::NONE
    }

    pub fn clear_NONE(&mut self) -> Self {
        self.inner &= UpdateFlag::NONE.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_SELF() -> Self {
        Self {
            inner: UpdateFlag::SELF,
            transport: None,
            melee_attacking: None,
            high_guid: None,
            all: None,
            living: None,
        }
    }

    pub fn set_SELF(&mut self) -> Self {
        self.inner |= UpdateFlag::SELF;
        self.clone()
    }

    pub const fn get_SELF(&self) -> bool {
        (self.inner & UpdateFlag::SELF) != 0
    }

    pub fn clear_SELF(&mut self) -> Self {
        self.inner &= UpdateFlag::SELF.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_TRANSPORT(transport: MovementBlockUpdateFlagTRANSPORT) -> Self {
        Self {
            inner: UpdateFlag::TRANSPORT,
            transport: Some(transport),
            melee_attacking: None,
            high_guid: None,
            all: None,
            living: None,
        }
    }

    pub fn set_TRANSPORT(&mut self, transport: MovementBlockUpdateFlagTRANSPORT) -> Self {
        self.inner |= UpdateFlag::TRANSPORT;
        self.transport = Some(transport);
        self.clone()
    }

    pub const fn get_TRANSPORT(&self) -> Option<&MovementBlockUpdateFlagTRANSPORT> {
        self.transport.as_ref()
    }

    pub fn clear_TRANSPORT(&mut self) -> Self {
        self.inner &= UpdateFlag::TRANSPORT.reverse_bits();
        self.transport = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_MELEE_ATTACKING(melee_attacking: MovementBlockUpdateFlagMELEE_ATTACKING) -> Self {
        Self {
            inner: UpdateFlag::MELEE_ATTACKING,
            transport: None,
            melee_attacking: Some(melee_attacking),
            high_guid: None,
            all: None,
            living: None,
        }
    }

    pub fn set_MELEE_ATTACKING(&mut self, melee_attacking: MovementBlockUpdateFlagMELEE_ATTACKING) -> Self {
        self.inner |= UpdateFlag::MELEE_ATTACKING;
        self.melee_attacking = Some(melee_attacking);
        self.clone()
    }

    pub const fn get_MELEE_ATTACKING(&self) -> Option<&MovementBlockUpdateFlagMELEE_ATTACKING> {
        self.melee_attacking.as_ref()
    }

    pub fn clear_MELEE_ATTACKING(&mut self) -> Self {
        self.inner &= UpdateFlag::MELEE_ATTACKING.reverse_bits();
        self.melee_attacking = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_HIGH_GUID() -> Self {
        Self {
            inner: UpdateFlag::HIGH_GUID,
            transport: None,
            melee_attacking: None,
            high_guid: None,
            all: None,
            living: None,
        }
    }

    pub fn set_HIGH_GUID(&mut self) -> Self {
        self.inner |= UpdateFlag::HIGH_GUID;
        self.clone()
    }

    pub const fn get_HIGH_GUID(&self) -> bool {
        (self.inner & UpdateFlag::HIGH_GUID) != 0
    }

    pub fn clear_HIGH_GUID(&mut self) -> Self {
        self.inner &= UpdateFlag::HIGH_GUID.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_ALL() -> Self {
        Self {
            inner: UpdateFlag::ALL,
            transport: None,
            melee_attacking: None,
            high_guid: None,
            all: None,
            living: None,
        }
    }

    pub fn set_ALL(&mut self) -> Self {
        self.inner |= UpdateFlag::ALL;
        self.clone()
    }

    pub const fn get_ALL(&self) -> bool {
        (self.inner & UpdateFlag::ALL) != 0
    }

    pub fn clear_ALL(&mut self) -> Self {
        self.inner &= UpdateFlag::ALL.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_LIVING(living: MovementBlockUpdateFlagLIVING) -> Self {
        Self {
            inner: living.as_flag_value(),
            transport: None,
            melee_attacking: None,
            high_guid: None,
            all: None,
            living: Some(living),
        }
    }

    pub fn set_LIVING(&mut self, living: MovementBlockUpdateFlagLIVING) -> Self {
        self.inner |= living.as_flag_value();
        self.living = Some(living);
        self.clone()
    }

    pub const fn get_LIVING(&self) -> Option<&MovementBlockUpdateFlagLIVING> {
        self.living.as_ref()
    }

    pub fn clear_LIVING(&mut self) -> Self {
        self.inner &= UpdateFlag::LIVING.reverse_bits();
        self.living = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

}
impl VariableSized for MovementBlockUpdateFlag {
    fn size(&self) -> usize {
        1 // inner
        + {
            if let Some(s) = &self.transport {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.melee_attacking {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.high_guid {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.all {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.living {
                s.size()
            } else {
                0
            }
        }
    }
}

impl MaximumPossibleSized for MovementBlockUpdateFlag {
    fn maximum_possible_size() -> usize {
        1 // inner
        + MovementBlockUpdateFlagTRANSPORT::maximum_possible_size() // TRANSPORT enumerator
        + MovementBlockUpdateFlagMELEE_ATTACKING::maximum_possible_size() // MELEE_ATTACKING enumerator
        + MovementBlockUpdateFlagHIGH_GUID::maximum_possible_size() // HIGH_GUID enumerator
        + MovementBlockUpdateFlagALL::maximum_possible_size() // ALL enumerator
        + MovementBlockUpdateFlagLIVING::maximum_possible_size() // LIVING enumerator
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MovementBlockUpdateFlagTRANSPORT {
    pub transport_progress_in_ms: u32,
}

impl VariableSized for MovementBlockUpdateFlagTRANSPORT {
    fn size(&self) -> usize {
        4 // transport_progress_in_ms: u32
    }
}

impl MaximumPossibleSized for MovementBlockUpdateFlagTRANSPORT {
    fn maximum_possible_size() -> usize {
        4 // transport_progress_in_ms: u32
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MovementBlockUpdateFlagMELEE_ATTACKING {
    pub guid: Guid,
}

impl VariableSized for MovementBlockUpdateFlagMELEE_ATTACKING {
    fn size(&self) -> usize {
        self.guid.size() // guid: Guid
    }
}

impl MaximumPossibleSized for MovementBlockUpdateFlagMELEE_ATTACKING {
    fn maximum_possible_size() -> usize {
        9 // guid: Guid
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MovementBlockUpdateFlagHIGH_GUID {
}

impl VariableSized for MovementBlockUpdateFlagHIGH_GUID {
    fn size(&self) -> usize {
        4 // unknown0: u32
    }
}

impl MaximumPossibleSized for MovementBlockUpdateFlagHIGH_GUID {
    fn maximum_possible_size() -> usize {
        4 // unknown0: u32
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MovementBlockUpdateFlagALL {
}

impl VariableSized for MovementBlockUpdateFlagALL {
    fn size(&self) -> usize {
        4 // unknown1: u32
    }
}

impl MaximumPossibleSized for MovementBlockUpdateFlagALL {
    fn maximum_possible_size() -> usize {
        4 // unknown1: u32
    }
}
