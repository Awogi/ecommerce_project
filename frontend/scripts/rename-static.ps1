<#
Rename static image files to safe kebab-case names (lowercase, spaces/underscores -> hyphens).

Run from the repository (PowerShell) like:
  cd frontend
  pwsh ./scripts/rename-static.ps1

This script will:
 - scan frontend/static for files
 - compute a sanitized filename for each (lowercase, spaces/underscores replaced with '-', remove unsafe chars)
 - rename the file and print the mapping
#>

Try {
    $scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
    $staticDir = Join-Path $scriptDir "..\static" | Resolve-Path -ErrorAction Stop
    $staticPath = $staticDir.Path
} Catch {
    Write-Error "Cannot determine static folder path. Run this script from the repository (frontend/scripts)."
    Exit 1
}

Write-Host "Renaming files in: $staticPath`n"

Get-ChildItem -Path $staticPath -File | ForEach-Object {
    $file = $_
    $orig = $file.Name

    # produce sanitized kebab-case filename
    $newName = $orig.ToLower()
    $newName = $newName -replace ' ', '-'
    $newName = $newName -replace '_', '-'
    # remove any characters except a-z, 0-9, dot and hyphen
    $newName = [regex]::Replace($newName, '[^a-z0-9\.-]', '')
    # collapse multiple hyphens
    $newName = [regex]::Replace($newName, '-{2,}', '-')

    if ($newName -ne $orig) {
        $oldPath = $file.FullName
        $newPath = Join-Path $file.DirectoryName $newName

        # if target exists, append an index
        $idx = 1
        $baseName = [System.IO.Path]::GetFileNameWithoutExtension($newName)
        $ext = [System.IO.Path]::GetExtension($newName)
        while (Test-Path $newPath) {
            $newName = "$baseName-$idx$ext"
            $newPath = Join-Path $file.DirectoryName $newName
            $idx++
            if ($idx -gt 50) { Write-Warning "Too many conflicts for $orig"; break }
        }

        try {
            Rename-Item -Path $oldPath -NewName $newName -ErrorAction Stop
            Write-Host "Renamed: $orig -> $newName"
        } Catch {
            Write-Warning ("Failed to rename {0}: {1}" -f $orig, $_)
        }
    } else {
        Write-Host "Skipped (already safe): $orig"
    }
}

Write-Host "Done. If any files were renamed, re-run your dev server (trunk serve) to pick up changes." 
