## Description
Add integration tests for `/health` and claim maturity check, and fix rate-limiting middleware issue.

Closes #101
Closes #116

## Changes proposed

### What were you told to do?
1. **Health Endpoint Test (#101)**:
   - Add integration test for `/health`.
   - Verify 200 OK and status JSON body.
2. **Claim Before Maturity (#116)**:
   - Implement maturity check for plan claims.
   - Add integration test ensuring claim fails before due date.

### What did I do?
#### Backend Implementation
- **Maturity Check**: Added logic to `PlanService::claim_plan` to verify if a plan is mature based on its distribution method and creation time using the existing `is_due_for_claim` utility.
- **Rate-Limiting Fix**: Updated `backend/src/main.rs` to use `into_make_service_with_connect_info::<SocketAddr>()`. This prevents `tower-governor` from failing with a 500 error when it cannot detect the client's IP.

#### Integration Tests
- **health_tests.rs**: Verifies `GET /health` and `GET /health/db` (from upstream).
- **claim_tests.rs**: 
  - Verifies that a claim attempt on an immature plan returns `400 Bad Request`.
  - Verifies that a claim attempt on a mature plan succeeds (200 OK).

## Check List (Check all the applicable boxes)
- [x] My code follows the code style of this project.
- [x] This PR does not contain plagiarized content.
- [x] The title and description of the PR is clear and explains the approach.
- [x] I am making a pull request against the master branch (left side).
- [x] My commit messages styles matches our requested structure.
- [x] My code additions will fail neither code linting checks nor unit test.
- [x] I am only making changes to files I was requested to.

## Screenshots / Testing Evidence
```bash
$ cargo test --test health_tests
...
test test_health_endpoint ... ok

$ cargo test --test claim_tests
...
test test_claim_after_maturity_succeeds ... ok
test test_claim_before_maturity_returns_400 ... ok
```
