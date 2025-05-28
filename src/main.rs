use cursive::direction::{Direction, Orientation};
use cursive::view::{Nameable, Resizable};
use cursive::views::{EditView, LinearLayout, TextView};
use cursive::{Cursive, View};

// What the app does:
// * App starts with edit view disabled
// * Press "q" to quit the application
// * Press "e" to enable the edit view and give it focus
// Observation:
// On pressing "e", the edit view is enabled. However, the user cannot
// yet type directly in it. Instead, the user needs to click on the
// view before they can type in it. I would expect "take_focus" do this
// without requiring a mouse click.
fn main() {
    let mut c = cursive::default();

    c.add_fullscreen_layer(
        LinearLayout::new(Orientation::Vertical)
            .child(
                TextView::new("Q to quit\nE to edit")
                    .min_height(5)
                    .full_width(),
            )
            .child(EditView::new().disabled().with_name("EDIT")),
    );

    c.add_global_callback('q', Cursive::quit);
    c.add_global_callback('e', |c| {
        c.call_on_name("EDIT", |edit_view: &mut EditView| {
            edit_view.enable();
            edit_view.take_focus(Direction::none()).unwrap();
        });
    });

    c.run();
}
