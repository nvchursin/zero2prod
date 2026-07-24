#!/usr/bin/env bash
set -Eeuo pipefail

readonly DEPLOY_DIR="/opt/zero2prod"
readonly IMAGE_NAME="zero2prod"
readonly NEW_TAG="${1:-}"
readonly IMAGE_ARCHIVE="${DEPLOY_DIR}/zero2prod-image.tar.gz"
readonly INCOMING_ENV="${DEPLOY_DIR}/production.env"
readonly ACTIVE_ENV="${DEPLOY_DIR}/.env"
readonly PREVIOUS_ENV="${DEPLOY_DIR}/.env.previous"
readonly CURRENT_RELEASE="${DEPLOY_DIR}/.current-release"
readonly PREVIOUS_RELEASE="${DEPLOY_DIR}/.previous-release"

cd "$DEPLOY_DIR"
umask 077

exec 9>"${DEPLOY_DIR}/.deploy.lock"
if ! flock -n 9; then
  echo "Another deployment is already running." >&2
  exit 1
fi

if [[ ! "$NEW_TAG" =~ ^[0-9a-f]{40}$ ]]; then
  echo "Usage: $0 <40-character-git-sha>" >&2
  exit 1
fi

for required_file in "$IMAGE_ARCHIVE" "$INCOMING_ENV" compose.yaml deploy/Caddyfile; do
  if [[ ! -f "$required_file" ]]; then
    echo "Required deployment file is missing: $required_file" >&2
    exit 1
  fi
done

previous_tag=""
if [[ -f "$CURRENT_RELEASE" ]]; then
  previous_tag="$(<"$CURRENT_RELEASE")"
  if [[ ! "$previous_tag" =~ ^[0-9a-f]{40}$ ]]; then
    echo "Ignoring invalid current release marker." >&2
    previous_tag=""
  fi
fi

had_active_env=false
if [[ -f "$ACTIVE_ENV" ]]; then
  had_active_env=true
  install -m 600 "$ACTIVE_ENV" "$PREVIOUS_ENV"
fi

restore_active_env() {
  if [[ "$had_active_env" == true ]]; then
    install -m 600 "$PREVIOUS_ENV" "$ACTIVE_ENV"
  else
    rm -f "$ACTIVE_ENV"
  fi
}

install -m 600 "$INCOMING_ENV" "${ACTIVE_ENV}.new"
mv -f "${ACTIVE_ENV}.new" "$ACTIVE_ENV"
rm -f "$INCOMING_ENV"

export IMAGE_TAG="$NEW_TAG"
if ! docker compose --env-file "$ACTIVE_ENV" config --quiet; then
  echo "The production Compose configuration is invalid." >&2
  restore_active_env
  exit 1
fi

echo "Loading image ${IMAGE_NAME}:${NEW_TAG}..."
if ! gzip -dc "$IMAGE_ARCHIVE" | docker load; then
  echo "Failed to load the transferred image." >&2
  restore_active_env
  exit 1
fi
rm -f "$IMAGE_ARCHIVE"

if ! docker image inspect "${IMAGE_NAME}:${NEW_TAG}" >/dev/null 2>&1; then
  echo "The transferred archive did not contain ${IMAGE_NAME}:${NEW_TAG}." >&2
  restore_active_env
  exit 1
fi

wait_until_healthy() {
  local tag="$1"
  local container_id
  local health

  for _ in {1..40}; do
    container_id="$(IMAGE_TAG="$tag" docker compose --env-file "$ACTIVE_ENV" ps --all --quiet app)"
    if [[ -n "$container_id" ]]; then
      health="$(docker inspect --format '{{if .State.Health}}{{.State.Health.Status}}{{else}}{{.State.Status}}{{end}}' "$container_id" 2>/dev/null || true)"
      case "$health" in
        healthy)
          return 0
          ;;
        unhealthy|exited|dead)
          return 1
          ;;
      esac
    fi
    sleep 3
  done

  return 1
}

rollback() {
  if [[ -z "$previous_tag" || ! -f "$PREVIOUS_ENV" ]]; then
    echo "No previous release is available for automatic rollback." >&2
    return 1
  fi
  if ! docker image inspect "${IMAGE_NAME}:${previous_tag}" >/dev/null 2>&1; then
    echo "The previous image ${IMAGE_NAME}:${previous_tag} is missing." >&2
    return 1
  fi

  echo "Rolling back to ${IMAGE_NAME}:${previous_tag}..." >&2
  install -m 600 "$PREVIOUS_ENV" "${ACTIVE_ENV}.rollback"
  mv -f "${ACTIVE_ENV}.rollback" "$ACTIVE_ENV"

  if ! IMAGE_TAG="$previous_tag" docker compose --env-file "$ACTIVE_ENV" up -d --remove-orphans; then
    echo "Failed to start the previous release." >&2
    return 1
  fi

  if ! wait_until_healthy "$previous_tag"; then
    echo "The previous release did not become healthy." >&2
    return 1
  fi

  echo "Rollback completed." >&2
}

echo "Starting ${IMAGE_NAME}:${NEW_TAG}..."
if ! docker compose --env-file "$ACTIVE_ENV" up -d --remove-orphans; then
  rollback || true
  exit 1
fi

if ! wait_until_healthy "$NEW_TAG"; then
  echo "The new release did not become healthy." >&2
  IMAGE_TAG="$NEW_TAG" docker compose --env-file "$ACTIVE_ENV" logs --no-color --tail 100 app >&2 || true
  rollback || true
  exit 1
fi

if [[ -n "$previous_tag" && "$previous_tag" != "$NEW_TAG" ]]; then
  printf '%s\n' "$previous_tag" > "${PREVIOUS_RELEASE}.new"
  mv -f "${PREVIOUS_RELEASE}.new" "$PREVIOUS_RELEASE"
else
  rm -f "$PREVIOUS_RELEASE"
fi
printf '%s\n' "$NEW_TAG" > "${CURRENT_RELEASE}.new"
mv -f "${CURRENT_RELEASE}.new" "$CURRENT_RELEASE"

while read -r tag; do
  if [[ "$tag" =~ ^[0-9a-f]{40}$ && "$tag" != "$NEW_TAG" && "$tag" != "$previous_tag" ]]; then
    docker image rm "${IMAGE_NAME}:${tag}" >/dev/null 2>&1 || true
  fi
done < <(docker image ls "$IMAGE_NAME" --format '{{.Tag}}')

echo "Release ${NEW_TAG} is healthy."
