# Project log

## 9/24/22
* Fixed a panic that was occurring when trying to use the accessor to receive new values in its on_change function. This was holding up progress on the web implmentation.
* Fixed a bug that was causing an accessor's on_change function to be called with the previoius stale value.

### Next Step:
* Release a new version of the heartwood crate with these bugfixes in place.
* Resume work on the vanilla js web example and find more bugs/missing features to work on.
