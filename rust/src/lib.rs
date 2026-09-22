//! OAS protobuf contract의 Rust types다.

#![forbid(unsafe_code)]

pub mod vehicle {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/oas.vehicle.v1.rs"));
    }
}

#[cfg(test)]
mod tests {
    use prost::Message;

    use crate::vehicle::v1::{HmiCapability, HmiFreshness, HmiState, SteeringState, VehicleState};

    #[test]
    fn vehicle_state_round_trips_on_the_wire() {
        let state = VehicleState {
            timestamp_ns: Some(1_000),
            vehicle_speed_mps: Some(12.5),
            night_mode: Some(true),
            raw_signals: [("CGW1.CF_Gway_DrvDrSw".into(), 1.0)].into(),
            steering: Some(SteeringState {
                angle_rad: Some(0.5),
                torque_nm: None,
            }),
            ..VehicleState::default()
        };

        assert_eq!(
            VehicleState::decode(state.encode_to_vec().as_slice()).unwrap(),
            state
        );
        assert_eq!(state.night_mode, Some(true));
        assert_eq!(state.raw_signals["CGW1.CF_Gway_DrvDrSw"], 1.0);
    }

    #[test]
    fn hmi_state_carries_runtime_policy_without_recalculation() {
        let state = HmiState {
            vehicle_state: Some(VehicleState::default()),
            freshness: HmiFreshness::Fresh as i32,
            media_playback: HmiCapability::Locked as i32,
            media_playback_reason: "not_parked".into(),
            diagnostics: HmiCapability::Allowed as i32,
            vehicle_controls: HmiCapability::Unavailable as i32,
        };

        assert_eq!(HmiState::decode(state.encode_to_vec().as_slice()).unwrap(), state);
    }
}
