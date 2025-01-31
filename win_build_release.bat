@echo off

if exist "release\windows\assets\config.json" (
    del "release\windows\assets\config.json"
    echo File deleted: assets\config.json
) else (
    echo File not found: release\windows\assets\config.json
)

REM Build the project with cargo in release mode for the entire workspace
cargo build --release --workspace

REM Create the release/windows directory if it does not exist
if not exist release\windows (
    mkdir release\windows
)

REM Copy assets to the release/windows/assets directory
xcopy /E /I /Y target\release\assets release\windows\assets

REM Copy the executables to the release/windows directory
copy /Y target\release\Group13.exe release\windows\Group13.exe
copy /Y target\release\spawn_gui.exe release\windows\spawn_gui.exe
copy /Y target\release\spawn_popup.exe release\windows\spawn_popup.exe
copy /Y target\release\uninstall.exe release\windows\uninstall.exe

echo Build and copy process completed successfully.
