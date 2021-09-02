use crate::model::{ReactiveEntityInstance, ReactiveRelationInstance};
use crate::reactive::connector::ConnectorProperties;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

pub fn create_relation_instance_with_properties(
    outbound_entity: Arc<ReactiveEntityInstance>,
    type_name: String,
    inbound_entity: Arc<ReactiveEntityInstance>,
    outbound_property_name: String,
    inbound_property_name: String,
) -> ReactiveRelationInstance {
    let mut properties = HashMap::new();
    properties.insert(
        ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string(),
        json!(outbound_property_name),
    );
    properties.insert(
        ConnectorProperties::INBOUND_PROPERTY_NAME.to_string(),
        json!(inbound_property_name),
    );
    ReactiveRelationInstance::create_with_properties(
        outbound_entity.clone(),
        type_name.clone(),
        inbound_entity.clone(),
        properties,
    )
}
