use egui::{AboveOrBelow, Ui, Id, Response, Align2, Area, Order, Align, Layout, Frame, Key};


/// Shows a popup above or below another widget.
///
/// Useful for drop-down menus (combo boxes) or suggestion menus under text fields.
///
/// The opened popup will have the same width as the parent.
///
/// You must open the popup with [`Memory::open_popup`] or  [`Memory::toggle_popup`].
///
/// Returns `None` if the popup is not open.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// let response = ui.button("Open popup");
/// let popup_id = ui.make_persistent_id("my_unique_id");
/// if response.clicked() {
///     ui.memory_mut(|mem| mem.toggle_popup(popup_id));
/// }
/// let below = egui::AboveOrBelow::Below;
/// egui::popup::popup_above_or_below_widget(ui, popup_id, &response, below, |ui| {
///     ui.set_min_width(200.0); // if you want to control the size
///     ui.label("Some more info, or things you can select:");
///     ui.label("…");
/// });
/// # });
/// ```
pub fn popup_above_or_below_widget<R>(
    ui: &Ui,
    popup_id: Id,
    widget_response: &Response,
    above_or_below: AboveOrBelow,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    if ui.memory(|mem| mem.is_popup_open(popup_id)) {
        let (pos, pivot) = match above_or_below {
            AboveOrBelow::Above => (widget_response.rect.left_top(), Align2::LEFT_BOTTOM),
            AboveOrBelow::Below => (widget_response.rect.left_bottom(), Align2::LEFT_TOP),
        };

        let inner = Area::new(popup_id)
            .order(Order::Foreground)
            .constrain(true)
            .fixed_pos(pos)
            .pivot(pivot)
            .show(ui.ctx(), |ui| {
                // Note: we use a separate clip-rect for this area, so the popup can be outside the parent.
                // See https://github.com/emilk/egui/issues/825
                let frame = Frame::popup(ui.style());
                let frame_margin = frame.total_margin();
                frame
                    .show(ui, |ui| {
                        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                            ui.set_width(widget_response.rect.width() - frame_margin.sum().x);
                            add_contents(ui)
                        })
                        .inner
                    })
                    .inner
            })
            .inner;

        if ui.input(|i| i.key_pressed(Key::Escape)) {
            ui.memory_mut(|mem| mem.close_popup());
        }
        Some(inner)
    } else {
        None
    }
}
