@echo off
rem ---------------------------------------------------------------------------
rem Launch with the NVIDIA RTX max-FPS speed preset (Series S 540p, Smooth
rem Motion, UMA Bridge, and Velocity I/O enabled).
rem
rem   scripts\launch-game-fast.bat
rem   scripts\launch-game-fast.bat -NoPrecompile
rem   scripts\launch-game-fast.bat -Resolution 480
rem   scripts\launch-game-fast.bat -NoSmoothMotion
rem ---------------------------------------------------------------------------
setlocal EnableExtensions

set "VICTORY_UMA_BRIDGE=1"
set "VICTORY_VELOCITY_IO=1"
set "VICTORY_VELOCITY_IO_LEGACY_BCPACK=1"

call "%~dp0launch-game.bat" %*
exit /b %ERRORLEVEL%
