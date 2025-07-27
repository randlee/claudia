# Known Issues

## Issue #1: Duplicate Message Output (High Priority)
**Status:** Identified, needs fix  
**Branch:** `fix/duplicate-message-output`

**Problem:** Messages from Claude are being displayed 4 times in the UI.

**Root Cause:** The ClaudeCodeSession component has both generic and session-specific event listeners that fire simultaneously, causing the same message to be processed multiple times.

**Location:** 
- File: `src/components/ClaudeCodeSession.tsx`
- Function: `handleStreamMessage` (line ~517)

**Solution:** Add message deduplication logic to prevent duplicate processing.

---

## Issue #2: Incorrect Command References (Medium Priority)
**Status:** Identified, needs fix  
**Branch:** `fix/claude-command-references`

**Problem:** When users type commands without the "claude" prefix, the system tries to run "claude-code" instead of "claude".

**Root Cause:** Outdated command references or Claude AI making assumptions about command syntax.

**Solution:** Update command interpretation logic to use correct "claude" commands.

---

## Completed Fixes

### ✅ Windows Compilation Issues
- **Branch:** `windows-fixes` 
- **Status:** Fixed and merged
- Fixed Tauri icon compilation issues
- Added Windows-specific Claude binary detection
- Improved build configuration for Windows

### ✅ Claude Connection Issues  
- **Branch:** `windows-fixes`
- **Status:** Fixed and merged
- Enhanced Claude binary discovery with Windows "where" command
- Added Windows npm installation paths
- Fixed "Error loading installations" in Settings