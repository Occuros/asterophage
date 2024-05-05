use bevy::ecs::system::Command;
use bevy::prelude::*;

pub struct CloneEntity {
    pub source: Entity,
    pub destination: Entity,
}

// This allows the command to be used in systems
impl Command for CloneEntity {
    // fn write(self, world: &mut World) {
    //     self.clone_entity(world)
    // }

    fn apply(self, world: &mut World) {
        self.clone_entity(world)
    }
}

impl CloneEntity {
    // Copy all components from an entity to another.
    // Using an entity with no components as the destination creates a copy of the source entity.
    // Panics if:
    // - the components are not registered in the type registry,
    // - the world does not have a type registry
    // - the source or destination entity do not exist
    fn clone_entity(self, world: &mut World) {
        let components = {
            let registry = world.get_resource::<AppTypeRegistry>().unwrap().read();
         world
                .get_entity(self.source)
                .unwrap()
                .archetype()
                .components()
                .map(|component_id| {
                    world
                        .components()
                        .get_info(component_id)
                        .unwrap()
                        .type_id()
                        .unwrap()
                })
                .filter_map(|type_id| {
                    let type_registry_entry = registry
                    .get(type_id);
                    
                    match type_registry_entry {
                        Some(r) => {
                            let reflect_component = r.data::<ReflectComponent>();
                            match reflect_component {
                                Some(r) => Some(r.clone()),
                                None => {
                                    info!("component is missing reflect {:?}", type_id);
                                    None
                                },
                            }
                        },
                        None => {
                            info!("reflection missing for {:?}", {type_id});
                            None
                        },
                    }
           

                
                })
                .collect::<Vec<_>>()
        };

        

        // for component in components {
        //     let source = component
        //         .reflect(world.get_entity(self.source).unwrap())
        //         .unwrap()
        //         .clone_value();

            // let mut destination = world.get_entity_mut(self.destination).unwrap();

            // component.apply_or_insert(&mut destination, &*source);
        // }
    }
}