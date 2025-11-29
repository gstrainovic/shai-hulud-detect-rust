✅ Completed maintenance tasks:
- cargo upgrade
- cargo fmt
- cargo clippy --fix --allow-dirty -- -W clippy::pedantic

Fixed bugs:
- Updated ureq API for v3.x (timeout via config)
- Removed needless continue in semver.rs