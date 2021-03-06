// TODO: Move unit test to plugin

use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::reactive::{Connector, LogicalGateProperties};
use crate::tests::create_default_connector;
use crate::entity::logical_gates::AndGate;
use crate::entity::EntityBehaviour;
use crate::relation::connectors::ConnectorBehaviour;

use std::sync::Arc;
use serde_json::json;
use crate::relation::connectors::DefaultConnector;

#[test]
fn and_gates_test () {
    // Now it's very convenient to create AND-Gates
    let and_gate_1 = AndGate::new().unwrap();
    let and_gate_2 = AndGate::new().unwrap();
    let and_gate_3 = AndGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let and_1 = and_gate_1.entity.clone();
    let and_2 = and_gate_2.entity.clone();
    let and_3 = and_gate_3.entity.clone();
    // In real world, the and gate have to be registered in the registry (!)

    // Connect the results of the first two AND-Gates with the inputs of the third AND-Gate
    let r_and_1_and_3 = Arc::new(create_default_connector(
        and_1.clone(),
        and_3.clone(),
        LogicalGateProperties::RESULT.to_string(),
        LogicalGateProperties::LHS.to_string()
    ));
    let c_and_1_and_3 = Connector::from_relation(r_and_1_and_3.clone(), DefaultConnector::OPERATION);
    assert_ne!(0, c_and_1_and_3.handle_id);

    let r_and_2_and_3 = Arc::new(create_default_connector(
        and_2.clone(),
        and_3.clone(),
        LogicalGateProperties::RESULT.to_string(),
        LogicalGateProperties::RHS.to_string()
    ));
    let c_and_2_and_3 = Connector::from_relation(r_and_2_and_3.clone(), DefaultConnector::OPERATION);
    assert_ne!(0, c_and_2_and_3.handle_id);

    and_1.set(LogicalGateProperties::LHS.to_string(), json!(true));
    assert_eq!(false, and_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, and_3.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    and_1.set(LogicalGateProperties::RHS.to_string(), json!(true));
    assert_eq!(true, and_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, and_3.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    and_2.set(LogicalGateProperties::LHS.to_string(), json!(true));
    assert_eq!(false, and_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, and_3.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    and_2.set(LogicalGateProperties::RHS.to_string(), json!(true));
    assert_eq!(true, and_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(true, and_3.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

}
