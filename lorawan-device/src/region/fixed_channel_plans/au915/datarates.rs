use super::{Bandwidth, Datarate, SpreadingFactor, NUM_DATARATES};

pub(crate) const DATARATES: [Option<Datarate>; NUM_DATARATES as usize] = [
    // DR0
    Some(Datarate {
        spreading_factor: SpreadingFactor::_12,
        bandwidth: Bandwidth::_125KHz,
        max_mac_payload_size: 59,
        max_mac_payload_size_with_dwell_time: 0,
    }),
    // DR1
    Some(Datarate {
        spreading_factor: SpreadingFactor::_11,
        bandwidth: Bandwidth::_125KHz,
        max_mac_payload_size: 59,
        max_mac_payload_size_with_dwell_time: 0,
    }),
    // DR2
    Some(Datarate {
        spreading_factor: SpreadingFactor::_10,
        bandwidth: Bandwidth::_125KHz,
        max_mac_payload_size: 59,
        max_mac_payload_size_with_dwell_time: 19,
    }),
    // DR3
    Some(Datarate {
        spreading_factor: SpreadingFactor::_9,
        bandwidth: Bandwidth::_125KHz,
        max_mac_payload_size: 123,
        max_mac_payload_size_with_dwell_time: 61,
    }),
    // DR4
    Some(Datarate {
        spreading_factor: SpreadingFactor::_8,
        bandwidth: Bandwidth::_125KHz,
        max_mac_payload_size: 250,
        max_mac_payload_size_with_dwell_time: 133,
    }),
    // DR5
    Some(Datarate {
        spreading_factor: SpreadingFactor::_7,
        bandwidth: Bandwidth::_125KHz,
        max_mac_payload_size: 250,
        max_mac_payload_size_with_dwell_time: 250,
    }),
    // DR6
    Some(Datarate {
        spreading_factor: SpreadingFactor::_8,
        bandwidth: Bandwidth::_500KHz,
        max_mac_payload_size: 250,
        max_mac_payload_size_with_dwell_time: 250,
    }),
    // TODO: DR7: LR-FHSS CR1/3: 1.523 MHz BW
    None,
    // DR8
    Some(Datarate {
        spreading_factor: SpreadingFactor::_12,
        bandwidth: Bandwidth::_500KHz,
        max_mac_payload_size: 61,
        max_mac_payload_size_with_dwell_time: 61,
    }),
    // DR9
    Some(Datarate {
        spreading_factor: SpreadingFactor::_11,
        bandwidth: Bandwidth::_500KHz,
        max_mac_payload_size: 137,
        max_mac_payload_size_with_dwell_time: 137,
    }),
    // DR10
    Some(Datarate {
        spreading_factor: SpreadingFactor::_10,
        bandwidth: Bandwidth::_500KHz,
        max_mac_payload_size: 250,
        max_mac_payload_size_with_dwell_time: 250,
    }),
    // DR11
    Some(Datarate {
        spreading_factor: SpreadingFactor::_9,
        bandwidth: Bandwidth::_500KHz,
        max_mac_payload_size: 250,
        max_mac_payload_size_with_dwell_time: 250,
    }),
    // DR12
    Some(Datarate {
        spreading_factor: SpreadingFactor::_8,
        bandwidth: Bandwidth::_500KHz,
        max_mac_payload_size: 250,
        max_mac_payload_size_with_dwell_time: 250,
    }),
    // DR13
    Some(Datarate {
        spreading_factor: SpreadingFactor::_7,
        bandwidth: Bandwidth::_500KHz,
        max_mac_payload_size: 250,
        max_mac_payload_size_with_dwell_time: 250,
    }),
    // DR14: RFU
    None,
];
