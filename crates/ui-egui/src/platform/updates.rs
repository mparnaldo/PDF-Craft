//! Help ▸ Check for updates and the check at start, through `craft_update`: the flow itself is its
//! `Updater`; this module supplies the suite and the quit path, and shows the result in
//! `craft_update`'s `CheckResultDialog`.

#[cfg(not(target_arch = "wasm32"))]
use craft_update::CheckResultResponse;
#[cfg(not(target_arch = "wasm32"))]
use craft_update::updater::{Action, CheckSource, Updater};

use crate::PdfCraftApp;

/// Where every PdfCraft release is listed.
pub const RELEASES_PAGE: &str = "https://github.com/storytold/pdfcraft/releases";

/// Returns `true` only when running on one of Marcos Pires's two development PCs
/// (`server-sif` / `serversif` or `andsif00`). On any other PC, update checks are disabled.
pub fn is_dev_machine() -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        for var in ["COMPUTERNAME", "HOSTNAME", "USERNAME"] {
            if let Ok(val) = std::env::var(var) {
                let norm = val.trim().to_ascii_lowercase().replace('-', "");
                if norm == "serversif" || norm == "andsif00" {
                    return true;
                }
            }
        }
        false
    }
    #[cfg(target_arch = "wasm32")]
    {
        false
    }
}

/// Stands in for the network in tests.
#[cfg(not(target_arch = "wasm32"))]
pub type UpdateSource = CheckSource;

#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Default)]
pub(crate) struct UpstreamAuditState {
    pub checking: bool,
    pub latest_tag: Option<String>,
    pub downloading: bool,
    pub downloaded_dir: Option<String>,
    pub status_log: String,
}

pub(crate) struct Updates {
    #[cfg(not(target_arch = "wasm32"))]
    updater: Updater,
    /// The Updates dialog is showing.
    pub(crate) open: bool,
    /// Controlled upstream repository audit state.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) upstream_audit: Arc<Mutex<UpstreamAuditState>>,
    /// Ask for a newer release at start, unless Preferences says not to; the desktop app turns
    /// it on, so tests never reach the network.
    #[cfg(not(target_arch = "wasm32"))]
    startup_check: bool,
    #[cfg(not(target_arch = "wasm32"))]
    check_requested: bool,
    /// How many times `on_exit` called [`Updates::exiting`].
    #[cfg(test)]
    exits: u32,
    /// Stands in for the close that a finished installation queues in the `Updater`.
    #[cfg(test)]
    close_queued: bool,
}

impl Updates {
    /// The prompt that offers a newer release, after the check at start or Help ▸ Check for
    /// updates, is open over the whole window. It isn't the Updates dialog (`open`), and it leaves
    /// the menus on.
    pub(crate) fn prompt_open(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.updater.prompt_open()
        }
        #[cfg(target_arch = "wasm32")]
        {
            false
        }
    }
}

impl Default for Updates {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            updater: Updater::new(&crate::UPDATE_APP, "0.20.0"),
            open: false,
            #[cfg(not(target_arch = "wasm32"))]
            upstream_audit: Arc::new(Mutex::new(UpstreamAuditState::default())),
            #[cfg(not(target_arch = "wasm32"))]
            startup_check: false,
            #[cfg(not(target_arch = "wasm32"))]
            check_requested: false,
            #[cfg(test)]
            exits: 0,
            #[cfg(test)]
            close_queued: false,
        }
    }
}

impl Updates {
    /// Starts the helper of an update that waits for the app to exit. Later calls do nothing.
    pub(crate) fn exiting(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        self.updater.start_pending_update();
        #[cfg(test)]
        {
            self.exits += 1;
        }
    }

    /// Draws the startup prompt and returns what it asks for. `can_start` is false while unsaved
    /// work holds back a ready update.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)]
    fn show_prompt(&mut self, ctx: &egui::Context, can_start: bool) -> Action {
        #[cfg(test)]
        if can_start && std::mem::take(&mut self.close_queued) {
            return Action::Close;
        }
        self.updater.show_if(ctx, can_start)
    }

    /// Asks the next frame that may close to close the app, so that a ready update installs.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)]
    fn queue_close(&mut self) {
        self.updater.restart_to_update();
        #[cfg(test)]
        {
            self.close_queued = true;
        }
    }

    /// Takes the close that a finished installation queued, so a save after Cancel does not close
    /// the app. `show_prompt` draws an open startup prompt, so this is for when none is open.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)]
    fn take_queued_close(&mut self, ctx: &egui::Context) {
        let _ = self.show_prompt(ctx, true);
    }
}

impl PdfCraftApp {
    /// Whether the app asks for a newer release on its first frame (restricted exclusively
    /// to development machines `server-sif` and `andsif00` in controlled audit mode).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_startup_update_check(&mut self, on: bool) {
        self.updates.startup_check = on && is_dev_machine();
    }

    /// Replaces the network with `source` (tests).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_update_source(&mut self, source: UpdateSource) {
        self.updates.updater = Updater::new(&crate::UPDATE_APP, "0.20.0").with_source(source);
    }

    /// Help ▸ Check for updates: open the Controlled Upstream Update & Source Audit window
    /// (restricted to `server-sif` and `andsif00`).
    pub fn check_for_updates(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if !is_dev_machine() && !cfg!(test) {
                self.notify("As verificações de atualização estão restritas aos ambientes de desenvolvimento (server-sif / andsif00).");
                return;
            }
            self.updates.open = true;
            start_upstream_tag_check(&self.updates.upstream_audit, self.ctx.clone());
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.open_url(RELEASES_PAGE);
        }
    }

    /// Starts the checks and picks up their answers (each frame).
    pub(crate) fn poll_updates(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let Some(ctx) = self.ctx.clone() else { return };
            let _ = std::mem::take(&mut self.updates.startup_check);
            let updates = &mut self.updates;
            if std::mem::take(&mut updates.check_requested) {
                updates.updater.check_now(&ctx);
            }
            updates.updater.tick(&ctx);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn start_upstream_tag_check(state: &Arc<Mutex<UpstreamAuditState>>, ctx: Option<egui::Context>) {
    if let Ok(mut s) = state.lock() {
        if s.checking {
            return;
        }
        s.checking = true;
        s.status_log = "Consultando tags de versão no repositório oficial (github.com/storytold/pdfcraft)...".to_string();
    }
    let state = Arc::clone(state);
    std::thread::spawn(move || {
        let mut cmd = std::process::Command::new("git");
        cmd.args(["ls-remote", "--tags", "--refs", "https://github.com/storytold/pdfcraft.git"]);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt as _;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        let output = cmd.output();
        let mut latest = "v0.20.0".to_string();
        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            let mut best: Option<(String, (u64, u64, u64))> = None;
            for line in text.lines() {
                if let Some(tag_part) = line.split("refs/tags/").nth(1) {
                    let tag = tag_part.trim();
                    let clean = tag.trim_start_matches('v');
                    let mut parts = clean.split('.');
                    let major = parts.next().and_then(|p| p.parse::<u64>().ok()).unwrap_or(0);
                    let minor = parts.next().and_then(|p| p.parse::<u64>().ok()).unwrap_or(0);
                    let patch = parts.next().and_then(|p| p.split('-').next()).and_then(|p| p.parse::<u64>().ok()).unwrap_or(0);
                    let semver = (major, minor, patch);
                    if best.as_ref().is_none_or(|(_, best_v)| semver > *best_v) {
                        best = Some((tag.to_string(), semver));
                    }
                }
            }
            if let Some((best_tag, _)) = best {
                latest = best_tag;
            }
        }
        if let Ok(mut s) = state.lock() {
            s.checking = false;
            s.latest_tag = Some(latest.clone());
            s.status_log = format!(
                "Consulta concluída! Última versão oficial encontrada no GitHub: {latest}.\nVocê pode baixar o código-fonte dessa versão em uma pasta isolada para análise sem afetar sua versão customizada."
            );
        }
        if let Some(ctx) = ctx {
            ctx.request_repaint();
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn start_upstream_source_download(state: &Arc<Mutex<UpstreamAuditState>>, tag: String, ctx: Option<egui::Context>) {
    if let Ok(mut s) = state.lock() {
        if s.downloading {
            return;
        }
        s.downloading = true;
        s.status_log = format!("Baixando código-fonte oficial ({tag}) para pasta isolada de análise...");
    }
    let state = Arc::clone(state);
    std::thread::spawn(move || {
        let base_dir = std::path::PathBuf::from(r"E:\PROJETOS - SISTEMAS\PRJ-VISUAL STUDIO\PDF_CRAFT\upstream_releases");
        let target_base = if base_dir.parent().is_some_and(|p| p.exists()) {
            base_dir
        } else {
            std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")).join("upstream_releases")
        };
        let _ = std::fs::create_dir_all(&target_base);
        let clean_tag = tag.trim_start_matches('v');
        let folder_name = format!("pdfcraft-v{clean_tag}");
        let dest = target_base.join(&folder_name);
        if dest.exists() {
            let _ = std::fs::remove_dir_all(&dest);
        }
        let dest_str = dest.to_string_lossy().into_owned();
        let mut cmd = std::process::Command::new("git");
        cmd.args(["clone", "--depth", "1", "--branch", &tag, "https://github.com/storytold/pdfcraft.git", &dest_str]);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt as _;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        let status = cmd.output();
        if let Ok(mut s) = state.lock() {
            s.downloading = false;
            match status {
                Ok(out) if out.status.success() => {
                    s.downloaded_dir = Some(dest_str.clone());
                    s.status_log = format!(
                        "✓ Código-fonte da versão {tag} baixado com sucesso em:\n{dest_str}\n\nAgora basta pedir no Antigravity para comparar essa pasta com a nossa distribuição e migrar apenas o que for interessante!"
                    );
                }
                Ok(out) => {
                    let err = String::from_utf8_lossy(&out.stderr);
                    s.status_log = format!("✗ Falha ao clonar {tag}: {err}");
                }
                Err(e) => {
                    s.status_log = format!("✗ Erro ao executar git clone: {e}");
                }
            }
        }
        if let Some(ctx) = ctx {
            ctx.request_repaint();
        }
    });
}

/// The startup prompt, and the Controlled Upstream Update dialog that answers Help ▸ Check for updates.
pub(crate) fn dialog(app: &mut PdfCraftApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if !app.updates.open {
            return;
        }
        let t = crate::theme::Tokens::get(ctx);
        let mut close_modal = false;
        let mut trigger_check = false;
        let mut trigger_download: Option<String> = None;
        let mut open_folder: Option<String> = None;

        let snapshot = app.updates.upstream_audit.lock().ok().map(|s| s.clone()).unwrap_or_default();

        egui::Window::new("Central de Atualizações Controladas (PDF Craft)")
            .id(egui::Id::unique("controlled_upstream_updates_window"))
            .collapsible(false)
            .resizable(false)
            .fixed_size([580.0, 390.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 8.0;
                ui.horizontal(|ui| {
                    crate::widgets::artcraft_mark(ui, 36.0);
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("PDF Craft — Gestão Segura de Atualizações (Upstream)").font(crate::theme::semibold(16.0)));
                        ui.label(egui::RichText::new("Sua versão customizada está protegida contra sobrescrita automática.").small().color(t.text_muted));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if crate::icons::button(ui, "x", 26.0, false, "Fechar").clicked() {
                            close_modal = true;
                        }
                    });
                });
                ui.separator();

                egui::Frame::new()
                    .fill(t.hover)
                    .stroke(egui::Stroke::new(1.0, t.border))
                    .corner_radius(egui::CornerRadius::same(8))
                    .inner_margin(egui::Margin::symmetric(12, 10))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.label(egui::RichText::new("📦 Versão Instalada (Sua Distribuição Customizada): v0.20.0 (Build 20261111.10)").strong());
                        let upstream_label = snapshot.latest_tag.as_deref().unwrap_or(if snapshot.checking { "Verificando..." } else { "v0.20.0" });
                        ui.label(egui::RichText::new(format!("🌐 Última Versão no Repositório Oficial (storytold/pdfcraft): {upstream_label}")).strong());
                        ui.label(egui::RichText::new("🖥 Estação Autorizada: server-sif / andsif00 (Desenvolvimento — Marcos Pires)").small().color(t.accent_text));
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(
                                "🛡 Por segurança, as atualizações do repositório oficial NÃO são instaladas por cima do seu executável (pois apagariam o Compartilhamento WhatsApp/Outlook, OCR em Lote, Menu no Topo e Painéis de Impressão).",
                            )
                            .small()
                            .color(t.text_muted),
                        );
                    });

                if snapshot.checking || snapshot.downloading {
                    ui.add(egui::ProgressBar::new(0.65).text(if snapshot.downloading { "Baixando código-fonte oficial em pasta isolada..." } else { "Consultando GitHub..." }).animate(true));
                }

                ui.label(egui::RichText::new("📋 Status e Auditoria de Código-Fonte:").small().strong());
                egui::Frame::new()
                    .fill(egui::Color32::from_black_alpha(22))
                    .corner_radius(egui::CornerRadius::same(6))
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        egui::ScrollArea::vertical().max_height(95.0).show(ui, |ui| {
                            ui.monospace(if snapshot.status_log.is_empty() {
                                "Pronto para verificar ou baixar o código-fonte oficial para análise."
                            } else {
                                &snapshot.status_log
                            });
                        });
                    });

                ui.add_space(4.0);
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.add_enabled(!snapshot.checking && !snapshot.downloading, egui::Button::new("🔄 Verificar Novamente")).clicked() {
                        trigger_check = true;
                    }
                    let tag_to_dl = snapshot.latest_tag.clone().unwrap_or_else(|| "v0.20.0".to_string());
                    if ui
                        .add_enabled(
                            !snapshot.checking && !snapshot.downloading,
                            egui::Button::new(format!("📥 Baixar Código-Fonte ({tag_to_dl}) na Pasta do Projeto")),
                        )
                        .clicked()
                    {
                        trigger_download = Some(tag_to_dl);
                    }
                    if let Some(ref dir) = snapshot.downloaded_dir
                        && ui.button("📂 Abrir Pasta Baixada").clicked()
                    {
                        open_folder = Some(dir.clone());
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if crate::widgets::pill_button(ui, "Fechar", true).clicked() {
                            close_modal = true;
                        }
                    });
                });
            });

        if trigger_check {
            start_upstream_tag_check(&app.updates.upstream_audit, Some(ctx.clone()));
        }
        if let Some(tag) = trigger_download {
            start_upstream_source_download(&app.updates.upstream_audit, tag, Some(ctx.clone()));
        }
        if let Some(folder) = open_folder {
            let _ = std::process::Command::new("explorer.exe").arg(folder).spawn();
        }
        if close_modal {
            app.updates.open = false;
        }
    }
    #[cfg(target_arch = "wasm32")]
    let _ = (app, ctx);
}

/// Carries out the answer to the Updates dialog.
#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
fn answer_result(app: &mut PdfCraftApp, ctx: &egui::Context, response: CheckResultResponse) {
    match response {
        CheckResultResponse::None => {}
        CheckResultResponse::Close => {
            app.updates.open = false;
            app.updates.updater.dismiss_status();
        }
        CheckResultResponse::CheckAgain => app.updates.check_requested = true,
        CheckResultResponse::RestartToUpdate => restart_to_update(app, ctx),
    }
}

/// Restart to Update. With unsaved changes the Updates dialog closes and the quit path asks about
/// them; Cancel there leaves the update ready. An open startup prompt keeps its own Restart to
/// Update and save-first note, so the app stays open.
#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
fn restart_to_update(app: &mut PdfCraftApp, ctx: &egui::Context) {
    if app.first_dirty().is_none() {
        app.updates.queue_close();
        return;
    }
    app.updates.open = false;
    app.updates.updater.dismiss_status();
    if app.updates.updater.prompt_open() {
        return;
    }
    app.updates.take_queued_close(ctx);
    app.request_quit(ctx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CloseRequest;
    use crate::editing::SaveTarget;
    use craft_update::updater::Status;
    use eframe::App as _;
    use pdfcraft_engine::{Edit, FieldValue};
    use std::path::PathBuf;
    use std::sync::Arc;

    /// An app with one open document holding a form, and with `dirty` an unsaved change to it.
    fn app(dirty: bool) -> PdfCraftApp {
        let mut app = PdfCraftApp::new();
        app.open_bytes("source.pdf", None, include_bytes!("../../tests/data/form.pdf").to_vec()).unwrap();
        if dirty {
            assert!(app.apply_edit(Edit::SetFieldValue { name: "name".into(), value: FieldValue::Text("Changed".into()) }));
            assert!(app.first_dirty().is_some());
        }
        app.updates.open = true;
        app
    }

    /// An app with an unsaved change to a document in a new temporary folder, which Save writes
    /// without asking where. Returns the document's path too.
    fn app_with_file(test: &str) -> (PdfCraftApp, PathBuf) {
        let dir = std::env::temp_dir().join(format!("pdfcraft-ui-updates-{test}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("source.pdf");
        let bytes = include_bytes!("../../tests/data/form.pdf").to_vec();
        std::fs::write(&path, &bytes).unwrap();
        let mut app = PdfCraftApp::new();
        app.open_bytes("source.pdf", Some(path.to_string_lossy().into_owned()), bytes).unwrap();
        assert!(app.apply_edit(Edit::SetFieldValue { name: "name".into(), value: FieldValue::Text("Changed".into()) }));
        app.updates.open = true;
        (app, path)
    }

    #[test]
    fn restart_to_update_with_unsaved_changes_opens_the_save_prompt() {
        let mut app = app(true);
        let ctx = egui::Context::default();
        answer_result(&mut app, &ctx, CheckResultResponse::RestartToUpdate);
        assert!(!app.updates.open, "the modal Updates dialog closes");
        assert_eq!(app.close_request, Some(CloseRequest::Quit), "the save prompt opens");
        assert!(!app.allow_quit);
        app.resolve_close(&ctx, None);
        assert!(app.close_request.is_none(), "Cancel closes the prompt");
        assert!(!app.allow_quit, "Cancel keeps the app open");
    }

    #[test]
    fn restart_to_update_with_unsaved_changes_dismisses_the_check_result() {
        let mut app = app(true);
        app.set_update_source(Arc::new(|| Ok(None)));
        let ctx = egui::Context::default();
        app.updates.updater.check_now(&ctx);
        assert!(!matches!(app.updates.updater.status(), Status::Idle));
        answer_result(&mut app, &ctx, CheckResultResponse::RestartToUpdate);
        assert!(matches!(app.updates.updater.status(), Status::Idle));
    }

    #[test]
    fn saving_at_the_restart_prompt_writes_the_file_and_quits() {
        let (mut app, path) = app_with_file("save");
        let ctx = egui::Context::default();
        answer_result(&mut app, &ctx, CheckResultResponse::RestartToUpdate);
        app.resolve_close(&ctx, Some(true));
        assert!(app.allow_quit, "the app quits once the document is saved");

        let mut reopened = PdfCraftApp::new();
        reopened.open_bytes("source.pdf", Some(path.to_string_lossy().into_owned()), std::fs::read(&path).unwrap()).unwrap();
        let doc = reopened.session.get(reopened.views[0].id).unwrap();
        let name = doc.form.iter().find(|f| f.name == "name").and_then(|f| f.value.first().cloned());
        assert_eq!(name.as_deref(), Some("Changed"), "the change is in the file");
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn saving_after_canceling_the_restart_keeps_the_app_open() {
        let (mut app, path) = app_with_file("cancel");
        app.updates.close_queued = true;
        let ctx = egui::Context::default();
        answer_result(&mut app, &ctx, CheckResultResponse::RestartToUpdate);
        app.resolve_close(&ctx, None);
        assert!(app.save_active(SaveTarget::InPlace));
        assert!(app.first_dirty().is_none());
        dialog(&mut app, &ctx);
        assert!(!app.allow_quit, "the save does not close the app");
        assert!(app.close_request.is_none());
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn restart_to_update_with_nothing_unsaved_leaves_the_close_to_the_updater() {
        let mut app = app(false);
        let ctx = egui::Context::default();
        answer_result(&mut app, &ctx, CheckResultResponse::RestartToUpdate);
        assert!(app.updates.open, "the dialog stays until the updater closes the app");
        assert!(app.close_request.is_none());
        assert!(!app.allow_quit);
        dialog(&mut app, &ctx);
        assert!(app.allow_quit, "the next frame closes the app");
    }

    #[test]
    fn restart_to_update_with_the_startup_prompt_open_leaves_the_app_open() {
        let mut app = app(true);
        app.updates.updater.offer(craft_update::Release {
            tag: "v99.0.0".into(),
            version: craft_update::parse_version("99.0.0"),
            name: "PdfCraft 99.0.0".into(),
            html_url: "https://github.com/storytold/pdfcraft/releases/tag/v99.0.0".into(),
            ..Default::default()
        });
        assert!(app.updates.updater.prompt_open());
        answer_result(&mut app, &egui::Context::default(), CheckResultResponse::RestartToUpdate);
        assert!(!app.updates.open, "the result dialog closes");
        assert!(app.updates.updater.prompt_open(), "the prompt stays");
        assert!(app.close_request.is_none(), "the prompt carries the restart");
    }

    #[test]
    fn on_exit_calls_the_update_exit_hook_once() {
        let mut app = PdfCraftApp::new();
        app.on_exit(None);
        assert_eq!(app.updates.exits, 1);
    }
}
