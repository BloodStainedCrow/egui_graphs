use egui::Pos2;
use petgraph::{stable_graph::IndexType, Direction::Incoming};
use serde::{Deserialize, Serialize};

use crate::{
    layouts::{Layout, LayoutState},
    Graph,
};

const ROW_DIST: usize = 50;
const NODE_DIST: usize = 50;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct State {
    triggered: bool,
}

impl LayoutState for State {}

/// Places Nodes depending on their distance to a root node. Applies once.
#[derive(Debug, Default)]
pub struct Tree {
    state: State,
}

impl Layout<State> for Tree {
    fn next<N, E, Ty, Ix, Dn, De>(&mut self, g: &mut Graph<N, E, Ty, Ix, Dn, De>, _: &egui::Ui)
    where
        N: Clone,
        E: Clone,
        Ty: petgraph::EdgeType,
        Ix: IndexType,
        Dn: crate::DisplayNode<N, E, Ty, Ix>,
        De: crate::DisplayEdge<N, E, Ty, Ix, Dn>,
    {
        if self.state.triggered {
            return;
        }

        let toporesult = petgraph::algo::toposort(&g.g(), None);

        match toporesult {
            Ok(topo_sort) => {
                let mut levels = vec![0usize; topo_sort.len()];

                let mut level_counts: Vec<usize> = vec![];

                for node in topo_sort {
                    let level = g.g().neighbors_directed(node, Incoming).map(|pred| levels[pred.index()]).max().map(|pred_level| pred_level + 1).unwrap_or(0);

                    levels[node.index()] = level;

                    let level_count = match level_counts.get_mut(level) {
                        Some(count) => count,
                        None => {
                            debug_assert_eq!(level_counts.len(), level);
                            level_counts.push(0);
                            level_counts.last_mut().unwrap()
                        },
                    };

                    *level_count += 1;

                    let node = &mut g.g_mut()[node];
                    node.set_location(Pos2::new((*level_count * NODE_DIST) as f32, (level * ROW_DIST) as f32));
                }
            },
            Err(e) => {
                panic!("Graph using tree layout contains cycle: {e:?}!");
            },
        }


        self.state.triggered = true;
    }

    fn state(&self) -> State {
        self.state.clone()
    }

    fn from_state(state: State) -> impl Layout<State> {
        Self { state }
    }
}
