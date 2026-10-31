---
name: Crypt
description: A local password vault filed like a library card catalog.
colors:
  board: "#4b2a9a"
  board-hover: "#5a35b4"
  board-ink: "#f5f1ff"
  board-muted: "#cbbcf3"
  active: "#6239d6"
  active-dark: "#b89dff"
  select: "#ece4ff"
  select-dark: "#2d2445"
  rule: "#e867c3"
  weak: "#c2410c"
  weak-dark: "#fb8b4e"
  fair: "#a85a06"
  fair-dark: "#f2b54b"
  strong: "#15803d"
  strong-dark: "#4fd585"
  paper: "#fbfaff"
  paper-dark: "#1b1626"
  ground: "#f2eff9"
  ground-dark: "#13101b"
  card: "#ffffff"
  card-dark: "#221c2f"
  ink: "#1e1730"
  ink-dark: "#eee8fa"
  muted: "#5f5576"
  muted-dark: "#aba1c2"
  hair: "#e8e2f3"
  hair-dark: "#2f2842"
  line: "#d3c9e8"
  line-dark: "#42395a"
typography:
  headline:
    fontFamily: "Atkinson Hyperlegible Next Variable, ui-sans-serif, system-ui, sans-serif"
    fontSize: "30px"
    fontWeight: 800
    lineHeight: 1.25
    letterSpacing: "-0.01em"
  title:
    fontFamily: "Atkinson Hyperlegible Next Variable, ui-sans-serif, system-ui, sans-serif"
    fontSize: "16px"
    fontWeight: 700
    lineHeight: 1.5
  body:
    fontFamily: "Atkinson Hyperlegible Next Variable, ui-sans-serif, system-ui, sans-serif"
    fontSize: "14px"
    fontWeight: 400
    lineHeight: 1.43
  body-strong:
    fontFamily: "Atkinson Hyperlegible Next Variable, ui-sans-serif, system-ui, sans-serif"
    fontSize: "14px"
    fontWeight: 600
    lineHeight: 1.25
  label:
    fontFamily: "Atkinson Hyperlegible Next Variable, ui-sans-serif, system-ui, sans-serif"
    fontSize: "11px"
    fontWeight: 600
    letterSpacing: "0.08em"
  tab:
    fontFamily: "Atkinson Hyperlegible Next Variable, ui-sans-serif, system-ui, sans-serif"
    fontSize: "13px"
    fontWeight: 800
  typed:
    fontFamily: "Courier Prime, ui-monospace, monospace"
    fontSize: "17px"
    fontWeight: 400
  typed-input:
    fontFamily: "Courier Prime, ui-monospace, monospace"
    fontSize: "15px"
    fontWeight: 400
  typed-small:
    fontFamily: "Courier Prime, ui-monospace, monospace"
    fontSize: "13px"
    fontWeight: 400
  stamp:
    fontFamily: "Courier Prime, ui-monospace, monospace"
    fontSize: "13px"
    fontWeight: 700
    lineHeight: 1
    letterSpacing: "0.12em"
rounded:
  stamp: "3px"
  md: "6px"
  lg: "8px"
  full: "9999px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "12px"
  lg: "16px"
  xl: "24px"
  card: "32px"
components:
  button-primary:
    backgroundColor: "{colors.board}"
    textColor: "{colors.board-ink}"
    typography: "{typography.body-strong}"
    rounded: "{rounded.lg}"
    padding: "0 16px"
    height: "40px"
  button-primary-hover:
    backgroundColor: "{colors.board-hover}"
  button-secondary:
    backgroundColor: "{colors.card}"
    textColor: "{colors.ink}"
    typography: "{typography.body-strong}"
    rounded: "{rounded.lg}"
    padding: "0 16px"
    height: "40px"
  button-danger:
    backgroundColor: "{colors.ink}"
    textColor: "{colors.card}"
    typography: "{typography.body-strong}"
    rounded: "{rounded.lg}"
    padding: "0 16px"
    height: "40px"
  button-ghost:
    textColor: "{colors.muted}"
    typography: "{typography.body-strong}"
    rounded: "{rounded.lg}"
    padding: "0 16px"
    height: "40px"
  button-ghost-hover:
    backgroundColor: "{colors.select}"
    textColor: "{colors.ink}"
  input:
    backgroundColor: "{colors.card}"
    textColor: "{colors.ink}"
    typography: "{typography.body}"
    rounded: "{rounded.lg}"
    padding: "0 12px"
    height: "40px"
  input-typed:
    backgroundColor: "{colors.card}"
    textColor: "{colors.ink}"
    typography: "{typography.typed}"
    rounded: "{rounded.lg}"
    padding: "0 12px"
    height: "40px"
  guide-tab:
    backgroundColor: "{colors.board}"
    textColor: "{colors.board-ink}"
    typography: "{typography.tab}"
    height: "24px"
    width: "52px"
  index-card:
    backgroundColor: "{colors.card}"
    textColor: "{colors.ink}"
    rounded: "{rounded.lg}"
    padding: "28px 32px"
    width: "640px"
  drawer-row:
    backgroundColor: "{colors.paper}"
    textColor: "{colors.ink}"
    typography: "{typography.body-strong}"
    padding: "0 0 0 16px"
    height: "44px"
  drawer-row-selected:
    backgroundColor: "{colors.select}"
    textColor: "{colors.active}"
  rail:
    backgroundColor: "{colors.board}"
    textColor: "{colors.board-muted}"
    width: "64px"
  stamp:
    textColor: "{colors.active}"
    typography: "{typography.stamp}"
    rounded: "{rounded.stamp}"
    padding: "6px 8px"
---

# Design System: Crypt

## Overview

**Creative North Star: "The Card Catalog Drawer"**

Crypt files every login as a typed index card in a library card-catalog drawer. A violet pressboard rail holds the drawer's tools. Logins sit A–Z behind violet guide tabs that step across the drawer in thirds and stick while you scroll. Opening a login lifts its index card into the pane. The card carries the matching guide tab on its top edge and a thin pink header rule. Credential data is typewritten in Courier Prime. Confirmations land as tilted rubber stamps.

The density suits a working tool: 44px rows, 40px controls, 1px hairlines, and no shadows. Depth comes from paper stock: a tinted ground, a lighter drawer paper, and white cards. Color has fixed meanings. Violet means yours or active. Pink appears only as the card rule and in the logo. Vermilion, amber, and green belong to password strength. Light and dark themes follow the OS. Violet pressboard and pink stay constant across both.

**Key Characteristics:**
- Violet pressboard rail and guide tabs (#4b2a9a), identical in light and dark.
- Index cards: white stock, 1px hairline border, 2px pink header rule.
- Typed credentials in Courier Prime; UI in Atkinson Hyperlegible Next.
- Rubber stamps for confirmations: Courier caps in a bordered box, tilted −3°.
- Flat surfaces separated by hairlines; no shadows, no gradients outside the logo.

## Colors

A cool violet card-stock palette with one violet voice and a reserved strength traffic light.

### Primary
- **Pressboard Violet** (board): rail background, guide tabs, the 2px top edge of tabbed cards, and the primary button. Same value in both themes.
- **Pressboard Hover** (board-hover): hover fill for the primary button.
- **Pressboard Ink** (board-ink): text and icons on pressboard, and the rail's solid New button.
- **Pressboard Muted** (board-muted): idle rail icons.
- **Active Violet** (active / active-dark): focus outlines, input focus border and ring, selected row title, current thumb-index letter, link text, caret, and the default stamp tone.
- **Selection Lilac** (select / select-dark): selected drawer row, hover wash on ghost and icon buttons, and the recovery-question panel.

### Secondary
- **Catalog Pink** (rule): the index card's 2px header rule and the logo gradient. Nowhere else.

### Tertiary
- **Vermilion** (weak / weak-dark): weak-password stamps, the weak meter segments, the drawer's weak count, and inline error messages.
- **Amber** (fair / fair-dark): fair-strength meter and label.
- **Ledger Green** (strong / strong-dark): strong meter and label, strong stamps, and the copy button's brief check confirmation.

### Neutral
- **Ground** (ground / ground-dark): app body and the card pane background.
- **Drawer Paper** (paper / paper-dark): the drawer column and sticky guide-tab bands.
- **Card Stock** (card / card-dark): index cards, inputs, secondary buttons, kbd keys.
- **Ink** (ink / ink-dark): primary text and the solid danger button.
- **Muted Ink** (muted / muted-dark): field labels, usernames in rows, meta lines, placeholders.
- **Hairline** (hair / hair-dark): row dividers, card borders, field separators.
- **Line** (line / line-dark): input and secondary-button borders, scrollbar thumb, the lock card's border.

### Named Rules
**The Yours-Is-Violet Rule.** Violet marks only what is yours or active: the rail, guide tabs, the primary action, selection, and focus. Never use violet for decoration or status.

**The Pink Rule Rule.** Catalog Pink appears only as the index card's 2px header rule and in the logo. It never fills, outlines, or labels anything.

**The Strength Owns Traffic Lights Rule.** Vermilion, amber, and green report password strength. Vermilion also carries error text. Destructive actions use solid ink, never vermilion.

## Typography

**Display Font:** Atkinson Hyperlegible Next Variable (with ui-sans-serif, system-ui)
**Body Font:** Atkinson Hyperlegible Next Variable
**Label/Mono Font:** Courier Prime 400/700 (with ui-monospace)

**Character:** A highly legible humanist sans runs the interface. A typewriter face types the card data, so stored values read as filed records.

### Hierarchy
- **Headline** (800, 30px, 1.25, −0.01em): the open card's title (the site name). Drops to 24px below 640px. The lock card title uses it too.
- **Title** (700, 16px): section headings inside cards (settings sections, Weak logins, Keyboard).
- **Body** (400, 14px): meta lines, helper text, empty states. Notes use it with relaxed leading, max 60ch.
- **Body Strong** (600, 14px): row site names, buttons, inline errors.
- **Label** (600, 11px, 0.08em, uppercase): field labels on forms and the card's definition terms (Username, Password, Notes).
- **Tab** (800, 13px): guide-tab letters.
- **Typed** (Courier Prime 400, 17px on cards, 15px in inputs, 13px in rows): usernames, passwords, notes inputs, countdown notes.
- **Stamp** (Courier Prime 700, 13px, 0.12em, uppercase; 11px small): stamps only.

### Named Rules
**The Typewriter Rule.** Courier Prime is for stored credential data and stamps. Interface text, headings, and buttons stay in Atkinson.

## Layout

The unlocked app is a three-column grid: a 64px rail, a drawer of 300–380px, and a flexible pane. The pane holds one card at a time, max 640px wide, padded 28px × 32px. Below 760px the grid becomes a 56px rail plus the drawer. The pane then covers the drawer as a fixed overlay, only while it has something to show. Each card shows a "Drawer" back button there. Below 640px card padding tightens to 20px.

Drawer rows are 44px tall with a 16px left inset. Guide-tab bands are 32px tall and sticky. Their tabs step across the drawer width in thirds by group order. An A–Z thumb index (24px wide) appears on the right edge once 15 or more logins are visible. Card field rows use a 112px label column, value, then actions.

Spacing follows a 4px base: 4, 8, 12, 16, 24, and 32px. The lock screen centers a single 360px card on the ground.

## Elevation & Depth

The system is flat. No element casts a shadow. Depth comes from paper tone (ground, drawer paper, card stock) and 1px hairlines. Inputs get a 3px Active Violet ring at 20% on focus. That ring is a state signal, not elevation.

### Named Rules
**The Hairline Rule.** Surfaces separate with 1px Hairline or Line borders and tonal steps. Never add box-shadows.

## Shapes

Corners are gently rounded: 8px on cards, buttons, inputs, and icon buttons; 6px on guide-tab tops and small inline controls; 3px on stamps; fully round on strength-meter segments. A card carrying a guide tab has square top corners and a 2px Pressboard Violet top edge. The tab (52px × 24px, 6px top radius) sits on that edge. On entry cards the tab aligns with its letter's third in the drawer. Stamps rotate −3°.

## Components

### Buttons
Quiet, solid, and compact.
- **Shape:** gently rounded (8px), 40px tall, 16px side padding, 14px semibold.
- **Primary:** Pressboard Violet with Pressboard Ink text; hover to Pressboard Hover. Presses 1px down.
- **Secondary:** Card Stock with a Line border; hover tints the border violet and washes Selection Lilac.
- **Danger:** solid Ink with Card Stock text; hover lightens to 85%. Used for confirmed deletes only.
- **Ghost:** Muted Ink text, no fill; hover washes Selection Lilac and darkens text to Ink.
- **Focus:** 2px Active Violet outline, 2px offset. Disabled drops to 45% opacity.
- **Icon buttons:** 36px square, Muted Ink icon, hover to Selection Lilac with Active Violet icon. Icons are 1.75px-stroke line SVGs.

### Cards / Containers
- **Corner Style:** 8px; square top when tabbed.
- **Background:** Card Stock.
- **Shadow Strategy:** none (see Elevation & Depth).
- **Border:** 1px Hairline; 2px Pressboard Violet top when tabbed.
- **Internal Padding:** 28px top, 32px sides; header, then 2px pink rule, then body.
- **Motion:** cards rise 10px from 40% opacity over 220ms on open.

### Inputs / Fields
- **Style:** Card Stock fill, 1px Line border, 8px radius, 40px tall, 12px inset.
- **Focus:** border turns Active Violet with a 3px ring at 20%. Hover tints the border violet at 50%.
- **Typed variant:** credential fields use Courier Prime at 15px.
- **Error:** message below in Vermilion, 14px semibold. The lock card shakes on a failed unlock.

### Navigation
- **Rail:** Pressboard Violet column with the light logo mark, a solid Pressboard Ink New button, then Settings and Lock at the bottom. Idle icons are Pressboard Muted. Current is Pressboard Ink on a 15% Pressboard Ink wash.
- **Drawer rows:** site name (14px semibold) over typed username (13px Muted). Selected rows wash Selection Lilac with an Active Violet title. Each row has a copy button.
- **Search:** standard input with a leading icon and a `/` kbd hint.

### Guide Tabs (signature)
Pressboard Violet tabs (52px × 24px, 6px top radius, 13px extrabold letter) on a 2px Pressboard Violet band. They stick while scrolling and stagger across thirds. The lock card's tab names the vault state in uppercase instead of a letter.

### Stamps (signature)
Courier Prime bold caps in a 1.5px bordered box, 3px radius, rotated −3°. Tones: Active Violet (default confirmations such as Filed and Copied), Vermilion (Weak), Ledger Green (strong). Full stamps land once over 180ms from 130% scale. Small stamps (10px) sit inline in rows without motion. A typed countdown can sit under a stamp.

### Strength Meter
Four 4px rounded segments, 128px wide total, filled Vermilion, Amber, or Ledger Green. Empty segments use Hairline. A 12px semibold label in the same color follows.

## Do's and Don'ts

### Do:
- **Do** keep violet for yours and active: rail, guide tabs, primary action, selection, focus.
- **Do** use the 2px Catalog Pink rule under every index card header, and pink nowhere else outside the logo.
- **Do** separate surfaces with 1px Hairline borders and paper-tone steps.
- **Do** type stored credentials and stamps in Courier Prime; keep interface text in Atkinson Hyperlegible Next.
- **Do** confirm actions with a −3° stamp rather than a toast.
- **Do** give tabbed cards square top corners and a 2px Pressboard Violet top edge.
- **Do** respect prefers-reduced-motion; all animation collapses to 1ms.

### Don't:
- **Don't** add box-shadows to cards, rows, buttons, or menus.
- **Don't** use gradients anywhere except the logo mark.
- **Don't** use vermilion, amber, or green for anything but strength, with errors the one vermilion exception.
- **Don't** style destructive actions in vermilion; they use solid Ink.
- **Don't** use pink as a fill, border, text color, or badge.
- **Don't** set interface headings or buttons in Courier Prime.
