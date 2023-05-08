# Copied from mesa, big kudos
#
# https://gitlab.freedesktop.org/mesa/mesa/-/blob/master/.gitlab-ci/windows/mesa_container.ps1
# https://gitlab.freedesktop.org/mesa/mesa/-/blob/34e3e164936d1d3cef267da7780e87f062fedf39/.gitlab-ci/windows/mesa_container.ps1

# Implements the equivalent of ci-templates container-ifnot-exists, using
# Docker directly as we don't have buildah/podman/skopeo available under
# Windows, nor can we execute Docker-in-Docker
$registry_uri = $args[0]
$registry_username = $args[1]
$registry_password = $args[2]
$registry_user_image = $args[3]
$registry_central_image = $args[4]
$dockerfile = $args[5]

docker --config "windows-docker.conf" login -u "$registry_username" -p "$registry_password" "$registry_uri"
if (!$?) {
  Write-Host "docker login failed to $registry_uri"
  Exit 1
}

# if the image already exists, don't rebuild it
docker --config "windows-docker.conf" pull "$registry_user_image"
if ($?) {
  Write-Host "User image $registry_user_image already exists; not rebuilding"
  docker --config "windows-docker.conf" logout "$registry_uri"
  Exit 0
}

# if the image already exists upstream, copy it
docker --config "windows-docker.conf" pull "$registry_central_image"
if ($?) {
  Write-Host "Copying central image $registry_central_image to user image $registry_user_image"
  docker --config "windows-docker.conf" tag "$registry_central_image" "$registry_user_image"
  docker --config "windows-docker.conf" push "$registry_user_image"
  $pushstatus = $?
  docker --config "windows-docker.conf" logout "$registry_uri"
  if (!$pushstatus) {
    Write-Host "Pushing image to $registry_user_image failed"
    Exit 1
  }
  Exit 0
}

Write-Host "No image found at $registry_user_image or $registry_central_image; rebuilding"
docker --config "windows-docker.conf" build $DOCKER_BUILD_ARGS --no-cache -t "$registry_user_image" -f "$dockerfile" "./gitlab/docker/windows"
if (!$?) {
  Write-Host "Container build failed"
  docker --config "windows-docker.conf" logout "$registry_uri"
  Exit 1
}
Get-Date

docker --config "windows-docker.conf" push "$registry_user_image"
$pushstatus = $?
docker --config "windows-docker.conf" logout "$registry_uri"
if (!$pushstatus) {
  Write-Host "Pushing image to $registry_user_image failed"
  Exit 1
}
