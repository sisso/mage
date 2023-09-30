use std::f32::consts::PI;

use approx::assert_abs_diff_eq;
use log::LevelFilter;
use specs::prelude::*;

use domain::components::*;
use domain::models::*;
use domain::player::{Player, PlayerInput};
use domain::{cfg, unwrap_or_continue, Api};

const DELTA_TIME: DeltaTime = DeltaTime(0.1);

fn new_scenery() -> Api {
    let mut api = Api::default();
    api.start_scenery(SceneryParams {
        screen_size: screen_size(),
        seed: 0,
    })
    .unwrap();
    api
}

fn screen_size() -> V2 {
    V2::new(600.0, 400.0)
}

fn get_mouse_angle_0(api: &Api) -> V2 {
    let (players, positions): (ReadStorage<Player>, ReadStorage<Position>) =
        api.world.system_data();
    let player_pos = (&players, &positions).join().next().unwrap().1.pos;
    player_pos + V2::new(1.0, 0.0)
}

fn get_player_data(
    (players, positions, velocities): (
        ReadStorage<Player>,
        ReadStorage<Position>,
        ReadStorage<Velocity>,
    ),
) -> (Player, Position, Velocity) {
    (&players, &positions, &velocities)
        .join()
        .next()
        .map(|(a, b, c)| (a.clone(), b.clone(), c.clone()))
        .unwrap()
}

fn get_player_casting((players, casters): (ReadStorage<Player>, ReadStorage<Caster>)) -> Caster {
    (&players, &casters)
        .join()
        .next()
        .map(|(_, c)| c.clone())
        .unwrap()
}

#[test]
fn test_api_move_player_by_input_forward() {
    let mut api = new_scenery();

    let (_, pos, vel) = get_player_data(api.world.system_data());
    assert_abs_diff_eq!(300.0, pos.pos.x);
    assert_abs_diff_eq!(200.0, pos.pos.y);
    assert_abs_diff_eq!(0.0, pos.angle);
    assert_abs_diff_eq!(0.0, vel.vel.length());

    // move player right
    api.set_player_input(PlayerInput {
        input_dir: V2::new(1.0, 0.0),
        mouse_pos: V2::ZERO,
        cast: false,
        upgrade: None,
    })
    .unwrap();

    api.update(DELTA_TIME).unwrap();

    let (_, pos, _) = get_player_data(api.world.system_data());
    assert_abs_diff_eq!(310.0, pos.pos.x);
    assert_abs_diff_eq!(200.0, pos.pos.y);
}

#[test]
fn test_api_rotate_player() {
    let mut api = new_scenery();

    let (_, pos, _) = get_player_data(api.world.system_data());

    for (mouse_pos, angle) in vec![
        (pos.pos + V2::new(0.0, -1.0), -PI * 0.5),
        (pos.pos + V2::new(0.0, 1.0), PI * 0.5),
        (pos.pos + V2::new(1.0, 0.0), 0.0),
        (pos.pos + V2::new(-1.0, 0.0), PI),
    ] {
        api.set_player_input(PlayerInput {
            input_dir: V2::ZERO,
            mouse_pos,
            cast: false,
            upgrade: None,
        })
        .unwrap();
        api.update(DELTA_TIME).unwrap();

        let (_, pos, _) = get_player_data(api.world.system_data());
        eprintln!("checking {:?}", mouse_pos);
        assert_abs_diff_eq!(angle, pos.angle);
    }
}

#[test]
fn test_api_cast() {
    try_init_log();

    let mut api = new_scenery();

    // check initial state
    let pd = get_player_casting(api.world.system_data());
    assert_eq!(false, pd.casting.has_casted());
    assert_eq!(false, pd.casting.get_casting().is_some());
    assert_eq!(false, pd.casting.get_calm_down().is_some());

    // compute time to cast
    let time_to_cast = cfg::FIRE_MISSILE.time_to_cast(pd.casting_skill);
    let time_to_calm = cfg::FIRE_MISSILE.time_to_calm(pd.casting_skill);
    let half_calm_down = time_to_calm.mult(0.5);

    // check casting
    let mut player_input = PlayerInput::default();
    player_input.mouse_pos = get_mouse_angle_0(&api);
    player_input.cast = true;
    api.set_player_input(player_input).unwrap();
    api.update(DELTA_TIME).unwrap();

    let pd = get_player_casting(api.world.system_data());
    assert_eq!(false, pd.casting.has_casted());
    assert_eq!(true, pd.casting.get_casting().is_some());
    assert_eq!(false, pd.casting.get_calm_down().is_some());

    // check not casted
    check_added(&mut api, cfg::MODEL_MAGIC_MISSILE, false);

    // check casted
    api.update(time_to_cast).unwrap();

    let pd = get_player_casting(api.world.system_data());
    assert_eq!(true, pd.casting.has_casted());
    assert_eq!(false, pd.casting.get_casting().is_some());
    assert_eq!(false, pd.casting.get_calm_down().is_some());

    // checked casted spell
    check_added(&mut api, cfg::MODEL_MAGIC_MISSILE, true);

    // check calm down
    _ = api.update(half_calm_down);

    // check not casted
    check_added(&mut api, cfg::MODEL_MAGIC_MISSILE, false);

    let pd = get_player_casting(api.world.system_data());
    assert_eq!(false, pd.casting.has_casted());
    assert_eq!(false, pd.casting.get_casting().is_some());
    assert_eq!(true, pd.casting.get_calm_down().is_some());

    // check calm down complete
    _ = api.update(half_calm_down.add_seconds(DELTA_TIME.as_seconds_f32()));

    // check not casted
    check_added(&mut api, cfg::MODEL_MAGIC_MISSILE, false);

    let pd = get_player_casting(api.world.system_data());
    assert_eq!(false, pd.casting.has_casted());
    assert_eq!(false, pd.casting.get_casting().is_some());
    assert_eq!(false, pd.casting.get_calm_down().is_some());
}

fn check_added(api: &mut Api, model: &str, expected: bool) {
    let events = api.take_events();
    let storage = api.world.read_storage::<HasModel>();
    for e in events.added {
        let has_model = unwrap_or_continue!(storage.get(e));
        if has_model.model.as_ref() == model {
            assert_eq!(expected, true);
            return;
        }
    }
    assert_eq!(expected, false);
}

fn try_init_log() {
    _ = env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .try_init();
}
