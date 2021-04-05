use nom::sequence::tuple;
use nom::{bits, IResult};
use nom::{complete::take, error::Error};

use crate::components::FrameControl;
use crate::frame_types::*;

pub fn parse_frame_control(input: &[u8]) -> IResult<&[u8], FrameControl> {
    let (remaining, (protocol_version, frame_type, frame_subtype, flags)) =
        bits::<_, (u8, u8, u8, u8), Error<(&[u8], usize)>, _, _>(tuple((
            take(2usize),
            take(2usize),
            take(4usize),
            take(8usize),
        )))(input)?;

    let frame_type = parse_frame_type(frame_type);

    // The next 4 bits are then used to determine the frame sub-type.
    // The sub-type depends on the current FrameType
    let frame_subtype = match frame_type {
        FrameType::Management => management_frame_subtype(frame_subtype),
        FrameType::Control => control_frame_subtype(frame_subtype),
        FrameType::Data => data_frame_subtype(frame_subtype),
        FrameType::Unknown => FrameSubType::UnHandled,
    };

    Ok((
        remaining,
        FrameControl {
            protocol_version,
            frame_type,
            frame_subtype,
            flags,
        },
    ))
}

/// Get the FrameType from bit 3-4
fn parse_frame_type(byte: u8) -> FrameType {
    match (byte & 0b0000_1100) >> 2 {
        0 => FrameType::Management,
        1 => FrameType::Control,
        2 => FrameType::Data,
        _ => FrameType::Unknown,
    }
}

/// Get the FrameSubType from bit 4-7 under the assumption
/// that this is a management frame.
fn management_frame_subtype(byte: u8) -> FrameSubType {
    match byte >> 4 {
        0 => FrameSubType::AssoReq,
        1 => FrameSubType::AssoResp,
        2 => FrameSubType::ReassoReq,
        3 => FrameSubType::ReassoResp,
        4 => FrameSubType::ProbeReq,
        5 => FrameSubType::ProbeResp,
        8 => FrameSubType::Beacon,
        9 => FrameSubType::Atim,
        10 => FrameSubType::Disasso,
        11 => FrameSubType::Auth,
        12 => FrameSubType::Deauth,
        _ => FrameSubType::UnHandled,
    }
}

/// Get the FrameSubType from bit 4-7 under the assumption
/// that this is a control frame.
fn control_frame_subtype(byte: u8) -> FrameSubType {
    match byte >> 4 {
        0 => FrameSubType::Reserved,
        1 => FrameSubType::Reserved,
        2 => FrameSubType::Trigger,
        3 => FrameSubType::Tack,
        4 => FrameSubType::BeamformingReportPoll,
        5 => FrameSubType::NdpAnnouncement,
        6 => FrameSubType::ControlFrameExtension,
        7 => FrameSubType::ControlWrapper,
        8 => FrameSubType::BlockAckRequest,
        9 => FrameSubType::BlockAck,
        10 => FrameSubType::PsPoll,
        11 => FrameSubType::Rts,
        12 => FrameSubType::Cts,
        13 => FrameSubType::Ack,
        14 => FrameSubType::CfEnd,
        15 => FrameSubType::CfEndCfAck,
        _ => FrameSubType::UnHandled,
    }
}

/// Get the FrameSubType from bit 4-7 under the assumption
/// that this is a data frame.
fn data_frame_subtype(byte: u8) -> FrameSubType {
    match byte >> 4 {
        0 => FrameSubType::Data,
        1 => FrameSubType::DataCfAck,
        2 => FrameSubType::DataCfPull,
        3 => FrameSubType::DataCfAckCfPull,
        4 => FrameSubType::NullData,
        5 => FrameSubType::CfAck,
        6 => FrameSubType::CfPull,
        7 => FrameSubType::CfAckCfPull,
        8 => FrameSubType::QoS,
        10 => FrameSubType::QoSCfPull,
        11 => FrameSubType::QoSCfAckCfPull,
        12 => FrameSubType::QoSNullData,
        13 => FrameSubType::Reserved,
        _ => FrameSubType::UnHandled,
    }
}
