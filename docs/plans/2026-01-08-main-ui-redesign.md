# Main UI Redesign - Design Document

> Modernize the main window with auto-hiding sidebar, subtle depth/polish, and warm/cool color tints.

## Goals

- Remove outdated feel
- Add subtle depth and polish (Linear/Notion-inspired)
- Keep it minimal and distraction-free per original design doc
- No scope creep - visual refinement only

---

## Auto-Hide Sidebar

### Behavior

- **Default state:** Editor takes full width, distraction-free writing
- **Reveal trigger:** Mouse enters left edge hot zone (~20px)
- **Animation:** Sidebar slides in, pushing editor content right (200ms ease-out)
- **Hide trigger:** User clicks in editor or starts typing
- **Animation out:** Sidebar slides away, editor expands back (150ms)

### Pin Mode

- **Pin button:** In sidebar header, toggles pinned state
- **Keyboard shortcut:** `Ctrl+B` to toggle pin
- **When pinned:** Sidebar stays visible, doesn't auto-hide

### Details

- Editor content reflows gracefully (no jarring jump)
- Subtle shadow on sidebar's right edge for depth
- Hot zone invisible, no visual indicator needed

---

## Colors

### Light Mode (warm undertone)

| Element | Color | Notes |
|---------|-------|-------|
| Editor background | `#fffefb` | Barely-there warm cream |
| Sidebar background | `#f8f7f4` | Soft warm gray |
| Primary text | `#1c1917` | Warm near-black |
| Secondary text | `#78716c` | Muted warm gray |
| Borders | `rgba(120,110,90,0.08)` | Warm tint, almost invisible |
| Hover state | `rgba(0,0,0,0.04)` | Soft background shift |
| Shadows | `rgba(100,80,60,0.08)` | Warm undertone |

### Dark Mode (deep blue undertone)

| Element | Color | Notes |
|---------|-------|-------|
| Editor background | `#0c0d10` | Hint of midnight blue |
| Sidebar background | `#12131a` | Slightly lifted, blue tint |
| Primary text | `#e4e6eb` | Cool off-white |
| Secondary text | `#71717a` | Muted cool gray |
| Borders | `rgba(140,160,200,0.08)` | Cool blue tint |
| Hover state | `rgba(255,255,255,0.06)` | Soft background shift |
| Shadows | `rgba(0,0,20,0.4)` | Deep blue undertone |

---

## Depth & Shadows

- Sidebar shadow: `0 4px 24px` with mode-appropriate color
- Minimal hard borders - use background color shifts instead
- Layers feel distinct but soft

---

## Typography

| Element | Size | Weight | Notes |
|---------|------|--------|-------|
| Note title (sidebar) | 14px | 500 | Letter-spacing: -0.01em |
| Note preview | 13px | 400 | Secondary text color |
| Timestamp | 12px | 400 | Tertiary/muted color |
| UI labels | 13px | 500 | |
| Editor body | 15px | 400 | User preference for font |

---

## Spacing

| Element | Value | Notes |
|---------|-------|-------|
| Sidebar width | 280px | When visible |
| Sidebar padding | 16px | All sides |
| Note item padding | 14px vertical | More breathing room |
| Editor horizontal margin | 48-64px | Centered, calm |
| Hot zone width | 20px | Left edge trigger |

---

## Hover & Selection States

### Hover

- Background shift, not harsh highlight
- Light: `rgba(0,0,0,0.04)`
- Dark: `rgba(255,255,255,0.06)`
- Transition: 150ms ease

### Selected Note

- Subtle accent background tint (10-15% opacity)
- 3px left border in accent color
- Smooth transition

---

## Transitions

| Action | Duration | Easing |
|--------|----------|--------|
| Sidebar reveal | 200ms | ease-out |
| Sidebar hide | 150ms | ease-in |
| Hover states | 150ms | ease |
| Content reflow | 200ms | ease-out |

---

## Frameless Title Bar (Obsidian-style)

### Layout

- Custom draggable area (~40px height)
- Left side: App name "Gravity" (subtle, 13px, weight 500)
- Right side: Window controls (minimize, maximize, close)
- Seamlessly blends with content below

### Window Controls

- Windows-style icons (line icons, not macOS dots)
- Positioned on right: `─` `□` `✕`
- Subtle by default (muted color)
- Hover: background highlight, icon more visible
- Close button hover: red background tint

### Behavior

- Entire title bar area is draggable (except buttons)
- Double-click to maximize/restore
- Background matches sidebar when visible, editor when hidden

---

## Rounded Corners

| Element | Radius |
|---------|--------|
| Window corners | 8-10px |
| Cards / Note items | 8px |
| Buttons | 6px |
| Input fields | 6px |
| Tooltips | 6px |
| Modals | 12px |

---

## General Component Polish

### Buttons

- Subtle background, no harsh borders
- Hover: gentle background shift
- Active: slightly darker
- Focus: soft accent ring (30% opacity)

### Icons

- Stroke weight: 1.5px (thinner, more refined)
- Consistent sizing: 16px for UI, 14px for inline

### Inputs

- Soft border (`rgba` based, not solid gray)
- Focus: accent-colored ring, subtle glow
- Placeholder text: muted, not distracting

### Tooltips

- Rounded (6px)
- Subtle shadow
- Quick fade-in (100ms)
- Dark background in light mode, light in dark mode

### Transitions

- All interactive elements: 150ms ease
- Layout shifts: 200ms ease-out
- Nothing instant, nothing slow

---

## Implementation Notes

### Files to modify

- `src/lib/windows/MainWindow.svelte` - layout, sidebar behavior, title bar
- `src/lib/components/TitleBar.svelte` - new component for custom title bar
- `src/lib/components/NoteList.svelte` - rounded corners, polish
- `src/lib/components/SearchBar.svelte` - input polish
- `src/app.css` - color variables, shadows, global polish
- `tailwind.config.js` - custom colors
- `src-tauri/tauri.conf.json` - set decorations: false for main window

### New state

- `sidebarVisible: boolean` - current visibility
- `sidebarPinned: boolean` - pin mode
- Mouse position tracking for edge detection

### New components

- `TitleBar.svelte` - custom frameless title bar with window controls

### Tauri changes

- Main window: `decorations: false` for frameless
- Handle window dragging via title bar
- Handle minimize/maximize/close via Tauri API

### Accessibility

- Keyboard toggle must work (`Ctrl+B`)
- Pin button must be focusable
- Sidebar should be navigable when visible
- Window controls keyboard accessible
