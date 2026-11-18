//! Walks structs that are not public.

use lenz::Lenz;

#[derive(Lenz)]
struct Shape {
    radius: f32,
    at: Point,
}

#[derive(Lenz)]
struct Point(f32, f32);

mod scene {
    use lenz::Lenz;

    #[derive(Lenz)]
    pub(super) struct Layer {
        pub(super) depth: u32,
    }

    #[derive(Lenz)]
    pub(crate) struct Clip<T: 'static> {
        pub(crate) value: T,
    }
}

#[test]
fn walks_a_private_struct() {
    let mut shape = Shape {
        radius: 1.0,
        at: Point(2.0, 3.0),
    };

    *Shape::cursor()
        .radius()
        .accessor()
        .get_mut(&mut shape)
        .unwrap() = 4.0;
    *Shape::cursor()
        .at()
        ._1()
        .accessor()
        .get_mut(&mut shape)
        .unwrap() = 5.0;

    assert_eq!(shape.radius, 4.0);
    assert_eq!((shape.at.0, shape.at.1), (2.0, 5.0));
}

#[test]
fn walks_a_restricted_struct_from_outside_its_module() {
    use scene::{ClipCursor, LayerCursor};

    let layer = scene::Layer { depth: 2 };
    let clip = scene::Clip { value: 'a' };

    let depth = scene::Layer::cursor().depth().accessor();
    let value = scene::Clip::<char>::cursor().value().accessor();

    assert_eq!(depth.get(&layer), Some(&2));
    assert_eq!(value.get(&clip), Some(&'a'));
}
