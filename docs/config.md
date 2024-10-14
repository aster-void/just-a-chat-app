# Workspace Configuration

- Open (bool) -- open server will be open to anyone for join
- Owner([]User | len(self) > 0) -- owners can delete workspace.
- Admin ([]User | len(self) > 0) -- admins can configure workspace config.

## Roles

- default: Role -- default role
- Roles ([]Role) -- custom roles.

struct Role |

- Name: name of this role
- Color: color for this role
- Can Invite (bool) -- if someone with this role can invite
- Can Delete messages

## Channel Configuration

- publicity Publicity -- who the channel is visible to

enum Publicity |

- public -- visible to anyone
- restricted([]Role) -- restricted to those roles
- private -- only invited people can see

- use E2EE (bool | self => publicity is private)) -- whether to use E2EE.

# User Configuration

- name -- Username.

## Client Configuration

- dark mode (?bool) -- to use dark mode. if not set, will use client default.
- theme (Theme) -- theme to use.
- custom servers ([]addr) -- allow use of private servers

## Auth

- auth info -- []AuthInfo

enum AuthInfo |

- Firbase ID Token

## E2EE

- e2ee priv key (E2EEKey(priv)) -- one per client, saved client-side
- e2ee pub key ([]E2EEKey(pub)) -- one per client, saved server-side

enum E2EEKey |

- priv(PRIVATE_KEY)
- pub(PUBLIC_KEY)
