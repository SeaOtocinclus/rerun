from __future__ import annotations

import rerun as rr
import semver


def test_version() -> None:
    ver = semver.VersionInfo.parse(rr.__version__)

    assert ver.major == rr.__version_info__[0]
    assert ver.minor == rr.__version_info__[1]
    assert ver.patch == rr.__version_info__[2]

    if ver.prerelease:
        assert ver.prerelease == rr.__version_info__[3]
    else:
        # The last field is `None` if there is no prerelease.
        # TODO(andreas): We released `0.18.0` `"None"` as prerelease, but really it should be `None`.
        # assert rr.__version_info__[3] is None
        assert len(rr.__version_info__) == 4

    assert rr.__version__ in rr.version()


if __name__ == "__main__":
    test_version()
