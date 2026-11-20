import os
import re
import subprocess
from pathlib import Path


PACKAGE_FILES = (Path("runtime/Cargo.toml"), Path("macros/Cargo.toml"))
README_FILES = (Path("README.md"), Path("runtime/README.md"), Path("macros/README.md"))


def read_version(text: str) -> str:
    match = re.search(r'^version = "([^"]+)"$', text, re.MULTILINE)
    if match is None:
        raise RuntimeError("package version is missing")
    return match.group(1)


def next_version(current: str) -> str:
    major, minor, patch = map(int, current.split("."))
    if patch < 9:
        patch += 1
    else:
        minor += 1
        patch = 0
    return f"{major}.{minor}.{patch}"


def main() -> None:
    current = [read_version(path.read_text()) for path in PACKAGE_FILES]
    previous = [
        read_version(subprocess.check_output(["git", "show", f"HEAD^:{path}"], text=True))
        for path in PACKAGE_FILES
    ]
    if len(set(current)) != 1:
        raise RuntimeError(f"package versions must match: {current}")

    version = current[0]
    bumped = current == previous
    if bumped:
        updated = next_version(version)
        for path in PACKAGE_FILES:
            text = path.read_text()
            path.write_text(text.replace(f'version = "{version}"', f'version = "{updated}"', 1))
        for path in README_FILES:
            text = path.read_text()
            text = re.sub(r'(arcadia-framework = ")[0-9]+\.[0-9]+\.[0-9]+("?)', rf'\g<1>{updated}\g<2>', text)
            text = re.sub(r'(arcadia-macros = ")[0-9]+\.[0-9]+\.[0-9]+("?)', rf'\g<1>{updated}\g<2>', text)
            text = re.sub(r'(tag = "v)[0-9]+\.[0-9]+\.[0-9]+("?)', rf'\g<1>{updated}\g<2>', text)
            path.write_text(text)
        version = updated

    with open(os.environ["GITHUB_OUTPUT"], "a") as output:
        output.write(f"version={version}\n")
        output.write(f"bumped={str(bumped).lower()}\n")


if __name__ == "__main__":
    main()
