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
        let _ = {
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

    }
}

pub fn find_child_with_name(entity: Entity, name: &str, children_q: &Query<&Children>, name_q: &Query<&Name>) -> Option<Entity> {
    let Ok(children) =  children_q.get(entity) else { return None};
    for child in children.iter() {

        if let Ok(child_name) = name_q.get(*child)  {

            if child_name.to_string() == name {
                return Some(*child)
            }
        }

        let child_result = find_child_with_name(*child, name, children_q, name_q);
        if child_result.is_some() {return child_result}
    }

    None

}