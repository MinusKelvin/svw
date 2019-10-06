use std::collections::HashMap;
use std::collections::HashSet;

use graph::*;
use Stemma;

pub type LayoutGraph<V> = DirectedGraph<(V, Point), u32>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32
}

struct LayoutContext {
    rightmost: u32,
    bottommost: u32,
    new_data: HashMap<VertexID, Point>,
}

pub fn layout<V>(graph: Stemma<V>) -> (LayoutGraph<V>, u32, u32) {
    let root = find_root_of(&graph);
    let mut values = HashMap::new();
    find_owning_edges(&graph, &mut values, root, 0);
    let set: HashSet<_> = values.into_iter().map(|(_,(_,e))| e).collect();

    let ownership_graph = graph.map_edges(|id, e| (e.data, set.contains(&id)));

    do_layout(ownership_graph)
}

/// "ownership" basically means which edge is responsible for laying out the child vertex
fn find_owning_edges<V>(
    graph: &DirectedGraph<V, u32>,
    values: &mut HashMap<VertexID, (u32, EdgeID)>,
    vertex: VertexID,
    value: u32
) {
    for &edge_id in graph.vertex(vertex).outgoing() {
        let edge = graph.edge(edge_id);
        let v = value + edge.data;
        if values.contains_key(&edge.to()) {
            if values[&edge.to()].0 < v {
                values.insert(edge.to(), (v, edge_id));
            }
        } else {
            values.insert(edge.to(), (v, edge_id));
        }
        find_owning_edges(graph, values, edge.to(), v);
    }
}

fn do_layout<V>(graph: DirectedGraph<V, (u32, bool)>) -> (LayoutGraph<V>, u32, u32) {
    let root = find_root_of(&graph);

    let mut context = LayoutContext {
        rightmost: 0,
        bottommost: 0,
        new_data: HashMap::new(),
    };

    context.new_data.insert(root, Point { x: 0, y: 0 });

    layout_vertex(&mut context, &graph, root);

    (graph.map_edges(|_, e| e.data.0).map_vertices(|id, v| {
        (v.data, context.new_data[&id])
    }), context.rightmost as u32, context.bottommost as u32)
}

fn find_root_of<V, E>(graph: &DirectedGraph<V, E>) -> VertexID {
    if graph.vertices().len() == 0 {
        eprintln!("Stemma contains no nodes");
        std::process::exit(1);
    }
    let mut vertex = VertexID(0);
    loop {
        let incoming = graph.vertex(vertex).incoming();
        if incoming.is_empty() {
            break
        } else {
            vertex = graph.edge(incoming[0]).from();
        }
    }
    vertex
}

/// Returns width of vertex
fn layout_vertex<V>(
    context: &mut LayoutContext,
    graph: &DirectedGraph<V, (u32, bool)>,
    vertex: VertexID
) -> u32 {
    let mut x_offset = 0;
    let mut sum_children_x = 0;
    let mut owned_children = 0;

    let mut edges: Vec<_> = graph.vertex(vertex).outgoing().iter()
            .map(|&id| graph.edge(id))
            .filter(|e| e.data.1) // Only "owned" edges
            .collect();
    edges.sort_by(|a, b| a.data.0.cmp(&b.data.0).reverse());

    for edge in edges {
        let parent_position = context.new_data[&vertex];

        // Initially position child
        let x = parent_position.x + x_offset;
        if x > context.rightmost {
            context.rightmost = x;
        }
        let y = parent_position.y + 5 * (1 + edge.data.0);
        if y > context.bottommost {
            context.bottommost = y;
        }
        context.new_data.insert(edge.to(), Point { x, y });
        
        // Setup next child offset
        x_offset += layout_vertex(context, graph, edge.to());
        sum_children_x += context.new_data[&edge.to()].x;
        owned_children += 1;
    }
    if owned_children > 0 {
        context.new_data.get_mut(&vertex).unwrap().x = sum_children_x / owned_children;
        // x_offset accumulates total children width; return that
        x_offset
    } else {
        // Leaf vertex width
        15
    }
}