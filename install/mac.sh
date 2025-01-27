#!/bin/bash

DESTINATION="${HOME}/.local/bin"

mkdir -p "${DESTINATION}/todo-app"

cp target/release/todo-app "${DESTINATION}/todo-app/tap"
echo "DATABASE_URL=sqlite://${DESTINATION}/todo-app/database.db" > "${DESTINATION}/todo-app/.env"

ln -sf "${DESTINATION}/todo-app/tap" "${DESTINATION}/tap"

if [[ ":$PATH:" != *":$DESTINATION:"* ]]; then
  echo "export PATH=\"\$PATH:${DESTINATION}\"" >> "${HOME}/.bashrc"
  echo "Installation successful! Restart your terminal or run 'source ~/.bashrc' to use the CLI."
else
  echo "Installation successful! Try 'tap'."
fi