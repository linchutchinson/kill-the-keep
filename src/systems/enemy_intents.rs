use crate::prelude::*;

#[system(for_each)]
pub fn create_enemy_intents(entity: &Entity, _: &Enemy, commands: &mut CommandBuffer) {
    let damage = thread_rng().gen_range(1..7);

    commands.push(((), EnemyAttackIntent{ enemy: *entity, damage }));
}

#[system(for_each)]
#[read_component(Vec2)]
#[read_component(Sprite)]
pub fn draw_enemy_intents(ecs: &mut SubWorld, enemy_intents: &EnemyAttackIntent, #[resource] ui_tex: &UITextures) {
    let enemy = ecs.entry_ref(enemy_intents.enemy).unwrap();
    let base_pos = enemy.get_component::<Vec2>().unwrap();
    let sprite = enemy.get_component::<Sprite>().unwrap();

    let render_pos = *base_pos - (Vec2::Y * sprite.texture.height());

    draw_texture(ui_tex.attack_intent, render_pos.x, render_pos.y, Color::new(1.0, 0.0, 0.0, 1.0));
    draw_text(&format!("{}", enemy_intents.damage), render_pos.x, render_pos.y, 32.0, TEXT_COLOR);
}

#[system(for_each)]
#[read_component(Player)]
#[write_component(Health)]
pub fn resolve_enemy_intents(ecs: &mut SubWorld, entity: &Entity, intent: &EnemyAttackIntent, commands: &mut CommandBuffer) {
    if let Some((_, health)) = <(&Player, &mut Health)>::query()
        .iter_mut(ecs)
        .nth(0)
    {
        health.current -= intent.damage;
    }
    
    commands.remove(*entity);
}
