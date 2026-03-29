@echo off
where mdbook >nul 2>&1
if errorlevel 1 (
  echo mdbook not found. Install with: cargo install mdbook
  exit /b 1
)
mdbook build book
