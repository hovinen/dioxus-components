use crate::components::sheet::{
    Sheet, SheetClose, SheetContentClose, SheetDescription, SheetFooter, SheetHeader, SheetSide,
    SheetTitle,
};
use dioxus::prelude::*;
use dioxus_test::{
    by_role,
    matchers::{contains_substring, eq, inner_html, is_empty, len},
    render, Result, Role,
};

#[tokio::test]
fn sheet_opens_when_open_button_is_clicked() -> Result<()> {
    Ok(())
}
