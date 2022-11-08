use cursive::{
    theme::{BorderStyle, Palette},
    traits::*,
    views::{Checkbox, Dialog, EditView, LinearLayout, ListView, SelectView, TextArea, TextView},
};

fn main() {
    let mut siv = cursive::default();
    siv.set_theme(cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette: Palette::default().with(|palette| {
            use cursive::theme::BaseColor::*;
            use cursive::theme::Color::*;
            use cursive::theme::PaletteColor::*;

            palette[Background] = TerminalDefault;
            palette[View] = TerminalDefault;
            palette[Primary] = White.dark();
            palette[TitlePrimary] = Blue.light();
            palette[Secondary] = Blue.light();
            palette[Highlight] = Blue.dark();
        }),
    });

    siv.add_layer(Dialog::new().title("Hello").content(ListView::new().child(
        "Is this ok? ",
        Checkbox::new().on_change(|s, checked| {
            // Enable/Disable the next field depending on this checkbox
        }),
    )));
    siv.run()
}

// This example uses a ListView.
//
// ListView can be used to build forms, with a list of inputs.

// fn main() {
//     let mut siv = cursive::default();

//     siv.set_theme(cursive::theme::Theme {
//         shadow: true,
//         borders: BorderStyle::Simple,
//         palette: Palette::default().with(|palette| {
//             use cursive::theme::BaseColor::*;
//             use cursive::theme::Color::*;
//             use cursive::theme::PaletteColor::*;

//             palette[Background] = TerminalDefault;
//             palette[View] = TerminalDefault;
//             palette[Primary] = White.dark();
//             palette[TitlePrimary] = Blue.light();
//             palette[Secondary] = Blue.light();
//             palette[Highlight] = Blue.dark();
//         }),
//     });
//     siv.add_layer(
//         Dialog::new()
//             .title("Please fill out this form")
//             .button("Ok", |s| s.quit())
//             .content(
//                 ListView::new()
//                     // Each child is a single-line view with a label
//                     .child("Name", EditView::new().fixed_width(10))
//                     .child("Presentation", TextArea::new().min_height(4))
//                     .child(
//                         "Receive spam?",
//                         Checkbox::new().on_change(|s, checked| {
//                             // Enable/Disable the next field depending on this checkbox
//                             for name in &["email1", "email2"] {
//                                 s.call_on_name(name, |view: &mut EditView| {
//                                     view.set_enabled(checked)
//                                 });
//                                 if checked {
//                                     s.focus_name("email1").unwrap();
//                                 }
//                             }
//                         }),
//                     )
//                     .child(
//                         "Email",
//                         // Each child must have a height of 1 line,
//                         // but we can still combine multiple views!
//                         LinearLayout::horizontal()
//                             .child(
//                                 EditView::new()
//                                     .disabled()
//                                     .with_name("email1")
//                                     .fixed_width(15),
//                             )
//                             .child(TextView::new("@"))
//                             .child(
//                                 EditView::new()
//                                     .disabled()
//                                     .with_name("email2")
//                                     .fixed_width(10),
//                             ),
//                     )
//                     // Delimiter currently are just a blank line
//                     .delimiter()
//                     .child(
//                         "Age",
//                         // Popup-mode SelectView are small enough to fit here
//                         SelectView::new()
//                             .popup()
//                             .item_str("0-18")
//                             .item_str("19-30")
//                             .item_str("31-40")
//                             .item_str("41+"),
//                     )
//                     .with(|list| {
//                         // We can also add children procedurally
//                         for i in 0..50 {
//                             list.add_child(&format!("Item {}", i), EditView::new());
//                         }
//                     })
//                     .scrollable(),
//             ),
//     );

//     siv.run();
// }
