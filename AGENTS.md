# Agents and Responsibilities
This document describes the internal conceptual agents of the application. These are not runtime processes or services, but logical components that define responsibility boundaries within the codebase.
The goal is to keep the system small, testable, and easy to reason about.

## CLI Agent
Responsibility: User interaction via the terminal.
Parse command-line arguments
Enter interactive capture mode
Read user input and display output
Handle clean exits (:q, Ctrl-C)
The CLI agent does not know how notes are stored or synced. It delegates all logic to the core layer.

## Core Note Agent
Responsibility: Note creation and invariants.
Create note objects
Generate UUIDs
Attach timestamps
Enforce append-only behavior
This agent defines what a “note” is and ensures it is always valid before persistence.

## Local Storage Agent
Responsibility: Durable local persistence.
Store notes in SQLite
Retrieve notes by ID
List notes in chronological order
This agent guarantees that once a note is saved, it cannot be lost due to crashes or restarts.

## Sync State Agent
Responsibility: Local sync bookkeeping (no network).
Track sync_status (pending, synced, failed)
Record last sync attempt and error
Expose sync status summaries
This agent exists even before a remote backend is introduced.

## Remote Sync Agent (future)
Responsibility: Communication with the remote backend.
Send notes to a Cloudflare Worker API
Perform idempotent upserts by note ID
Handle authentication and retries
This agent must never block note capture and must tolerate network failure gracefully.

## Agent Interaction Rules
CLI → Core → Local Storage is the critical capture path
Sync-related agents must never interfere with capture
No agent bypasses another agent’s responsibility
Keeping these boundaries clear is essential to maintaining the app’s reliability and simplicity.
