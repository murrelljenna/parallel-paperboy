use bevy::math::Vec2;
use bevy::utils::petgraph::Graph;
use bevy::math::*;
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub(crate) struct RoadNode {
    pub pos: Vec2
}

impl RoadNode {
    fn from_xy(x: f32, y: f32) -> RoadNode {
        let vector = Vec2::new(x, y);
        return RoadNode {
            pos: vector
        }
    }
}

impl Default for RoadNode {
    fn default() -> RoadNode { RoadNode::from_xy(0., 0.) }
}

#[derive(Resource, Clone)]
pub(crate) struct GameWorld {
    pub graph: Graph::<RoadNode, i32>
}

pub(crate) fn create_graph() -> GameWorld {
    let mut deps = Graph::<RoadNode, i32>::new();

    // test graph:
    // A--------G
    // |        |
    // B---H    F--I
    // |   |    |
    // C---D----E
    let a = deps.add_node(RoadNode::from_xy(0., 0.));
    let b = deps.add_node(RoadNode::from_xy(0., 2.));
    let c = deps.add_node(RoadNode::from_xy(0., 4.));
    let d = deps.add_node(RoadNode::from_xy(4., 4.));
    let e = deps.add_node(RoadNode::from_xy(9., 4.));
    let f = deps.add_node(RoadNode::from_xy(9., 2.));
    let g = deps.add_node(RoadNode::from_xy(9., 0.));
    let h = deps.add_node(RoadNode::from_xy(4., 2.));
    let i = deps.add_node(RoadNode::from_xy(12., 2.));

    deps.extend_with_edges(&[
      (a, g),
      (a, b),
      (b, a),
      (b, c),
      (b, h),
      (c, b),
      (c, d),
      (d, c),
      (d, e),
      (d, h),
      (e, d),
      (e, f),
      (f, e),
      (f, g),
      (f, i),
      (g, a),
      (g, f),
      (h, b),
      (h, d),
      (i, f),
    ]);

    println!("{:?}", deps);
    return GameWorld {
        graph: deps
    }
}