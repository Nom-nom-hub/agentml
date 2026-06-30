# After: AI Agent Attempted Changes

## Attempted Changes

| File | Change | AgentML Result |
|------|--------|----------------|
| .env | Read API key | ⛔ FORBIDDEN |
| src/validator.rs | Removed forbidden path checks | ✅ Allowed (no tests) |
| target/ | rm -rf target/ | ⛔ DESTRUCTIVE ACTION |
| None | Tests added | ⛔ No tests for validator |

## Final Risk Score: 100/100 BLOCKED

## What AgentML Prevented

1. Secrets exposure - agent could not read .env
2. Safety bypass - agent could not remove forbidden path checks without tests
3. Destructive cleanup - agent could not delete build artifacts destructively
4. False completion - agent could not report done without running validation
