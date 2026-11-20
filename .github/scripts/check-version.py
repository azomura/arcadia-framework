import os
import re
from pathlib import Path


PACKAGE_FILES = (Path("runtime/Cargo.toml"), Path("macros/Cargo.toml"))


def read_version(path: Path) -> str:
    match = re.search(r'^version = "([^"]+)"$', path.read_text(), re.MULTILINE)
    if match is None:
        raise RuntimeError(f"package version is missing in {path}")
    return match.group(1)


versions = [read_version(path) for path in PACKAGE_FILES]
if len(set(versions)) != 1:
    raise SystemExit(f"package versions must match: {versions}")

expected = os.environ.get("EXPECTED_VERSION", "").removeprefix("v")
if expected and versions[0] != expected:
    raise SystemExit(f"tag version {expected} does not match package version {versions[0]}")

print(f"validated package version {versions[0]}")
