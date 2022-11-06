use cursive::{
    theme::{BorderStyle, Palette},
    views::Dialog,
    Cursive, With,
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

    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(
        Dialog::text("Hello this is diaglog")
            .title("This is title")
            .button("Quit", |s| s.quit())
            .button("Next", show_next),
    );

    siv.run();
}

fn show_next(siv: &mut Cursive) {
    siv.pop_layer();
    siv.add_layer(
        Dialog::text("Choose yes or no")
            .button("Yes!", |s| show_answer(s, "I knew it! Well done!"))
            .button("No!", |s| show_answer(s, "I knew you couldn't be trusted!"))
            .button("Uh?", |s| s.add_layer(Dialog::info("Try again"))),
    )
}

fn show_answer(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(
        Dialog::text(msg)
            .title("Results")
            .button("Finish", |s| s.quit()),
    );
}
