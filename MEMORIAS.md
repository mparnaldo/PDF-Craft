# 🧠 MEMÓRIAS DO PROJETO PDF CRAFT

> **Documento de Continuidade e Contexto Completo do Projeto**
> **Autor das Melhorias & Proprietário da Customização:** Marcos Pires
> **Data de Atualização:** 11/11/2026
> **Versão do Projeto:** v0.20.0 (Build 20261111.10)
> **Branch de Trabalho:** main

---

## 👤 Perfil do Usuário e Diretrizes de Interação

### 1. Perfil e Preferências
- **Usuário:** Marcos Pires (`mparnaldo` no GitHub, máquina de desenvolvimento `server-sif / andsif00`).
- **Condição Crítica:** O usuário possui TOC (Transtorno Obsessivo-Compulsivo) severo com relação a organização, precisão, estabilidade, acabamento visual e integridade de software.
- **Exigências Inegociáveis:**
  - **Zero Gambiarras / Zero Ambiguidades:** Soluções devem ser determinísticas, idempotentes e limpas.
  - **Zero Telas Pretas / Flashes Estranhos:** Nenhuma janela de console (`cmd`, `git`, etc.) pode piscar ou abrir desnecessariamente ao iniciar ou executar ações no aplicativo. Todos os processos em segundo plano usam `CREATE_NO_WINDOW (0x08000000)`.
  - **Design Executivo e Harmonioso:** Interfaces não podem parecer "carnaval". Ícones devem ser minimalistas, bem proporcionados, elegantes e com significado claro (como a suíte DevExpress / Lucide).
  - **Previsibilidade:** Todas as janelas, diálogos e barras de rolagem devem ter limites proporcionais (`max_height`, `ScrollArea`), sem vazar a barra de tarefas ou esconder botões de confirmação.

### 2. Postura e Tom do Assistente (Antigravity / IA)
- **Idioma:** Sempre Português do Brasil (`pt-BR`).
- **Tom de Comunicação:** Engenheiro de Software Sênior experiente, bem-humorado, direto ao ponto, técnico, sem rodeios ou enrolações teóricas.
- **Rigor de Código:** Seguir estritamente as regras de `AGENTS.md` do repositório:
  - **Nunca quebrar (`Never crash`):** Zero `unwrap()`, zero `expect()`, zero `panic!` em código de produção fora de testes. Tratamento rigoroso de erros com `Result`, checagens de limites e bounds checking.
  - **Clean-room:** Código limpo e independente. Preservar licenças MIT/Apache-2.0.

---

## 🚀 Histórico Completo de Melhorias & Implementações

### 1. Identidade Visual e Ícones 3D com Transparência
- **Ícone do Executável do Programa (Grupo 1 PE - `pdfcraft.ico`):**
  - Baseado na renderização tridimensional de alta fidelidade do origami em formato de letra "P" tridimensional sobre fundo transparente.
  - Gerado com múltiplas camadas (16, 20, 24, 32, 40, 48, 64, 128, 256 px).
  - Aplicado em `assets/app-icon/pdfcraft.ico`, `assets/app-icon/pdfcraft-1024.png` e conjunto Linux `hicolor`.
- **Ícone dos Arquivos PDF (Grupo 2 PE - `pdfcraft-document.ico`):**
  - O usuário solicitou a mudança porque o ícone original em uma pasta cheia parecia quadrado e poluído.
  - Criado através de fusão: formato de documento/papel encadernado com dobra e faixa lateral elegante (Modelo 5) combinado com a fita/etiqueta vermelha com a inscrição "PDF" posicionada no canto inferior direito.
  - Fundo 100% transparente com recorte alfa e sem borda quadrada, integrando-se nativamente ao Windows Explorer.
- **Injeção PE Resource Determinística:**
  - Script em C# / PowerShell (`scratch/embed_icons_clean.ps1` usando as APIs Win32 `BeginUpdateResource`, `UpdateResource` e `EndUpdateResource`) injeta o Grupo 1 (ID 1) e o Grupo 2 (ID 2) diretamente no cabeçalho PE de `bin\pdfcraft.exe`.
  - O instalador associa `Software\Classes\PdfCraft.Document\DefaultIcon` ao `pdfcraft-document.ico`.

### 2. Módulo de Compartilhamento Direto (`crates/ui-egui/src/share.rs`)
- Localizado na barra de ferramentas superior (`mode_bar`) ao lado dos botões Salvar e Imprimir, e também nos menus de contexto de página do visualizador, painel de páginas e organizador.
- **Destinos Suportados:**
  - **WhatsApp:** Copia o caminho do arquivo para a área de transferência e abre o WhatsApp Web / WhatsApp Desktop via protocolo `whatsapp://send` ou `https://web.whatsapp.com`.
  - **Microsoft Outlook:** Invoca o Outlook diretamente via linha de comando (`outlook.exe /a "<caminho_do_arquivo>"`) anexando o PDF em uma nova mensagem pronta para envio.
  - **Cliente de E-mail Padrão / WebMail:** Utiliza protocolo `mailto:` com suporte a anexo ou orientação amigável.
- **Compartilhamento de Páginas Específicas:** Se o usuário selecionar 1 ou mais páginas específicas no documento ou no painel de páginas, o aplicativo gera instantaneamente um arquivo temporário contendo apenas as páginas selecionadas e anexa esse recorte na mensagem.

### 3. Reconhecimento OCR Avançado em Lote e Único (`crates/ui-egui/src/workflows/ocr_ui.rs`)
- **Interface Responsiva:**
  - Cabeçalho e título fixados no topo.
  - Conteúdo de opções e pré-processamento envelopado em `ScrollArea::vertical()` com altura adaptativa.
  - Rodapé fixado na base com os botões "Reconhecer texto" e "Cancelar" permanentemente acessíveis em qualquer resolução ou escala de DPI.
- **Pré-processamento de Imagem Ativo:**
  - Auto-rotação (`auto_rotate: true`): Detecta e desvira automaticamente páginas escaneadas a 90°, 180° ou 270°.
  - Deskew (`deskew: true`): Detecta e alinha linhas de texto inclinadas/tortas.
  - Denoise (`denoise: true`): Filtro de remoção de manchas e ruídos de scanner.
- **Opção de Sobrescrita Direta (`overwrite_original: bool`):**
  - Permite atualizar o próprio arquivo original no mesmo diretório, evitando criar múltiplos arquivos com sufixo `_ocr`.
- **Janela Flutuante de Progresso em Tempo Real (`show_ocr_progress_windows`):**
  - **Em lote:** Exibe nome do arquivo atual, barra de progresso da página atual, barra de progresso geral do lote, console de log em tempo real com histórico de processamento e botão de cancelamento seguro.
  - **Em documento único:** Barra de progresso da página atual com cancelamento suave.

### 4. Integração Nativa com Windows Explorer (`--ocr-shell`)
- Permite ao usuário clicar com o botão direito do mouse sobre qualquer arquivo PDF ou pasta no Windows Explorer e escolher:
  - *"Reconhecer Texto (OCR) com PDF Craft"*
- **Modo Standalone / Headless:**
  - Disparado pelo parâmetro `--ocr-shell <arquivo_ou_pasta>`.
  - Não carrega a interface pesada do aplicativo nem o visualizador de documentos.
  - Abre diretamente a janela detalhada de progresso do OCR com as barras e o terminal de logs.
  - Ao terminar o reconhecimento, salva o PDF pesquisável e encerra o processo de forma limpa.

### 5. Eliminação Definitiva de Telas Pretas (Janelas de Console do Windows)
- Eliminada qualquer abertura momentânea de janela de console preta (`cmd.exe` ou `git.exe`) no Windows.
- Configurada a flag `CREATE_NO_WINDOW = 0x0800_0000` em todas as invocações de subprocessos (`std::process::Command`), incluindo checagem de atualizações e chamadas de oráculo.

### 6. Refinamento Visual do Painel de Impressão (`crates/ui-egui/src/workflows/print_ui/body.rs`)
- Adicionados ícones minimalistas e modernos da suíte DevExpress / Lucide em todas as seções e botões de comando:
  - *Impressora, Cópias, Frente e Verso, Papel, Tamanho, Pôster, Múltiplas, Livreto, Rótulo, Ajustar, Escala, Orientação, Avançado, Fechar e Imprimir*.
  - Harmonia estética mantida, sem poluição visual.

### 7. Tela Inicial (Home) no Padrão Adobe Acrobat (`crates/ui-egui/src/home.rs`)
- **Miniaturas Reais de Documentos:**
  - Thread assíncrona em segundo plano (`ensure_recent_thumb_worker`) renderiza a primeira página real dos arquivos recentes em escala nítida usando `pdfcraft_render::PageRenderer`.
  - Apresentação em formato de folha de papel retrato com sombra suave (*drop shadow*), borda sutil de 1px e cantos ligeiramente arredondados.
  - Fallback elegante procedural (cartão de documento com dobra superior e linhas) enquanto a miniatura carrega.
- **Disposição de Metadados:**
  - Linha 1: Nome do arquivo em destaque.
  - Linha 2: `PDF  ·  <caminho_completo_da_pasta>`.
  - Lado direito: Quantidade de páginas e tamanho do arquivo em formato legível (ex: `11 páginas · 407 KB`).
- **Menu Recentes:** Substituído o ícone genérico azul de texto pelo ícone 3D oficial do PDF Craft na lista *"Arquivo ▸ Abrir recentes"*.

### 8. Temas Executivos e Galeria de Temas (`crates/ui-egui/src/theme.rs` e `appearance.rs`)
- **Barra Superior Contrastante de Ponta a Ponta:**
  - A barra superior inteira (`mode_bar`) agora possui contraste executivo em tom chumbo escuro / grafite (`#1e2024` ou similar), separando a barra de menu do corpo do documento em todos os temas.
- **8 Temas Profissionais Disponíveis:**
  - *Claro Clássico (Barra Chumbo)*
  - *Escuro Clássico (Dark Gray)*
  - *Gelo Nórdico (Nordic Ice)*
  - *Papiro Executivo (Leitura Suave)*
  - *Prata Corporativo (Platinum)*
  - *Ardósia Executivo (Slate Dark)*
  - *Azul Cobalto Executivo*
  - *Grafite Profundo (OLED Black)*
- **Galeria de Temas Redimensionada:**
  - Janela modal ajustada para `660x480` com área de rolagem limitada a `350px`.
  - Miniaturas de preview reduzidas para `68px` de altura, mantendo todos os botões de controle inferiores perfeitamente visíveis e sem extrapolar os limites da tela.

### 9. Nova Aba de OCR nas Preferências (`crates/ui-egui/src/dialogs/preferences.rs`)
- Adicionado o **Card 7: Reconhecimento de Texto (OCR)** com ícone `scan-text`:
  - Configuração do idioma padrão do OCR.
  - Configuração do modo de salvamento padrão (novo arquivo vs sobrescrever original).
  - Toggles para Auto-rotate, Deskew e Denoise.
  - Toggles para tratamento de páginas com texto existente (pular vs forçar).
  - Integração nativa com a busca em tempo real das preferências.

### 10. Portabilidade Total no Windows (Fix `VCRUNTIME140.dll`)
- As bibliotecas de tempo de execução da Microsoft (`vcruntime140.dll`, `vcruntime140_1.dll`, `msvcp140.dll`, `msvcp140_1.dll`, `msvcp140_2.dll`, `msvcp140_atomic_wait.dll` e `msvcp140_codecvt_ids.dll`) foram incluídas no diretório `{app}` pelo instalador.
- Isso elimina o erro de DLL ausente ao rodar em computadores recém-formatados sem dependência de instalação prévia do Visual C++ Redistributable.

### 11. Gestão e Auditoria Upstream (`crates/ui-egui/src/platform/updates.rs`)
- Correção do bug de ordenação ASCII no `git ls-remote`: implementação de comparação numérica SemVer `(major, minor, patch)`, garantindo que `v0.20.0` seja sempre reconhecido como superior a tags anteriores como `v0.9.0`.
- Janela de atualizações exibe a estação autorizada de desenvolvimento de Marcos Pires e protege as customizações contra sobrescrita acidental por builds oficiais padrão.

### 12. Instalador Profissional com Inno Setup 6 (`packaging/windows/setup.iss`)
- Gera o arquivo `dist\PDFCraft-Setup-0.20.0.exe` (Build `20261111.10`).
- Registra tipo de arquivo `.pdf`, ProgID `PdfCraft.Document`, ícone de documento no registro do Windows e menus de contexto no Explorer.
- Verificação determinística de versão instalada impedindo downgrades acidentais.
- Compressão máxima `lzma2/ultra64`.

---

## 💻 Ambiente de Desenvolvimento e Comandos de Compilação

Caso o PC seja restaurado, aqui estão as variáveis de ambiente e comandos exatos para compilar o projeto sem erros:

### 1. Configuração do Toolchain MSVC C++ (PowerShell)
```powershell
$vc15 = "C:\Program Files\Microsoft Visual Studio\18\Enterprise\SDK\ScopeCppSDK\vc15"
$env:PATH = "$vc15\VC\bin;$env:PATH"
$env:LIB = "$vc15\VC\lib;$vc15\SDK\lib"
$env:INCLUDE = "$vc15\VC\include;$vc15\SDK\include\ucrt;$vc15\SDK\include\um;$vc15\SDK\include\shared"
$env:CRAFT_I18N_DIR = "C:\Users\serversif\.gemini\antigravity\brain\13552758-4e3c-47e2-9855-886a50577bfd\scratch\craft-i18n"
```

### 2. Comandos de Verificação e Testes
```powershell
# Checagem de tipos e compilação rápida do frontend
cargo check -p pdfcraft-ui-egui

# Execução de testes unitários
cargo test -p pdfcraft-ui-egui --lib dialogs::appearance
cargo test -p pdfcraft-ui-egui --lib dialogs::preferences

# Checagem estrita de Lints (0 avisos permitidos)
cargo clippy -p pdfcraft-ui-egui --all-targets -- -D warnings

# Formatação de código
cargo fmt --check
cargo fmt
```

### 3. Compilação Release do Executável
```powershell
cargo build --release -p pdfcraft
Copy-Item "target\release\pdfcraft.exe" "bin\pdfcraft.exe" -Force
```

### 4. Injeção dos Ícones 3D no Executável
```powershell
powershell -ExecutionPolicy Bypass -File "scratch\embed_icons_clean.ps1"
```

### 5. Compilação do Instalador Final
```powershell
& "C:\Users\serversif\AppData\Local\Programs\Inno Setup 6\ISCC.exe" "packaging\windows\setup.iss"
```
O instalador final é gerado em:
`dist\PDFCraft-Setup-0.20.0.exe`

---

## 📌 Arquivos Principais Modificados no Projeto
- `crates/ui-egui/src/home.rs`: Miniaturas assíncronas reais dos arquivos recentes e layout Acrobat.
- `crates/ui-egui/src/menus.rs`: Ícone oficial 3D nos menus de recentes.
- `crates/ui-egui/src/share.rs`: Módulo completo de compartilhamento (WhatsApp, Outlook, WebMail).
- `crates/ui-egui/src/workflows/ocr_ui.rs`: Modal de OCR responsivo, progresso em tempo real, sobrescrita direta e modo `--ocr-shell`.
- `crates/ui-egui/src/workflows/print_ui/body.rs`: Ícones minimalistas nos botões e painéis de impressão.
- `crates/ui-egui/src/dialogs/appearance.rs`: Galeria de temas redimensionada para 660x480 e cards compactos de 68px.
- `crates/ui-egui/src/dialogs/preferences.rs`: Card 7 de preferências para OCR padrão.
- `crates/ui-egui/src/platform/updates.rs`: SemVer numérico na consulta do upstream.
- `crates/ui-egui/src/dialogs.rs`: Caixa Sobre atualizada para v0.20.0 Build 20261111.10.
- `packaging/windows/setup.iss`: Script do Inno Setup com DLLs de runtime, registro e menus do Windows Explorer.
- `Cargo.toml`: Versão do workspace unificada em 0.20.0.
