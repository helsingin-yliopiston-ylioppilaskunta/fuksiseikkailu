# Architecture

This document overlays the architecture of the fuksiseikkailu client.

## Generic

The app uses Vite.js, typescript, and react as it's basic framework.
It is divided to different components utilized by the admin and user sides.

**User side** displays information about the ongoing fuksiseikkailu event,
it's checkpoints, team scores and news.

**Admin side** allows an admin to modify the information in the application.

Both sides communicate with the server using json, which is then rendered by
the react components.

All text elements are fetched from the server to support translations.

## Components

Here is an overview of the components used by the application

### CheckpointList
CheckpointList shows all the checkpoints, which you can filter by various
criteria

### Checkpoint
Checkpoint displays all the information of one checkpoint, also allowing the
user to rate the checkpoint and report it if necessary.

### NewsList
Show all the news that have been posted.

### NewsArticle
Display one news article.

### Map
Allow browsing the checkpoints on a map.

### Scoreboard
Show information on who is winning the event

### Info
Display general info about the event

