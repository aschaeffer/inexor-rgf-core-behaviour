// use crate::model::ReactiveEntityInstance;
// use crate::BehaviourCreationError;
// use std::sync::Arc;
//
// pub trait EntityBehaviourFactory<T> {
//     fn build(
//         &self,
//         entity_instance: Arc<ReactiveEntityInstance>,
//     ) -> Result<Arc<T>, BehaviourCreationError>;
// }
//
// // TODO: remove this? No more used.
// #[macro_export]
// macro_rules! entity_behaviour_factory {
//     ($factory_name:ident, $reactive_type:ty, $behaviour_type:ty) => {
//         pub struct $factory_name {}
//         // impl crate::behaviour::entity::EntityBehaviourFactory<$reactive_type> for $factory_name {
//         impl inexor_ecs_behaviour::entity::EntityBehaviourFactory<$reactive_type>
//             for $factory_name
//         {
//             fn build(
//                 &self,
//                 entity_instance: Arc<ReactiveEntityInstance>,
//             ) -> Result<Arc<$reactive_type>, BehaviourCreationError> {
//                 let behaviour_instance = <$behaviour_type>::from_entity_instance(entity_instance);
//                 if behaviour_instance.is_ok() {
//                     return Ok(Arc::new(behaviour_instance.unwrap()));
//                 }
//                 Err(BehaviourCreationError.into())
//             }
//         }
//     };
// }
//
// // TODO: remove this? No more used.
// #[macro_export]
// macro_rules! create_entity_behaviour_factory {
//     ($create_factory_name:ident, $factory_name:ident) => {
//         #[waiter_di::provides]
//         fn $create_factory_name() -> crate::factory::entity::$factory_name {
//             $factory_name {}
//         }
//     };
// }
