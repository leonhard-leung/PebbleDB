#define MyAppName "PebbleDB"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "Leonhard Leung"
#define MyAppExeName "pebbledb.exe"

[Setup]
AppId={{90200633-2fe8-407e-b529-79605f3d925b}}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
UninstallDisplayName={#MyAppName}

DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}

OutputDir=.
OutputBaseFilename=PebbleDB-Setup-{#MyAppVersion}

Compression=lzma
SolidCompression=yes

ArchitecturesInstallIn64BitMode=x64compatible

PrivilegesRequired=admin
ChangesEnvironment=yes

VersionInfoVersion={#MyAppVersion}
VersionInfoCompany={#MyAppPublisher}
VersionInfoDescription=CLI-first local database engine
VersionInfoProductName={#MyAppName}
VersionInfoProductVersion={#MyAppVersion}

[Tasks]
Name: "desktopicon"; \
    Description: "Create a desktop shortcut"; \
    GroupDescription: "Additional shortcuts:"

Name: "addtopath"; \
    Description: "Add PebbleDB to PATH"; \
    GroupDescription: "Additional options:"

[Files]
Source: "..\target\release\{#MyAppExeName}"; \
    DestDir: "{app}"; \
    Flags: ignoreversion

[Icons]
Name: "{group}\PebbleDB"; \
    Filename: "{app}\{#MyAppExeName}"

Name: "{autodesktop}\PebbleDB"; \
    Filename: "{app}\{#MyAppExeName}"; \
    Tasks: desktopicon

[Registry]
Root: HKLM; \
    Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
    ValueType: expandsz; \
    ValueName: "Path"; \
    ValueData: "{olddata};{app}"; \
    Check: WizardIsTaskSelected('addtopath'); \
    Flags: preservestringtype

[UninstallDelete]
Type: filesandordirs; \
    Name: "{app}"

[Code]

procedure RemovePebbleDBFromPath;
var
  PathValue: string;
  AppPath: string;
begin
  AppPath := ExpandConstant('{app}');

  if RegQueryStringValue(
    HKEY_LOCAL_MACHINE,
    'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
    'Path',
    PathValue
  ) then
  begin
    StringChangeEx(
      PathValue,
      ';' + AppPath,
      '',
      True
    );

    StringChangeEx(
      PathValue,
      AppPath + ';',
      '',
      True
    );

    if PathValue = AppPath then
      PathValue := '';

    RegWriteStringValue(
      HKEY_LOCAL_MACHINE,
      'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
      'Path',
      PathValue
    );
  end;
end;

procedure CurUninstallStepChanged(UninstallStep: TUninstallStep);
begin
  if UninstallStep = usUninstall then
    RemovePebbleDBFromPath;
end;