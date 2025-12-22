# note

A terminal-based, offline-first text note application built for reliability and speed.

The app is designed around a simple rule: notes are always saved locally first. Capturing text is never blocked by network availability. Synchronization to a remote backend can happen later and independently.

This tool focuses strictly on plain text notes. There are no tags, attachments, rich formatting, editing, or deletion. Each note is append-only and stored durably in a local SQLite database. The interface is minimal and predictable, optimized for quick capture from the command line.

## Features
- Quick capture: note <text> saves a note immediately and exits
- Interactive capture: note enters a prompt to capture notes one by one
- Offline-first local storage using SQLite
- Future-ready sync model with local sync state

## Usage

### Quick capture

```sh
note this is a quick thought
```

### Interactive capture 

```sh
note 
```

Enter text to save a note. Each line creates a new note. 

Exit interactive mode with: 
```sh
:q
```

## Design principles
- Offline-first: capture must never fail due to network issues
- Transparency: clear feedback and predictable behavior Non-goals

## Roadmap (high level) 
1. Basic capture and UI 
2. Durable local persistence 
3. Read-only visibility 
4. Local sync state 
5. Deterministic sync engine 
6. Cloudflare backend integration 
7. Quality-of-life improvements 

## Target
This app aims to be small, boring, and trustworthy
