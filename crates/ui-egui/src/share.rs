//! Sharing integration for PDF Craft: WhatsApp, Microsoft Outlook, and WebMail.

use crate::{PdfCraftApp, widgets};
use egui::Ui;
use std::path::Path;

const WHATSAPP_PNG: &[u8] = include_bytes!("../../../assets/icons/whatsapp.png");
const OUTLOOK_PNG: &[u8] = include_bytes!("../../../assets/icons/outlook.png");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShareTarget {
    WhatsApp,
    Outlook,
    WebMail,
}

impl ShareTarget {
    pub fn label(&self) -> &'static str {
        match self {
            Self::WhatsApp => "WhatsApp",
            Self::Outlook => "Microsoft Outlook",
            Self::WebMail => "WebMail (Navegador)",
        }
    }
}

/// Renders the share dropdown menu with the user's colorful brand icons.
pub fn share_menu(ui: &mut Ui, mut on_pick: impl FnMut(ShareTarget)) {
    ui.set_min_width(220.0);
    ui.spacing_mut().item_spacing.y = 4.0;

    let row = |ui: &mut Ui, uri: &'static str, bytes: &'static [u8], label: &str| -> egui::Response {
        let (rect, resp) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 32.0), egui::Sense::click());
        let press = crate::theme::Press::track(ui, &resp);
        let t = crate::theme::Tokens::get(ui.ctx());
        press.wash(ui, rect, 6, false);
        let body = rect.translate(press.offset());

        let img_rect = egui::Rect::from_min_size(body.min + egui::vec2(6.0, 5.0), egui::vec2(22.0, 22.0));
        if let Some(tex) = crate::widgets::load_png_texture(ui.ctx(), uri, bytes) {
            ui.painter().image(tex.id(), img_rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
        }

        let label_galley = crate::bidi::layout_ui(ui, label.to_owned(), crate::theme::medium(13.0), t.text);
        ui.painter().galley(body.left_center() + egui::vec2(36.0, -label_galley.size().y / 2.0), label_galley, t.text);
        crate::theme::hand(resp)
    };

    if row(ui, "bytes://whatsapp.png", WHATSAPP_PNG, "WhatsApp").clicked() {
        on_pick(ShareTarget::WhatsApp);
        ui.close();
    }
    if row(ui, "bytes://outlook.png", OUTLOOK_PNG, "Microsoft Outlook").clicked() {
        on_pick(ShareTarget::Outlook);
        ui.close();
    }
    if widgets::ghost_button_sized(ui, "mail", "WebMail (Navegador)", 30.0).clicked() {
        on_pick(ShareTarget::WebMail);
        ui.close();
    }
}

/// Shares the active document through the chosen destination.
pub fn share_document(app: &mut PdfCraftApp, target: ShareTarget) {
    let Some((_view_idx, id)) = app.active_ids() else {
        app.notify("Nenhum documento ativo para compartilhar.");
        return;
    };

    let (doc_name, doc_path, is_dirty, count) = match app.session.get(id) {
        Some(d) => (d.name.clone(), d.path.clone(), d.dirty, d.info.pages.len()),
        None => {
            app.notify("Documento não encontrado.");
            return;
        }
    };

    let stem = doc_name.trim_end_matches(".pdf").to_string();

    if let Some(ref path) = doc_path {
        let p = Path::new(path);
        if !is_dirty && p.exists() {
            dispatch_share(app, p, &stem, target);
            return;
        }
    }

    let all_pages: Vec<usize> = (0..count).collect();
    match app.session.extract(id, &all_pages) {
        Ok(bytes) => {
            let temp_dir = std::env::temp_dir().join("PDF_Craft_Share");
            let _ = std::fs::create_dir_all(&temp_dir);
            let file_path = temp_dir.join(format!("{stem}.pdf"));
            match std::fs::write(&file_path, bytes.as_slice()) {
                Ok(_) => dispatch_share(app, &file_path, &stem, target),
                Err(e) => app.notify(format!("Erro ao preparar arquivo temporário: {e}")),
            }
        }
        Err(e) => app.notify(format!("Erro ao exportar documento para compartilhamento: {e}")),
    }
}

/// Extracts the given pages and shares the resulting PDF through the chosen destination.
pub fn share_pages(app: &mut PdfCraftApp, pages: &[usize], target: ShareTarget) {
    if pages.is_empty() {
        app.notify("Nenhuma página selecionada para compartilhar.");
        return;
    }

    let Some((_view_idx, id)) = app.active_ids() else {
        app.notify("Nenhum documento ativo para extrair páginas.");
        return;
    };

    let doc = match app.session.get(id) {
        Some(d) => d,
        None => return,
    };

    let stem = doc.name.trim_end_matches(".pdf").to_string();
    let mut sorted_pages = pages.to_vec();
    sorted_pages.sort_unstable();

    let range_str = if sorted_pages.len() == 1 {
        format!("pág_{}", sorted_pages[0] + 1)
    } else {
        format!("págs_{}-{}_total_{}", sorted_pages[0] + 1, sorted_pages[sorted_pages.len() - 1] + 1, sorted_pages.len())
    };

    match app.session.extract(id, &sorted_pages) {
        Ok(bytes) => {
            let temp_dir = std::env::temp_dir().join("PDF_Craft_Share");
            let _ = std::fs::create_dir_all(&temp_dir);
            let file_name = format!("{stem}_{range_str}.pdf");
            let file_path = temp_dir.join(&file_name);
            match std::fs::write(&file_path, bytes.as_slice()) {
                Ok(_) => {
                    app.notify(format!("{} página(s) extraída(s) com sucesso!", sorted_pages.len()));
                    dispatch_share(app, &file_path, &stem, target);
                }
                Err(e) => app.notify(format!("Erro ao salvar páginas extraídas: {e}")),
            }
        }
        Err(e) => app.notify(format!("Erro ao extrair páginas: {e}")),
    }
}

fn dispatch_share(app: &mut PdfCraftApp, file_path: &Path, doc_name: &str, target: ShareTarget) {
    match target {
        ShareTarget::WhatsApp => {
            if let (Some(path_str), Some(ctx)) = (file_path.to_str(), app.ctx.as_ref()) {
                ctx.copy_text(path_str.to_owned());
            }
            app.notify("Pronto! Caminho do PDF copiado. Cole (Ctrl+V) ou anexe no WhatsApp.");
            app.open_url("https://web.whatsapp.com");
        }
        ShareTarget::Outlook => {
            #[cfg(windows)]
            {
                let mut launched = false;
                if let Ok(child) = std::process::Command::new("outlook.exe").arg("/a").arg(file_path).spawn() {
                    launched = true;
                    let _ = child.id();
                } else {
                    let known = [
                        r"C:\Program Files\Microsoft Office\root\Office16\OUTLOOK.EXE",
                        r"C:\Program Files (x86)\Microsoft Office\root\Office16\OUTLOOK.EXE",
                        r"C:\Program Files\Microsoft Office\Office15\OUTLOOK.EXE",
                    ];
                    for p in known {
                        if Path::new(p).exists()
                            && let Ok(child) = std::process::Command::new(p).arg("/a").arg(file_path).spawn()
                        {
                            launched = true;
                            let _ = child.id();
                            break;
                        }
                    }
                }

                if launched {
                    app.notify("Nova mensagem aberta com anexo no Microsoft Outlook!");
                } else {
                    let subject = doc_name.replace(' ', "%20");
                    let mailto = format!("mailto:?subject={subject}");
                    app.open_url(&mailto);
                    app.notify("Abrindo cliente de e-mail padrão...");
                }
            }
            #[cfg(not(windows))]
            {
                let subject = doc_name.replace(' ', "%20");
                let mailto = format!("mailto:?subject={subject}");
                app.open_url(&mailto);
                app.notify("Abrindo cliente de e-mail...");
            }
        }
        ShareTarget::WebMail => {
            if let (Some(path_str), Some(ctx)) = (file_path.to_str(), app.ctx.as_ref()) {
                ctx.copy_text(path_str.to_owned());
            }
            app.notify("Abrindo WebMail no navegador. O caminho do arquivo foi copiado para facilitar.");
            app.open_url("https://mail.google.com/mail/?view=cm&fs=1");
        }
    }
}
