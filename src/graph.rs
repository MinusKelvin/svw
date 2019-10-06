#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct VertexID(#[doc(hidden)] pub usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EdgeID(#[doc(hidden)] pub usize);

#[derive(Clone, Debug)]
pub struct Vertex<V> {
    incoming: Vec<EdgeID>,
    outgoing: Vec<EdgeID>,
    pub data: V
}

impl<V> Vertex<V> {
    #[inline]
    pub fn incoming(&self) -> &[EdgeID] {
        &self.incoming
    }

    #[inline]
    pub fn outgoing(&self) -> &[EdgeID] {
        &self.outgoing
    }
}

#[derive(Clone, Debug)]
pub struct Edge<E> {
    from: VertexID,
    to: VertexID,
    pub data: E
}

impl<E> Edge<E> {
    #[inline]
    pub fn from(&self) -> VertexID {
        self.from
    }

    #[inline]
    pub fn to(&self) -> VertexID {
        self.to
    }
}

#[derive(Clone, Debug)]
pub struct DirectedGraph<V, E> {
    vertices: Vec<Vertex<V>>,
    edges: Vec<Edge<E>>
}

impl<V, E> DirectedGraph<V, E> {
    pub fn new() -> Self {
        DirectedGraph {
            vertices: vec![],
            edges: vec![]
        }
    }

    pub fn add_vertex(&mut self, data: V) -> VertexID {
        let id = VertexID(self.vertices.len());
        self.vertices.push(Vertex { incoming: vec![], outgoing: vec![], data });
        id
    }

    pub fn add_edge(&mut self, from: VertexID, to: VertexID, data: E) -> EdgeID {
        let id = EdgeID(self.edges.len());
        self.edges.push(Edge { to, from, data });
        self.vertices[to.0].incoming.push(id);
        self.vertices[from.0].outgoing.push(id);
        id
    }

    pub fn vertex(&self, vertex: VertexID) -> &Vertex<V> {
        &self.vertices[vertex.0]
    }

    pub fn edge(&self, edge: EdgeID) -> &Edge<E> {
        &self.edges[edge.0]
    }

    pub fn vertex_mut(&mut self, vertex: VertexID) -> &mut Vertex<V> {
        &mut self.vertices[vertex.0]
    }

    pub fn edge_mut(&mut self, edge: EdgeID) -> &mut Edge<E> {
        &mut self.edges[edge.0]
    }

    pub fn edges(&self) -> &[Edge<E>] {
        &self.edges
    }

    pub fn vertices(&self) -> &[Vertex<V>] {
        &self.vertices
    }

    pub fn edges_mut(&mut self) -> &mut [Edge<E>] {
        &mut self.edges
    }

    pub fn vertices_mut(&mut self) -> &mut [Vertex<V>] {
        &mut self.vertices
    }

    pub fn map_vertices<V2, F>(self, f: F) -> DirectedGraph<V2, E>
    where F: Fn(VertexID, Vertex<V>) -> V2 {
        DirectedGraph {
            vertices: self.vertices.into_iter()
                    .enumerate()
                    .map(|(i,v)|
                        Vertex {
                            incoming: v.incoming.clone(),
                            outgoing: v.outgoing.clone(),
                            data: f(VertexID(i), v)
                        }
                    )
                    .collect(),
            edges: self.edges
        }
    }

    pub fn map_edges<E2, F>(self, f: F) -> DirectedGraph<V, E2>
    where F: Fn(EdgeID, Edge<E>) -> E2 {
        DirectedGraph {
            vertices: self.vertices,
            edges: self.edges.into_iter()
                    .enumerate()
                    .map(|(i,e)|
                        Edge {
                            to: e.to,
                            from: e.from,
                            data: f(EdgeID(i), e)
                        }
                    )
                    .collect()
        }
    }
}