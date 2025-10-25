@echo off
echo Building Kanban Overlay...
echo.

REM Check if Rust is installed
where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Error: Rust is not installed!
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo Building release version (optimized)...
cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ========================================
    echo Build successful!
    echo ========================================
    echo.
    echo Executable location:
    echo target\release\kanban-overlay.exe
    echo.
    echo To run:
    echo   cargo run --release
    echo   or
    echo   .\target\release\kanban-overlay.exe
    echo.
    echo Press Ctrl+Shift+K to toggle the overlay
    echo Press Escape to hide it
    echo.
) else (
    echo.
    echo Build failed! Check the errors above.
)

pause
