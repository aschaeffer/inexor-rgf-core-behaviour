// TODO: Move unit test to plugin

use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter};
use serde_json::json;
use crate::reactive::arithmetic_gate::ArithmeticGateProperties;
use crate::entity::arithmetic_gates::DivGate;
use crate::entity::EntityBehaviour;

const LHS: ArithmeticGateProperties = ArithmeticGateProperties::LHS;
const RHS: ArithmeticGateProperties = ArithmeticGateProperties::RHS;
const RESULT: ArithmeticGateProperties = ArithmeticGateProperties::RESULT;

#[test]
fn div_gate_test () {
    let div_gate = DivGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let div = div_gate.entity.clone();

    div.set(LHS.to_string(), json!(10));
    div.set(RHS.to_string(), json!(5));
    assert_eq!(2, div.as_i64(RESULT.to_string()).unwrap());
}
