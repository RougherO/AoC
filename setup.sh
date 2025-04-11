#!/bin/bash

# Usage: ./init_advent_project.sh Y2024

if [ $# -ne 1 ]; then
  echo "Usage: $0 <project-name>"
  exit 1
fi

PROJECT_NAME=$1

# Initialize new Cargo project
cargo init "$PROJECT_NAME" --vcs none
cd "$PROJECT_NAME" || exit 1

# Prepare directories and files
for day in $(seq -w 1 25); do
  for part in 1 2; do
    DIR="src/D${day}/P${part}/bin"
    mkdir -p "$DIR"
    echo "// Day $day Part $part" > "$DIR/main.rs"
    echo "" >> "$DIR/main.rs"
    echo "fn main() {" >> "$DIR/main.rs"
    echo "    println!(\"Day $day Part $part\");" >> "$DIR/main.rs"
    echo "}" >> "$DIR/main.rs"

    # Create empty input file
    touch "src/D${day}/P${part}/input.txt"

    # Add to Cargo.toml
    echo "" >> Cargo.toml
    echo "[[bin]]" >> Cargo.toml
    echo "name = \"D${day}P${part}\"" >> Cargo.toml
    echo "path = \"src/D${day}/P${part}/bin/main.rs\"" >> Cargo.toml
  done
done

echo "Project $PROJECT_NAME initialized with Day 1–25, Part 1 & 2 setup!"

