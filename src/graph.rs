use bevy::math::Vec2;
use bevy::utils::petgraph::Graph;
use bevy::math::*;
use bevy::prelude::*;

#[derive(Component, Debug)]
struct RoadNode {
    pos: Vec2
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

pub(crate) fn graph_example() {
    let mut deps = Graph::<RoadNode, &str>::new();
    let startingNode = deps.add_node(RoadNode::from_xy(0., 0.));
    let nodeB = deps.add_node(RoadNode::from_xy(2., 2.));
    deps.extend_with_edges(&[
        (startingNode, nodeB)
    ]);

    println!("{:?}", deps);
}