#[cfg(not(docsrs))]
mod generated;

#[cfg(test)]
mod tests {
    use crate::json;
    use fhirbolt_model::r4b::{
        resources::{Observation, ObservationComponent},
        types::{Code, CodeableConcept},
    };

    #[test]
    fn test_missing_field_error() {
        assert_eq!(
            json::to_string(
                &Observation {
                    ..Default::default()
                },
                None
            )
            .unwrap_err()
            .to_string(),
            "missing required field `Observation.status`"
        );

        assert_eq!(
            json::to_string(
                &Observation {
                    status: Code {
                        ..Default::default()
                    },
                    code: CodeableConcept {
                        ..Default::default()
                    }
                    .into(),
                    component: vec![ObservationComponent {
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                None
            )
            .unwrap_err()
            .to_string(),
            "missing required field `Observation.component.code`"
        );

        assert!(json::to_string(
            &Observation {
                status: Code {
                    ..Default::default()
                },
                code: CodeableConcept {
                    ..Default::default()
                }
                .into(),
                component: vec![ObservationComponent {
                    code: Default::default(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            None
        )
        .is_ok());
    }
}
