use crate::model::{ReactiveEntityInstance, ReactiveRelationInstance};
use crate::reactive::Connector;
use crate::tests::utils::create_relation_instance_with_properties::create_relation_instance_with_properties;
use std::sync::Arc;

const DEFAULT_CONNECTOR_TYPE_NAME: &str = "default_connector";

pub fn create_default_connector<S: Into<String>>(
    outbound_entity: Arc<ReactiveEntityInstance>,
    inbound_entity: Arc<ReactiveEntityInstance>,
    outbound_property_name: S,
    inbound_property_name: S,
) -> ReactiveRelationInstance {
    create_relation_instance_with_properties(
        outbound_entity,
        Connector::type_name(
            DEFAULT_CONNECTOR_TYPE_NAME,
            "bit_1", // TODO: outbound_property_name.into()
            "bit_1", // TODO: inbound_property_name.into()
        ),
        inbound_entity,
        outbound_property_name.into(),
        inbound_property_name.into(),
    )
}
