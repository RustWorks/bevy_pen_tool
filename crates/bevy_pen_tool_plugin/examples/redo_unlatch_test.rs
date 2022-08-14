use bevy_pen_tool_plugin::{pen::*, BevyPenToolPlugin, Bezier};
use bevy_pen_tool_spawner::util::*;

use bevy::prelude::*;
use std::collections::HashMap;

pub struct TargetLatches(pub HashMap<CurveIdEdge, CurveIdEdge>);

fn main() {
    let mut app = App::new();
    app.insert_resource(BezierTestHashed(HashMap::new()))
        .insert_resource(TargetLatches(HashMap::new()))
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyPenToolPlugin)
        .add_system(update_bez);

    // Run systems once
    app.update();

    let mut pen_commands = app.world.get_resource_mut::<PenCommandVec>().unwrap();

    let positions1 = BezierPositions::ZERO;
    let positions2 = BezierPositions::ZERO;

    let id1 = pen_commands.spawn(positions1);
    let id2 = pen_commands.spawn(positions2);

    // the app needs some time to perform the tasks,
    // since they are event and asset based
    app.update();
    app.update();
    app.update();

    let mut pen_commands = app.world.get_resource_mut::<PenCommandVec>().unwrap();

    let latch1 = CurveIdEdge {
        id: id1,
        anchor_edge: AnchorEdge::Start,
    };
    let latch2 = CurveIdEdge {
        id: id2,
        anchor_edge: AnchorEdge::Start,
    };

    pen_commands.latch(latch1, latch2);

    // let mut target_latches = app.world.get_resource_mut::<TargetLatches>().unwrap();

    // target_latches.0.insert(latch1, latch2);

    app.update();
    app.update();
    app.update();

    let mut pen_commands = app.world.get_resource_mut::<PenCommandVec>().unwrap();
    pen_commands.unlatch(
        CurveIdEdge {
            id: id1,
            anchor_edge: AnchorEdge::Start,
        },
        CurveIdEdge {
            id: id2,
            anchor_edge: AnchorEdge::Start,
        },
    );

    app.update();
    app.update();
    app.update();

    let mut pen_commands = app.world.get_resource_mut::<PenCommandVec>().unwrap();
    pen_commands.undo();

    app.update();
    app.update();
    app.update();

    let mut pen_commands = app.world.get_resource_mut::<PenCommandVec>().unwrap();
    pen_commands.redo();

    app.update();
    app.update();
    app.update();

    // let maps = app.world.resource::<Maps>();
    let bezier_curves = app.world.resource::<BezierTestHashed>();
    let bezier1 = bezier_curves.0.get(&id1).unwrap();
    let bezier2 = bezier_curves.0.get(&id2).unwrap();

    assert!(bezier1.latches.is_empty());
    assert!(bezier2.latches.is_empty());

    println!("redo_unlatch_test passed");
}

pub struct BezierTestHashed(pub HashMap<BezierId, Bezier>);

pub fn update_bez(
    bezier_curves: Res<Assets<Bezier>>,
    mut bezier_curves_test: ResMut<BezierTestHashed>,
) {
    // if bezier_curves.is_changed() {
    for (handle_id, bez) in bezier_curves.iter() {
        let id = BezierId(handle_id);
        bezier_curves_test.0.insert(id, bez.clone());
    }
    // }
}
