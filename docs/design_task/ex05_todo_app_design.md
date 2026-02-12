# Task Planner TUI

A terminal-based task planner with day navigation, input form, and
history view. Designed as a near-expert mini project for Rust TUI
practice.

## Views

### 1. Day View

    ┌──────────────────────────────────────────────────────────────┐
    │  TASK PLANNER                                                │
    │  ◀ Yesterday   Today · 2026-02-12   Tomorrow ▶               │
    ├──────────────────────────────────────────────────────────────┤
    │                                                              │
    │   ▸  09:00  Design clean architecture                          │
    │      14:00  Fix GitHub connection                               │
    │   ✓  18:00  Finish Rust TUI Level 4                             │
    │                                                              │
    │                                                              │
    ├──────────────────────────────────────────────────────────────┤
    │  Press n to add new task                                      │
    ├──────────────────────────────────────────────────────────────┤
    │  ←/→ Change day   n New   h History   ? Help   q Quit         │
    └──────────────────────────────────────────────────────────────┘

### 2. Input View

    ┌──────────────────────────────────────────────────────────────┐
    │  ADD NEW TASK                                                │
    ├──────────────────────────────────────────────────────────────┤
    │                                                              │
    │  Date: 2026-02-13                                             │
    │  Time: 14:00                                                 │
    │  Title: build clean backend                                  │
    │                                                              │
    ├──────────────────────────────────────────────────────────────┤
    │  Tab Next   Enter Save   Esc Cancel                           │
    └──────────────────────────────────────────────────────────────┘

### 3. History View

    ┌──────────────────────────────────────────────────────────────┐
    │  HISTORY                                                     │
    │  Completed & past tasks                                      │
    ├──────────────────────────────────────────────────────────────┤
    │                                                              │
    │  2026-02-10                                                  │
    │    ✓ 09:00 Fix auth bug                                      │
    │    ✓ 14:00 Deploy service                                    │
    │                                                              │
    │  2026-02-11                                                  │
    │    ✓ 11:00 Write docs                                        │
    │                                                              │
    ├──────────────────────────────────────────────────────────────┤
    │  Esc Back to planner                                         │
    └──────────────────────────────────────────────────────────────┘

### 4. Help View

    ┌──────────────────────────────────────────────────────────────┐
    │  HELP                                                        │
    ├──────────────────────────────────────────────────────────────┤
    │                                                              │
    │  ←/→    Change day                                           │
    │  ↑/↓    Move selection                                       │
    │  Enter  Toggle done                                          │
    │  n      New task                                             │
    │  d      Delete                                               │
    │  h      History                                              │
    │  q      Quit                                                 │
    │                                                              │
    ├──────────────────────────────────────────────────────────────┤
    │  Esc Back                                                    │
    └──────────────────────────────────────────────────────────────┘

## Navigation Flow

Day View -\> Input (n) -\> Save -\> Day View\
Day View -\> History (h) -\> Esc -\> Day View

## Goal

Practice layout, state management, and multi-view navigation in a Rust
TUI app.
