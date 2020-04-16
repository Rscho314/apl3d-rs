extern crate kiss3d;
#[macro_use] extern crate lazy_static;
extern crate nalgebra as na;
extern crate serde;

use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use kiss3d::window::Window;
use na::{Point3, UnitQuaternion, Vector3};
use serde::{Serialize, Deserialize};
use std::cell::RefCell;
use std::rc::Rc;

//TYPES
type Coord3d = Point3<f32>;

struct MemberFn<'a> {
    pub foo: &'a mut dyn FnMut(usize) -> usize,
}

#[derive(Serialize, Deserialize)]
enum CellContent {
    Number {value: i64},
    MemberFn
}

#[derive(Serialize, Deserialize)]
struct Cell {
    input: Vec<Coord3d>,
    output: Vec<Coord3d>,
    position: Coord3d,
    content: CellContent
}

type Program = Vec<Cell>;
type CellPoints = Vec<Coord3d>;

//CONSTANTS
lazy_static! {
    static ref ORIGIN: Coord3d = Coord3d::new(0.0, 0.0, 0.0);
    static ref BASE_CELL_POINTS: CellPoints =
        vec![*ORIGIN, Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)];
    static ref CELL_FACES: Vec<Point3<u16>> = vec![Point3::new(0, 1, 2)];
}

//FUNCTIONS
fn cell_to_mesh(cell: Cell, cell_shape_transform: impl Fn(Cell) -> CellPoints) -> Mesh {
    Mesh::new(cell_shape_transform(cell), (*CELL_FACES).clone(), None, None, false)
}

fn cell_shape_reset(cell: Cell) -> CellPoints {
    BASE_CELL_POINTS.clone()
}

fn cell_shape_transform(cell: Cell) -> CellPoints {
    BASE_CELL_POINTS.clone()
}

fn main() {
    let mut window = Window::new("APL3D");
    let mesh = Rc::new(RefCell::new(Mesh::new(
        (*BASE_CELL_POINTS).clone(), (*CELL_FACES).clone(), None, None, false,
    )));
    let mut c = window.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));

    c.set_color(0.0, 1.0, 0.0);
    c.enable_backface_culling(false);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        c.prepend_to_local_rotation(&rot);
    }
}


