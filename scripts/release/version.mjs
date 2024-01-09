import semver from "semver";
import path from "path";
import { findWorkspacePackagesNoCheck } from "@pnpm/find-workspace-packages";

async function getCommitId() {
	const result = await $`git rev-parse --short HEAD`;
	return result.stdout.replace("\n", "");
}
export async function getLastVersion(root) {
	const pkgPath = path.resolve(root, "./packages/rspack/package.json");
	const result = await import(pkgPath, {
		assert: {
			type: "json"
		}
	});
	return result.default.version;
}
export async function getSnapshotVersion(lastVersion) {
	const commitId = await getCommitId();
	const dateTime = new Date()
		.toISOString()
		.replace(/\.\d{3}Z$/, "")
		.replace(/[^\d]/g, "");
	return `${lastVersion}-canary-${commitId}-${dateTime}`;
}
export async function version_handler(version) {
	const allowedVersion = ["major", "minor", "patch", "snapshot"];
	if (!allowedVersion.includes(version)) {
		throw new Error(
			`version must be one of ${allowedVersion}, but you passed ${version}`
		);
	}
	const root = process.cwd();

	const lastVersion = await getLastVersion(root);
	let nextVersion;
	if (version === "snapshot") {
		nextVersion = await getSnapshotVersion(lastVersion);
	} else {
		nextVersion = semver.inc(lastVersion, version);
	}
	const workspaces = await findWorkspacePackagesNoCheck(root);
	for (const workspace of workspaces) {
		// skip all example upgrade
		if (
			workspace.manifest.name?.includes("example-") ||
			(workspace.manifest.private === true &&
				workspace.manifest.name !== "monorepo")
		) {
			continue;
		}
		const newManifest = {
			...workspace.manifest,
			version: nextVersion
		};
		workspace.writeProjectManifest(newManifest);
	}
}