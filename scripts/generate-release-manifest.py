#!/usr/bin/env python3
"""Generate the versioned release manifest consumed by AlacoUp."""

import hashlib
import json
import os
from pathlib import Path


def describe(path: Path, base_url: str) -> dict[str, str]:
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    return {
        "filename": path.name,
        "url": f"{base_url}/{path.name}",
        "sha256": digest,
    }


def main() -> None:
    repository = os.environ["GITHUB_REPOSITORY"]
    tag = os.environ["GITHUB_REF_NAME"]
    version = tag.removeprefix("v")
    base_url = f"https://github.com/{repository}/releases/download/{tag}"

    assets = sorted(Path(".").glob("*.tar.gz"))
    stdlib_path = Path(f"alaco-{version}-stdlib.tar.gz")
    if stdlib_path not in assets:
        raise SystemExit(f"missing standard library archive: {stdlib_path}")

    targets: dict[str, dict[str, dict[str, str]]] = {}
    prefix = f"alaco-{version}-"
    for path in assets:
        if path == stdlib_path or not path.name.startswith(prefix):
            continue

        suffix = path.name[len(prefix) :]
        if suffix.endswith("-binary.tar.gz"):
            target = suffix.removesuffix("-binary.tar.gz")
            targets.setdefault(target, {})["binary"] = describe(path, base_url)
        elif suffix.endswith(".tar.gz"):
            target = suffix.removesuffix(".tar.gz")
            targets.setdefault(target, {})["bundle"] = describe(path, base_url)

    if not targets:
        raise SystemExit("no platform archives found")
    for target, downloads in targets.items():
        if set(downloads) != {"binary", "bundle"}:
            raise SystemExit(f"incomplete archives for {target}: {sorted(downloads)}")

    manifest = {
        "schema_version": 1,
        "version": version,
        "tag": tag,
        "channel": "prerelease" if "-" in version else "stable",
        "stdlib": describe(stdlib_path, base_url),
        "targets": targets,
    }
    Path("alacoup-manifest.json").write_text(
        json.dumps(manifest, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
