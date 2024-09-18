# Workspace Configuration

- Workspace Config
  - Open (bool) -- open server will be open to anyone for join
  - Owner([]User | len(self) > 0) -- owners can delete workspace.
  - Admin ([]User | len(self) > 0) -- admins can configure workspace config.
  - Roles ([]string) -- custom roles.
- Role Authorization
  - Allow Invitation ([]ROLE) -- who can invite (anyone with role above ROLE will be able to)

# Channel Configuration

- public (bool | !(self && use E2EE)) -- if the channel is visible to anyone in the workspace.
- use E2EE (bool | !(self && public)) -- whether to use E2EE. not allowed on a public channel.

# Client Configuration

- dark mode (?bool) -- to use dark mode. if not set, will use client default.
- theme (Theme) -- theme to use.

- warn non-E2EE (bool = false) -- show a warning if a channel doesn't use E2EE
- custom servers ([]addr) -- allow use of private servers
