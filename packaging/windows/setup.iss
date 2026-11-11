; =====================================================================
; PDF Craft - Instalador Inno Setup Profissional e Determinístico
; =====================================================================
#define MyAppName "PDF Craft"
#define MyAppVersion "0.20.0"
#define MyBuildNumber "20261111.10"
#define MyAppPublisher "ArtCraft • Melhorias e Customizações por Marcos Pires"
#define MyAppURL "https://getartcraft.com/apps/pdfcraft"
#define MyAppExeName "pdfcraft.exe"
#define MyAppId "{{EA00C367-7270-4C50-9D8B-A7A5C88AD27F}}"

[Setup]
AppId={#MyAppId}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} v{#MyAppVersion} (Build {#MyBuildNumber})
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\ArtCraft\PdfCraft
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
LicenseFile=..\..\LICENSE-MIT
OutputDir=..\..\dist
OutputBaseFilename=PDFCraft-Setup-{#MyAppVersion}
SetupIconFile=..\..\assets\app-icon\pdfcraft.ico
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64compatible
DisableWelcomePage=no
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
CloseApplications=yes
RestartApplications=no
UninstallDisplayIcon={app}\pdfcraft.ico

[Languages]
Name: "brazilianportuguese"; MessagesFile: "compiler:Languages\BrazilianPortuguese.isl"

[Messages]
brazilianportuguese.WelcomeLabel1=Bem-vindo ao Assistente de Instalação do [name]
brazilianportuguese.WelcomeLabel2=Este assistente instalará o [name/ver] no seu computador.%n%n• Projeto Base Original: ArtCraft (Código Aberto)%n• Melhorias, Otimizações, Temas Executivos, OCR em Lote e Integrações: Desenvolvido e aprimorado por Marcos Pires%n%nRecomendamos fechar outros aplicativos antes de continuar.

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"
Name: "contextmenu"; Description: "Adicionar opções do PDF Craft (incluindo Reconhecer OCR Direto) ao menu de contexto do Windows Explorer"; GroupDescription: "Integração com o Sistema:"

[Files]
; Binários principais e Ícones 3D com Fundo Transparente
Source: "..\..\bin\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\bin\pdfcraft-cli.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\assets\app-icon\pdfcraft.ico"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\assets\app-icon\pdfcraft-document.ico"; DestDir: "{app}"; Flags: ignoreversion
; Bibliotecas de Tempo de Execução C/C++ da Microsoft (Portabilidade 100% sem dependências externas)
Source: "..\..\bin\vcruntime140*.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\bin\msvcp140*.dll"; DestDir: "{app}"; Flags: ignoreversion
; Modelos de OCR e Inteligência Visual
Source: "C:\Program Files\ArtCraft\PdfCraft\models\*"; DestDir: "{app}\models"; Flags: ignoreversion recursesubdirs createallsubdirs
; Licenças e Documentação
Source: "..\..\NOTICE"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\LICENSE-APACHE"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\LICENSE-MIT"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\pdfcraft.ico"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon; IconFilename: "{app}\pdfcraft.ico"

[Registry]
; Metadados para controle determinístico de versão e integridade
Root: HKA; Subkey: "Software\{#MyAppName}"; ValueType: string; ValueName: "Version"; ValueData: "{#MyAppVersion}"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\{#MyAppName}"; ValueType: string; ValueName: "BuildNumber"; ValueData: "{#MyBuildNumber}"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\{#MyAppName}"; ValueType: string; ValueName: "InstallPath"; ValueData: "{app}"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\{#MyAppName}"; ValueType: string; ValueName: "InstallDate"; ValueData: "{#GetDateTimeString('yyyy-mm-dd hh:nn:ss', '-', ':')}"; Flags: uninsdeletekey

; Associação de Ícone de Documentos PDF (Fusão Modelo 5 + Faixa PDF Direita)
Root: HKA; Subkey: "Software\Classes\PdfCraft.Document"; ValueType: string; ValueName: ""; ValueData: "Documento PDF"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\PdfCraft.Document\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: """{app}\pdfcraft-document.ico"""
Root: HKA; Subkey: "Software\Classes\PdfCraft.Document\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\.pdf\OpenWithProgids"; ValueType: string; ValueName: "PdfCraft.Document"; ValueData: ""; Flags: uninsdeletevalue

; Menus de contexto em Português do Brasil para arquivos PDF
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\.pdf\shell\PdfCraft.OCR"; ValueType: string; ValueName: ""; ValueData: "Reconhecer Texto (OCR) com PDF Craft"; Tasks: contextmenu; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\.pdf\shell\PdfCraft.OCR"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\pdfcraft.ico"""; Tasks: contextmenu
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\.pdf\shell\PdfCraft.OCR\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" --ocr-shell ""%1"""; Tasks: contextmenu

Root: HKA; Subkey: "Software\Classes\PdfCraft.Document\shell\PdfCraft.OCR"; ValueType: string; ValueName: ""; ValueData: "Reconhecer Texto (OCR) com PDF Craft"; Tasks: contextmenu; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\PdfCraft.Document\shell\PdfCraft.OCR"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\pdfcraft.ico"""; Tasks: contextmenu
Root: HKA; Subkey: "Software\Classes\PdfCraft.Document\shell\PdfCraft.OCR\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" --ocr-shell ""%1"""; Tasks: contextmenu

Root: HKA; Subkey: "Software\Classes\Directory\shell\PdfCraft.OCR"; ValueType: string; ValueName: ""; ValueData: "Reconhecer Texto (OCR) da pasta com PDF Craft"; Tasks: contextmenu; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\Directory\shell\PdfCraft.OCR"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\pdfcraft.ico"""; Tasks: contextmenu
Root: HKA; Subkey: "Software\Classes\Directory\shell\PdfCraft.OCR\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" --ocr-shell ""%1"""; Tasks: contextmenu

Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\.pdf\shell\PdfCraft.Edit"; ValueType: string; ValueName: ""; ValueData: "Editar com o PDF Craft"; Tasks: contextmenu; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\.pdf\shell\PdfCraft.Edit"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\pdfcraft.ico"""; Tasks: contextmenu
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\.pdf\shell\PdfCraft.Edit\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""; Tasks: contextmenu

Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\.pdf\shell\PdfCraft.Combine"; ValueType: string; ValueName: ""; ValueData: "Combinar arquivos no PDF Craft"; Tasks: contextmenu; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\.pdf\shell\PdfCraft.Combine"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\pdfcraft.ico"""; Tasks: contextmenu
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\.pdf\shell\PdfCraft.Combine\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" --combine ""%1"""; Tasks: contextmenu

; Menu de contexto para imagens (Criar PDF)
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\.png\shell\PdfCraft.CreatePdf"; ValueType: string; ValueName: ""; ValueData: "Criar PDF com o PDF Craft…"; Tasks: contextmenu; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\.png\shell\PdfCraft.CreatePdf"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\pdfcraft.ico"""; Tasks: contextmenu
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\.png\shell\PdfCraft.CreatePdf\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""; Tasks: contextmenu

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[Code]
// Notificação da Win32 API para atualizar o cache de ícones do Windows Explorer
procedure SHChangeNotify(wEventId: Integer; uFlags: Cardinal; dwItem1, dwItem2: Integer);
  external 'SHChangeNotify@shell32.dll stdcall';

const
  SHCNE_ASSOCCHANGED = $08000000;
  SHCNF_IDLIST = $0000;

// =====================================================================
// Máquina de Estados Determinística (Anti-Bagunça / Zero Ambiguidade)
// =====================================================================
function InitializeSetup(): Boolean;
var
  InstalledVersion: String;
  InstalledBuild: String;
  CurrentVersion: String;
  CurrentBuild: String;
begin
  CurrentVersion := '{#MyAppVersion}';
  CurrentBuild := '{#MyBuildNumber}';
  Result := True;

  // Checa se já existe versão registrada
  if RegQueryStringValue(HKA, 'Software\{#MyAppName}', 'Version', InstalledVersion) then
  begin
    RegQueryStringValue(HKA, 'Software\{#MyAppName}', 'BuildNumber', InstalledBuild);

    // CASO 1: A mesma versão e compilação exata já está instalada
    if (InstalledVersion = CurrentVersion) and (InstalledBuild = CurrentBuild) then
    begin
      MsgBox('O PDF Craft já está instalado nesta mesma versão e compilação exata:' + #13#10 + #13#10 +
             '• Versão: ' + InstalledVersion + #13#10 +
             '• Compilação (Build): ' + InstalledBuild + #13#10 + #13#10 +
             'Nenhum arquivo precisou ser alterado e seu sistema permanece 100% íntegro e consistente.',
             mbInformation, MB_OK);
      Result := False; // Aborta determinísticamente sem tocar em nada
      Exit;
    end;

    // CASO 2: A versão instalada é MAIS NOVA que o instalador (Bloqueio de Downgrade)
    if (InstalledBuild > CurrentBuild) or (InstalledVersion > CurrentVersion) then
    begin
      MsgBox('Uma compilação mais recente do PDF Craft já foi detectada neste computador:' + #13#10 + #13#10 +
             '• Versão Instalada: ' + InstalledVersion + ' (Build ' + InstalledBuild + ')' + #13#10 +
             '• Versão Deste Instalador: ' + CurrentVersion + ' (Build ' + CurrentBuild + ')' + #13#10 + #13#10 +
             'A instalação foi interrompida para evitar inconsistências e regressão de arquivos.',
             mbCriticalError, MB_OK);
      Result := False; // Impede downgrade acidental
      Exit;
    end;

    // CASO 3: Atualização limpa e direta (InstalledBuild < CurrentBuild)
    // Prossegue sem exibir telas inúteis de Repair/Remove
  end;
end;

procedure InitializeWizard();
var
  CreditLabel: TNewStaticText;
begin
  CreditLabel := TNewStaticText.Create(WizardForm);
  CreditLabel.Parent := WizardForm;
  CreditLabel.Left := ScaleX(18);
  CreditLabel.Top := WizardForm.CancelButton.Top + ScaleY(6);
  CreditLabel.Caption := 'Base ArtCraft  |  Melhorias e Customizações: Marcos Pires';
  CreditLabel.Font.Style := [fsBold];
  CreditLabel.Font.Color := $0078501E;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
  begin
    // Força o Windows Shell a reconstruir a associação de ícones imediatamente
    SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, 0, 0);
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usPostUninstall then
  begin
    SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, 0, 0);
  end;
end;
