# School Uniform Store — Dev & Integration Notes

This README provides quick instructions to run the frontend and backend together during development and to produce a single-server production build where the backend serves the built frontend assets.

## Dev workflow (recommended for school demos)

1. Start the backend (first). This prints the bound port and LAN URL. The backend will try the port configured in `backend/.env` (default `8080`) and fall back to the next free ports if needed.

```powershell
cd backend
# Use this to run with the .env values
cargo run
# Or run with an explicit port for determinism:
$env:PORT="8081"
cargo run
```

2. Start the frontend dev server (Trunk) in a separate terminal. Pick a port that doesn't conflict with the backend (we usually use `8082`):

```powershell
cd frontend
trunk serve --port 8082
```

3. Point the frontend to the backend. The frontend resolves the API base using this precedence:
- `localStorage['api_base']` (runtime override)
- `<meta name="api-base" content="...">` in `frontend/index.html`
- fallback `http://127.0.0.1:8080`

Quick runtime override (no rebuild): open the browser console for the Trunk-served app and run:

```javascript
localStorage.setItem('api_base', 'http://127.0.0.1:8081');
```

Then open the frontend URL (e.g. `http://127.0.0.1:8082`) and the app will call the backend at that URL.

Note: there is a `scripts/dev.ps1` helper that starts backend and frontend in new PowerShell windows. From the repo root run:

```powershell
.\scripts\dev.ps1
# Or pass ports:
.\scripts\dev.ps1 -BackendPort 8081 -FrontendPort 8082
```

## Production build — single server

1. Build the frontend with Trunk and copy the `dist/` output into `backend/static`.

You can use the included script which builds and copies automatically:

```powershell
.\frontend\scripts\deploy_frontend.ps1
```

2. Start the backend (it will now serve the frontend at `/` and the API at `/api`):

```powershell
cd backend
cargo run --release
# or for dev testing without --release
cargo run
```

3. Open the backend Local URL printed by the server (e.g. `http://127.0.0.1:8080`). The frontend is served from `/` and the API endpoints are exposed under `/api/*`.

## Environment variables

- `backend/.env` contains recommended development defaults:
  - `DATABASE_URL` — Postgres connection string
  - `HOST` — host to bind (default `127.0.0.1`)
  - `PORT` — starting port to try (default `8080`)
  - `PORT_TRIES` — how many incremental ports the backend will try if the configured port is in use (default `10`)
  - `RUST_LOG` — logs level

Edit `backend/.env` to change these values for your environment.

## Notes and tips

- For development you can keep CORS permissive, but restrict it for production deployments.
- If you see address-in-use errors, either stop the process using the port or change `PORT` in `backend/.env` or pass a different `-BackendPort` to `dev.ps1`.
- The frontend's runtime `api_base()` allows you to switch backend targets without rebuilding. Prefer the `localStorage` override during development.

If you'd like, I can also add a short `Makefile` or extra PowerShell helpers to automate the release steps further.
