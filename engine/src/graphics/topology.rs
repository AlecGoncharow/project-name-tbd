use std::slice::Iter;
use wgpu::PrimitiveTopology;

// @HACK @FIXME
// this is a hack so I can use Topology(PolygonMode) as keys for pipeline hashmap
// might want to be more clever eventually
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Topology {
    PointList(PolygonMode),
    LineList(PolygonMode),
    LineStrip(PolygonMode),
    TriangleList(PolygonMode),
    TriangleStrip(PolygonMode),
    //TriangleFan(PolygonMode),
    //LineListWithAdjacency(PolygonMode),
    //LineStripWithAdjacency(PolygonMode),
    //TriangleListWithAdjacency(PolygonMode),
    //TriangleStripWithAdjacency(PolygonMode),
    //PatchList(PolygonMode),
}

impl Topology {
    pub fn inner(self) -> PolygonMode {
        match self {
            Topology::PointList(PolygonMode::Fill)
            | Topology::LineList(PolygonMode::Fill)
            | Topology::LineStrip(PolygonMode::Fill)
            | Topology::TriangleList(PolygonMode::Fill)
            | Topology::TriangleStrip(PolygonMode::Fill) => PolygonMode::Fill,
            //
            Topology::PointList(PolygonMode::Line)
            | Topology::LineList(PolygonMode::Line)
            | Topology::LineStrip(PolygonMode::Line)
            | Topology::TriangleList(PolygonMode::Line)
            | Topology::TriangleStrip(PolygonMode::Line) => PolygonMode::Line,
            //
            Topology::PointList(PolygonMode::Point)
            | Topology::LineList(PolygonMode::Point)
            | Topology::LineStrip(PolygonMode::Point)
            | Topology::TriangleList(PolygonMode::Point)
            | Topology::TriangleStrip(PolygonMode::Point) => PolygonMode::Point,
        }
    }

    pub fn iterator() -> Iter<'static, Topology> {
        // generate all variants
        static TOPOLOGIES: [Topology; 15] = [
            Topology::PointList(PolygonMode::Fill),
            Topology::PointList(PolygonMode::Line),
            Topology::PointList(PolygonMode::Point),
            //
            Topology::LineList(PolygonMode::Fill),
            Topology::LineList(PolygonMode::Line),
            Topology::LineList(PolygonMode::Point),
            //
            Topology::LineStrip(PolygonMode::Fill),
            Topology::LineStrip(PolygonMode::Line),
            Topology::LineStrip(PolygonMode::Point),
            //
            Topology::TriangleList(PolygonMode::Fill),
            Topology::TriangleList(PolygonMode::Line),
            Topology::TriangleList(PolygonMode::Point),
            //
            Topology::TriangleStrip(PolygonMode::Fill),
            Topology::TriangleStrip(PolygonMode::Line),
            Topology::TriangleStrip(PolygonMode::Point),
        ];

        TOPOLOGIES.iter()
    }
}

impl From<&Topology> for PrimitiveTopology {
    fn from(top: &Topology) -> PrimitiveTopology {
        match top {
            Topology::PointList(_) => PrimitiveTopology::PointList,
            Topology::LineList(_) => PrimitiveTopology::LineList,
            Topology::LineStrip(_) => PrimitiveTopology::LineStrip,
            Topology::TriangleList(_) => PrimitiveTopology::TriangleList,
            Topology::TriangleStrip(_) => PrimitiveTopology::TriangleStrip,
        }
    }
}

impl From<Topology> for usize {
    fn from(top: Topology) -> Self {
        let shifted: usize = match top {
            Topology::PointList(_) => 1 << 2,
            Topology::LineList(_) => 2 << 2,
            Topology::LineStrip(_) => 3 << 2,
            Topology::TriangleList(_) => 4 << 2,
            Topology::TriangleStrip(_) => 5 << 2,
        };
        shifted + usize::from(top.inner())
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum PolygonMode {
    Fill,
    Line,
    Point,
}

impl From<PolygonMode> for wgpu::PolygonMode {
    fn from(mode: PolygonMode) -> Self {
        match mode {
            PolygonMode::Fill => Self::Fill,
            PolygonMode::Line => Self::Line,
            PolygonMode::Point => Self::Point,
        }
    }
}

impl From<PolygonMode> for usize {
    fn from(mode: PolygonMode) -> Self {
        match mode {
            PolygonMode::Fill => 0,
            PolygonMode::Line => 1,
            PolygonMode::Point => 1 << 2,
        }
    }
}
