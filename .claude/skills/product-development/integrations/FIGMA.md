# Figma Integration for GUI & TUI Design

## Overview

Figma integration for designing user interfaces (GUI) and terminal user interfaces (TUI). This guide covers design-to-implementation workflows, component organization, and developer handoff.

## Figma Project Structure

```
{Project Name}/
├── 🎨 Design System
│   ├── Colors
│   ├── Typography
│   ├── Spacing
│   ├── Icons
│   └── Components
├── 📱 GUI Designs
│   ├── Desktop
│   ├── Tablet
│   └── Mobile
├── 💻 TUI Designs
│   ├── Full Screen
│   ├── Dialogs
│   └── Components
├── 🔄 User Flows
│   ├── {Flow 1}
│   └── {Flow 2}
└── 📋 Specifications
    ├── Component Specs
    └── Interaction Specs
```

## Design System Setup

### Color Tokens

```
// Colors - map to CSS/Tailwind variables
Primary/
├── primary-50:  #f0f9ff
├── primary-100: #e0f2fe
├── primary-500: #0ea5e9
├── primary-600: #0284c7
└── primary-900: #0c4a6e

Semantic/
├── success: #22c55e
├── warning: #f59e0b
├── error:   #ef4444
└── info:    #3b82f6

Neutral/
├── gray-50:  #f9fafb
├── gray-100: #f3f4f6
├── gray-500: #6b7280
├── gray-900: #111827
```

### Typography Scale

```
Heading/
├── h1: 2.25rem / 2.5rem (36px/40px)
├── h2: 1.875rem / 2.25rem (30px/36px)
├── h3: 1.5rem / 2rem (24px/32px)
├── h4: 1.25rem / 1.75rem (20px/28px)
└── h5: 1.125rem / 1.75rem (18px/28px)

Body/
├── lg: 1.125rem / 1.75rem (18px/28px)
├── base: 1rem / 1.5rem (16px/24px)
├── sm: 0.875rem / 1.25rem (14px/20px)
└── xs: 0.75rem / 1rem (12px/16px)

Monospace/ (for TUI)
├── base: 0.875rem / 1.25rem (14px/20px)
└── sm: 0.75rem / 1rem (12px/16px)
```

### Spacing Scale

```
spacing-0:  0
spacing-1:  0.25rem (4px)
spacing-2:  0.5rem (8px)
spacing-3:  0.75rem (12px)
spacing-4:  1rem (16px)
spacing-5:  1.25rem (20px)
spacing-6:  1.5rem (24px)
spacing-8:  2rem (32px)
spacing-10: 2.5rem (40px)
spacing-12: 3rem (48px)
spacing-16: 4rem (64px)
```

## GUI Component Library

### Component Naming Convention

```
{Category}/{Component}/{Variant}/{State}

Examples:
Button/Primary/Default
Button/Primary/Hover
Button/Primary/Disabled
Input/Text/Default
Input/Text/Focus
Input/Text/Error
Card/Default/Default
Card/Interactive/Hover
```

### Required Components

| Category | Components |
|----------|------------|
| **Buttons** | Primary, Secondary, Tertiary, Ghost, Destructive |
| **Inputs** | Text, Password, Number, Textarea, Select, Checkbox, Radio, Toggle |
| **Feedback** | Alert, Toast, Badge, Progress, Skeleton |
| **Navigation** | Navbar, Sidebar, Tabs, Breadcrumb, Pagination |
| **Data Display** | Table, Card, List, Avatar, Tooltip |
| **Overlays** | Modal, Drawer, Dropdown, Popover |
| **Layout** | Container, Grid, Stack, Divider |

### Component States

Each interactive component needs these states:

| State | Description |
|-------|-------------|
| Default | Normal appearance |
| Hover | Mouse over (cursor pointer) |
| Focus | Keyboard focus (visible ring) |
| Active | Being clicked/pressed |
| Disabled | Non-interactive |
| Loading | In progress |
| Error | Validation failed |
| Success | Action completed |

## TUI Component Library

### Terminal Color Palette

```
Standard Colors (ANSI):
├── Black:   #000000 / #333333 (bright)
├── Red:     #cc0000 / #ff0000 (bright)
├── Green:   #00cc00 / #00ff00 (bright)
├── Yellow:  #cccc00 / #ffff00 (bright)
├── Blue:    #0000cc / #0000ff (bright)
├── Magenta: #cc00cc / #ff00ff (bright)
├── Cyan:    #00cccc / #00ffff (bright)
└── White:   #cccccc / #ffffff (bright)

Extended (256-color):
├── Primary:   #5fafff (75)
├── Secondary: #af87ff (141)
├── Success:   #5fff5f (83)
├── Warning:   #ffaf5f (215)
└── Error:     #ff5f5f (203)
```

### TUI Component Set

```
Box Drawing Characters:
├── Corners: ┌ ┐ └ ┘ (light) ╔ ╗ ╚ ╝ (heavy)
├── Lines:   ─ │ (light) ═ ║ (heavy)
├── T-joins: ├ ┤ ┬ ┴ (light) ╠ ╣ ╦ ╩ (heavy)
└── Cross:   ┼ (light) ╬ (heavy)

Progress Indicators:
├── Spinner: ⠋ ⠙ ⠹ ⠸ ⠼ ⠴ ⠦ ⠧ ⠇ ⠏
├── Bar:     [████████░░░░░░░░]
└── Dots:    ⣾ ⣽ ⣻ ⢿ ⡿ ⣟ ⣯ ⣷

Status Icons:
├── Success: ✓ ✔ ●
├── Error:   ✗ ✘ ●
├── Warning: ⚠ △
├── Info:    ℹ ○
└── Pending: ◌ ○
```

### TUI Layout Templates

#### Full Screen Layout

```
┌─────────────────────────────────────────────────────────────┐
│ {App Name}                                    {Status} {Time}│
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  {Main Content Area}                                        │
│                                                             │
│                                                             │
│                                                             │
│                                                             │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│ {Status Bar}                                    {Keybindings}│
└─────────────────────────────────────────────────────────────┘
```

#### Split Pane Layout

```
┌──────────────────────┬──────────────────────────────────────┐
│ {Sidebar}            │ {Main Content}                        │
│                      │                                       │
│ > Item 1             │  Title                                │
│   Item 2             │  ─────                                │
│   Item 3             │                                       │
│   Item 4             │  Content goes here...                 │
│                      │                                       │
│                      │                                       │
├──────────────────────┴──────────────────────────────────────┤
│ q:quit  ↑↓:navigate  enter:select  ?:help                   │
└─────────────────────────────────────────────────────────────┘
```

#### Dialog/Modal

```
                    ┌─────────────────────────────┐
                    │ Confirm Action              │
                    ├─────────────────────────────┤
                    │                             │
                    │ Are you sure you want to    │
                    │ delete this item?           │
                    │                             │
                    │    [Cancel]    [Delete]     │
                    │                             │
                    └─────────────────────────────┘
```

## Design-to-Code Workflow

### 1. Export Design Tokens

From Figma, export as JSON:

```json
{
  "colors": {
    "primary": {
      "50": "#f0f9ff",
      "500": "#0ea5e9",
      "900": "#0c4a6e"
    }
  },
  "spacing": {
    "1": "0.25rem",
    "2": "0.5rem"
  },
  "typography": {
    "heading": {
      "h1": {
        "fontSize": "2.25rem",
        "lineHeight": "2.5rem"
      }
    }
  }
}
```

### 2. Generate CSS/Tailwind Config

```javascript
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#f0f9ff',
          500: '#0ea5e9',
          900: '#0c4a6e',
        },
      },
    },
  },
}
```

### 3. Component Specification

For each component, document:

```markdown
## Component: Button

### Figma Link
[Button Component](https://www.figma.com/file/{id}?node-id={node})

### Props
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| variant | 'primary' \| 'secondary' | 'primary' | Visual style |
| size | 'sm' \| 'md' \| 'lg' | 'md' | Button size |
| disabled | boolean | false | Disabled state |
| loading | boolean | false | Loading state |

### Measurements
| Property | sm | md | lg |
|----------|----|----|---|
| Height | 32px | 40px | 48px |
| Padding X | 12px | 16px | 24px |
| Font Size | 14px | 16px | 18px |
| Border Radius | 6px | 8px | 10px |

### States
- Default: bg-primary-500, text-white
- Hover: bg-primary-600
- Focus: ring-2 ring-primary-500 ring-offset-2
- Active: bg-primary-700
- Disabled: bg-gray-300, cursor-not-allowed
```

## Figma Plugins Recommended

| Plugin | Purpose |
|--------|---------|
| **Tokens Studio** | Design token management |
| **Figma to Code** | Export to React/Vue/HTML |
| **Contrast** | Accessibility checker |
| **Content Reel** | Realistic placeholder content |
| **Autoflow** | User flow arrows |

## Developer Handoff Checklist

For each screen/component:

- [ ] All states designed (default, hover, focus, error, etc.)
- [ ] Responsive variants (mobile, tablet, desktop)
- [ ] Design tokens used (no hardcoded values)
- [ ] Measurements annotated
- [ ] Interactions documented
- [ ] Edge cases covered (empty states, loading, errors)
- [ ] Accessibility reviewed (contrast, focus states)
- [ ] Linked to user story/use case

## TUI Framework Recommendations

| Framework | Language | Pros | Cons |
|-----------|----------|------|------|
| **Bubble Tea** | Go | Elm architecture, composable | Go only |
| **Ink** | TypeScript/React | React-like, familiar | Node dependency |
| **Textual** | Python | Rich widgets, CSS-like styling | Python only |
| **Ratatui** | Rust | Fast, flexible | Rust learning curve |
| **Charm** | Go | Full ecosystem (Gum, VHS, etc.) | Go only |

## Integration with Planning

Store Figma links in story files:

```markdown
# Story: S001_user-registration

## Design

| Screen | Figma Link | Status |
|--------|------------|--------|
| Registration Form | [Link](https://figma.com/...) | ✅ Approved |
| Success State | [Link](https://figma.com/...) | ✅ Approved |
| Error States | [Link](https://figma.com/...) | 🟡 In Review |

## TUI Design

| Screen | ASCII Preview | Status |
|--------|--------------|--------|
| Registration Form | See below | ✅ Approved |

```
┌─────────────────────────────────────┐
│ Create Account                      │
├─────────────────────────────────────┤
│                                     │
│ Email:    [                      ]  │
│ Password: [                      ]  │
│ Confirm:  [                      ]  │
│                                     │
│         [Cancel]  [Register]        │
│                                     │
└─────────────────────────────────────┘
```
```
