
use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect::<Vec<(_, _)>>();
    victims.iter().for_each(
        |(message, victim)| {
            if let Ok(mut health) = ecs
                .entry_mut(*victim)
                .unwrap()
                .get_component_mut::<Health>()
            {
                // println!("Health before attack: {}", health.current);
                health.current -= 1;
                if health.current < 1 {
                    commands.remove(*victim);
                }
                // println!("Health after attack: {}", health.current);
            }
            commands.remove(*message);
        }
    );
}