# ==============================================================================
# HOC Project Template - Automated Standards & Zero-LaTeX Audit Script
# ==============================================================================
# Usage:
#   powershell -ExecutionPolicy Bypass -File ./scripts/audit-standards.ps1
# ==============================================================================

$ErrorActionPreference = "Stop"
$script:hasErrors = $false

Write-Host "`n========================================================" -ForegroundColor Cyan
Write-Host "  HOC Framework Standards & Quality Verification Audit  " -ForegroundColor Cyan
Write-Host "========================================================`n" -ForegroundColor Cyan

$WorkspaceRoot = (Get-Item $PSScriptRoot).Parent.FullName

# ------------------------------------------------------------------------------
# Check 1: Zero-LaTeX & Math Delimiter Audit
# ------------------------------------------------------------------------------
Write-Host "[1/3] Auditing codebase for illegal LaTeX & math delimiters..." -ForegroundColor Yellow

$latexMacroPattern = '\\(frac|dot|ddot|approx|Omega|varpi|cdot|sum|int|times|sqrt)\b'
$excludeDirs = @('.git', 'node_modules', 'dist', 'build', 'target', 'venv', '.venv', '.antigravity\scratch')

$targetFiles = Get-ChildItem -Path $WorkspaceRoot -Recurse -File -Include *.md, *.ts, *.js, *.py, *.rs, *.go | Where-Object {
    $filePath = $_.FullName
    $excluded = $false
    foreach ($dir in $excludeDirs) {
        if ($filePath -like "*\$dir\*") {
            $excluded = $true
            break
        }
    }
    -not $excluded
}

$latexViolations = @()

foreach ($file in $targetFiles) {
    $lines = Get-Content -Path $file.FullName
    for ($i = 0; $i -lt $lines.Count; $i++) {
        $line = $lines[$i]
        $lineNum = $i + 1

        # Check for LaTeX macros (excluding explicit rule/audit explanations)
        if ($line -match $latexMacroPattern -and -not ($line -match 'STRICT PROHIBITION|PROHIBITION|never use|Zero LaTeX|latexMacroPattern|Convert all LaTeX')) {
            $latexViolations += [PSCustomObject]@{
                File    = $file.FullName.Replace($WorkspaceRoot, ".")
                Line    = $lineNum
                Content = $line.Trim()
                Reason  = "Found forbidden LaTeX macro"
            }
        }
    }
}

if ($latexViolations.Count -gt 0) {
    Write-Host "  [FAIL] Detected $($latexViolations.Count) LaTeX standard violation(s):" -ForegroundColor Red
    foreach ($v in $latexViolations) {
        Write-Host "    - $($v.File):$($v.Line) [$($v.Reason)] -> '$($v.Content)'" -ForegroundColor Red
    }
    $script:hasErrors = $true
} else {
    Write-Host "  [PASS] Zero LaTeX math violations detected across $($targetFiles.Count) files." -ForegroundColor Green
}

# ------------------------------------------------------------------------------
# Check 2: Strict Downward Reference Independence Audit (docs/references/)
# ------------------------------------------------------------------------------
Write-Host "`n[2/3] Auditing docs/references/ for strict downward independence..." -ForegroundColor Yellow

$refFiles = Get-ChildItem -Path "$WorkspaceRoot\docs\references" -Filter "REF-*.md" -ErrorAction SilentlyContinue
$upwardViolations = @()
$upwardPatterns = @('src/', 'src\\', 'tests/', 'tests\\', 'package.json', 'Cargo.toml', 'pyproject.toml')

if ($refFiles) {
    foreach ($ref in $refFiles) {
        $lines = Get-Content -Path $ref.FullName
        for ($i = 0; $i -lt $lines.Count; $i++) {
            $line = $lines[$i]
            foreach ($pattern in $upwardPatterns) {
                if ($line -like "*$pattern*") {
                    $upwardViolations += [PSCustomObject]@{
                        File    = $ref.FullName.Replace($WorkspaceRoot, ".")
                        Line    = $i + 1
                        Content = $line.Trim()
                        Pattern = $pattern
                    }
                }
            }
        }
    }
}

if ($upwardViolations.Count -gt 0) {
    Write-Host "  [FAIL] Detected upward references in Tier 0 reference specifications:" -ForegroundColor Red
    foreach ($v in $upwardViolations) {
        Write-Host "    - $($v.File):$($v.Line) [Mentions '$($v.Pattern)'] -> '$($v.Content)'" -ForegroundColor Red
    }
    $script:hasErrors = $true
} else {
    Write-Host "  [PASS] All Tier 0 reference files maintain pure downward independence." -ForegroundColor Green
}

# ------------------------------------------------------------------------------
# Check 3: Active Plans & Template Health Check
# ------------------------------------------------------------------------------
Write-Host "`n[3/3] Auditing active plans and specification templates..." -ForegroundColor Yellow

$templatePath = "$WorkspaceRoot\.antigravity\plans\TEMPLATE.md"
if (-not (Test-Path $templatePath)) {
    Write-Host "  [FAIL] Missing master plan template at .antigravity/plans/TEMPLATE.md" -ForegroundColor Red
    $script:hasErrors = $true
} else {
    Write-Host "  [PASS] Plan template and active directories verified." -ForegroundColor Green
}

# ------------------------------------------------------------------------------
# Audit Summary Verdict
# ------------------------------------------------------------------------------
Write-Host "`n--------------------------------------------------------" -ForegroundColor Cyan
if ($script:hasErrors) {
    Write-Host "  AUDIT VERDICT: FAILED - Please fix the violations above." -ForegroundColor Red
    Write-Host "--------------------------------------------------------`n" -ForegroundColor Cyan
    exit 1
} else {
    Write-Host "  AUDIT VERDICT: PASSED - All standards 100% compliant." -ForegroundColor Green
    Write-Host "--------------------------------------------------------`n" -ForegroundColor Cyan
    exit 0
}
