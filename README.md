# bypass-hub

Web panel for managing VPN routing infrastructure.

## What it does

- Registers 3x-ui servers and MikroTik routers
- Manages route lists (domains and IPs) for selective VPN routing
- Distributes lists to endpoints via unique UUID:
    - `GET /lists/{uuid}/bypass-site.dat` — geosite protobuf for xray
    - `GET /lists/{uuid}/bypass-ip.dat` — geoip protobuf for xray
    - `GET /lists/mikrotik/{uuid}` — RouterOS script for scheduler
- Generates MikroTik init scripts for zero-touch provisioning

## Stack

- **Backend:** Rust + Axum + SQLx + SQLite
- **Frontend:** React + TypeScript + Ant Design

## MikroTik onboarding flow

1. Register MikroTik in the panel (name, server, WireGuard inbound)
2. Download generated `init.rsc`
3. Run script on the router — configures WG, DNS, routing, scheduler
4. Copy the public key printed by the script
5. Paste it into the panel → router becomes active

After that the router pulls updated route lists automatically every 24h.

## Requirements

- RouterOS 7.19+
- WAN interface list must exist on the MikroTik (`/interface list print`)
- Running 3x-ui instance with API key

## Running

```bash
cp .env.example .env
# fill BASE_URL, DATABASE_URL
sqlx migrate run
cargo run --release
```

Frontend:

```bash
cd frontend
bun install
bun run dev
```

## Environment

| Variable         | Description                                               |
|------------------|-----------------------------------------------------------|
| `DATABASE_URL`   | `sqlite://bypass-hub.db`                                  |
| `BASE_URL`       | Public URL of this server (used in MikroTik init scripts) |
| `BACKEND_PORT`   | Port to listen on (default: `8080`)                       |
| `ADMIN_USERNAME` | Admin login                                               |
| `ADMIN_PASSWORD` | Admin password                                            |
| `DEV`            | If set (any value) — dev mode; if absent — production     |