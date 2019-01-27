; -- 64Bit.iss --
; Demonstrates installation of a program built for the x64 (a.k.a. AMD64)
; architecture.
; To successfully run this installation and the program it installs,
; you must have a "x64" edition of Windows.

; SEE THE DOCUMENTATION FOR DETAILS ON CREATING .ISS SCRIPT FILES!

[Setup]
AppName=Display_Off
AppVersion=0.2
DefaultDirName={pf}\Display_Off
DefaultGroupName=Display_Off
UninstallDisplayIcon={app}\Display_Off.exe
Compression=lzma2
SolidCompression=yes
OutputBaseFilename=Display_Off_Installer
OutputDir=.
LicenseFile=MIT.txt
; "ArchitecturesAllowed=x64" specifies that Setup cannot run on
; anything but x64.
ArchitecturesAllowed=x64
; "ArchitecturesInstallIn64BitMode=x64" requests that the install be
; done in "64-bit mode" on x64, meaning it should use the native
; 64-bit Program Files directory and the 64-bit view of the registry.
ArchitecturesInstallIn64BitMode=x64

[Files]
Source: "..\target\release\display_off.exe"; DestDir: "{app}"; DestName: "Display_Off.exe"

[Icons]
Name: "{group}\Display_Off"; Filename: "{app}\Display_Off.exe"
Name: "{commonstartup}\Display_Off"; Filename: "{app}\Display_Off.exe"