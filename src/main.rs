use cursive::{views::Dialog, Cursive};

fn main() {
    let mut siv = cursive::default();

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
