# phoxal-runtime-power

`phoxal-runtime-power` executes explicit reboot and poweroff commands through the
Balena Supervisor when that integration is configured.

It subscribes to `runtime/power/command` and publishes the latched
`runtime/power/state` once per second. The state starts idle, moves to accepted
when the Supervisor accepts the command, rejected when the Supervisor is missing
or returns a non-success status, and failed when the HTTP request itself fails.
The last command result remains visible until another command arrives.

Configuration:

- `--balena-supervisor-address` / `BALENA_SUPERVISOR_ADDRESS`
- `--balena-supervisor-api-key` / `BALENA_SUPERVISOR_API_KEY`

If the Supervisor address is not configured, the runtime still runs and rejects
incoming commands with `supervisor unavailable`. HTTPS Supervisor endpoints use
bundled Mozilla roots through Rustls so startup does not depend on host CA store
availability.

Not in scope: low-level autonomy products triggering power actions.
