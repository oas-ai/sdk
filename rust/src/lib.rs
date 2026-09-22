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

    use crate::vehicle::v1::{SteeringState, VehicleState};

    #[test]
    fn vehicle_state_round_trips_on_the_wire() {
        let state = VehicleState {
            timestamp_ns: Some(1_000),
            vehicle_speed_mps: Some(12.5),
            night_mode: Some(true),
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
    }
}
