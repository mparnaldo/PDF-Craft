//! Preferences › Appearance: the appearance mode (follow the system, light or dark) above a light
//! and a dark theme card, each with a small picture of PdfCraft in that theme.

use egui::{CornerRadius, Rect, RichText, Stroke, pos2, vec2};

use crate::PdfCraftApp;
use crate::theme::{self, ALL_EXECUTIVE_THEMES, ExecutiveTheme, ThemeKind, ThemePreference, Tokens};

/// The themes each card offers. One of each today; a new theme is one more row here.
pub const LIGHT_THEMES: &[(ThemeKind, &str)] = &[(ThemeKind::Light, "Light gray")];
pub const DARK_THEMES: &[(ThemeKind, &str)] = &[(ThemeKind::Dark, "Dark gray")];

fn mode_label(preference: ThemePreference) -> &'static str {
    match preference {
        ThemePreference::System => "Use system setting",
        ThemePreference::Light => "Light",
        ThemePreference::Dark => "Dark",
    }
}

fn mode_command(preference: ThemePreference) -> &'static str {
    match preference {
        ThemePreference::System => "view.theme.system",
        ThemePreference::Light => "view.theme.light",
        ThemePreference::Dark => "view.theme.dark",
    }
}

/// A miniature PdfCraft window in `kind`'s colours: contrasting menu bar at the top, document tabs
/// below it, the page thumbnails, a page on the pasteboard and the right-hand panel.
fn preview(ui: &mut egui::Ui, kind: ThemeKind, width: f32) {
    let exec = match kind {
        ThemeKind::Light => ExecutiveTheme::LightClassic,
        ThemeKind::Dark => ExecutiveTheme::DarkClassic,
    };
    preview_executive(ui, exec, width);
}

fn preview_executive(ui: &mut egui::Ui, exec: ExecutiveTheme, width: f32) {
    let p = Tokens::for_executive(exec);
    let prev = theme::active_executive_theme();
    theme::set_active_executive_theme(exec);
    let bar_p = p.for_menubar();
    theme::set_active_executive_theme(prev);

    let (rect, _) = ui.allocate_exact_size(vec2(width, 68.0), egui::Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 3.0, p.chrome);
    // 1. Top contrasting full-width Menu Bar (mode_bar).
    let menubar = Rect::from_min_size(rect.min, vec2(rect.width(), 8.0));
    painter.rect_filled(menubar, 0.0, bar_p.chrome);
    for i in 0..5 {
        let c = if i == 0 { bar_p.accent } else { bar_p.text };
        painter.rect_filled(Rect::from_min_size(menubar.min + vec2(5.0 + i as f32 * 8.5, 2.5), vec2(6.0, 2.5)), 0.5, c);
    }
    // 2. Document tab strip below the Menu Bar.
    let title = Rect::from_min_size(pos2(rect.left(), menubar.bottom()), vec2(rect.width(), 7.0));
    painter.rect_filled(title, 0.0, p.titlebar);
    painter.rect_filled(Rect::from_min_size(title.min + vec2(4.0, 1.0), vec2(26.0, 6.0)), 1.5, p.panel);
    painter.rect_filled(Rect::from_min_size(title.min + vec2(6.0, 3.0), vec2(14.0, 1.5)), 0.5, p.text);
    painter.circle_filled(pos2(title.right() - 5.0, title.center().y), 1.5, p.icon);
    let body = Rect::from_min_max(pos2(rect.left(), title.bottom() + 1.0), rect.max);
    // Page thumbnails on the left, the first one selected.
    let rail = Rect::from_min_size(body.min, vec2((body.width() * 0.16).round(), body.height()));
    painter.rect_filled(rail, 0.0, p.panel);
    for i in 0..2 {
        let thumb = Rect::from_min_size(rail.min + vec2(rail.width() * 0.2, 4.0 + i as f32 * 17.0), vec2(rail.width() * 0.6, 13.0));
        if i == 0 {
            painter.rect_filled(thumb.expand(1.0), 1.0, p.accent_soft);
        }
        painter.rect_filled(thumb, 1.0, egui::Color32::WHITE);
        painter.rect_stroke(thumb, 1.0, egui::Stroke::new(0.5, p.border), egui::StrokeKind::Outside);
    }
    // The right-hand panel: a heading and a few rows.
    let side = Rect::from_min_max(pos2(body.right() - (body.width() * 0.22).round(), body.top()), body.max);
    painter.rect_filled(side, 0.0, p.panel);
    painter.rect_filled(Rect::from_min_size(side.min + vec2(4.0, 4.0), vec2(side.width() * 0.5, 2.0)), 0.5, p.text);
    for i in 0..3 {
        let y = side.top() + 9.0 + i as f32 * 5.0;
        painter.rect_filled(Rect::from_min_size(pos2(side.left() + 4.0, y), vec2(side.width() * 0.65, 1.5)), 0.5, p.text_muted);
    }
    // The page, white in both themes, with a heading and lines of text.
    let paste = Rect::from_min_max(pos2(rail.right(), body.top()), pos2(side.left(), body.bottom()));
    painter.rect_filled(paste, 0.0, p.pasteboard);
    let page_h = paste.height() - 6.0;
    let page = Rect::from_center_size(paste.center(), vec2(page_h * 0.77, page_h));
    painter.rect_filled(page.translate(vec2(1.0, 1.0)), 1.0, p.page_shadow);
    painter.rect_filled(page, 1.0, egui::Color32::WHITE);
    let ink = egui::Color32::from_gray(0x60);
    painter.rect_filled(Rect::from_min_size(page.min + vec2(4.0, 4.0), vec2(page.width() * 0.5, 2.0)), 0.5, egui::Color32::from_gray(0x30));
    for i in 0..5 {
        let w = if i % 3 == 2 { 0.5 } else { 0.8 };
        painter.rect_filled(Rect::from_min_size(page.min + vec2(4.0, 8.5 + i as f32 * 4.5), vec2(page.width() * w - 4.0, 1.2)), 0.5, ink);
    }
}

/// One theme card: its title, Active when it is what shows now, and a preview of the chosen theme.
/// Returns whether the card was clicked and the control's Response.
fn card(ui: &mut egui::Ui, t: &Tokens, title: &str, active: bool, themes: &[(ThemeKind, &str)], query: &str) -> (bool, egui::Response) {
    let mut card_clicked = false;
    let frame_stroke = if active { Stroke::new(1.5, t.accent) } else { Stroke::new(1.0, t.border) };
    let scope_res = ui.scope_builder(egui::UiBuilder::new().id_salt(title).sense(egui::Sense::click()), |ui| {
        egui::Frame::new().fill(t.card).stroke(frame_stroke).corner_radius(CornerRadius::same(6)).inner_margin(egui::Margin::same(8)).show(
            ui,
            |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    let font = theme::semibold(12.5);
                    let ranges = crate::dialogs::preferences::find_match_ranges(title, query);
                    let galley = if ranges.is_empty() {
                        crate::bidi::layout_ui(ui, title.to_owned(), font, t.text)
                    } else {
                        let (job, _) = crate::dialogs::preferences::create_highlight_job(title, &ranges, font, t.text);
                        ui.painter().layout_job(job)
                    };
                    let (title_rect, _) = ui.allocate_exact_size(galley.size(), egui::Sense::hover());
                    ui.painter().galley(title_rect.min, galley, t.text);
                    if active {
                        ui.label(RichText::new(tl!("Active")).color(t.accent_text).small());
                    }
                });
                ui.add_space(4.0);
                let shown = themes.first().map_or(ThemeKind::Light, |(kind, _)| *kind);
                preview(ui, shown, ui.available_width());
            },
        );
    });

    let resp = scope_res.response;
    resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::RadioButton, true, active, title));

    if resp.has_focus() {
        ui.painter().rect_stroke(resp.rect, CornerRadius::same(6), Stroke::new(2.0, t.accent_text), egui::StrokeKind::Outside);
    }

    let resp = theme::hand(resp);
    let key_select = resp.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Space) || i.key_pressed(egui::Key::Enter));
    if resp.clicked() || key_select {
        card_clicked = true;
    }
    (card_clicked, resp)
}

/// The Appearance section of Preferences.
#[cfg_attr(not(test), expect(dead_code, reason = "called by unit tests"))]
pub fn preferences_section(ui: &mut egui::Ui, app: &mut PdfCraftApp, t: &Tokens) {
    let mut ctx = crate::dialogs::preferences::PrefSearchContext::new("", false);
    preferences_section_with_context(ui, app, t, &mut ctx);
}

/// The Appearance section of Preferences with live search highlighting and scrolling context.
pub(crate) fn preferences_section_with_context(
    ui: &mut egui::Ui,
    app: &mut PdfCraftApp,
    t: &Tokens,
    ctx: &mut crate::dialogs::preferences::PrefSearchContext,
) {
    ctx.label(ui, tl_ctx!("theme", "Appearance"), theme::semibold(13.0), t.text);
    ui.horizontal(|ui| {
        let label = ctx.label(ui, tl!("Appearance mode"), theme::regular(13.0), t.text);
        let combo = craft_ui::ComboBox::from_id_salt("appearance-mode").selected_text(tl!(mode_label(app.theme_preference))).show_ui(ui, |ui| {
            for preference in [ThemePreference::System, ThemePreference::Light, ThemePreference::Dark] {
                if ui.selectable_label(app.theme_preference == preference, tl!(mode_label(preference))).clicked() {
                    app.execute(mode_command(preference));
                    ui.close();
                }
            }
        });
        // The visible label names the combo box for assistive technology and tests.
        combo.response.labelled_by(label.id);
    });
    ui.add_space(4.0);
    let light_active = app.theme == ThemeKind::Light;
    let light_title = tl!("Light theme");
    let dark_title = tl!("Dark theme");
    let light_matches = crate::dialogs::preferences::find_match_ranges(light_title, ctx.query).len();
    let dark_matches = crate::dialogs::preferences::find_match_ranges(dark_title, ctx.query).len();
    ctx.total_matches += light_matches + dark_matches;

    ui.columns(2, |cols| {
        let [light, dark] = cols else { return };
        let (light_clicked, light_resp) = card(light, t, light_title, light_active, LIGHT_THEMES, ctx.query);
        if light_clicked {
            app.execute(mode_command(ThemePreference::Light));
        }
        ctx.scroll_if_first_match(&light_resp, light_matches);

        let (dark_clicked, dark_resp) = card(dark, t, dark_title, !light_active, DARK_THEMES, ctx.query);
        if dark_clicked {
            app.execute(mode_command(ThemePreference::Dark));
        }
        ctx.scroll_if_first_match(&dark_resp, dark_matches);
    });
    ui.add_space(8.0);
    egui::Frame::new()
        .fill(t.accent_soft.gamma_multiply(0.35))
        .stroke(Stroke::new(1.0, t.accent.gamma_multiply(0.45)))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                crate::icons::paint_inline(ui, "palette", 16.0, t.accent);
                ui.vertical(|ui| {
                    ui.label(RichText::new("Galeria de Temas Profissionais (8 Temas Executivos)").font(theme::semibold(13.0)).color(t.text));
                    ui.label(
                        RichText::new(format!(
                            "Tema atual: {} — todos com contraste dedicado na barra superior.",
                            app.current_executive_theme().title()
                        ))
                        .size(11.5)
                        .color(t.text_muted),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if crate::widgets::pill_button(ui, "Mais temas…", true).clicked() {
                        app.themes_gallery_open = true;
                    }
                });
            });
        });
    ui.add_space(6.0);
    // Acrobat's "Highlight existing fields": shared with the toggle in the form bar.
    ctx.checkbox(ui, &mut app.view_defaults.highlight_fields, tl!("Highlight fields"), t);
}

/// Dedicated floating gallery window with 8 minimalist, professional themes (2 classic + 6 executive).
pub fn show_themes_gallery_window(app: &mut PdfCraftApp, ctx: &egui::Context) {
    if !app.themes_gallery_open {
        return;
    }
    let t = Tokens::get(ctx);
    let mut open = app.themes_gallery_open;
    let mut picked: Option<ExecutiveTheme> = None;
    let mut close_clicked = false;

    egui::Window::new("🎨 Galeria de Temas Profissionais — PDF Craft")
        .open(&mut open)
        .collapsible(false)
        .resizable(true)
        .default_size([660.0, 480.0])
        .min_width(580.0)
        .min_height(380.0)
        .max_height(520.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                crate::icons::paint_inline(ui, "swatch-book", 18.0, t.accent);
                ui.vertical(|ui| {
                    ui.label(RichText::new("Escolha o Tema Visual do PDF Craft").font(theme::semibold(14.0)).color(t.text));
                    ui.label(
                        RichText::new("Paletas minimalistas e corporativas com barra superior inteira contrastante de ponta a ponta.")
                            .size(11.5)
                            .color(t.text_muted),
                    );
                });
            });
            ui.add_space(6.0);
            ui.separator();
            ui.add_space(4.0);

            let active_exec = app.current_executive_theme();

            egui::ScrollArea::vertical().id_salt("themes_gallery_scroll").auto_shrink([false, false]).max_height(350.0).show(ui, |ui| {
                for pair in ALL_EXECUTIVE_THEMES.chunks(2) {
                    ui.columns(2, |cols| {
                        for (col_idx, &exec) in pair.iter().enumerate() {
                            let col_ui = &mut cols[col_idx];
                            let is_active = active_exec == exec;
                            let stroke = if is_active { Stroke::new(2.0, t.accent) } else { Stroke::new(1.0, t.border) };
                            let fill = if is_active { t.accent_soft.gamma_multiply(0.28) } else { t.card };
                            let scope = col_ui.scope_builder(
                                egui::UiBuilder::new().id_salt(("exec_theme_card", exec.to_u8())).sense(egui::Sense::click()),
                                |ui| {
                                    egui::Frame::new()
                                        .fill(fill)
                                        .stroke(stroke)
                                        .corner_radius(CornerRadius::same(6))
                                        .inner_margin(egui::Margin::symmetric(8, 6))
                                        .show(ui, |ui| {
                                            ui.set_width(ui.available_width());
                                            ui.horizontal(|ui| {
                                                let kind_icon = match exec.kind() {
                                                    ThemeKind::Light => "sun",
                                                    ThemeKind::Dark => "moon",
                                                };
                                                crate::icons::paint_inline(ui, kind_icon, 13.0, t.accent);
                                                ui.label(RichText::new(exec.title()).font(theme::semibold(12.5)).color(t.text));
                                                if is_active {
                                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                        ui.label(RichText::new("✓ Ativo").font(theme::semibold(11.0)).color(t.accent_text));
                                                    });
                                                }
                                            });
                                            ui.add_space(1.0);
                                            ui.label(RichText::new(exec.subtitle()).size(10.5).color(t.text_muted));
                                            ui.add_space(4.0);
                                            preview_executive(ui, exec, ui.available_width());
                                        });
                                },
                            );
                            if theme::hand(scope.response).clicked() {
                                picked = Some(exec);
                            }
                        }
                    });
                    ui.add_space(4.0);
                }
            });

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.button("Usar configuração do sistema").clicked() {
                    app.set_theme_preference(ThemePreference::System);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if crate::widgets::pill_button(ui, "Concluído", true).clicked() {
                        close_clicked = true;
                    }
                });
            });
        });

    if let Some(exec) = picked {
        app.set_executive_theme(exec);
    }
    if close_clicked {
        open = false;
    }
    app.themes_gallery_open = open;
}

#[cfg(test)]
mod tests {
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    use super::*;

    /// The mode is chosen in Preferences and both theme cards show, the one on screen marked
    /// Active.
    #[test]
    fn preferences_choose_the_appearance_mode_and_show_both_themes() {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        let mut fonts = false;
        let mut h = Harness::builder().with_size(egui::vec2(640.0, 400.0)).build_ui_state(
            move |ui, app: &mut PdfCraftApp| {
                // The app's fonts (the semibold headings) take effect from the next frame.
                if !fonts {
                    ui.ctx().set_fonts(theme::font_definitions());
                    fonts = true;
                    return;
                }
                let t = Tokens::get(ui.ctx());
                preferences_section(ui, app, &t);
            },
            app,
        );
        h.run();
        assert!(h.query_by_label("Light theme").is_some() && h.query_by_label("Dark theme").is_some());
        assert_eq!(h.query_all_by_label("Active").count(), 1);
        if let Ok(path) = std::env::var("PDFCRAFT_APPEARANCE_SHOT") {
            h.render().unwrap().save(path).unwrap();
        }

        // The field-highlight preference is on for new settings, and the box turns it off.
        assert!(h.state().view_defaults.highlight_fields);
        h.get_by_label("Highlight fields").click();
        h.run();
        assert!(!h.state().view_defaults.highlight_fields);

        let combo = egui::accesskit::Role::ComboBox;
        h.get_by_role(combo).click();
        h.run();
        h.get_by_label("Dark").click();
        h.run();
        assert_eq!(h.state().theme_preference, ThemePreference::Dark);
        h.get_by_role(combo).click();
        h.run();
        h.get_by_label("Use system setting").click();
        h.run();
        assert_eq!(h.state().theme_preference, ThemePreference::System);
    }

    /// Clicking a theme card picks that theme, not only the mode combo box.
    #[test]
    fn clicking_a_theme_card_picks_that_theme() {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        let mut fonts = false;
        let mut h = Harness::builder().with_size(egui::vec2(640.0, 400.0)).build_ui_state(
            move |ui, app: &mut PdfCraftApp| {
                if !fonts {
                    ui.ctx().set_fonts(theme::font_definitions());
                    fonts = true;
                    return;
                }
                let t = Tokens::get(ui.ctx());
                preferences_section(ui, app, &t);
            },
            app,
        );
        h.run();
        h.get_by_label("Dark theme").click();
        h.run();
        assert_eq!((h.state().theme_preference, h.state().theme), (ThemePreference::Dark, ThemeKind::Dark));
        h.get_by_label("Light theme").click();
        h.run();
        assert_eq!((h.state().theme_preference, h.state().theme), (ThemePreference::Light, ThemeKind::Light));
    }
}
