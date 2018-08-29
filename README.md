# Zircond
   An experimental IRC server written in Rust.

## Goals
Eventually, good compatibility with [RFC1459] and [RFC2813].

However, Zircond strives to be a fast and modern IRC server, so targeting old and/or obsolete parts of the IRC protocol or implementing obscure features used or even invented by some other IRC servers out there is not part of our agenda.

Also, Zircond will most likely provide very minimal CTCP support and no DCC support at all.

## Usability
At the moment, Zircond is usable as a very basic IRC server.

However, please keep in mind the following limitations:
- No SSL support yet, so there's no protection against eavesdropping.
- No ident server / SASL support yet, so there's no protection against impersonation.
- Zircond only supports a small subset of commands, and no user- or channel-modes. The lack of modes also means that there is no way for any user to obtain server- or channel-operator status, so there is no support for channel moderation, banning or silencing users, excluding users from channels, etc.

**Use at your own risk, no guarantees of any kind given.**

## Features

Fully implemented:
- [x] PING

Fully implemented (testing):
- [x] NICK
  - [x] Collision detection (ERR_NICKNAMEINUSE)
- [x] PRIVMSG
  - [x] User to Channel
  - [x] User to User
- [x] PART
  - [x] Multiple channels
  - [x] Notify other users
  - [x] Custom part message

Partially implemented:
- [x] USER
  - [x] Username/Realname
  - [ ] Hostname/Servername
- [x] JOIN
  - [x] Multiple channels
  - [x] Notify other users
  - [ ] Channel keys


[RFC1459]: https://tools.ietf.org/html/rfc1459
[RFC2813]: https://tools.ietf.org/html/rfc2813