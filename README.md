# Editor project

This is a text editor project. This project is currently in its infancy. 

## Goals
Roughly organized in order from near-term to pipe-dream.

Basic features:
* View text :heavy_check_mark:
* Save/edit text
* Syntax highlighting

Medium-term goals:
* Configuration
  - Discoverability (can we do better than Vim?)
* Modal editing

Stretch goals:
* autocomplete / LSP integration
* GUI frontend
* collaborative real-time editing




Two portions: frontend/backend.

Frontend has two parts: it captures input and sends to backend. backend updates and sends a diff back. frontend updates the view based on the diff.

Backend is basically a big state machine.