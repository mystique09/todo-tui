use cursive::{
    self,
    traits::{Nameable, Resizable, Scrollable},
    views::{Dialog, EditView, ListView, TextView},
    Cursive,
};

fn main() {
    let mut siv = cursive::default();
    let list_view = ListView::new().with_name("list");

    let content = Dialog::around(TextView::new("To Do(s)"))
        .title("Todo App")
        .content(list_view)
        .button("Add new Todo", add_todo)
        .button("Quit", |s| s.quit())
        .full_width()
        .full_height()
        .scrollable();

    siv.add_layer(content);
    siv.add_screen();

    siv.run();
}

fn add_todo(s: &mut Cursive) {
    let text_area = EditView::new()
        .on_submit(|c: &mut Cursive, text| {
            c.call_on_name("list", |l: &mut ListView| {
                l.add_child("[ ]", TextView::new(text));
            });
            c.set_screen(0);
        })
        .full_width();

    let content = Dialog::around(text_area);
    s.set_screen(1);
    s.add_layer(
        Dialog::around(TextView::new("Add new Todo"))
            .title("Add New Todo")
            .content(content)
            .button("Home", |s| s.set_screen(0)),
    );
}
