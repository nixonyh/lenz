//! Drives a field's `#[lenz(tag = ...)]` tag through to a patch, the
//! way a host crate would, across a chain, an `Option`, and a
//! generic.
//!
//! `FieldPatch` and `Bindable` stand in for the host crate's own
//! traits: `FieldPatch` on tag types, a blanket `Bindable` over
//! tagged markers.

use lenz::{Cursor, FieldPath, Tagged};

/// Stands in for the host crate's per-field patch trait. Implemented
/// on tag types.
trait FieldPatch<H> {
    type Target;
    fn patch(host: &mut H, value: &Self::Target);
}

/// Stands in for the host crate's blanket bound. Nothing implements
/// it by hand.
trait Bindable<H>: FieldPath {
    fn patch(host: &mut H, value: &Self::Target);
}

impl<P, H> Bindable<H> for P
where
    P: FieldPath + Tagged,
    P::Tag: FieldPatch<H, Target = P::Target>,
{
    fn patch(host: &mut H, value: &P::Target) {
        <P::Tag as FieldPatch<H>>::patch(host, value)
    }
}

/// A `bind`-shaped helper. Takes a cursor, drives its terminal
/// field's patch against the host.
fn bind<P, H>(_cursor: Cursor<P>, host: &mut H, value: &P::Target)
where
    P: Bindable<H>,
{
    P::patch(host, value);
}

/// The mock host. Records each patch in order.
#[derive(Default)]
struct Ui {
    log: Vec<String>,
}

mod simple {
    use lenz::Lenz;

    #[derive(Lenz)]
    pub struct Frame {
        #[lenz(tag = crate::SetWidth)]
        pub width: u32,
        pub height: u32,
    }
}

pub struct SetWidth;

impl FieldPatch<Ui> for SetWidth {
    type Target = u32;

    fn patch(ui: &mut Ui, value: &u32) {
        ui.log.push(format!("width={value}"));
    }
}

use simple::{Frame, FrameCursor};

#[test]
fn single_hop_tag_drives_its_patch() {
    let mut ui = Ui::default();

    bind(Frame::cursor().width(), &mut ui, &42);

    assert_eq!(ui.log, ["width=42"]);
}

#[test]
fn untagged_field_still_walks_but_cannot_bind() {
    let frame = Frame {
        width: 1,
        height: 9,
    };

    // `height` has no tag: it still walks, but `bind(.., height())`
    // would not compile.
    let height = Frame::cursor().height().accessor();
    assert_eq!(height.get(&frame), Some(&9));
}

mod deep {
    use lenz::Lenz;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Color {
        Red,
        Blue,
    }

    #[derive(Lenz)]
    pub struct Icon {
        #[lenz(tag = crate::SetIconColor)]
        pub color: Color,
    }

    #[derive(Lenz)]
    pub struct Button {
        pub icon: Option<Icon>,
        #[lenz(tag = crate::SetFill)]
        pub fill: Color,
    }
}

pub struct SetIconColor;

impl FieldPatch<Ui> for SetIconColor {
    type Target = deep::Color;

    fn patch(ui: &mut Ui, value: &deep::Color) {
        ui.log.push(format!("icon.color={value:?}"));
    }
}

pub struct SetFill;

impl FieldPatch<Ui> for SetFill {
    type Target = deep::Color;

    fn patch(ui: &mut Ui, value: &deep::Color) {
        ui.log.push(format!("fill={value:?}"));
    }
}

use deep::{Button, ButtonCursor, Color, IconCursor};

#[test]
fn tag_resolves_through_chain_and_option() {
    let mut ui = Ui::default();

    bind(Button::cursor().icon().color(), &mut ui, &Color::Red);
    bind(Button::cursor().fill(), &mut ui, &Color::Blue);

    assert_eq!(ui.log, ["icon.color=Red", "fill=Blue"]);
}

mod generic {
    use lenz::Lenz;

    #[derive(Lenz)]
    pub struct Slot<T: 'static> {
        #[lenz(tag = crate::SetValue)]
        pub value: T,
    }
}

pub struct SetValue;

impl FieldPatch<Ui> for SetValue {
    type Target = u32;

    fn patch(ui: &mut Ui, value: &u32) {
        ui.log.push(format!("value={value}"));
    }
}

use generic::{Slot, SlotCursor};

#[test]
fn tag_on_generic_field_drives_its_patch() {
    let mut ui = Ui::default();

    bind(Slot::<u32>::cursor().value(), &mut ui, &7);

    assert_eq!(ui.log, ["value=7"]);
}
