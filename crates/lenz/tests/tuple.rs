//! Walks the numbered fields of tuple structs.

use lenz::Lenz;

#[derive(Lenz)]
pub struct Page(usize);

/// A field left out keeps the numbers of the ones after it.
#[derive(Lenz)]
pub struct Row(u32, #[lenz(ignore)] bool, Option<Page>);

#[derive(Lenz)]
pub struct Wrap<T: 'static>(T);

#[test]
fn reads_and_writes_a_newtype() {
    let mut page = Page(3);
    let at = Page::cursor()._0().accessor();

    *at.get_mut(&mut page).unwrap() = 4;

    assert_eq!(page.0, 4);
}

#[test]
fn walks_past_an_ignored_field_and_through_an_option() {
    let mut row = Row(1, true, Some(Page(7)));
    let page = Row::cursor()._2()._0().accessor();

    *page.get_mut(&mut row).unwrap() = 8;
    assert_eq!(page.get(&row), Some(&8));
    assert!(row.1, "the field left out is not the one written");

    row.2 = None;
    assert_eq!(page.get(&row), None);
}

#[test]
fn a_generic_newtype_walks_to_its_parameter() {
    let wrap = Wrap(Page(9));
    let at = Wrap::<Page>::cursor()._0()._0().accessor();

    assert_eq!(at.get(&wrap), Some(&9));
}

#[test]
fn the_same_number_on_two_structs_keys_apart() {
    assert_ne!(Row::cursor()._0().key(), Page::cursor()._0().key(),);
}
